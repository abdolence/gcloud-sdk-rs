/// Request message for
/// \[AccountBudgetProposalService.MutateAccountBudgetProposal][google.ads.googleads.v10.services.AccountBudgetProposalService.MutateAccountBudgetProposal\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAccountBudgetProposalRequest {
    /// Required. The ID of the customer.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The operation to perform on an individual account-level budget proposal.
    #[prost(message, optional, tag = "2")]
    pub operation: ::core::option::Option<AccountBudgetProposalOperation>,
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
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "account_budget_proposal_operation::Operation", tags = "2, 1")]
    pub operation: ::core::option::Option<account_budget_proposal_operation::Operation>,
}
/// Nested message and enum types in `AccountBudgetProposalOperation`.
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
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for account-level budget mutate operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAccountBudgetProposalResponse {
    /// The result of the mutate.
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<MutateAccountBudgetProposalResult>,
}
/// The result for the account budget proposal mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAccountBudgetProposalResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod account_budget_proposal_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A service for managing account-level budgets via proposals."]
    #[doc = ""]
    #[doc = " A proposal is a request to create a new budget or make changes to an"]
    #[doc = " existing one."]
    #[doc = ""]
    #[doc = " Mutates:"]
    #[doc = " The CREATE operation creates a new proposal."]
    #[doc = " UPDATE operations aren't supported."]
    #[doc = " The REMOVE operation cancels a pending proposal."]
    #[derive(Debug, Clone)]
    pub struct AccountBudgetProposalServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AccountBudgetProposalServiceClient<T>
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
        ) -> AccountBudgetProposalServiceClient<InterceptedService<T, F>>
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
            AccountBudgetProposalServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes account budget proposals.  Operation statuses"]
        #[doc = " are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AccountBudgetProposalError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [StringLengthError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.AccountBudgetProposalService/MutateAccountBudgetProposal") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[AccountLinkService.CreateAccountLink][google.ads.googleads.v10.services.AccountLinkService.CreateAccountLink\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAccountLinkRequest {
    /// Required. The ID of the customer for which the account link is created.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The account link to be created.
    #[prost(message, optional, tag = "2")]
    pub account_link: ::core::option::Option<super::resources::AccountLink>,
}
/// Response message for
/// \[AccountLinkService.CreateAccountLink][google.ads.googleads.v10.services.AccountLinkService.CreateAccountLink\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAccountLinkResponse {
    /// Returned for successful operations. Resource name of the account link.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// Request message for \[AccountLinkService.MutateAccountLink][google.ads.googleads.v10.services.AccountLinkService.MutateAccountLink\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAccountLinkRequest {
    /// Required. The ID of the customer being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The operation to perform on the link.
    #[prost(message, optional, tag = "2")]
    pub operation: ::core::option::Option<AccountLinkOperation>,
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
/// A single update on an account link.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLinkOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The operation to perform.
    #[prost(oneof = "account_link_operation::Operation", tags = "2, 3")]
    pub operation: ::core::option::Option<account_link_operation::Operation>,
}
/// Nested message and enum types in `AccountLinkOperation`.
pub mod account_link_operation {
    /// The operation to perform.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Update operation: The account link is expected to have
        /// a valid resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::AccountLink),
        /// Remove operation: A resource name for the account link to remove is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/accountLinks/{account_link_id}`
        #[prost(string, tag = "3")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for account link mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAccountLinkResponse {
    /// Result for the mutate.
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<MutateAccountLinkResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The result for the account link mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAccountLinkResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod account_link_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " This service allows management of links between Google Ads accounts and other"]
    #[doc = " accounts."]
    #[derive(Debug, Clone)]
    pub struct AccountLinkServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AccountLinkServiceClient<T>
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
        ) -> AccountLinkServiceClient<InterceptedService<T, F>>
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
            AccountLinkServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates an account link."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ThirdPartyAppAnalyticsLinkError]()"]
        pub async fn create_account_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAccountLinkRequest>,
        ) -> Result<tonic::Response<super::CreateAccountLinkResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.AccountLinkService/CreateAccountLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates or removes an account link."]
        #[doc = " From V5, create is not supported through"]
        #[doc = " AccountLinkService.MutateAccountLink. Please use"]
        #[doc = " AccountLinkService.CreateAccountLink instead."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AccountLinkError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn mutate_account_link(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAccountLinkRequest>,
        ) -> Result<tonic::Response<super::MutateAccountLinkResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.AccountLinkService/MutateAccountLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[AdGroupAdLabelService.MutateAdGroupAdLabels][google.ads.googleads.v10.services.AdGroupAdLabelService.MutateAdGroupAdLabels\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupAdLabelsRequest {
    /// Required. ID of the customer whose ad group ad labels are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on ad group ad labels.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AdGroupAdLabelOperation>,
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
    pub operation: ::core::option::Option<ad_group_ad_label_operation::Operation>,
}
/// Nested message and enum types in `AdGroupAdLabelOperation`.
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
        /// `customers/{customer_id}/adGroupAdLabels/{ad_group_id}~{ad_id}_{label_id}`
        #[prost(string, tag = "2")]
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateAdGroupAdLabelResult>,
}
/// The result for an ad group ad label mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupAdLabelResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_ad_label_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage labels on ad group ads."]
    #[derive(Debug, Clone)]
    pub struct AdGroupAdLabelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupAdLabelServiceClient<T>
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
        ) -> AdGroupAdLabelServiceClient<InterceptedService<T, F>>
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
            AdGroupAdLabelServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates and removes ad group ad labels."]
        #[doc = " Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [LabelError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.AdGroupAdLabelService/MutateAdGroupAdLabels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[AdGroupAdService.MutateAdGroupAds][google.ads.googleads.v10.services.AdGroupAdService.MutateAdGroupAds\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupAdsRequest {
    /// Required. The ID of the customer whose ads are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual ads.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AdGroupAdOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on an ad group ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAdOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Configuration for how policies are validated.
    #[prost(message, optional, tag = "5")]
    pub policy_validation_parameter:
        ::core::option::Option<super::common::PolicyValidationParameter>,
    /// The mutate operation.
    #[prost(oneof = "ad_group_ad_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<ad_group_ad_operation::Operation>,
}
/// Nested message and enum types in `AdGroupAdOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateAdGroupAdResult>,
}
/// The result for the ad mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupAdResult {
    /// The resource name returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated ad group ad with only mutable fields after mutate. The field
    /// will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub ad_group_ad: ::core::option::Option<super::resources::AdGroupAd>,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_ad_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage ads in an ad group."]
    #[derive(Debug, Clone)]
    pub struct AdGroupAdServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupAdServiceClient<T>
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
        ) -> AdGroupAdServiceClient<InterceptedService<T, F>>
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
            AdGroupAdServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes ads. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AdCustomizerError]()"]
        #[doc = "   [AdError]()"]
        #[doc = "   [AdGroupAdError]()"]
        #[doc = "   [AdSharingError]()"]
        #[doc = "   [AdxError]()"]
        #[doc = "   [AssetError]()"]
        #[doc = "   [AssetLinkError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [ContextError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FeedAttributeReferenceError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [FunctionError]()"]
        #[doc = "   [FunctionParsingError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [ImageError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [ListOperationError]()"]
        #[doc = "   [MediaBundleError]()"]
        #[doc = "   [MediaFileError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperationAccessDeniedError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [PolicyFindingError]()"]
        #[doc = "   [PolicyValidationParameterError]()"]
        #[doc = "   [PolicyViolationError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
        #[doc = "   [UrlFieldError]()"]
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
                "/google.ads.googleads.v10.services.AdGroupAdService/MutateAdGroupAds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[AdGroupAssetService.MutateAdGroupAssets][google.ads.googleads.v10.services.AdGroupAssetService.MutateAdGroupAssets\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupAssetsRequest {
    /// Required. The ID of the customer whose ad group assets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual ad group assets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AdGroupAssetOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on an ad group asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAssetOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "ad_group_asset_operation::Operation", tags = "1, 3, 2")]
    pub operation: ::core::option::Option<ad_group_asset_operation::Operation>,
}
/// Nested message and enum types in `AdGroupAssetOperation`.
pub mod ad_group_asset_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new ad group
        /// asset.
        #[prost(message, tag = "1")]
        Create(super::super::resources::AdGroupAsset),
        /// Update operation: The ad group asset is expected to have a valid resource
        /// name.
        #[prost(message, tag = "3")]
        Update(super::super::resources::AdGroupAsset),
        /// Remove operation: A resource name for the removed ad group asset is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/adGroupAssets/{ad_group_id}~{asset_id}~{field_type}`
        #[prost(string, tag = "2")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for an ad group asset mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupAssetsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "1")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateAdGroupAssetResult>,
}
/// The result for the ad group asset mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupAssetResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated ad group asset with only mutable fields after
    /// mutate. The field will only be returned when response_content_type is set
    /// to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub ad_group_asset: ::core::option::Option<super::resources::AdGroupAsset>,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_asset_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage ad group assets."]
    #[derive(Debug, Clone)]
    pub struct AdGroupAssetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupAssetServiceClient<T>
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
        ) -> AdGroupAssetServiceClient<InterceptedService<T, F>>
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
            AdGroupAssetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes ad group assets. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AssetLinkError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [ContextError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NotAllowlistedError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn mutate_ad_group_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAdGroupAssetsRequest>,
        ) -> Result<tonic::Response<super::MutateAdGroupAssetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.AdGroupAssetService/MutateAdGroupAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[AdGroupBidModifierService.MutateAdGroupBidModifiers][google.ads.googleads.v10.services.AdGroupBidModifierService.MutateAdGroupBidModifiers\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupBidModifiersRequest {
    /// Required. ID of the customer whose ad group bid modifiers are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual ad group bid modifiers.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AdGroupBidModifierOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove, update) on an ad group bid modifier.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupBidModifierOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "ad_group_bid_modifier_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<ad_group_bid_modifier_operation::Operation>,
}
/// Nested message and enum types in `AdGroupBidModifierOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateAdGroupBidModifierResult>,
}
/// The result for the criterion mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupBidModifierResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated ad group bid modifier with only mutable fields after mutate.
    /// The field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub ad_group_bid_modifier: ::core::option::Option<super::resources::AdGroupBidModifier>,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_bid_modifier_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage ad group bid modifiers."]
    #[derive(Debug, Clone)]
    pub struct AdGroupBidModifierServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupBidModifierServiceClient<T>
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
        ) -> AdGroupBidModifierServiceClient<InterceptedService<T, F>>
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
            AdGroupBidModifierServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes ad group bid modifiers."]
        #[doc = " Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AdGroupBidModifierError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [ContextError]()"]
        #[doc = "   [CriterionError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.AdGroupBidModifierService/MutateAdGroupBidModifiers") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[AdGroupCriterionCustomizerService.MutateAdGroupCriterionCustomizers][google.ads.googleads.v10.services.AdGroupCriterionCustomizerService.MutateAdGroupCriterionCustomizers\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupCriterionCustomizersRequest {
    /// Required. The ID of the customer whose ad group criterion customizers are being
    /// modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual ad group criterion
    /// customizers.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AdGroupCriterionCustomizerOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove) on an customizer attribute.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCriterionCustomizerOperation {
    /// The mutate operation.
    #[prost(
        oneof = "ad_group_criterion_customizer_operation::Operation",
        tags = "1, 2"
    )]
    pub operation: ::core::option::Option<ad_group_criterion_customizer_operation::Operation>,
}
/// Nested message and enum types in `AdGroupCriterionCustomizerOperation`.
pub mod ad_group_criterion_customizer_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new ad group
        /// criterion customizer.
        #[prost(message, tag = "1")]
        Create(super::super::resources::AdGroupCriterionCustomizer),
        /// Remove operation: A resource name for the removed ad group criterion
        /// customizer is expected, in this format:
        ///
        /// `customers/{customer_id}/adGroupCriterionCustomizers/{ad_group_id}~{criterion_id}~{customizer_attribute_id}`
        #[prost(string, tag = "2")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for an ad group criterion customizer mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupCriterionCustomizersResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateAdGroupCriterionCustomizerResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The result for the ad group criterion customizer mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupCriterionCustomizerResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated AdGroupCriterionCustomizer with only mutable fields after
    /// mutate. The field will only be returned when response_content_type is set
    /// to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub ad_group_criterion_customizer:
        ::core::option::Option<super::resources::AdGroupCriterionCustomizer>,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_criterion_customizer_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage ad group criterion customizer"]
    #[derive(Debug, Clone)]
    pub struct AdGroupCriterionCustomizerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupCriterionCustomizerServiceClient<T>
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
        ) -> AdGroupCriterionCustomizerServiceClient<InterceptedService<T, F>>
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
            AdGroupCriterionCustomizerServiceClient::new(InterceptedService::new(
                inner,
                interceptor,
            ))
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
        #[doc = " Creates, updates or removes ad group criterion customizers. Operation"]
        #[doc = " statuses are returned."]
        pub async fn mutate_ad_group_criterion_customizers(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAdGroupCriterionCustomizersRequest>,
        ) -> Result<tonic::Response<super::MutateAdGroupCriterionCustomizersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.AdGroupCriterionCustomizerService/MutateAdGroupCriterionCustomizers") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[AdGroupCriterionLabelService.MutateAdGroupCriterionLabels][google.ads.googleads.v10.services.AdGroupCriterionLabelService.MutateAdGroupCriterionLabels\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupCriterionLabelsRequest {
    /// Required. ID of the customer whose ad group criterion labels are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on ad group criterion labels.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AdGroupCriterionLabelOperation>,
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
    pub operation: ::core::option::Option<ad_group_criterion_label_operation::Operation>,
}
/// Nested message and enum types in `AdGroupCriterionLabelOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateAdGroupCriterionLabelResult>,
}
/// The result for an ad group criterion label mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupCriterionLabelResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_criterion_label_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage labels on ad group criteria."]
    #[derive(Debug, Clone)]
    pub struct AdGroupCriterionLabelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupCriterionLabelServiceClient<T>
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
        ) -> AdGroupCriterionLabelServiceClient<InterceptedService<T, F>>
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
            AdGroupCriterionLabelServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates and removes ad group criterion labels."]
        #[doc = " Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.AdGroupCriterionLabelService/MutateAdGroupCriterionLabels") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[AdGroupCriterionService.MutateAdGroupCriteria][google.ads.googleads.v10.services.AdGroupCriterionService.MutateAdGroupCriteria\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupCriteriaRequest {
    /// Required. ID of the customer whose criteria are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual criteria.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AdGroupCriterionOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove, update) on an ad group criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCriterionOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
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
    pub exempt_policy_violation_keys: ::prost::alloc::vec::Vec<super::common::PolicyViolationKey>,
    /// The mutate operation.
    #[prost(oneof = "ad_group_criterion_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<ad_group_criterion_operation::Operation>,
}
/// Nested message and enum types in `AdGroupCriterionOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateAdGroupCriterionResult>,
}
/// The result for the criterion mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupCriterionResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated ad group criterion with only mutable fields after mutate. The
    /// field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub ad_group_criterion: ::core::option::Option<super::resources::AdGroupCriterion>,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_criterion_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage ad group criteria."]
    #[derive(Debug, Clone)]
    pub struct AdGroupCriterionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupCriterionServiceClient<T>
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
        ) -> AdGroupCriterionServiceClient<InterceptedService<T, F>>
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
            AdGroupCriterionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes criteria. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AdGroupCriterionError]()"]
        #[doc = "   [AdxError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [BiddingError]()"]
        #[doc = "   [BiddingStrategyError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [ContextError]()"]
        #[doc = "   [CriterionError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MultiplierError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperationAccessDeniedError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [PolicyViolationError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
        #[doc = "   [UrlFieldError]()"]
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
                "/google.ads.googleads.v10.services.AdGroupCriterionService/MutateAdGroupCriteria",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[AdGroupCustomizerService.MutateAdGroupCustomizers][google.ads.googleads.v10.services.AdGroupCustomizerService.MutateAdGroupCustomizers\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupCustomizersRequest {
    /// Required. The ID of the customer whose ad group customizers are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual ad group customizers.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AdGroupCustomizerOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove) on an customizer attribute.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCustomizerOperation {
    /// The mutate operation.
    #[prost(oneof = "ad_group_customizer_operation::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<ad_group_customizer_operation::Operation>,
}
/// Nested message and enum types in `AdGroupCustomizerOperation`.
pub mod ad_group_customizer_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new ad group
        /// customizer
        #[prost(message, tag = "1")]
        Create(super::super::resources::AdGroupCustomizer),
        /// Remove operation: A resource name for the removed ad group customizer is
        /// expected, in this format:
        /// `customers/{customer_id}/adGroupCustomizers/{ad_group_id}~{customizer_attribute_id}`
        #[prost(string, tag = "2")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for an ad group customizer mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupCustomizersResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateAdGroupCustomizerResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The result for the ad group customizer mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupCustomizerResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated AdGroupCustomizer with only mutable fields after mutate.
    /// The field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub ad_group_customizer: ::core::option::Option<super::resources::AdGroupCustomizer>,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_customizer_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage ad group customizer"]
    #[derive(Debug, Clone)]
    pub struct AdGroupCustomizerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupCustomizerServiceClient<T>
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
        ) -> AdGroupCustomizerServiceClient<InterceptedService<T, F>>
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
            AdGroupCustomizerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates or removes ad group customizers. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_ad_group_customizers(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAdGroupCustomizersRequest>,
        ) -> Result<tonic::Response<super::MutateAdGroupCustomizersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.AdGroupCustomizerService/MutateAdGroupCustomizers") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[AdGroupExtensionSettingService.MutateAdGroupExtensionSettings][google.ads.googleads.v10.services.AdGroupExtensionSettingService.MutateAdGroupExtensionSettings\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupExtensionSettingsRequest {
    /// Required. The ID of the customer whose ad group extension settings are being
    /// modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual ad group extension
    /// settings.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AdGroupExtensionSettingOperation>,
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
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
    /// The mutate operation.
    #[prost(
        oneof = "ad_group_extension_setting_operation::Operation",
        tags = "1, 2, 3"
    )]
    pub operation: ::core::option::Option<ad_group_extension_setting_operation::Operation>,
}
/// Nested message and enum types in `AdGroupExtensionSettingOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateAdGroupExtensionSettingResult>,
}
/// The result for the ad group extension setting mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupExtensionSettingResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated AdGroupExtensionSetting with only mutable fields after mutate.
    /// The field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub ad_group_extension_setting:
        ::core::option::Option<super::resources::AdGroupExtensionSetting>,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_extension_setting_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage ad group extension settings."]
    #[derive(Debug, Clone)]
    pub struct AdGroupExtensionSettingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupExtensionSettingServiceClient<T>
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
        ) -> AdGroupExtensionSettingServiceClient<InterceptedService<T, F>>
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
            AdGroupExtensionSettingServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes ad group extension settings. Operation"]
        #[doc = " statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [CriterionError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [ExtensionSettingError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [ListOperationError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperationAccessDeniedError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
        #[doc = "   [UrlFieldError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.AdGroupExtensionSettingService/MutateAdGroupExtensionSettings") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[AdGroupFeedService.MutateAdGroupFeeds][google.ads.googleads.v10.services.AdGroupFeedService.MutateAdGroupFeeds\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupFeedsRequest {
    /// Required. The ID of the customer whose ad group feeds are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual ad group feeds.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AdGroupFeedOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on an ad group feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupFeedOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "ad_group_feed_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<ad_group_feed_operation::Operation>,
}
/// Nested message and enum types in `AdGroupFeedOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateAdGroupFeedResult>,
}
/// The result for the ad group feed mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupFeedResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated ad group feed with only mutable fields after mutate. The field
    /// will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub ad_group_feed: ::core::option::Option<super::resources::AdGroupFeed>,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_feed_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage ad group feeds."]
    #[derive(Debug, Clone)]
    pub struct AdGroupFeedServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupFeedServiceClient<T>
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
        ) -> AdGroupFeedServiceClient<InterceptedService<T, F>>
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
            AdGroupFeedServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes ad group feeds. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AdGroupFeedError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FunctionError]()"]
        #[doc = "   [FunctionParsingError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
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
                "/google.ads.googleads.v10.services.AdGroupFeedService/MutateAdGroupFeeds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[AdGroupLabelService.MutateAdGroupLabels][google.ads.googleads.v10.services.AdGroupLabelService.MutateAdGroupLabels\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupLabelsRequest {
    /// Required. ID of the customer whose ad group labels are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on ad group labels.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AdGroupLabelOperation>,
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
    pub operation: ::core::option::Option<ad_group_label_operation::Operation>,
}
/// Nested message and enum types in `AdGroupLabelOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateAdGroupLabelResult>,
}
/// The result for an ad group label mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupLabelResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_label_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage labels on ad groups."]
    #[derive(Debug, Clone)]
    pub struct AdGroupLabelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupLabelServiceClient<T>
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
        ) -> AdGroupLabelServiceClient<InterceptedService<T, F>>
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
            AdGroupLabelServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates and removes ad group labels."]
        #[doc = " Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [LabelError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.AdGroupLabelService/MutateAdGroupLabels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[AdGroupService.MutateAdGroups][google.ads.googleads.v10.services.AdGroupService.MutateAdGroups\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupsRequest {
    /// Required. The ID of the customer whose ad groups are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual ad groups.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AdGroupOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on an ad group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "ad_group_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<ad_group_operation::Operation>,
}
/// Nested message and enum types in `AdGroupOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateAdGroupResult>,
}
/// The result for the ad group mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated ad group with only mutable fields after mutate. The field will
    /// only be returned when response_content_type is set to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub ad_group: ::core::option::Option<super::resources::AdGroup>,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage ad groups."]
    #[derive(Debug, Clone)]
    pub struct AdGroupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupServiceClient<T>
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
        ) -> AdGroupServiceClient<InterceptedService<T, F>>
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
            AdGroupServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes ad groups. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AdGroupError]()"]
        #[doc = "   [AdxError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [BiddingError]()"]
        #[doc = "   [BiddingStrategyError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [ListOperationError]()"]
        #[doc = "   [MultiplierError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperationAccessDeniedError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        #[doc = "   [SettingError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
        #[doc = "   [UrlFieldError]()"]
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
                "/google.ads.googleads.v10.services.AdGroupService/MutateAdGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[AdParameterService.MutateAdParameters][google.ads.googleads.v10.services.AdParameterService.MutateAdParameters\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdParametersRequest {
    /// Required. The ID of the customer whose ad parameters are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual ad parameters.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AdParameterOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on ad parameter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdParameterOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "ad_parameter_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<ad_parameter_operation::Operation>,
}
/// Nested message and enum types in `AdParameterOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateAdParameterResult>,
}
/// The result for the ad parameter mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdParameterResult {
    /// The resource name returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated AdParameter with only mutable fields after mutate. The field
    /// will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub ad_parameter: ::core::option::Option<super::resources::AdParameter>,
}
#[doc = r" Generated client implementations."]
pub mod ad_parameter_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage ad parameters."]
    #[derive(Debug, Clone)]
    pub struct AdParameterServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdParameterServiceClient<T>
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
        ) -> AdParameterServiceClient<InterceptedService<T, F>>
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
            AdParameterServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes ad parameters. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AdParameterError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [ContextError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.AdParameterService/MutateAdParameters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[AdService.GetAd][google.ads.googleads.v10.services.AdService.GetAd\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdRequest {
    /// Required. The resource name of the ad to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// Request message for \[AdService.MutateAds][google.ads.googleads.v10.services.AdService.MutateAds\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdsRequest {
    /// Required. The ID of the customer whose ads are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual ads.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AdOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "4")]
    pub partial_failure: bool,
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// A single update operation on an ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Configuration for how policies are validated.
    #[prost(message, optional, tag = "3")]
    pub policy_validation_parameter:
        ::core::option::Option<super::common::PolicyValidationParameter>,
    /// The mutate operation.
    #[prost(oneof = "ad_operation::Operation", tags = "1")]
    pub operation: ::core::option::Option<ad_operation::Operation>,
}
/// Nested message and enum types in `AdOperation`.
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
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateAdResult>,
}
/// The result for the ad mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdResult {
    /// The resource name returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated ad with only mutable fields after mutate. The field will only
    /// be returned when response_content_type is set to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub ad: ::core::option::Option<super::resources::Ad>,
}
#[doc = r" Generated client implementations."]
pub mod ad_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage ads."]
    #[derive(Debug, Clone)]
    pub struct AdServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdServiceClient<T>
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
        ) -> AdServiceClient<InterceptedService<T, F>>
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
            AdServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns the requested ad in full detail."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.AdService/GetAd",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates ads. Operation statuses are returned. Updating ads is not supported"]
        #[doc = " for TextAd, ExpandedDynamicSearchAd, GmailAd and ImageAd."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AdCustomizerError]()"]
        #[doc = "   [AdError]()"]
        #[doc = "   [AdSharingError]()"]
        #[doc = "   [AdxError]()"]
        #[doc = "   [AssetError]()"]
        #[doc = "   [AssetLinkError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FeedAttributeReferenceError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [FunctionError]()"]
        #[doc = "   [FunctionParsingError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [ImageError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [ListOperationError]()"]
        #[doc = "   [MediaBundleError]()"]
        #[doc = "   [MediaFileError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [PolicyFindingError]()"]
        #[doc = "   [PolicyViolationError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
        #[doc = "   [UrlFieldError]()"]
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
                "/google.ads.googleads.v10.services.AdService/MutateAds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[AssetGroupAssetService.MutateAssetGroupAssets][google.ads.googleads.v10.services.AssetGroupAssetService.MutateAssetGroupAssets\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetGroupAssetsRequest {
    /// Required. The ID of the customer whose asset group assets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual asset group assets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AssetGroupAssetOperation>,
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
/// A single operation (create, remove) on an asset group asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetGroupAssetOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "asset_group_asset_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<asset_group_asset_operation::Operation>,
}
/// Nested message and enum types in `AssetGroupAssetOperation`.
pub mod asset_group_asset_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new asset group
        /// asset
        #[prost(message, tag = "1")]
        Create(super::super::resources::AssetGroupAsset),
        /// Update operation: The asset group asset sis expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::AssetGroupAsset),
        /// Remove operation: A resource name for the removed asset group asset is
        /// expected, in this format:
        /// `customers/{customer_id}/assetGroupAssets/{asset_group_id}~{asset_id}~{field_type}`
        #[prost(string, tag = "3")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for an asset group asset mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetGroupAssetsResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateAssetGroupAssetResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The result for the asset group asset mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetGroupAssetResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod asset_group_asset_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage asset group asset."]
    #[derive(Debug, Clone)]
    pub struct AssetGroupAssetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AssetGroupAssetServiceClient<T>
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
        ) -> AssetGroupAssetServiceClient<InterceptedService<T, F>>
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
            AssetGroupAssetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates or removes asset group assets. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_asset_group_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAssetGroupAssetsRequest>,
        ) -> Result<tonic::Response<super::MutateAssetGroupAssetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.AssetGroupAssetService/MutateAssetGroupAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[AssetGroupListingGroupFilterService.MutateAssetGroupListingGroupFilters][google.ads.googleads.v10.services.AssetGroupListingGroupFilterService.MutateAssetGroupListingGroupFilters\].
/// partial_failure is not supported because the tree needs to be validated
/// together.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetGroupListingGroupFiltersRequest {
    /// Required. The ID of the customer whose asset group listing group filters are being
    /// modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual asset group listing group
    /// filters.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AssetGroupListingGroupFilterOperation>,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "4"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove) on an asset group listing group filter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetGroupListingGroupFilterOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(
        oneof = "asset_group_listing_group_filter_operation::Operation",
        tags = "1, 2, 3"
    )]
    pub operation: ::core::option::Option<asset_group_listing_group_filter_operation::Operation>,
}
/// Nested message and enum types in `AssetGroupListingGroupFilterOperation`.
pub mod asset_group_listing_group_filter_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new asset group
        /// listing group filter.
        #[prost(message, tag = "1")]
        Create(super::super::resources::AssetGroupListingGroupFilter),
        /// Update operation: The asset group listing group filter is expected to
        /// have a valid resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::AssetGroupListingGroupFilter),
        /// Remove operation: A resource name for the removed asset group listing
        /// group filter is expected, in this format:
        /// `customers/{customer_id}/assetGroupListingGroupFilters/{asset_group_id}~{listing_group_filter_id}`
        /// An entity can be removed only if it's not referenced by other
        /// parent_listing_group_id. If multiple entities are being deleted, the
        /// mutates must be in the correct order.
        #[prost(string, tag = "3")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for an asset group listing group filter mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetGroupListingGroupFiltersResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateAssetGroupListingGroupFilterResult>,
}
/// The result for the asset group listing group filter mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetGroupListingGroupFilterResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated AssetGroupListingGroupFilter with only mutable fields after
    /// mutate. The field will only be returned when response_content_type is set
    /// to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub asset_group_listing_group_filter:
        ::core::option::Option<super::resources::AssetGroupListingGroupFilter>,
}
#[doc = r" Generated client implementations."]
pub mod asset_group_listing_group_filter_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage asset group listing group filter."]
    #[derive(Debug, Clone)]
    pub struct AssetGroupListingGroupFilterServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AssetGroupListingGroupFilterServiceClient<T>
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
        ) -> AssetGroupListingGroupFilterServiceClient<InterceptedService<T, F>>
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
            AssetGroupListingGroupFilterServiceClient::new(InterceptedService::new(
                inner,
                interceptor,
            ))
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
        #[doc = " Creates, updates or removes asset group listing group filters. Operation"]
        #[doc = " statuses are returned."]
        pub async fn mutate_asset_group_listing_group_filters(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAssetGroupListingGroupFiltersRequest>,
        ) -> Result<
            tonic::Response<super::MutateAssetGroupListingGroupFiltersResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.AssetGroupListingGroupFilterService/MutateAssetGroupListingGroupFilters") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[AssetGroupService.MutateAssetGroups][google.ads.googleads.v10.services.AssetGroupService.MutateAssetGroups\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetGroupsRequest {
    /// Required. The ID of the customer whose asset groups are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual asset groups.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AssetGroupOperation>,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, remove) on an asset group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetGroupOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "asset_group_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<asset_group_operation::Operation>,
}
/// Nested message and enum types in `AssetGroupOperation`.
pub mod asset_group_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new asset group
        #[prost(message, tag = "1")]
        Create(super::super::resources::AssetGroup),
        /// Update operation: The asset group is expected to have a valid resource
        /// name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::AssetGroup),
        /// Remove operation: A resource name for the removed asset group is
        /// expected, in this format:
        /// `customers/{customer_id}/assetGroups/{asset_group_id}`
        #[prost(string, tag = "3")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for an asset group mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetGroupsResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateAssetGroupResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The result for the asset group mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetGroupResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod asset_group_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage asset group"]
    #[derive(Debug, Clone)]
    pub struct AssetGroupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AssetGroupServiceClient<T>
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
        ) -> AssetGroupServiceClient<InterceptedService<T, F>>
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
            AssetGroupServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates or removes asset groups. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_asset_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAssetGroupsRequest>,
        ) -> Result<tonic::Response<super::MutateAssetGroupsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.AssetGroupService/MutateAssetGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[AssetGroupSignalService.MutateAssetGroupSignals][google.ads.googleads.v10.services.AssetGroupSignalService.MutateAssetGroupSignals\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetGroupSignalsRequest {
    /// Required. The ID of the customer whose asset group signals are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual asset group signals.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AssetGroupSignalOperation>,
    /// If true, successful operations will be carried out and invalid operations
    /// will return errors. If false, all operations will be carried out in one
    /// transaction if and only if they are all valid. Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove) on an asset group signal.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetGroupSignalOperation {
    /// The mutate operation. Update is not supported.
    #[prost(oneof = "asset_group_signal_operation::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<asset_group_signal_operation::Operation>,
}
/// Nested message and enum types in `AssetGroupSignalOperation`.
pub mod asset_group_signal_operation {
    /// The mutate operation. Update is not supported.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new asset group
        /// signal.
        #[prost(message, tag = "1")]
        Create(super::super::resources::AssetGroupSignal),
        /// Remove operation: A resource name for the removed asset group signal is
        /// expected, in this format:
        /// `customers/{customer_id}/assetGroupSignals/{asset_group_id}~{criterion_id}`
        #[prost(string, tag = "2")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for an asset group signal mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetGroupSignalsResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateAssetGroupSignalResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The result for the asset group signal mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetGroupSignalResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated AssetGroupSignal with only mutable fields after mutate. The
    /// field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub asset_group_signal: ::core::option::Option<super::resources::AssetGroupSignal>,
}
#[doc = r" Generated client implementations."]
pub mod asset_group_signal_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage asset group signal."]
    #[derive(Debug, Clone)]
    pub struct AssetGroupSignalServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AssetGroupSignalServiceClient<T>
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
        ) -> AssetGroupSignalServiceClient<InterceptedService<T, F>>
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
            AssetGroupSignalServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates or removes asset group signals. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_asset_group_signals(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAssetGroupSignalsRequest>,
        ) -> Result<tonic::Response<super::MutateAssetGroupSignalsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.AssetGroupSignalService/MutateAssetGroupSignals") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[AssetService.MutateAssets][google.ads.googleads.v10.services.AssetService.MutateAssets\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetsRequest {
    /// Required. The ID of the customer whose assets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual assets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AssetOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "5")]
    pub partial_failure: bool,
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "3"
    )]
    pub response_content_type: i32,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation to create an asset. Supported asset types are
/// YoutubeVideoAsset, MediaBundleAsset, ImageAsset, and LeadFormAsset. TextAsset
/// should be created with Ad inline.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "asset_operation::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<asset_operation::Operation>,
}
/// Nested message and enum types in `AssetOperation`.
pub mod asset_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new asset.
        #[prost(message, tag = "1")]
        Create(super::super::resources::Asset),
        /// Update operation: The asset is expected to have a valid resource name in
        /// this format:
        ///
        /// `customers/{customer_id}/assets/{asset_id}`
        #[prost(message, tag = "2")]
        Update(super::super::resources::Asset),
    }
}
/// Response message for an asset mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateAssetResult>,
}
/// The result for the asset mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetResult {
    /// The resource name returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated asset with only mutable fields after mutate. The field will
    /// only be returned when response_content_type is set to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub asset: ::core::option::Option<super::resources::Asset>,
}
#[doc = r" Generated client implementations."]
pub mod asset_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage assets. Asset types can be created with AssetService are"]
    #[doc = " YoutubeVideoAsset, MediaBundleAsset and ImageAsset. TextAsset should be"]
    #[doc = " created with Ad inline."]
    #[derive(Debug, Clone)]
    pub struct AssetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AssetServiceClient<T>
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
        ) -> AssetServiceClient<InterceptedService<T, F>>
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
            AssetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates assets. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AssetError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [CurrencyCodeError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [ListOperationError]()"]
        #[doc = "   [MediaUploadError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NotAllowlistedError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
        #[doc = "   [UrlFieldError]()"]
        #[doc = "   [YoutubeVideoRegistrationError]()"]
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
                "/google.ads.googleads.v10.services.AssetService/MutateAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[AssetSetAssetService.MutateAssetSetAssets][google.ads.googleads.v10.services.AssetSetAssetService.MutateAssetSetAssets\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetSetAssetsRequest {
    /// Required. The ID of the customer whose asset set assets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual asset set assets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AssetSetAssetOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove) on an asset set asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSetAssetOperation {
    /// The mutate operation.
    #[prost(oneof = "asset_set_asset_operation::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<asset_set_asset_operation::Operation>,
}
/// Nested message and enum types in `AssetSetAssetOperation`.
pub mod asset_set_asset_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new asset set
        /// asset
        #[prost(message, tag = "1")]
        Create(super::super::resources::AssetSetAsset),
        /// Remove operation: A resource name for the removed asset set asset is
        /// expected, in this format:
        /// `customers/{customer_id}/assetSetAssets/{asset_set_id}~{asset_id}`
        #[prost(string, tag = "2")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for an asset set asset mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetSetAssetsResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateAssetSetAssetResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The result for the asset set asset mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetSetAssetResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated asset set asset with only mutable fields after mutate. The
    /// field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub asset_set_asset: ::core::option::Option<super::resources::AssetSetAsset>,
}
#[doc = r" Generated client implementations."]
pub mod asset_set_asset_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage asset set asset."]
    #[derive(Debug, Clone)]
    pub struct AssetSetAssetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AssetSetAssetServiceClient<T>
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
        ) -> AssetSetAssetServiceClient<InterceptedService<T, F>>
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
            AssetSetAssetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates or removes asset set assets. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_asset_set_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAssetSetAssetsRequest>,
        ) -> Result<tonic::Response<super::MutateAssetSetAssetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.AssetSetAssetService/MutateAssetSetAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[AssetSetService.MutateAssetSets][google.ads.googleads.v10.services.AssetSetService.MutateAssetSets\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetSetsRequest {
    /// Required. The ID of the customer whose asset sets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual asset sets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AssetSetOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove) on an asset set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSetOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "asset_set_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<asset_set_operation::Operation>,
}
/// Nested message and enum types in `AssetSetOperation`.
pub mod asset_set_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new asset set
        #[prost(message, tag = "1")]
        Create(super::super::resources::AssetSet),
        /// Update operation: The asset set is expected to have a valid resource
        /// name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::AssetSet),
        /// Remove operation: A resource name for the removed asset set is
        /// expected, in this format:
        /// `customers/{customer_id}/assetSets/{asset_set_id}`
        #[prost(string, tag = "3")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for an asset set mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetSetsResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateAssetSetResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The result for the asset set mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetSetResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated asset set with only mutable fields after mutate. The field will
    /// only be returned when response_content_type is set to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub asset_set: ::core::option::Option<super::resources::AssetSet>,
}
#[doc = r" Generated client implementations."]
pub mod asset_set_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage asset set"]
    #[derive(Debug, Clone)]
    pub struct AssetSetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AssetSetServiceClient<T>
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
        ) -> AssetSetServiceClient<InterceptedService<T, F>>
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
            AssetSetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates or removes asset sets. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_asset_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAssetSetsRequest>,
        ) -> Result<tonic::Response<super::MutateAssetSetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.AssetSetService/MutateAssetSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[AudienceService.MutateAudiences][google.ads.googleads.v10.services.AudienceService.MutateAudiences\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAudiencesRequest {
    /// Required. The ID of the customer whose audiences are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual audiences.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<AudienceOperation>,
    /// If true, successful operations will be carried out and invalid operations
    /// will return errors. If false, all operations will be carried out in one
    /// transaction if and only if they are all valid. Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// Response message for an audience mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAudiencesResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateAudienceResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// A single operation (create, update) on an audience.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "audience_operation::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<audience_operation::Operation>,
}
/// Nested message and enum types in `AudienceOperation`.
pub mod audience_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new audience
        #[prost(message, tag = "1")]
        Create(super::super::resources::Audience),
        /// Update operation: The audience is expected to have a valid resource
        /// name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::Audience),
    }
}
/// The result for the audience mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAudienceResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated Audience with only mutable fields after mutate. The
    /// field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub audience: ::core::option::Option<super::resources::Audience>,
}
#[doc = r" Generated client implementations."]
pub mod audience_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage audiences."]
    #[derive(Debug, Clone)]
    pub struct AudienceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AudienceServiceClient<T>
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
        ) -> AudienceServiceClient<InterceptedService<T, F>>
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
            AudienceServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates audiences. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AudienceError]()"]
        pub async fn mutate_audiences(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAudiencesRequest>,
        ) -> Result<tonic::Response<super::MutateAudiencesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.AudienceService/MutateAudiences",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[BiddingDataExclusionService.MutateBiddingDataExclusions][google.ads.googleads.v10.services.BiddingDataExclusionService.MutateBiddingDataExclusions\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBiddingDataExclusionsRequest {
    /// Required. ID of the customer whose data exclusions are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual data exclusions.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<BiddingDataExclusionOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove, update) on a data exclusion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingDataExclusionOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(
        oneof = "bidding_data_exclusion_operation::Operation",
        tags = "1, 2, 3"
    )]
    pub operation: ::core::option::Option<bidding_data_exclusion_operation::Operation>,
}
/// Nested message and enum types in `BiddingDataExclusionOperation`.
pub mod bidding_data_exclusion_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new data
        /// exclusion.
        #[prost(message, tag = "1")]
        Create(super::super::resources::BiddingDataExclusion),
        /// Update operation: The data exclusion is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::BiddingDataExclusion),
        /// Remove operation: A resource name for the removed data exclusion
        /// is expected, in this format:
        ///
        /// `customers/{customer_id}/biddingDataExclusions/{data_exclusion_id}`
        #[prost(string, tag = "3")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for data exlusions mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBiddingDataExclusionsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateBiddingDataExclusionsResult>,
}
/// The result for the data exclusion mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBiddingDataExclusionsResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated bidding data exclusion with only mutable fields after mutate.
    /// The field will only be returned when response_content_type is set
    /// to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub bidding_data_exclusion: ::core::option::Option<super::resources::BiddingDataExclusion>,
}
#[doc = r" Generated client implementations."]
pub mod bidding_data_exclusion_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage bidding data exclusions."]
    #[derive(Debug, Clone)]
    pub struct BiddingDataExclusionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BiddingDataExclusionServiceClient<T>
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
        ) -> BiddingDataExclusionServiceClient<InterceptedService<T, F>>
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
            BiddingDataExclusionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes data exclusions."]
        #[doc = " Operation statuses are returned."]
        pub async fn mutate_bidding_data_exclusions(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateBiddingDataExclusionsRequest>,
        ) -> Result<tonic::Response<super::MutateBiddingDataExclusionsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.BiddingDataExclusionService/MutateBiddingDataExclusions") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[BiddingSeasonalityAdjustmentService.MutateBiddingSeasonalityAdjustments][google.ads.googleads.v10.services.BiddingSeasonalityAdjustmentService.MutateBiddingSeasonalityAdjustments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBiddingSeasonalityAdjustmentsRequest {
    /// Required. ID of the customer whose seasonality adjustments are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual seasonality adjustments.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<BiddingSeasonalityAdjustmentOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove, update) on a seasonality adjustment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingSeasonalityAdjustmentOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(
        oneof = "bidding_seasonality_adjustment_operation::Operation",
        tags = "1, 2, 3"
    )]
    pub operation: ::core::option::Option<bidding_seasonality_adjustment_operation::Operation>,
}
/// Nested message and enum types in `BiddingSeasonalityAdjustmentOperation`.
pub mod bidding_seasonality_adjustment_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new seasonality
        /// adjustment.
        #[prost(message, tag = "1")]
        Create(super::super::resources::BiddingSeasonalityAdjustment),
        /// Update operation: The seasonality adjustment is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::BiddingSeasonalityAdjustment),
        /// Remove operation: A resource name for the removed seasonality adjustment
        /// is expected, in this format:
        ///
        /// `customers/{customer_id}/biddingSeasonalityAdjustments/{seasonality_adjustment_id}`
        #[prost(string, tag = "3")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for seasonality adjustments mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBiddingSeasonalityAdjustmentsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateBiddingSeasonalityAdjustmentsResult>,
}
/// The result for the seasonality adjustment mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBiddingSeasonalityAdjustmentsResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated bidding seasonality adjustment with only mutable fields after
    /// mutate. The field will only be returned when response_content_type is set
    /// to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub bidding_seasonality_adjustment:
        ::core::option::Option<super::resources::BiddingSeasonalityAdjustment>,
}
#[doc = r" Generated client implementations."]
pub mod bidding_seasonality_adjustment_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage bidding seasonality adjustments."]
    #[derive(Debug, Clone)]
    pub struct BiddingSeasonalityAdjustmentServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BiddingSeasonalityAdjustmentServiceClient<T>
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
        ) -> BiddingSeasonalityAdjustmentServiceClient<InterceptedService<T, F>>
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
            BiddingSeasonalityAdjustmentServiceClient::new(InterceptedService::new(
                inner,
                interceptor,
            ))
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
        #[doc = " Creates, updates, or removes seasonality adjustments."]
        #[doc = " Operation statuses are returned."]
        pub async fn mutate_bidding_seasonality_adjustments(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateBiddingSeasonalityAdjustmentsRequest>,
        ) -> Result<
            tonic::Response<super::MutateBiddingSeasonalityAdjustmentsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.BiddingSeasonalityAdjustmentService/MutateBiddingSeasonalityAdjustments") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[BiddingStrategyService.MutateBiddingStrategies][google.ads.googleads.v10.services.BiddingStrategyService.MutateBiddingStrategies\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBiddingStrategiesRequest {
    /// Required. The ID of the customer whose bidding strategies are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual bidding strategies.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<BiddingStrategyOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on a bidding strategy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingStrategyOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "bidding_strategy_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<bidding_strategy_operation::Operation>,
}
/// Nested message and enum types in `BiddingStrategyOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateBiddingStrategyResult>,
}
/// The result for the bidding strategy mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBiddingStrategyResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated bidding strategy with only mutable fields after mutate. The
    /// field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub bidding_strategy: ::core::option::Option<super::resources::BiddingStrategy>,
}
#[doc = r" Generated client implementations."]
pub mod bidding_strategy_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage bidding strategies."]
    #[derive(Debug, Clone)]
    pub struct BiddingStrategyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BiddingStrategyServiceClient<T>
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
        ) -> BiddingStrategyServiceClient<InterceptedService<T, F>>
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
            BiddingStrategyServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes bidding strategies. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AdxError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [BiddingError]()"]
        #[doc = "   [BiddingStrategyError]()"]
        #[doc = "   [ContextError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperationAccessDeniedError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
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
                "/google.ads.googleads.v10.services.BiddingStrategyService/MutateBiddingStrategies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CampaignAssetService.MutateCampaignAssets][google.ads.googleads.v10.services.CampaignAssetService.MutateCampaignAssets\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignAssetsRequest {
    /// Required. The ID of the customer whose campaign assets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual campaign assets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CampaignAssetOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on a campaign asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignAssetOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "campaign_asset_operation::Operation", tags = "1, 3, 2")]
    pub operation: ::core::option::Option<campaign_asset_operation::Operation>,
}
/// Nested message and enum types in `CampaignAssetOperation`.
pub mod campaign_asset_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new campaign
        /// asset.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CampaignAsset),
        /// Update operation: The campaign asset is expected to have a valid resource
        /// name.
        #[prost(message, tag = "3")]
        Update(super::super::resources::CampaignAsset),
        /// Remove operation: A resource name for the removed campaign asset is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/campaignAssets/{campaign_id}~{asset_id}~{field_type}`
        #[prost(string, tag = "2")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for a campaign asset mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignAssetsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "1")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateCampaignAssetResult>,
}
/// The result for the campaign asset mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignAssetResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated campaign asset with only mutable fields after
    /// mutate. The field will only be returned when response_content_type is set
    /// to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub campaign_asset: ::core::option::Option<super::resources::CampaignAsset>,
}
#[doc = r" Generated client implementations."]
pub mod campaign_asset_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign assets."]
    #[derive(Debug, Clone)]
    pub struct CampaignAssetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignAssetServiceClient<T>
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
        ) -> CampaignAssetServiceClient<InterceptedService<T, F>>
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
            CampaignAssetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes campaign assets. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AssetLinkError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [ContextError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NotAllowlistedError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn mutate_campaign_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCampaignAssetsRequest>,
        ) -> Result<tonic::Response<super::MutateCampaignAssetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.CampaignAssetService/MutateCampaignAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CampaignAssetSetService.MutateCampaignAssetSets][google.ads.googleads.v10.services.CampaignAssetSetService.MutateCampaignAssetSets\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignAssetSetsRequest {
    /// Required. The ID of the customer whose campaign asset sets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual campaign asset sets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CampaignAssetSetOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove) on a campaign asset set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignAssetSetOperation {
    /// The mutate operation.
    #[prost(oneof = "campaign_asset_set_operation::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<campaign_asset_set_operation::Operation>,
}
/// Nested message and enum types in `CampaignAssetSetOperation`.
pub mod campaign_asset_set_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new campaign asset
        /// set.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CampaignAssetSet),
        /// Remove operation: A resource name for the removed campaign asset set is
        /// expected, in this format:
        /// `customers/{customer_id}/campaignAssetSets/{campaign_id}~{asset_set_id}`
        #[prost(string, tag = "2")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for a campaign asset set mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignAssetSetsResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateCampaignAssetSetResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The result for the campaign asset set mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignAssetSetResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated campaign asset set with only mutable fields after mutate. The
    /// field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub campaign_asset_set: ::core::option::Option<super::resources::CampaignAssetSet>,
}
#[doc = r" Generated client implementations."]
pub mod campaign_asset_set_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign asset set"]
    #[derive(Debug, Clone)]
    pub struct CampaignAssetSetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignAssetSetServiceClient<T>
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
        ) -> CampaignAssetSetServiceClient<InterceptedService<T, F>>
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
            CampaignAssetSetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates or removes campaign asset sets. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_campaign_asset_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCampaignAssetSetsRequest>,
        ) -> Result<tonic::Response<super::MutateCampaignAssetSetsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CampaignAssetSetService/MutateCampaignAssetSets") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[CampaignBidModifierService.MutateCampaignBidModifiers][google.ads.googleads.v10.services.CampaignBidModifierService.MutateCampaignBidModifiers\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignBidModifiersRequest {
    /// Required. ID of the customer whose campaign bid modifiers are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual campaign bid modifiers.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CampaignBidModifierOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove, update) on a campaign bid modifier.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignBidModifierOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "campaign_bid_modifier_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<campaign_bid_modifier_operation::Operation>,
}
/// Nested message and enum types in `CampaignBidModifierOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateCampaignBidModifierResult>,
}
/// The result for the criterion mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignBidModifierResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated campaign bid modifier with only mutable fields after mutate.
    /// The field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub campaign_bid_modifier: ::core::option::Option<super::resources::CampaignBidModifier>,
}
#[doc = r" Generated client implementations."]
pub mod campaign_bid_modifier_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign bid modifiers."]
    #[derive(Debug, Clone)]
    pub struct CampaignBidModifierServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignBidModifierServiceClient<T>
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
        ) -> CampaignBidModifierServiceClient<InterceptedService<T, F>>
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
            CampaignBidModifierServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes campaign bid modifiers."]
        #[doc = " Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [ContextError]()"]
        #[doc = "   [CriterionError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CampaignBidModifierService/MutateCampaignBidModifiers") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CampaignBudgetService.MutateCampaignBudgets][google.ads.googleads.v10.services.CampaignBudgetService.MutateCampaignBudgets\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignBudgetsRequest {
    /// Required. The ID of the customer whose campaign budgets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual campaign budgets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CampaignBudgetOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on a campaign budget.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignBudgetOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "campaign_budget_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<campaign_budget_operation::Operation>,
}
/// Nested message and enum types in `CampaignBudgetOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateCampaignBudgetResult>,
}
/// The result for the campaign budget mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignBudgetResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated campaign budget with only mutable fields after mutate. The
    /// field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub campaign_budget: ::core::option::Option<super::resources::CampaignBudget>,
}
#[doc = r" Generated client implementations."]
pub mod campaign_budget_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign budgets."]
    #[derive(Debug, Clone)]
    pub struct CampaignBudgetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignBudgetServiceClient<T>
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
        ) -> CampaignBudgetServiceClient<InterceptedService<T, F>>
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
            CampaignBudgetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes campaign budgets. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CampaignBudgetError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [OperationAccessDeniedError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        #[doc = "   [StringLengthError]()"]
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
                "/google.ads.googleads.v10.services.CampaignBudgetService/MutateCampaignBudgets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[CampaignConversionGoalService.MutateCampaignConversionGoals][google.ads.googleads.v10.services.CampaignConversionGoalService.MutateCampaignConversionGoals\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignConversionGoalsRequest {
    /// Required. The ID of the customer whose campaign conversion goals are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual campaign conversion goal.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CampaignConversionGoalOperation>,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// A single operation (update) on a campaign conversion goal.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignConversionGoalOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "campaign_conversion_goal_operation::Operation", tags = "1")]
    pub operation: ::core::option::Option<campaign_conversion_goal_operation::Operation>,
}
/// Nested message and enum types in `CampaignConversionGoalOperation`.
pub mod campaign_conversion_goal_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Update operation: The customer conversion goal is expected to have a
        /// valid resource name.
        #[prost(message, tag = "1")]
        Update(super::super::resources::CampaignConversionGoal),
    }
}
/// Response message for a campaign conversion goal mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignConversionGoalsResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateCampaignConversionGoalResult>,
}
/// The result for the campaign conversion goal mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignConversionGoalResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod campaign_conversion_goal_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign conversion goal."]
    #[derive(Debug, Clone)]
    pub struct CampaignConversionGoalServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignConversionGoalServiceClient<T>
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
        ) -> CampaignConversionGoalServiceClient<InterceptedService<T, F>>
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
            CampaignConversionGoalServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates or removes campaign conversion goals. Operation statuses"]
        #[doc = " are returned."]
        pub async fn mutate_campaign_conversion_goals(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCampaignConversionGoalsRequest>,
        ) -> Result<tonic::Response<super::MutateCampaignConversionGoalsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CampaignConversionGoalService/MutateCampaignConversionGoals") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CampaignCriterionService.MutateCampaignCriteria][google.ads.googleads.v10.services.CampaignCriterionService.MutateCampaignCriteria\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignCriteriaRequest {
    /// Required. The ID of the customer whose criteria are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual criteria.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CampaignCriterionOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on a campaign criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignCriterionOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "campaign_criterion_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<campaign_criterion_operation::Operation>,
}
/// Nested message and enum types in `CampaignCriterionOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateCampaignCriterionResult>,
}
/// The result for the criterion mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignCriterionResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated campaign criterion with only mutable fields after mutate. The
    /// field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub campaign_criterion: ::core::option::Option<super::resources::CampaignCriterion>,
}
#[doc = r" Generated client implementations."]
pub mod campaign_criterion_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign criteria."]
    #[derive(Debug, Clone)]
    pub struct CampaignCriterionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignCriterionServiceClient<T>
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
        ) -> CampaignCriterionServiceClient<InterceptedService<T, F>>
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
            CampaignCriterionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes criteria. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AdxError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CampaignCriterionError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [ContextError]()"]
        #[doc = "   [CriterionError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [FunctionError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperationAccessDeniedError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RegionCodeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CampaignCriterionService/MutateCampaignCriteria") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[CampaignCustomizerService.MutateCampaignCustomizers][google.ads.googleads.v10.services.CampaignCustomizerService.MutateCampaignCustomizers\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignCustomizersRequest {
    /// Required. The ID of the customer whose campaign customizers are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual campaign customizers.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CampaignCustomizerOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove) on an customizer attribute.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignCustomizerOperation {
    /// The mutate operation.
    #[prost(oneof = "campaign_customizer_operation::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<campaign_customizer_operation::Operation>,
}
/// Nested message and enum types in `CampaignCustomizerOperation`.
pub mod campaign_customizer_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new campaign
        /// customizer
        #[prost(message, tag = "1")]
        Create(super::super::resources::CampaignCustomizer),
        /// Remove operation: A resource name for the removed campaign customizer is
        /// expected, in this format:
        /// `customers/{customer_id}/campaignCustomizers/{campaign_id}~{customizer_attribute_id}`
        #[prost(string, tag = "2")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for an campaign customizer mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignCustomizersResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateCampaignCustomizerResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The result for the campaign customizer mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignCustomizerResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated CampaignCustomizer with only mutable fields after mutate.
    /// The field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub campaign_customizer: ::core::option::Option<super::resources::CampaignCustomizer>,
}
#[doc = r" Generated client implementations."]
pub mod campaign_customizer_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign customizer"]
    #[derive(Debug, Clone)]
    pub struct CampaignCustomizerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignCustomizerServiceClient<T>
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
        ) -> CampaignCustomizerServiceClient<InterceptedService<T, F>>
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
            CampaignCustomizerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates or removes campaign customizers. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_campaign_customizers(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCampaignCustomizersRequest>,
        ) -> Result<tonic::Response<super::MutateCampaignCustomizersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CampaignCustomizerService/MutateCampaignCustomizers") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CampaignDraftService.MutateCampaignDrafts][google.ads.googleads.v10.services.CampaignDraftService.MutateCampaignDrafts\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignDraftsRequest {
    /// Required. The ID of the customer whose campaign drafts are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual campaign drafts.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CampaignDraftOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// Request message for \[CampaignDraftService.PromoteCampaignDraft][google.ads.googleads.v10.services.CampaignDraftService.PromoteCampaignDraft\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromoteCampaignDraftRequest {
    /// Required. The resource name of the campaign draft to promote.
    #[prost(string, tag = "1")]
    pub campaign_draft: ::prost::alloc::string::String,
    /// If true, the request is validated but no Long Running Operation is created.
    /// Only errors are returned.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on a campaign draft.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignDraftOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "campaign_draft_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<campaign_draft_operation::Operation>,
}
/// Nested message and enum types in `CampaignDraftOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateCampaignDraftResult>,
}
/// The result for the campaign draft mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignDraftResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated campaign draft with only mutable fields after mutate. The field
    /// will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub campaign_draft: ::core::option::Option<super::resources::CampaignDraft>,
}
/// Request message for \[CampaignDraftService.ListCampaignDraftAsyncErrors][google.ads.googleads.v10.services.CampaignDraftService.ListCampaignDraftAsyncErrors\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCampaignDraftAsyncErrorsRequest {
    /// Required. The name of the campaign draft from which to retrieve the async errors.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Token of the page to retrieve. If not specified, the first
    /// page of results will be returned. Use the value obtained from
    /// `next_page_token` in the previous response in order to request
    /// the next page of results.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Number of elements to retrieve in a single page.
    /// When a page request is too large, the server may decide to
    /// further limit the number of returned resources.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// Response message for \[CampaignDraftService.ListCampaignDraftAsyncErrors][google.ads.googleads.v10.services.CampaignDraftService.ListCampaignDraftAsyncErrors\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCampaignDraftAsyncErrorsResponse {
    /// Details of the errors when performing the asynchronous operation.
    #[prost(message, repeated, tag = "1")]
    pub errors: ::prost::alloc::vec::Vec<super::super::super::super::rpc::Status>,
    /// Pagination token used to retrieve the next page of results.
    /// Pass the content of this string as the `page_token` attribute of
    /// the next request. `next_page_token` is not returned for the last
    /// page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod campaign_draft_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign drafts."]
    #[derive(Debug, Clone)]
    pub struct CampaignDraftServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignDraftServiceClient<T>
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
        ) -> CampaignDraftServiceClient<InterceptedService<T, F>>
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
            CampaignDraftServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes campaign drafts. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CampaignDraftError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.CampaignDraftService/MutateCampaignDrafts",
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
        #[doc = " [CampaignDraftService.ListCampaignDraftAsyncErrors][google.ads.googleads.v10.services.CampaignDraftService.ListCampaignDraftAsyncErrors] to view the list of"]
        #[doc = " error reasons."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CampaignDraftError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.CampaignDraftService/PromoteCampaignDraft",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns all errors that occurred during CampaignDraft promote. Throws an"]
        #[doc = " error if called before campaign draft is promoted."]
        #[doc = " Supports standard list paging."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CampaignDraftService/ListCampaignDraftAsyncErrors") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CampaignExperimentService.MutateCampaignExperiments][google.ads.googleads.v10.services.CampaignExperimentService.MutateCampaignExperiments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignExperimentsRequest {
    /// Required. The ID of the customer whose campaign experiments are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual campaign experiments.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CampaignExperimentOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single update operation on a campaign experiment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignExperimentOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "campaign_experiment_operation::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<campaign_experiment_operation::Operation>,
}
/// Nested message and enum types in `CampaignExperimentOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateCampaignExperimentResult>,
}
/// The result for the campaign experiment mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignExperimentResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated campaign experiment with only mutable fields after mutate. The
    /// field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub campaign_experiment: ::core::option::Option<super::resources::CampaignExperiment>,
}
/// Request message for \[CampaignExperimentService.CreateCampaignExperiment][google.ads.googleads.v10.services.CampaignExperimentService.CreateCampaignExperiment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCampaignExperimentRequest {
    /// Required. The ID of the customer whose campaign experiment is being created.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The campaign experiment to be created.
    #[prost(message, optional, tag = "2")]
    pub campaign_experiment: ::core::option::Option<super::resources::CampaignExperiment>,
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
    pub campaign_experiment: ::prost::alloc::string::String,
}
/// Request message for \[CampaignExperimentService.GraduateCampaignExperiment][google.ads.googleads.v10.services.CampaignExperimentService.GraduateCampaignExperiment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GraduateCampaignExperimentRequest {
    /// Required. The resource name of the campaign experiment to graduate.
    #[prost(string, tag = "1")]
    pub campaign_experiment: ::prost::alloc::string::String,
    /// Required. Resource name of the budget to attach to the campaign graduated from the
    /// experiment.
    #[prost(string, tag = "2")]
    pub campaign_budget: ::prost::alloc::string::String,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// Response message for campaign experiment graduate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GraduateCampaignExperimentResponse {
    /// The resource name of the campaign from the graduated experiment.
    /// This campaign is the same one as CampaignExperiment.experiment_campaign.
    #[prost(string, tag = "1")]
    pub graduated_campaign: ::prost::alloc::string::String,
}
/// Request message for \[CampaignExperimentService.PromoteCampaignExperiment][google.ads.googleads.v10.services.CampaignExperimentService.PromoteCampaignExperiment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromoteCampaignExperimentRequest {
    /// Required. The resource name of the campaign experiment to promote.
    #[prost(string, tag = "1")]
    pub campaign_experiment: ::prost::alloc::string::String,
    /// If true, the request is validated but no Long Running Operation is created.
    /// Only errors are returned.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
}
/// Request message for \[CampaignExperimentService.EndCampaignExperiment][google.ads.googleads.v10.services.CampaignExperimentService.EndCampaignExperiment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndCampaignExperimentRequest {
    /// Required. The resource name of the campaign experiment to end.
    #[prost(string, tag = "1")]
    pub campaign_experiment: ::prost::alloc::string::String,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
}
/// Request message for
/// \[CampaignExperimentService.ListCampaignExperimentAsyncErrors][google.ads.googleads.v10.services.CampaignExperimentService.ListCampaignExperimentAsyncErrors\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCampaignExperimentAsyncErrorsRequest {
    /// Required. The name of the campaign experiment from which to retrieve the async
    /// errors.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Token of the page to retrieve. If not specified, the first
    /// page of results will be returned. Use the value obtained from
    /// `next_page_token` in the previous response in order to request
    /// the next page of results.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Number of elements to retrieve in a single page.
    /// When a page request is too large, the server may decide to
    /// further limit the number of returned resources.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// Response message for
/// \[CampaignExperimentService.ListCampaignExperimentAsyncErrors][google.ads.googleads.v10.services.CampaignExperimentService.ListCampaignExperimentAsyncErrors\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCampaignExperimentAsyncErrorsResponse {
    /// Details of the errors when performing the asynchronous operation.
    #[prost(message, repeated, tag = "1")]
    pub errors: ::prost::alloc::vec::Vec<super::super::super::super::rpc::Status>,
    /// Pagination token used to retrieve the next page of results.
    /// Pass the content of this string as the `page_token` attribute of
    /// the next request. `next_page_token` is not returned for the last
    /// page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod campaign_experiment_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
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
    #[derive(Debug, Clone)]
    pub struct CampaignExperimentServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignExperimentServiceClient<T>
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
        ) -> CampaignExperimentServiceClient<InterceptedService<T, F>>
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
            CampaignExperimentServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CampaignExperimentError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DateRangeError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CampaignExperimentService/CreateCampaignExperiment") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates campaign experiments. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CampaignExperimentError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CampaignExperimentService/MutateCampaignExperiments") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Graduates a campaign experiment to a full campaign. The base and experiment"]
        #[doc = " campaigns will start running independently with their own budgets."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CampaignExperimentError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CampaignExperimentService/GraduateCampaignExperiment") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Promotes the changes in a experiment campaign back to the base campaign."]
        #[doc = ""]
        #[doc = " The campaign experiment is updated immediately with status PROMOTING."]
        #[doc = " This method return a long running operation that tracks the promoting of"]
        #[doc = " the experiment campaign. If the promoting fails, a list of errors can be"]
        #[doc = " retrieved using the ListCampaignExperimentAsyncErrors method."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CampaignExperimentService/PromoteCampaignExperiment") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Immediately ends a campaign experiment, changing the experiment's scheduled"]
        #[doc = " end date and without waiting for end of day. End date is updated to be the"]
        #[doc = " time of the request."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CampaignExperimentError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CampaignExperimentService/EndCampaignExperiment") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns all errors that occurred during CampaignExperiment create or"]
        #[doc = " promote (whichever occurred last)."]
        #[doc = " Supports standard list paging."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CampaignExperimentService/ListCampaignExperimentAsyncErrors") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[CampaignExtensionSettingService.MutateCampaignExtensionSettings][google.ads.googleads.v10.services.CampaignExtensionSettingService.MutateCampaignExtensionSettings\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignExtensionSettingsRequest {
    /// Required. The ID of the customer whose campaign extension settings are being
    /// modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual campaign extension
    /// settings.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CampaignExtensionSettingOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on a campaign extension setting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignExtensionSettingOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(
        oneof = "campaign_extension_setting_operation::Operation",
        tags = "1, 2, 3"
    )]
    pub operation: ::core::option::Option<campaign_extension_setting_operation::Operation>,
}
/// Nested message and enum types in `CampaignExtensionSettingOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateCampaignExtensionSettingResult>,
}
/// The result for the campaign extension setting mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignExtensionSettingResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated campaign extension setting with only mutable fields after
    /// mutate. The field will only be returned when response_content_type is set
    /// to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub campaign_extension_setting:
        ::core::option::Option<super::resources::CampaignExtensionSetting>,
}
#[doc = r" Generated client implementations."]
pub mod campaign_extension_setting_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign extension settings."]
    #[derive(Debug, Clone)]
    pub struct CampaignExtensionSettingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignExtensionSettingServiceClient<T>
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
        ) -> CampaignExtensionSettingServiceClient<InterceptedService<T, F>>
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
            CampaignExtensionSettingServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes campaign extension settings. Operation"]
        #[doc = " statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [CriterionError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [ExtensionSettingError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [ListOperationError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperationAccessDeniedError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
        #[doc = "   [UrlFieldError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CampaignExtensionSettingService/MutateCampaignExtensionSettings") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CampaignFeedService.MutateCampaignFeeds][google.ads.googleads.v10.services.CampaignFeedService.MutateCampaignFeeds\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignFeedsRequest {
    /// Required. The ID of the customer whose campaign feeds are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual campaign feeds.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CampaignFeedOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on a campaign feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignFeedOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "campaign_feed_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<campaign_feed_operation::Operation>,
}
/// Nested message and enum types in `CampaignFeedOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateCampaignFeedResult>,
}
/// The result for the campaign feed mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignFeedResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated campaign feed with only mutable fields after mutate. The field
    /// will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub campaign_feed: ::core::option::Option<super::resources::CampaignFeed>,
}
#[doc = r" Generated client implementations."]
pub mod campaign_feed_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign feeds."]
    #[derive(Debug, Clone)]
    pub struct CampaignFeedServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignFeedServiceClient<T>
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
        ) -> CampaignFeedServiceClient<InterceptedService<T, F>>
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
            CampaignFeedServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes campaign feeds. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CampaignFeedError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FunctionError]()"]
        #[doc = "   [FunctionParsingError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperationAccessDeniedError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
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
                "/google.ads.googleads.v10.services.CampaignFeedService/MutateCampaignFeeds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CampaignGroupService.MutateCampaignGroups][google.ads.googleads.v10.services.CampaignGroupService.MutateCampaignGroups\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignGroupsRequest {
    /// Required. The ID of the customer whose campaign groups are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual campaign groups.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CampaignGroupOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on a campaign group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignGroupOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "campaign_group_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<campaign_group_operation::Operation>,
}
/// Nested message and enum types in `CampaignGroupOperation`.
pub mod campaign_group_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new campaign
        /// group.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CampaignGroup),
        /// Update operation: The campaign group is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::CampaignGroup),
        /// Remove operation: A resource name for the removed campaign group is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/campaignGroups/{campaign_group_id}`
        #[prost(string, tag = "3")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for campaign group mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignGroupsResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateCampaignGroupResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The result for the campaign group mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignGroupResult {
    /// Required. Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated campaign group with only mutable fields after mutate. The field
    /// will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub campaign_group: ::core::option::Option<super::resources::CampaignGroup>,
}
#[doc = r" Generated client implementations."]
pub mod campaign_group_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign groups."]
    #[derive(Debug, Clone)]
    pub struct CampaignGroupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignGroupServiceClient<T>
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
        ) -> CampaignGroupServiceClient<InterceptedService<T, F>>
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
            CampaignGroupServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes campaign groups. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_campaign_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCampaignGroupsRequest>,
        ) -> Result<tonic::Response<super::MutateCampaignGroupsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.CampaignGroupService/MutateCampaignGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CampaignLabelService.MutateCampaignLabels][google.ads.googleads.v10.services.CampaignLabelService.MutateCampaignLabels\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignLabelsRequest {
    /// Required. ID of the customer whose campaign-label relationships are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on campaign-label relationships.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CampaignLabelOperation>,
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
    pub operation: ::core::option::Option<campaign_label_operation::Operation>,
}
/// Nested message and enum types in `CampaignLabelOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateCampaignLabelResult>,
}
/// The result for a campaign label mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignLabelResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod campaign_label_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage labels on campaigns."]
    #[derive(Debug, Clone)]
    pub struct CampaignLabelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignLabelServiceClient<T>
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
        ) -> CampaignLabelServiceClient<InterceptedService<T, F>>
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
            CampaignLabelServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates and removes campaign-label relationships."]
        #[doc = " Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [LabelError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.CampaignLabelService/MutateCampaignLabels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CampaignService.MutateCampaigns][google.ads.googleads.v10.services.CampaignService.MutateCampaigns\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignsRequest {
    /// Required. The ID of the customer whose campaigns are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual campaigns.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CampaignOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on a campaign.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "campaign_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<campaign_operation::Operation>,
}
/// Nested message and enum types in `CampaignOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateCampaignResult>,
}
/// The result for the campaign mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated campaign with only mutable fields after mutate. The field will
    /// only be returned when response_content_type is set to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub campaign: ::core::option::Option<super::resources::Campaign>,
}
#[doc = r" Generated client implementations."]
pub mod campaign_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaigns."]
    #[derive(Debug, Clone)]
    pub struct CampaignServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignServiceClient<T>
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
        ) -> CampaignServiceClient<InterceptedService<T, F>>
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
            CampaignServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes campaigns. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AdxError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [BiddingError]()"]
        #[doc = "   [BiddingStrategyError]()"]
        #[doc = "   [CampaignBudgetError]()"]
        #[doc = "   [CampaignError]()"]
        #[doc = "   [ContextError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DateRangeError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [ListOperationError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NotAllowlistedError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperationAccessDeniedError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RegionCodeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        #[doc = "   [SettingError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
        #[doc = "   [UrlFieldError]()"]
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
                "/google.ads.googleads.v10.services.CampaignService/MutateCampaigns",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CampaignSharedSetService.MutateCampaignSharedSets][google.ads.googleads.v10.services.CampaignSharedSetService.MutateCampaignSharedSets\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignSharedSetsRequest {
    /// Required. The ID of the customer whose campaign shared sets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual campaign shared sets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CampaignSharedSetOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove) on an campaign shared set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignSharedSetOperation {
    /// The mutate operation.
    #[prost(oneof = "campaign_shared_set_operation::Operation", tags = "1, 3")]
    pub operation: ::core::option::Option<campaign_shared_set_operation::Operation>,
}
/// Nested message and enum types in `CampaignSharedSetOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateCampaignSharedSetResult>,
}
/// The result for the campaign shared set mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignSharedSetResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated campaign shared set with only mutable fields after mutate. The
    /// field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub campaign_shared_set: ::core::option::Option<super::resources::CampaignSharedSet>,
}
#[doc = r" Generated client implementations."]
pub mod campaign_shared_set_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign shared sets."]
    #[derive(Debug, Clone)]
    pub struct CampaignSharedSetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignSharedSetServiceClient<T>
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
        ) -> CampaignSharedSetServiceClient<InterceptedService<T, F>>
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
            CampaignSharedSetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates or removes campaign shared sets. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CampaignSharedSetError]()"]
        #[doc = "   [ContextError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CampaignSharedSetService/MutateCampaignSharedSets") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[ConversionActionService.MutateConversionActions][google.ads.googleads.v10.services.ConversionActionService.MutateConversionActions\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConversionActionsRequest {
    /// Required. The ID of the customer whose conversion actions are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual conversion actions.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<ConversionActionOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on a conversion action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionActionOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "conversion_action_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<conversion_action_operation::Operation>,
}
/// Nested message and enum types in `ConversionActionOperation`.
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
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for \[ConversionActionService.MutateConversionActions][google.ads.googleads.v10.services.ConversionActionService.MutateConversionActions\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConversionActionsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateConversionActionResult>,
}
/// The result for the conversion action mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConversionActionResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated conversion action with only mutable fields after mutate. The
    /// field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub conversion_action: ::core::option::Option<super::resources::ConversionAction>,
}
#[doc = r" Generated client implementations."]
pub mod conversion_action_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage conversion actions."]
    #[derive(Debug, Clone)]
    pub struct ConversionActionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConversionActionServiceClient<T>
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
        ) -> ConversionActionServiceClient<InterceptedService<T, F>>
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
            ConversionActionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates or removes conversion actions. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [ConversionActionError]()"]
        #[doc = "   [CurrencyCodeError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        #[doc = "   [StringLengthError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.ConversionActionService/MutateConversionActions") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[ConversionCustomVariableService.MutateConversionCustomVariables][google.ads.googleads.v10.services.ConversionCustomVariableService.MutateConversionCustomVariables\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConversionCustomVariablesRequest {
    /// Required. The ID of the customer whose conversion custom variables are being
    /// modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual conversion custom
    /// variables.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<ConversionCustomVariableOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update) on a conversion custom variable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionCustomVariableOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(
        oneof = "conversion_custom_variable_operation::Operation",
        tags = "1, 2"
    )]
    pub operation: ::core::option::Option<conversion_custom_variable_operation::Operation>,
}
/// Nested message and enum types in `ConversionCustomVariableOperation`.
pub mod conversion_custom_variable_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new conversion
        /// custom variable.
        #[prost(message, tag = "1")]
        Create(super::super::resources::ConversionCustomVariable),
        /// Update operation: The conversion custom variable is expected to have a
        /// valid resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::ConversionCustomVariable),
    }
}
/// Response message for
/// \[ConversionCustomVariableService.MutateConversionCustomVariables][google.ads.googleads.v10.services.ConversionCustomVariableService.MutateConversionCustomVariables\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConversionCustomVariablesResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "1")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateConversionCustomVariableResult>,
}
/// The result for the conversion custom variable mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConversionCustomVariableResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated conversion custom variable with only mutable fields after
    /// mutate. The field will only be returned when response_content_type is set
    /// to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub conversion_custom_variable:
        ::core::option::Option<super::resources::ConversionCustomVariable>,
}
#[doc = r" Generated client implementations."]
pub mod conversion_custom_variable_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage conversion custom variables."]
    #[derive(Debug, Clone)]
    pub struct ConversionCustomVariableServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConversionCustomVariableServiceClient<T>
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
        ) -> ConversionCustomVariableServiceClient<InterceptedService<T, F>>
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
            ConversionCustomVariableServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates or updates conversion custom variables. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [ConversionCustomVariableError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn mutate_conversion_custom_variables(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateConversionCustomVariablesRequest>,
        ) -> Result<tonic::Response<super::MutateConversionCustomVariablesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.ConversionCustomVariableService/MutateConversionCustomVariables") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[ConversionGoalCampaignConfigService.MutateConversionGoalCampaignConfig][\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConversionGoalCampaignConfigsRequest {
    /// Required. The ID of the customer whose custom conversion goals are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual conversion goal campaign
    /// config.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<ConversionGoalCampaignConfigOperation>,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "4"
    )]
    pub response_content_type: i32,
}
/// A single operation (update) on a conversion goal campaign config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionGoalCampaignConfigOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(
        oneof = "conversion_goal_campaign_config_operation::Operation",
        tags = "1"
    )]
    pub operation: ::core::option::Option<conversion_goal_campaign_config_operation::Operation>,
}
/// Nested message and enum types in `ConversionGoalCampaignConfigOperation`.
pub mod conversion_goal_campaign_config_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Update operation: The conversion goal campaign config is expected to have
        /// a valid resource name.
        #[prost(message, tag = "1")]
        Update(super::super::resources::ConversionGoalCampaignConfig),
    }
}
/// Response message for a conversion goal campaign config mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConversionGoalCampaignConfigsResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateConversionGoalCampaignConfigResult>,
}
/// The result for the conversion goal campaign config mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConversionGoalCampaignConfigResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated ConversionGoalCampaignConfig with only mutable fields after
    /// mutate. The field will only be returned when response_content_type is set
    /// to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub conversion_goal_campaign_config:
        ::core::option::Option<super::resources::ConversionGoalCampaignConfig>,
}
#[doc = r" Generated client implementations."]
pub mod conversion_goal_campaign_config_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage conversion goal campaign config."]
    #[derive(Debug, Clone)]
    pub struct ConversionGoalCampaignConfigServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConversionGoalCampaignConfigServiceClient<T>
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
        ) -> ConversionGoalCampaignConfigServiceClient<InterceptedService<T, F>>
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
            ConversionGoalCampaignConfigServiceClient::new(InterceptedService::new(
                inner,
                interceptor,
            ))
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
        #[doc = " Creates, updates or removes conversion goal campaign config. Operation"]
        #[doc = " statuses are returned."]
        pub async fn mutate_conversion_goal_campaign_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateConversionGoalCampaignConfigsRequest>,
        ) -> Result<
            tonic::Response<super::MutateConversionGoalCampaignConfigsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.ConversionGoalCampaignConfigService/MutateConversionGoalCampaignConfigs") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[ConversionValueRuleService.MutateConversionValueRules][google.ads.googleads.v10.services.ConversionValueRuleService.MutateConversionValueRules\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConversionValueRulesRequest {
    /// Required. The ID of the customer whose conversion value rules are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual conversion value rules.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<ConversionValueRuleOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "5")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "4"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on a conversion value rule.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionValueRuleOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "conversion_value_rule_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<conversion_value_rule_operation::Operation>,
}
/// Nested message and enum types in `ConversionValueRuleOperation`.
pub mod conversion_value_rule_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new conversion
        /// value rule.
        #[prost(message, tag = "1")]
        Create(super::super::resources::ConversionValueRule),
        /// Update operation: The conversion value rule is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::ConversionValueRule),
        /// Remove operation: A resource name for the removed conversion value rule
        /// is expected, in this format:
        ///
        /// `customers/{customer_id}/conversionValueRules/{conversion_value_rule_id}`
        #[prost(string, tag = "3")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for
/// \[ConversionValueRuleService.MutateConversionValueRules][google.ads.googleads.v10.services.ConversionValueRuleService.MutateConversionValueRules\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConversionValueRulesResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateConversionValueRuleResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The result for the conversion value rule mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConversionValueRuleResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated conversion value rule with only mutable fields after
    /// mutate. The field will only be returned when response_content_type is set
    /// to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub conversion_value_rule: ::core::option::Option<super::resources::ConversionValueRule>,
}
#[doc = r" Generated client implementations."]
pub mod conversion_value_rule_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage conversion value rules."]
    #[derive(Debug, Clone)]
    pub struct ConversionValueRuleServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConversionValueRuleServiceClient<T>
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
        ) -> ConversionValueRuleServiceClient<InterceptedService<T, F>>
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
            ConversionValueRuleServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes conversion value rules. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_conversion_value_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateConversionValueRulesRequest>,
        ) -> Result<tonic::Response<super::MutateConversionValueRulesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.ConversionValueRuleService/MutateConversionValueRules") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[ConversionValueRuleSetService.MutateConversionValueRuleSets][google.ads.googleads.v10.services.ConversionValueRuleSetService.MutateConversionValueRuleSets\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConversionValueRuleSetsRequest {
    /// Required. The ID of the customer whose conversion value rule sets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual conversion value rule sets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<ConversionValueRuleSetOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "5")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "4"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on a conversion value rule set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionValueRuleSetOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(
        oneof = "conversion_value_rule_set_operation::Operation",
        tags = "1, 2, 3"
    )]
    pub operation: ::core::option::Option<conversion_value_rule_set_operation::Operation>,
}
/// Nested message and enum types in `ConversionValueRuleSetOperation`.
pub mod conversion_value_rule_set_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new conversion
        /// value rule set.
        #[prost(message, tag = "1")]
        Create(super::super::resources::ConversionValueRuleSet),
        /// Update operation: The conversion value rule set is expected to have a
        /// valid resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::ConversionValueRuleSet),
        /// Remove operation: A resource name for the removed conversion value rule
        /// set is expected, in this format:
        ///
        /// `customers/{customer_id}/conversionValueRuleSets/{conversion_value_rule_set_id}`
        #[prost(string, tag = "3")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for
/// \[ConversionValueRuleSetService.MutateConversionValueRuleSets][google.ads.googleads.v10.services.ConversionValueRuleSetService.MutateConversionValueRuleSets\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConversionValueRuleSetsResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateConversionValueRuleSetResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The result for the conversion value rule set mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConversionValueRuleSetResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated conversion value rule set with only mutable fields after
    /// mutate. The field will only be returned when response_content_type is set
    /// to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub conversion_value_rule_set: ::core::option::Option<super::resources::ConversionValueRuleSet>,
}
#[doc = r" Generated client implementations."]
pub mod conversion_value_rule_set_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage conversion value rule sets."]
    #[derive(Debug, Clone)]
    pub struct ConversionValueRuleSetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConversionValueRuleSetServiceClient<T>
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
        ) -> ConversionValueRuleSetServiceClient<InterceptedService<T, F>>
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
            ConversionValueRuleSetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates or removes conversion value rule sets. Operation statuses"]
        #[doc = " are returned."]
        pub async fn mutate_conversion_value_rule_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateConversionValueRuleSetsRequest>,
        ) -> Result<tonic::Response<super::MutateConversionValueRuleSetsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.ConversionValueRuleSetService/MutateConversionValueRuleSets") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[CustomConversionGoalService.MutateCustomConversionGoals][google.ads.googleads.v10.services.CustomConversionGoalService.MutateCustomConversionGoals\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomConversionGoalsRequest {
    /// Required. The ID of the customer whose custom conversion goals are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual custom conversion goal.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CustomConversionGoalOperation>,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "4"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove) on a custom conversion goal.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomConversionGoalOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(
        oneof = "custom_conversion_goal_operation::Operation",
        tags = "1, 2, 3"
    )]
    pub operation: ::core::option::Option<custom_conversion_goal_operation::Operation>,
}
/// Nested message and enum types in `CustomConversionGoalOperation`.
pub mod custom_conversion_goal_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new custom
        /// conversion goal
        #[prost(message, tag = "1")]
        Create(super::super::resources::CustomConversionGoal),
        /// Update operation: The custom conversion goal is expected to have a
        /// valid resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::CustomConversionGoal),
        /// Remove operation: A resource name for the removed custom conversion goal
        /// is expected, in this format:
        ///
        /// 'customers/{customer_id}/conversionActions/{ConversionGoal.custom_goal_config.conversion_type_ids}'
        #[prost(string, tag = "3")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for a custom conversion goal mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomConversionGoalsResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateCustomConversionGoalResult>,
}
/// The result for the custom conversion goal mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomConversionGoalResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated CustomConversionGoal with only mutable fields after mutate.
    /// The field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub custom_conversion_goal: ::core::option::Option<super::resources::CustomConversionGoal>,
}
#[doc = r" Generated client implementations."]
pub mod custom_conversion_goal_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage custom conversion goal."]
    #[derive(Debug, Clone)]
    pub struct CustomConversionGoalServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomConversionGoalServiceClient<T>
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
        ) -> CustomConversionGoalServiceClient<InterceptedService<T, F>>
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
            CustomConversionGoalServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates or removes custom conversion goals. Operation statuses"]
        #[doc = " are returned."]
        pub async fn mutate_custom_conversion_goals(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCustomConversionGoalsRequest>,
        ) -> Result<tonic::Response<super::MutateCustomConversionGoalsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CustomConversionGoalService/MutateCustomConversionGoals") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CustomerAssetService.MutateCustomerAssets][google.ads.googleads.v10.services.CustomerAssetService.MutateCustomerAssets\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerAssetsRequest {
    /// Required. The ID of the customer whose customer assets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual customer assets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CustomerAssetOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on a customer asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerAssetOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "customer_asset_operation::Operation", tags = "1, 3, 2")]
    pub operation: ::core::option::Option<customer_asset_operation::Operation>,
}
/// Nested message and enum types in `CustomerAssetOperation`.
pub mod customer_asset_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new customer
        /// asset.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CustomerAsset),
        /// Update operation: The customer asset is expected to have a valid resource
        /// name.
        #[prost(message, tag = "3")]
        Update(super::super::resources::CustomerAsset),
        /// Remove operation: A resource name for the removed customer asset is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/customerAssets/{asset_id}~{field_type}`
        #[prost(string, tag = "2")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for a customer asset mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerAssetsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "1")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateCustomerAssetResult>,
}
/// The result for the customer asset mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerAssetResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated customer asset with only mutable fields after
    /// mutate. The field will only be returned when response_content_type is set
    /// to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub customer_asset: ::core::option::Option<super::resources::CustomerAsset>,
}
#[doc = r" Generated client implementations."]
pub mod customer_asset_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage customer assets."]
    #[derive(Debug, Clone)]
    pub struct CustomerAssetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerAssetServiceClient<T>
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
        ) -> CustomerAssetServiceClient<InterceptedService<T, F>>
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
            CustomerAssetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes customer assets. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AssetLinkError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn mutate_customer_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCustomerAssetsRequest>,
        ) -> Result<tonic::Response<super::MutateCustomerAssetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.CustomerAssetService/MutateCustomerAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[CustomerConversionGoalService.MutateCustomerConversionGoals][google.ads.googleads.v10.services.CustomerConversionGoalService.MutateCustomerConversionGoals\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerConversionGoalsRequest {
    /// Required. The ID of the customer whose customer conversion goals are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual customer conversion goal.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CustomerConversionGoalOperation>,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// A single operation (update) on a customer conversion goal.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerConversionGoalOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "customer_conversion_goal_operation::Operation", tags = "1")]
    pub operation: ::core::option::Option<customer_conversion_goal_operation::Operation>,
}
/// Nested message and enum types in `CustomerConversionGoalOperation`.
pub mod customer_conversion_goal_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Update operation: The customer conversion goal is expected to have a
        /// valid resource name.
        #[prost(message, tag = "1")]
        Update(super::super::resources::CustomerConversionGoal),
    }
}
/// Response message for a customer conversion goal mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerConversionGoalsResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateCustomerConversionGoalResult>,
}
/// The result for the customer conversion goal mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerConversionGoalResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod customer_conversion_goal_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage customer conversion goal."]
    #[derive(Debug, Clone)]
    pub struct CustomerConversionGoalServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerConversionGoalServiceClient<T>
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
        ) -> CustomerConversionGoalServiceClient<InterceptedService<T, F>>
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
            CustomerConversionGoalServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates or removes customer conversion goals. Operation statuses"]
        #[doc = " are returned."]
        pub async fn mutate_customer_conversion_goals(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCustomerConversionGoalsRequest>,
        ) -> Result<tonic::Response<super::MutateCustomerConversionGoalsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CustomerConversionGoalService/MutateCustomerConversionGoals") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[CustomerCustomizerService.MutateCustomerCustomizers][google.ads.googleads.v10.services.CustomerCustomizerService.MutateCustomerCustomizers\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerCustomizersRequest {
    /// Required. The ID of the customer whose customer customizers are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual customer customizers.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CustomerCustomizerOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove) on an customizer attribute.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerCustomizerOperation {
    /// The mutate operation.
    #[prost(oneof = "customer_customizer_operation::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<customer_customizer_operation::Operation>,
}
/// Nested message and enum types in `CustomerCustomizerOperation`.
pub mod customer_customizer_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new customer
        /// customizer
        #[prost(message, tag = "1")]
        Create(super::super::resources::CustomerCustomizer),
        /// Remove operation: A resource name for the removed customer customizer is
        /// expected, in this format:
        /// `customers/{customer_id}/customerCustomizers/{customizer_attribute_id}`
        #[prost(string, tag = "2")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for an customizer attribute mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerCustomizersResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateCustomerCustomizerResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The result for the customizer attribute mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerCustomizerResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated CustomerCustomizer with only mutable fields after mutate.
    /// The field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub customer_customizer: ::core::option::Option<super::resources::CustomerCustomizer>,
}
#[doc = r" Generated client implementations."]
pub mod customer_customizer_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage customer customizer"]
    #[derive(Debug, Clone)]
    pub struct CustomerCustomizerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerCustomizerServiceClient<T>
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
        ) -> CustomerCustomizerServiceClient<InterceptedService<T, F>>
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
            CustomerCustomizerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates or removes customer customizers. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_customer_customizers(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCustomerCustomizersRequest>,
        ) -> Result<tonic::Response<super::MutateCustomerCustomizersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CustomerCustomizerService/MutateCustomerCustomizers") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[CustomerExtensionSettingService.MutateCustomerExtensionSettings][google.ads.googleads.v10.services.CustomerExtensionSettingService.MutateCustomerExtensionSettings\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerExtensionSettingsRequest {
    /// Required. The ID of the customer whose customer extension settings are being
    /// modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual customer extension
    /// settings.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CustomerExtensionSettingOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on a customer extension setting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerExtensionSettingOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(
        oneof = "customer_extension_setting_operation::Operation",
        tags = "1, 2, 3"
    )]
    pub operation: ::core::option::Option<customer_extension_setting_operation::Operation>,
}
/// Nested message and enum types in `CustomerExtensionSettingOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateCustomerExtensionSettingResult>,
}
/// The result for the customer extension setting mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerExtensionSettingResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated CustomerExtensionSetting with only mutable fields after mutate.
    /// The field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub customer_extension_setting:
        ::core::option::Option<super::resources::CustomerExtensionSetting>,
}
#[doc = r" Generated client implementations."]
pub mod customer_extension_setting_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage customer extension settings."]
    #[derive(Debug, Clone)]
    pub struct CustomerExtensionSettingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerExtensionSettingServiceClient<T>
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
        ) -> CustomerExtensionSettingServiceClient<InterceptedService<T, F>>
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
            CustomerExtensionSettingServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes customer extension settings. Operation"]
        #[doc = " statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [CriterionError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [ExtensionSettingError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [ListOperationError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
        #[doc = "   [UrlFieldError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CustomerExtensionSettingService/MutateCustomerExtensionSettings") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CustomerFeedService.MutateCustomerFeeds][google.ads.googleads.v10.services.CustomerFeedService.MutateCustomerFeeds\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerFeedsRequest {
    /// Required. The ID of the customer whose customer feeds are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual customer feeds.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CustomerFeedOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on a customer feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerFeedOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "customer_feed_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<customer_feed_operation::Operation>,
}
/// Nested message and enum types in `CustomerFeedOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateCustomerFeedResult>,
}
/// The result for the customer feed mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerFeedResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated customer feed with only mutable fields after mutate. The field
    /// will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub customer_feed: ::core::option::Option<super::resources::CustomerFeed>,
}
#[doc = r" Generated client implementations."]
pub mod customer_feed_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage customer feeds."]
    #[derive(Debug, Clone)]
    pub struct CustomerFeedServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerFeedServiceClient<T>
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
        ) -> CustomerFeedServiceClient<InterceptedService<T, F>>
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
            CustomerFeedServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes customer feeds. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [CustomerFeedError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [FunctionError]()"]
        #[doc = "   [FunctionParsingError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
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
                "/google.ads.googleads.v10.services.CustomerFeedService/MutateCustomerFeeds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CustomerLabelService.MutateCustomerLabels][google.ads.googleads.v10.services.CustomerLabelService.MutateCustomerLabels\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerLabelsRequest {
    /// Required. ID of the customer whose customer-label relationships are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on customer-label relationships.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CustomerLabelOperation>,
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
    pub operation: ::core::option::Option<customer_label_operation::Operation>,
}
/// Nested message and enum types in `CustomerLabelOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateCustomerLabelResult>,
}
/// The result for a customer label mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerLabelResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod customer_label_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage labels on customers."]
    #[derive(Debug, Clone)]
    pub struct CustomerLabelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerLabelServiceClient<T>
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
        ) -> CustomerLabelServiceClient<InterceptedService<T, F>>
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
            CustomerLabelServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates and removes customer-label relationships."]
        #[doc = " Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [LabelError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.CustomerLabelService/MutateCustomerLabels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[CustomerNegativeCriterionService.MutateCustomerNegativeCriteria][google.ads.googleads.v10.services.CustomerNegativeCriterionService.MutateCustomerNegativeCriteria\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerNegativeCriteriaRequest {
    /// Required. The ID of the customer whose criteria are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual criteria.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CustomerNegativeCriterionOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create or remove) on a customer level negative criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerNegativeCriterionOperation {
    /// The mutate operation.
    #[prost(
        oneof = "customer_negative_criterion_operation::Operation",
        tags = "1, 2"
    )]
    pub operation: ::core::option::Option<customer_negative_criterion_operation::Operation>,
}
/// Nested message and enum types in `CustomerNegativeCriterionOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateCustomerNegativeCriteriaResult>,
}
/// The result for the criterion mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerNegativeCriteriaResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated criterion with only mutable fields after mutate. The field will
    /// only be returned when response_content_type is set to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub customer_negative_criterion:
        ::core::option::Option<super::resources::CustomerNegativeCriterion>,
}
#[doc = r" Generated client implementations."]
pub mod customer_negative_criterion_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage customer negative criteria."]
    #[derive(Debug, Clone)]
    pub struct CustomerNegativeCriterionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerNegativeCriterionServiceClient<T>
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
        ) -> CustomerNegativeCriterionServiceClient<InterceptedService<T, F>>
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
            CustomerNegativeCriterionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates or removes criteria. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CriterionError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CustomerNegativeCriterionService/MutateCustomerNegativeCriteria") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CustomerService.MutateCustomer][google.ads.googleads.v10.services.CustomerService.MutateCustomer\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerRequest {
    /// Required. The ID of the customer being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The operation to perform on the customer
    #[prost(message, optional, tag = "4")]
    pub operation: ::core::option::Option<CustomerOperation>,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "6"
    )]
    pub response_content_type: i32,
}
/// Request message for \[CustomerService.CreateCustomerClient][google.ads.googleads.v10.services.CustomerService.CreateCustomerClient\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCustomerClientRequest {
    /// Required. The ID of the Manager under whom client customer is being created.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The new client customer to create. The resource name on this customer
    /// will be ignored.
    #[prost(message, optional, tag = "2")]
    pub customer_client: ::core::option::Option<super::resources::Customer>,
    /// Email address of the user who should be invited on the created client
    /// customer. Accessible only to customers on the allow-list.
    #[prost(string, optional, tag = "5")]
    pub email_address: ::core::option::Option<::prost::alloc::string::String>,
    /// The proposed role of user on the created client customer.
    /// Accessible only to customers on the allow-list.
    #[prost(enumeration = "super::enums::access_role_enum::AccessRole", tag = "4")]
    pub access_role: i32,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "6")]
    pub validate_only: bool,
}
/// A single update on a customer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerOperation {
    /// Mutate operation. Only updates are supported for customer.
    #[prost(message, optional, tag = "1")]
    pub update: ::core::option::Option<super::resources::Customer>,
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Response message for CreateCustomerClient mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCustomerClientResponse {
    /// The resource name of the newly created customer client.
    #[prost(string, tag = "2")]
    pub resource_name: ::prost::alloc::string::String,
    /// Link for inviting user to access the created customer. Accessible to
    /// allowlisted customers only.
    #[prost(string, tag = "3")]
    pub invitation_link: ::prost::alloc::string::String,
}
/// Response message for customer mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerResponse {
    /// Result for the mutate.
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<MutateCustomerResult>,
}
/// The result for the customer mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated customer with only mutable fields after mutate. The fields will
    /// only be returned when response_content_type is set to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub customer: ::core::option::Option<super::resources::Customer>,
}
/// Request message for \[CustomerService.ListAccessibleCustomers][google.ads.googleads.v10.services.CustomerService.ListAccessibleCustomers\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccessibleCustomersRequest {}
/// Response message for \[CustomerService.ListAccessibleCustomers][google.ads.googleads.v10.services.CustomerService.ListAccessibleCustomers\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccessibleCustomersResponse {
    /// Resource name of customers directly accessible by the
    /// user authenticating the call.
    #[prost(string, repeated, tag = "1")]
    pub resource_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[doc = r" Generated client implementations."]
pub mod customer_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage customers."]
    #[derive(Debug, Clone)]
    pub struct CustomerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerServiceClient<T>
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
        ) -> CustomerServiceClient<InterceptedService<T, F>>
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
            CustomerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Updates a customer. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [UrlFieldError]()"]
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
                "/google.ads.googleads.v10.services.CustomerService/MutateCustomer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns resource names of customers directly accessible by the"]
        #[doc = " user authenticating the call."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.CustomerService/ListAccessibleCustomers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new client under manager. The new client customer is returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AccessInvitationError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CurrencyCodeError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [ManagerLinkError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [StringLengthError]()"]
        #[doc = "   [TimeZoneError]()"]
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
                "/google.ads.googleads.v10.services.CustomerService/CreateCustomerClient",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[CustomizerAttributeService.MutateCustomizerAttributes][google.ads.googleads.v10.services.CustomizerAttributeService.MutateCustomizerAttributes\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomizerAttributesRequest {
    /// Required. The ID of the customer whose customizer attributes are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual customizer attributes.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CustomizerAttributeOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove) on an customizer attribute.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomizerAttributeOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "customizer_attribute_operation::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<customizer_attribute_operation::Operation>,
}
/// Nested message and enum types in `CustomizerAttributeOperation`.
pub mod customizer_attribute_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new customizer
        /// attribute
        #[prost(message, tag = "1")]
        Create(super::super::resources::CustomizerAttribute),
        /// Remove operation: A resource name for the removed customizer attribute is
        /// expected, in this format:
        /// `customers/{customer_id}/customizerAttributes/{customizer_attribute_id}`
        #[prost(string, tag = "2")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for an customizer attribute mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomizerAttributesResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateCustomizerAttributeResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The result for the customizer attribute mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomizerAttributeResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated CustomizerAttribute with only mutable fields after mutate.
    /// The field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub customizer_attribute: ::core::option::Option<super::resources::CustomizerAttribute>,
}
#[doc = r" Generated client implementations."]
pub mod customizer_attribute_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage customizer attribute"]
    #[derive(Debug, Clone)]
    pub struct CustomizerAttributeServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomizerAttributeServiceClient<T>
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
        ) -> CustomizerAttributeServiceClient<InterceptedService<T, F>>
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
            CustomizerAttributeServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates or removes customizer attributes. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_customizer_attributes(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCustomizerAttributesRequest>,
        ) -> Result<tonic::Response<super::MutateCustomizerAttributesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CustomizerAttributeService/MutateCustomizerAttributes") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[ExperimentArmService.MutateExperimentArms][google.ads.googleads.v10.services.ExperimentArmService.MutateExperimentArms\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateExperimentArmsRequest {
    /// Required. The ID of the customer whose experiments are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual experiment arm.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<ExperimentArmOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation on an experiment arm.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExperimentArmOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "experiment_arm_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<experiment_arm_operation::Operation>,
}
/// Nested message and enum types in `ExperimentArmOperation`.
pub mod experiment_arm_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation
        #[prost(message, tag = "1")]
        Create(super::super::resources::ExperimentArm),
        /// Update operation: The experiment arm is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::ExperimentArm),
        /// Remove operation: The experiment arm is expected to have a valid
        /// resource name, in this format:
        ///
        /// `customers/{customer_id}/experiments/{campaign_experiment_id}`
        #[prost(string, tag = "3")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for experiment arm mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateExperimentArmsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "1")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateExperimentArmResult>,
}
/// The result for the experiment arm mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateExperimentArmResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated experiment arm with only mutable fields after mutate. The
    /// field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub experiment_arm: ::core::option::Option<super::resources::ExperimentArm>,
}
#[doc = r" Generated client implementations."]
pub mod experiment_arm_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage experiment arms."]
    #[derive(Debug, Clone)]
    pub struct ExperimentArmServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ExperimentArmServiceClient<T>
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
        ) -> ExperimentArmServiceClient<InterceptedService<T, F>>
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
            ExperimentArmServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes experiment arms. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [ExperimentArmError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn mutate_experiment_arms(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateExperimentArmsRequest>,
        ) -> Result<tonic::Response<super::MutateExperimentArmsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.ExperimentArmService/MutateExperimentArms",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[ExperimentService.MutateExperiments][google.ads.googleads.v10.services.ExperimentService.MutateExperiments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateExperimentsRequest {
    /// Required. The ID of the customer whose experiments are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual experiments.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<ExperimentOperation>,
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
/// A single operation on an experiment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExperimentOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "experiment_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<experiment_operation::Operation>,
}
/// Nested message and enum types in `ExperimentOperation`.
pub mod experiment_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation
        #[prost(message, tag = "1")]
        Create(super::super::resources::Experiment),
        /// Update operation: The experiment is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::Experiment),
        /// Remove operation: The experiment is expected to have a valid
        /// resource name, in this format:
        ///
        /// `customers/{customer_id}/experiments/{campaign_experiment_id}`
        #[prost(string, tag = "3")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for experiment mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateExperimentsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "1")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateExperimentResult>,
}
/// The result for the campaign experiment mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateExperimentResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// Request message for \[ExperimentService.EndExperiment][google.ads.googleads.v10.services.ExperimentService.EndExperiment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndExperimentRequest {
    /// Required. The resource name of the campaign experiment to end.
    #[prost(string, tag = "1")]
    pub experiment: ::prost::alloc::string::String,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
}
/// Request message for
/// \[ExperimentService.ListExperimentAsyncErrors][google.ads.googleads.v10.services.ExperimentService.ListExperimentAsyncErrors\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExperimentAsyncErrorsRequest {
    /// Required. The name of the experiment from which to retrieve the async
    /// errors.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Token of the page to retrieve. If not specified, the first
    /// page of results will be returned. Use the value obtained from
    /// `next_page_token` in the previous response in order to request
    /// the next page of results.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Number of elements to retrieve in a single page.
    /// When a page request is too large, the server may decide to
    /// further limit the number of returned resources.
    /// The maximum page size is 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// Response message for
/// \[ExperimentService.ListExperimentAsyncErrors][google.ads.googleads.v10.services.ExperimentService.ListExperimentAsyncErrors\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExperimentAsyncErrorsResponse {
    /// details of the errors when performing the asynchronous operation.
    #[prost(message, repeated, tag = "1")]
    pub errors: ::prost::alloc::vec::Vec<super::super::super::super::rpc::Status>,
    /// Pagination token used to retrieve the next page of results.
    /// Pass the content of this string as the `page_token` attribute of
    /// the next request. `next_page_token` is not returned for the last
    /// page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for \[ExperimentService.GraduateExperiment][google.ads.googleads.v10.services.ExperimentService.GraduateExperiment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GraduateExperimentRequest {
    /// Required. The experiment to be graduated.
    #[prost(string, tag = "1")]
    pub experiment: ::prost::alloc::string::String,
    /// Required. List of campaign budget mappings for graduation. Each campaign that
    /// appears here will graduate, and will be assigned a new budget that is
    /// paired with it in the mapping. The maximum size is one.
    #[prost(message, repeated, tag = "2")]
    pub campaign_budget_mappings: ::prost::alloc::vec::Vec<CampaignBudgetMapping>,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// The mapping of experiment campaign and budget to be graduated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignBudgetMapping {
    /// Required. The experiment campaign to graduate.
    #[prost(string, tag = "1")]
    pub experiment_campaign: ::prost::alloc::string::String,
    /// Required. The budget that should be attached to the graduating experiment campaign.
    #[prost(string, tag = "2")]
    pub campaign_budget: ::prost::alloc::string::String,
}
/// Request message for \[ExperimentService.ScheduleExperiment][google.ads.googleads.v10.services.ExperimentService.ScheduleExperiment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleExperimentRequest {
    /// Required. The scheduled experiment.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
}
/// The metadata of the scheduled experiment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleExperimentMetadata {
    /// Required. The scheduled experiment.
    #[prost(string, tag = "1")]
    pub experiment: ::prost::alloc::string::String,
}
/// Request message for \[ExperimentService.PromoteExperiment][google.ads.googleads.v10.services.ExperimentService.PromoteExperiment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromoteExperimentRequest {
    /// Required. The resource name of the experiment to promote.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
}
/// The metadata of the promoted experiment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromoteExperimentMetadata {
    /// Required. The promoted experiment.
    #[prost(string, tag = "1")]
    pub experiment: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod experiment_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage experiments."]
    #[derive(Debug, Clone)]
    pub struct ExperimentServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ExperimentServiceClient<T>
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
        ) -> ExperimentServiceClient<InterceptedService<T, F>>
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
            ExperimentServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes experiments. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [ExperimentError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn mutate_experiments(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateExperimentsRequest>,
        ) -> Result<tonic::Response<super::MutateExperimentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.ExperimentService/MutateExperiments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Immediately ends an experiment, changing the experiment's scheduled"]
        #[doc = " end date and without waiting for end of day. End date is updated to be the"]
        #[doc = " time of the request."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [ExperimentError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn end_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::EndExperimentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.ExperimentService/EndExperiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns all errors that occurred during the last Experiment update (either"]
        #[doc = " scheduling or promotion)."]
        #[doc = " Supports standard list paging."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn list_experiment_async_errors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExperimentAsyncErrorsRequest>,
        ) -> Result<tonic::Response<super::ListExperimentAsyncErrorsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.ExperimentService/ListExperimentAsyncErrors",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Graduates an experiment to a full campaign."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [ExperimentError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn graduate_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::GraduateExperimentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.ExperimentService/GraduateExperiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Schedule an experiment. The in design campaign"]
        #[doc = " will be converted into a real campaign (called the experiment campaign)"]
        #[doc = " that will begin serving ads if successfully created."]
        #[doc = ""]
        #[doc = " The experiment is scheduled immediately with status INITIALIZING."]
        #[doc = " This method returns a long running operation that tracks the forking of the"]
        #[doc = " in design campaign. If the forking fails, a list of errors can be retrieved"]
        #[doc = " using the ListExperimentAsyncErrors method. The operation's"]
        #[doc = " metadata will be a string containing the resource name of the created"]
        #[doc = " experiment."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [ExperimentError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DateRangeError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn schedule_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::ScheduleExperimentRequest>,
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
                "/google.ads.googleads.v10.services.ExperimentService/ScheduleExperiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Promotes the trial campaign thus applying changes in the trial campaign"]
        #[doc = " to the base campaign."]
        #[doc = " This method returns a long running operation that tracks the promotion of"]
        #[doc = " the experiment campaign. If it fails, a list of errors can be retrieved"]
        #[doc = " using the ListExperimentAsyncErrors method. The operation's"]
        #[doc = " metadata will be a string containing the resource name of the created"]
        #[doc = " experiment."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [ExperimentError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn promote_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::PromoteExperimentRequest>,
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
                "/google.ads.googleads.v10.services.ExperimentService/PromoteExperiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[ExtensionFeedItemService.MutateExtensionFeedItems][google.ads.googleads.v10.services.ExtensionFeedItemService.MutateExtensionFeedItems\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateExtensionFeedItemsRequest {
    /// Required. The ID of the customer whose extension feed items are being
    /// modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual extension feed items.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<ExtensionFeedItemOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on an extension feed item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionFeedItemOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "extension_feed_item_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<extension_feed_item_operation::Operation>,
}
/// Nested message and enum types in `ExtensionFeedItemOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateExtensionFeedItemResult>,
}
/// The result for the extension feed item mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateExtensionFeedItemResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated extension feed item with only mutable fields after mutate. The
    /// field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub extension_feed_item: ::core::option::Option<super::resources::ExtensionFeedItem>,
}
#[doc = r" Generated client implementations."]
pub mod extension_feed_item_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage extension feed items."]
    #[derive(Debug, Clone)]
    pub struct ExtensionFeedItemServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ExtensionFeedItemServiceClient<T>
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
        ) -> ExtensionFeedItemServiceClient<InterceptedService<T, F>>
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
            ExtensionFeedItemServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes extension feed items. Operation"]
        #[doc = " statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [CountryCodeError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [ExtensionFeedItemError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [ImageError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [LanguageCodeError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [OperationAccessDeniedError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringLengthError]()"]
        #[doc = "   [UrlFieldError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.ExtensionFeedItemService/MutateExtensionFeedItems") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[FeedItemService.MutateFeedItems][google.ads.googleads.v10.services.FeedItemService.MutateFeedItems\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedItemsRequest {
    /// Required. The ID of the customer whose feed items are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual feed items.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<FeedItemOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on an feed item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "feed_item_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<feed_item_operation::Operation>,
}
/// Nested message and enum types in `FeedItemOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateFeedItemResult>,
}
/// The result for the feed item mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedItemResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated feed item with only mutable fields after mutate. The field will
    /// only be returned when response_content_type is set to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub feed_item: ::core::option::Option<super::resources::FeedItem>,
}
#[doc = r" Generated client implementations."]
pub mod feed_item_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage feed items."]
    #[derive(Debug, Clone)]
    pub struct FeedItemServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FeedItemServiceClient<T>
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
        ) -> FeedItemServiceClient<InterceptedService<T, F>>
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
            FeedItemServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes feed items. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [CriterionError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FeedItemError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [ListOperationError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
        #[doc = "   [UrlFieldError]()"]
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
                "/google.ads.googleads.v10.services.FeedItemService/MutateFeedItems",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[FeedItemSetLinkService.MutateFeedItemSetLinks][google.ads.googleads.v10.services.FeedItemSetLinkService.MutateFeedItemSetLinks\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedItemSetLinksRequest {
    /// Required. The ID of the customer whose feed item set links are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual feed item set links.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<FeedItemSetLinkOperation>,
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
/// A single operation (create, update, remove) on a feed item set link.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemSetLinkOperation {
    /// The mutate operation.
    #[prost(oneof = "feed_item_set_link_operation::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<feed_item_set_link_operation::Operation>,
}
/// Nested message and enum types in `FeedItemSetLinkOperation`.
pub mod feed_item_set_link_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the
        /// new feed item set link.
        #[prost(message, tag = "1")]
        Create(super::super::resources::FeedItemSetLink),
        /// Remove operation: A resource name for the removed feed item set link is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/feedItemSetLinks/{feed_id}_{feed_item_set_id}_{feed_item_id}`
        #[prost(string, tag = "2")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for a feed item set link mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedItemSetLinksResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateFeedItemSetLinkResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The result for the feed item set link mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedItemSetLinkResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod feed_item_set_link_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage feed item set links."]
    #[derive(Debug, Clone)]
    pub struct FeedItemSetLinkServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FeedItemSetLinkServiceClient<T>
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
        ) -> FeedItemSetLinkServiceClient<InterceptedService<T, F>>
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
            FeedItemSetLinkServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes feed item set links."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn mutate_feed_item_set_links(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateFeedItemSetLinksRequest>,
        ) -> Result<tonic::Response<super::MutateFeedItemSetLinksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.FeedItemSetLinkService/MutateFeedItemSetLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[FeedItemSetService.MutateFeedItemSets][google.ads.googleads.v10.services.FeedItemSetService.MutateFeedItemSets\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedItemSetsRequest {
    /// Required. The ID of the customer whose feed item sets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual feed item sets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<FeedItemSetOperation>,
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
/// A single operation (create, remove) on an feed item set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemSetOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "feed_item_set_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<feed_item_set_operation::Operation>,
}
/// Nested message and enum types in `FeedItemSetOperation`.
pub mod feed_item_set_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new feed item set
        #[prost(message, tag = "1")]
        Create(super::super::resources::FeedItemSet),
        /// Update operation: The feed item set is expected to have a valid resource
        /// name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::FeedItemSet),
        /// Remove operation: A resource name for the removed feed item is
        /// expected, in this format:
        /// `customers/{customer_id}/feedItems/{feed_id}~{feed_item_set_id}`
        #[prost(string, tag = "3")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for an feed item set mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedItemSetsResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateFeedItemSetResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The result for the feed item set mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedItemSetResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod feed_item_set_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage feed Item Set"]
    #[derive(Debug, Clone)]
    pub struct FeedItemSetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FeedItemSetServiceClient<T>
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
        ) -> FeedItemSetServiceClient<InterceptedService<T, F>>
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
            FeedItemSetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates or removes feed item sets. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn mutate_feed_item_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateFeedItemSetsRequest>,
        ) -> Result<tonic::Response<super::MutateFeedItemSetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.FeedItemSetService/MutateFeedItemSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[FeedItemTargetService.MutateFeedItemTargets][google.ads.googleads.v10.services.FeedItemTargetService.MutateFeedItemTargets\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedItemTargetsRequest {
    /// Required. The ID of the customer whose feed item targets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual feed item targets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<FeedItemTargetOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "4")]
    pub partial_failure: bool,
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// A single operation (create, remove) on an feed item target.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemTargetOperation {
    /// The mutate operation.
    #[prost(oneof = "feed_item_target_operation::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<feed_item_target_operation::Operation>,
}
/// Nested message and enum types in `FeedItemTargetOperation`.
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
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for an feed item target mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedItemTargetsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateFeedItemTargetResult>,
}
/// The result for the feed item target mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedItemTargetResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated feed item target with only mutable fields after mutate. The
    /// field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub feed_item_target: ::core::option::Option<super::resources::FeedItemTarget>,
}
#[doc = r" Generated client implementations."]
pub mod feed_item_target_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage feed item targets."]
    #[derive(Debug, Clone)]
    pub struct FeedItemTargetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FeedItemTargetServiceClient<T>
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
        ) -> FeedItemTargetServiceClient<InterceptedService<T, F>>
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
            FeedItemTargetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates or removes feed item targets. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CriterionError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FeedItemTargetError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
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
                "/google.ads.googleads.v10.services.FeedItemTargetService/MutateFeedItemTargets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[FeedMappingService.MutateFeedMappings][google.ads.googleads.v10.services.FeedMappingService.MutateFeedMappings\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedMappingsRequest {
    /// Required. The ID of the customer whose feed mappings are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual feed mappings.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<FeedMappingOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove) on a feed mapping.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMappingOperation {
    /// The mutate operation.
    #[prost(oneof = "feed_mapping_operation::Operation", tags = "1, 3")]
    pub operation: ::core::option::Option<feed_mapping_operation::Operation>,
}
/// Nested message and enum types in `FeedMappingOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateFeedMappingResult>,
}
/// The result for the feed mapping mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedMappingResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated feed mapping with only mutable fields after mutate. The field
    /// will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub feed_mapping: ::core::option::Option<super::resources::FeedMapping>,
}
#[doc = r" Generated client implementations."]
pub mod feed_mapping_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage feed mappings."]
    #[derive(Debug, Clone)]
    pub struct FeedMappingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FeedMappingServiceClient<T>
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
        ) -> FeedMappingServiceClient<InterceptedService<T, F>>
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
            FeedMappingServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates or removes feed mappings. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FeedMappingError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [OperationAccessDeniedError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
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
                "/google.ads.googleads.v10.services.FeedMappingService/MutateFeedMappings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[FeedService.MutateFeeds][google.ads.googleads.v10.services.FeedService.MutateFeeds\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedsRequest {
    /// Required. The ID of the customer whose feeds are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual feeds.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<FeedOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on an feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "feed_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<feed_operation::Operation>,
}
/// Nested message and enum types in `FeedOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateFeedResult>,
}
/// The result for the feed mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated feed with only mutable fields after mutate. The field will only
    /// be returned when response_content_type is set to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub feed: ::core::option::Option<super::resources::Feed>,
}
#[doc = r" Generated client implementations."]
pub mod feed_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage feeds."]
    #[derive(Debug, Clone)]
    pub struct FeedServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FeedServiceClient<T>
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
        ) -> FeedServiceClient<InterceptedService<T, F>>
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
            FeedServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes feeds. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FeedError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [ListOperationError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
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
                "/google.ads.googleads.v10.services.FeedService/MutateFeeds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[KeywordPlanAdGroupKeywordService.MutateKeywordPlanAdGroupKeywords][google.ads.googleads.v10.services.KeywordPlanAdGroupKeywordService.MutateKeywordPlanAdGroupKeywords\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanAdGroupKeywordsRequest {
    /// Required. The ID of the customer whose Keyword Plan ad group keywords are being
    /// modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual Keyword Plan ad group
    /// keywords.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<KeywordPlanAdGroupKeywordOperation>,
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
/// A single operation (create, update, remove) on a Keyword Plan ad group
/// keyword.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanAdGroupKeywordOperation {
    /// The FieldMask that determines which resource fields are modified in an
    /// update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(
        oneof = "keyword_plan_ad_group_keyword_operation::Operation",
        tags = "1, 2, 3"
    )]
    pub operation: ::core::option::Option<keyword_plan_ad_group_keyword_operation::Operation>,
}
/// Nested message and enum types in `KeywordPlanAdGroupKeywordOperation`.
pub mod keyword_plan_ad_group_keyword_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new Keyword Plan
        /// ad group keyword.
        #[prost(message, tag = "1")]
        Create(super::super::resources::KeywordPlanAdGroupKeyword),
        /// Update operation: The Keyword Plan ad group keyword is expected to have a
        /// valid resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::KeywordPlanAdGroupKeyword),
        /// Remove operation: A resource name for the removed Keyword Plan ad group
        /// keyword is expected, in this format:
        ///
        /// `customers/{customer_id}/keywordPlanAdGroupKeywords/{kp_ad_group_keyword_id}`
        #[prost(string, tag = "3")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for a Keyword Plan ad group keyword mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanAdGroupKeywordsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateKeywordPlanAdGroupKeywordResult>,
}
/// The result for the Keyword Plan ad group keyword mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanAdGroupKeywordResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod keyword_plan_ad_group_keyword_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage Keyword Plan ad group keywords. KeywordPlanAdGroup is"]
    #[doc = " required to add ad group keywords. Positive and negative keywords are"]
    #[doc = " supported. A maximum of 10,000 positive keywords are allowed per keyword"]
    #[doc = " plan. A maximum of 1,000 negative keywords are allower per keyword plan. This"]
    #[doc = " includes campaign negative keywords and ad group negative keywords."]
    #[derive(Debug, Clone)]
    pub struct KeywordPlanAdGroupKeywordServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> KeywordPlanAdGroupKeywordServiceClient<T>
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
        ) -> KeywordPlanAdGroupKeywordServiceClient<InterceptedService<T, F>>
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
            KeywordPlanAdGroupKeywordServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes Keyword Plan ad group keywords. Operation"]
        #[doc = " statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [KeywordPlanAdGroupKeywordError]()"]
        #[doc = "   [KeywordPlanError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        pub async fn mutate_keyword_plan_ad_group_keywords(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateKeywordPlanAdGroupKeywordsRequest>,
        ) -> Result<tonic::Response<super::MutateKeywordPlanAdGroupKeywordsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.KeywordPlanAdGroupKeywordService/MutateKeywordPlanAdGroupKeywords") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[KeywordPlanAdGroupService.MutateKeywordPlanAdGroups][google.ads.googleads.v10.services.KeywordPlanAdGroupService.MutateKeywordPlanAdGroups\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanAdGroupsRequest {
    /// Required. The ID of the customer whose Keyword Plan ad groups are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual Keyword Plan ad groups.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<KeywordPlanAdGroupOperation>,
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
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "keyword_plan_ad_group_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<keyword_plan_ad_group_operation::Operation>,
}
/// Nested message and enum types in `KeywordPlanAdGroupOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate. The order of the results is determined by the
    /// order of the keywords in the original request.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateKeywordPlanAdGroupResult>,
}
/// The result for the Keyword Plan ad group mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanAdGroupResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod keyword_plan_ad_group_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage Keyword Plan ad groups."]
    #[derive(Debug, Clone)]
    pub struct KeywordPlanAdGroupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> KeywordPlanAdGroupServiceClient<T>
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
        ) -> KeywordPlanAdGroupServiceClient<InterceptedService<T, F>>
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
            KeywordPlanAdGroupServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes Keyword Plan ad groups. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [KeywordPlanAdGroupError]()"]
        #[doc = "   [KeywordPlanError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.KeywordPlanAdGroupService/MutateKeywordPlanAdGroups") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[KeywordPlanCampaignKeywordService.MutateKeywordPlanCampaignKeywords][google.ads.googleads.v10.services.KeywordPlanCampaignKeywordService.MutateKeywordPlanCampaignKeywords\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanCampaignKeywordsRequest {
    /// Required. The ID of the customer whose campaign keywords are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual Keyword Plan campaign
    /// keywords.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<KeywordPlanCampaignKeywordOperation>,
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
/// A single operation (create, update, remove) on a Keyword Plan campaign
/// keyword.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanCampaignKeywordOperation {
    /// The FieldMask that determines which resource fields are modified in an
    /// update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(
        oneof = "keyword_plan_campaign_keyword_operation::Operation",
        tags = "1, 2, 3"
    )]
    pub operation: ::core::option::Option<keyword_plan_campaign_keyword_operation::Operation>,
}
/// Nested message and enum types in `KeywordPlanCampaignKeywordOperation`.
pub mod keyword_plan_campaign_keyword_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new Keyword Plan
        /// campaign keyword.
        #[prost(message, tag = "1")]
        Create(super::super::resources::KeywordPlanCampaignKeyword),
        /// Update operation: The Keyword Plan campaign keyword expected to have a
        /// valid resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::KeywordPlanCampaignKeyword),
        /// Remove operation: A resource name for the removed Keyword Plan campaign
        /// keywords expected in this format:
        ///
        /// `customers/{customer_id}/keywordPlanCampaignKeywords/{kp_campaign_keyword_id}`
        #[prost(string, tag = "3")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for a Keyword Plan campaign keyword mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanCampaignKeywordsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateKeywordPlanCampaignKeywordResult>,
}
/// The result for the Keyword Plan campaign keyword mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanCampaignKeywordResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod keyword_plan_campaign_keyword_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage Keyword Plan campaign keywords. KeywordPlanCampaign is"]
    #[doc = " required to add the campaign keywords. Only negative keywords are supported."]
    #[doc = " A maximum of 1000 negative keywords are allowed per plan. This includes both"]
    #[doc = " campaign negative keywords and ad group negative keywords."]
    #[derive(Debug, Clone)]
    pub struct KeywordPlanCampaignKeywordServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> KeywordPlanCampaignKeywordServiceClient<T>
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
        ) -> KeywordPlanCampaignKeywordServiceClient<InterceptedService<T, F>>
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
            KeywordPlanCampaignKeywordServiceClient::new(InterceptedService::new(
                inner,
                interceptor,
            ))
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
        #[doc = " Creates, updates, or removes Keyword Plan campaign keywords. Operation"]
        #[doc = " statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [KeywordPlanAdGroupKeywordError]()"]
        #[doc = "   [KeywordPlanCampaignKeywordError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        pub async fn mutate_keyword_plan_campaign_keywords(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateKeywordPlanCampaignKeywordsRequest>,
        ) -> Result<tonic::Response<super::MutateKeywordPlanCampaignKeywordsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.KeywordPlanCampaignKeywordService/MutateKeywordPlanCampaignKeywords") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[KeywordPlanCampaignService.MutateKeywordPlanCampaigns][google.ads.googleads.v10.services.KeywordPlanCampaignService.MutateKeywordPlanCampaigns\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanCampaignsRequest {
    /// Required. The ID of the customer whose Keyword Plan campaigns are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual Keyword Plan campaigns.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<KeywordPlanCampaignOperation>,
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
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "keyword_plan_campaign_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<keyword_plan_campaign_operation::Operation>,
}
/// Nested message and enum types in `KeywordPlanCampaignOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateKeywordPlanCampaignResult>,
}
/// The result for the Keyword Plan campaign mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanCampaignResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod keyword_plan_campaign_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage Keyword Plan campaigns."]
    #[derive(Debug, Clone)]
    pub struct KeywordPlanCampaignServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> KeywordPlanCampaignServiceClient<T>
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
        ) -> KeywordPlanCampaignServiceClient<InterceptedService<T, F>>
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
            KeywordPlanCampaignServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes Keyword Plan campaigns. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [KeywordPlanCampaignError]()"]
        #[doc = "   [KeywordPlanError]()"]
        #[doc = "   [ListOperationError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.KeywordPlanCampaignService/MutateKeywordPlanCampaigns") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[KeywordPlanService.MutateKeywordPlans][google.ads.googleads.v10.services.KeywordPlanService.MutateKeywordPlans\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlansRequest {
    /// Required. The ID of the customer whose keyword plans are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual keyword plans.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<KeywordPlanOperation>,
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
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "keyword_plan_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<keyword_plan_operation::Operation>,
}
/// Nested message and enum types in `KeywordPlanOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateKeywordPlansResult>,
}
/// The result for the keyword plan mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlansResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// Request message for \[KeywordPlanService.GenerateForecastCurve][google.ads.googleads.v10.services.KeywordPlanService.GenerateForecastCurve\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateForecastCurveRequest {
    /// Required. The resource name of the keyword plan to be forecasted.
    #[prost(string, tag = "1")]
    pub keyword_plan: ::prost::alloc::string::String,
}
/// Response message for \[KeywordPlanService.GenerateForecastCurve][google.ads.googleads.v10.services.KeywordPlanService.GenerateForecastCurve\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateForecastCurveResponse {
    /// List of forecast curves for the keyword plan campaign.
    /// One maximum.
    #[prost(message, repeated, tag = "1")]
    pub campaign_forecast_curves: ::prost::alloc::vec::Vec<KeywordPlanCampaignForecastCurve>,
}
/// Request message for \[KeywordPlanService.GenerateForecastTimeSeries][google.ads.googleads.v10.services.KeywordPlanService.GenerateForecastTimeSeries\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateForecastTimeSeriesRequest {
    /// Required. The resource name of the keyword plan to be forecasted.
    #[prost(string, tag = "1")]
    pub keyword_plan: ::prost::alloc::string::String,
}
/// Response message for \[KeywordPlanService.GenerateForecastTimeSeries][google.ads.googleads.v10.services.KeywordPlanService.GenerateForecastTimeSeries\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateForecastTimeSeriesResponse {
    /// List of weekly time series forecasts for the keyword plan campaign.
    /// One maximum.
    #[prost(message, repeated, tag = "1")]
    pub weekly_time_series_forecasts: ::prost::alloc::vec::Vec<KeywordPlanWeeklyTimeSeriesForecast>,
}
/// Request message for \[KeywordPlanService.GenerateForecastMetrics][google.ads.googleads.v10.services.KeywordPlanService.GenerateForecastMetrics\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateForecastMetricsRequest {
    /// Required. The resource name of the keyword plan to be forecasted.
    #[prost(string, tag = "1")]
    pub keyword_plan: ::prost::alloc::string::String,
}
/// Response message for \[KeywordPlanService.GenerateForecastMetrics][google.ads.googleads.v10.services.KeywordPlanService.GenerateForecastMetrics\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateForecastMetricsResponse {
    /// List of campaign forecasts.
    /// One maximum.
    #[prost(message, repeated, tag = "1")]
    pub campaign_forecasts: ::prost::alloc::vec::Vec<KeywordPlanCampaignForecast>,
    /// List of ad group forecasts.
    #[prost(message, repeated, tag = "2")]
    pub ad_group_forecasts: ::prost::alloc::vec::Vec<KeywordPlanAdGroupForecast>,
    /// List of keyword forecasts.
    #[prost(message, repeated, tag = "3")]
    pub keyword_forecasts: ::prost::alloc::vec::Vec<KeywordPlanKeywordForecast>,
}
/// A campaign forecast.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanCampaignForecast {
    /// The resource name of the Keyword Plan campaign related to the forecast.
    ///
    /// `customers/{customer_id}/keywordPlanCampaigns/{keyword_plan_campaign_id}`
    #[prost(string, optional, tag = "3")]
    pub keyword_plan_campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// The forecast for the Keyword Plan campaign.
    #[prost(message, optional, tag = "2")]
    pub campaign_forecast: ::core::option::Option<ForecastMetrics>,
}
/// An ad group forecast.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanAdGroupForecast {
    /// The resource name of the Keyword Plan ad group related to the forecast.
    ///
    /// `customers/{customer_id}/keywordPlanAdGroups/{keyword_plan_ad_group_id}`
    #[prost(string, optional, tag = "3")]
    pub keyword_plan_ad_group: ::core::option::Option<::prost::alloc::string::String>,
    /// The forecast for the Keyword Plan ad group.
    #[prost(message, optional, tag = "2")]
    pub ad_group_forecast: ::core::option::Option<ForecastMetrics>,
}
/// A keyword forecast.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanKeywordForecast {
    /// The resource name of the Keyword Plan keyword related to the forecast.
    ///
    /// `customers/{customer_id}/keywordPlanAdGroupKeywords/{keyword_plan_ad_group_keyword_id}`
    #[prost(string, optional, tag = "3")]
    pub keyword_plan_ad_group_keyword: ::core::option::Option<::prost::alloc::string::String>,
    /// The forecast for the Keyword Plan keyword.
    #[prost(message, optional, tag = "2")]
    pub keyword_forecast: ::core::option::Option<ForecastMetrics>,
}
/// The forecast curve for the campaign.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanCampaignForecastCurve {
    /// The resource name of the Keyword Plan campaign related to the forecast.
    ///
    /// `customers/{customer_id}/keywordPlanCampaigns/{keyword_plan_campaign_id}`
    #[prost(string, optional, tag = "3")]
    pub keyword_plan_campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// The max cpc bid forecast curve for the campaign.
    #[prost(message, optional, tag = "2")]
    pub max_cpc_bid_forecast_curve: ::core::option::Option<KeywordPlanMaxCpcBidForecastCurve>,
}
/// The max cpc bid forecast curve.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanMaxCpcBidForecastCurve {
    /// The forecasts for the Keyword Plan campaign at different max CPC bids.
    #[prost(message, repeated, tag = "1")]
    pub max_cpc_bid_forecasts: ::prost::alloc::vec::Vec<KeywordPlanMaxCpcBidForecast>,
}
/// The forecast of the campaign at a specific bid.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanMaxCpcBidForecast {
    /// The max cpc bid in micros.
    #[prost(int64, optional, tag = "3")]
    pub max_cpc_bid_micros: ::core::option::Option<i64>,
    /// The forecast for the Keyword Plan campaign at the specific bid.
    #[prost(message, optional, tag = "2")]
    pub max_cpc_bid_forecast: ::core::option::Option<ForecastMetrics>,
}
/// The weekly time series forecast for the keyword plan campaign.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanWeeklyTimeSeriesForecast {
    /// The resource name of the Keyword Plan campaign related to the forecast.
    ///
    /// `customers/{customer_id}/keywordPlanCampaigns/{keyword_plan_campaign_id}`
    #[prost(string, optional, tag = "1")]
    pub keyword_plan_campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// The forecasts for the Keyword Plan campaign at different max CPC bids.
    #[prost(message, repeated, tag = "2")]
    pub weekly_forecasts: ::prost::alloc::vec::Vec<KeywordPlanWeeklyForecast>,
}
/// The forecast of the campaign for the week starting start_date.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanWeeklyForecast {
    /// The start date, in yyyy-mm-dd format. This date is inclusive.
    #[prost(string, optional, tag = "1")]
    pub start_date: ::core::option::Option<::prost::alloc::string::String>,
    /// The forecast for the Keyword Plan campaign for the week.
    #[prost(message, optional, tag = "2")]
    pub forecast: ::core::option::Option<ForecastMetrics>,
}
/// Forecast metrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForecastMetrics {
    /// Impressions
    #[prost(double, optional, tag = "7")]
    pub impressions: ::core::option::Option<f64>,
    /// Ctr
    #[prost(double, optional, tag = "8")]
    pub ctr: ::core::option::Option<f64>,
    /// AVG cpc
    #[prost(int64, optional, tag = "9")]
    pub average_cpc: ::core::option::Option<i64>,
    /// Clicks
    #[prost(double, optional, tag = "10")]
    pub clicks: ::core::option::Option<f64>,
    /// Cost
    #[prost(int64, optional, tag = "11")]
    pub cost_micros: ::core::option::Option<i64>,
}
/// Request message for \[KeywordPlanService.GenerateHistoricalMetrics][google.ads.googleads.v10.services.KeywordPlanService.GenerateHistoricalMetrics\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateHistoricalMetricsRequest {
    /// Required. The resource name of the keyword plan of which historical metrics are
    /// requested.
    #[prost(string, tag = "1")]
    pub keyword_plan: ::prost::alloc::string::String,
    /// The aggregate fields to include in response.
    #[prost(message, optional, tag = "2")]
    pub aggregate_metrics: ::core::option::Option<super::common::KeywordPlanAggregateMetrics>,
    /// The options for historical metrics data.
    #[prost(message, optional, tag = "3")]
    pub historical_metrics_options: ::core::option::Option<super::common::HistoricalMetricsOptions>,
}
/// Response message for \[KeywordPlanService.GenerateHistoricalMetrics][google.ads.googleads.v10.services.KeywordPlanService.GenerateHistoricalMetrics\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateHistoricalMetricsResponse {
    /// List of keyword historical metrics.
    #[prost(message, repeated, tag = "1")]
    pub metrics: ::prost::alloc::vec::Vec<KeywordPlanKeywordHistoricalMetrics>,
    /// The aggregate metrics for all the keywords in the keyword planner plan.
    #[prost(message, optional, tag = "2")]
    pub aggregate_metric_results:
        ::core::option::Option<super::common::KeywordPlanAggregateMetricResults>,
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
    #[prost(string, optional, tag = "4")]
    pub search_query: ::core::option::Option<::prost::alloc::string::String>,
    /// The list of close variant queries for search_query whose search results
    /// are combined into the search_query.
    #[prost(string, repeated, tag = "3")]
    pub close_variants: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The historical metrics for the query associated with one or more
    /// ad_group_keywords in the plan.
    #[prost(message, optional, tag = "2")]
    pub keyword_metrics: ::core::option::Option<super::common::KeywordPlanHistoricalMetrics>,
}
#[doc = r" Generated client implementations."]
pub mod keyword_plan_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage keyword plans."]
    #[derive(Debug, Clone)]
    pub struct KeywordPlanServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> KeywordPlanServiceClient<T>
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
        ) -> KeywordPlanServiceClient<InterceptedService<T, F>>
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
            KeywordPlanServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes keyword plans. Operation statuses are"]
        #[doc = " returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [KeywordPlanError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        #[doc = "   [StringLengthError]()"]
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
                "/google.ads.googleads.v10.services.KeywordPlanService/MutateKeywordPlans",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the requested Keyword Plan forecast curve."]
        #[doc = " Only the bidding strategy is considered for generating forecast curve."]
        #[doc = " The bidding strategy value specified in the plan is ignored."]
        #[doc = ""]
        #[doc = " To generate a forecast at a value specified in the plan, use"]
        #[doc = " KeywordPlanService.GenerateForecastMetrics."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [KeywordPlanError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn generate_forecast_curve(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateForecastCurveRequest>,
        ) -> Result<tonic::Response<super::GenerateForecastCurveResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.KeywordPlanService/GenerateForecastCurve",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a forecast in the form of a time series for the Keyword Plan over"]
        #[doc = " the next 52 weeks."]
        #[doc = " (1) Forecasts closer to the current date are generally more accurate than"]
        #[doc = " further out."]
        #[doc = ""]
        #[doc = " (2) The forecast reflects seasonal trends using current and"]
        #[doc = " prior traffic patterns. The forecast period of the plan is ignored."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [KeywordPlanError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn generate_forecast_time_series(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateForecastTimeSeriesRequest>,
        ) -> Result<tonic::Response<super::GenerateForecastTimeSeriesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.KeywordPlanService/GenerateForecastTimeSeries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the requested Keyword Plan forecasts."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [KeywordPlanError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.KeywordPlanService/GenerateForecastMetrics",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the requested Keyword Plan historical metrics."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [KeywordPlanError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.KeywordPlanService/GenerateHistoricalMetrics",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[LabelService.MutateLabels][google.ads.googleads.v10.services.LabelService.MutateLabels\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateLabelsRequest {
    /// Required. ID of the customer whose labels are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on labels.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<LabelOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove, update) on a label.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "label_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<label_operation::Operation>,
}
/// Nested message and enum types in `LabelOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateLabelResult>,
}
/// The result for a label mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateLabelResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated label with only mutable fields after mutate. The field will
    /// only be returned when response_content_type is set to "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub label: ::core::option::Option<super::resources::Label>,
}
#[doc = r" Generated client implementations."]
pub mod label_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage labels."]
    #[derive(Debug, Clone)]
    pub struct LabelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LabelServiceClient<T>
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
        ) -> LabelServiceClient<InterceptedService<T, F>>
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
            LabelServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes labels. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [LabelError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
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
                "/google.ads.googleads.v10.services.LabelService/MutateLabels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[MediaFileService.MutateMediaFiles][google.ads.googleads.v10.services.MediaFileService.MutateMediaFiles\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateMediaFilesRequest {
    /// Required. The ID of the customer whose media files are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual media file.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<MediaFileOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation to create media file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaFileOperation {
    /// The mutate operation.
    #[prost(oneof = "media_file_operation::Operation", tags = "1")]
    pub operation: ::core::option::Option<media_file_operation::Operation>,
}
/// Nested message and enum types in `MediaFileOperation`.
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateMediaFileResult>,
}
/// The result for the media file mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateMediaFileResult {
    /// The resource name returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated media file with only mutable fields after mutate. The field
    /// will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub media_file: ::core::option::Option<super::resources::MediaFile>,
}
#[doc = r" Generated client implementations."]
pub mod media_file_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage media files."]
    #[derive(Debug, Clone)]
    pub struct MediaFileServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MediaFileServiceClient<T>
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
        ) -> MediaFileServiceClient<InterceptedService<T, F>>
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
            MediaFileServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates media files. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [ImageError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MediaBundleError]()"]
        #[doc = "   [MediaFileError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
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
                "/google.ads.googleads.v10.services.MediaFileService/MutateMediaFiles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[RemarketingActionService.MutateRemarketingActions][google.ads.googleads.v10.services.RemarketingActionService.MutateRemarketingActions\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateRemarketingActionsRequest {
    /// Required. The ID of the customer whose remarketing actions are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual remarketing actions.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<RemarketingActionOperation>,
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
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "remarketing_action_operation::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<remarketing_action_operation::Operation>,
}
/// Nested message and enum types in `RemarketingActionOperation`.
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateRemarketingActionResult>,
}
/// The result for the remarketing action mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateRemarketingActionResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod remarketing_action_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage remarketing actions."]
    #[derive(Debug, Clone)]
    pub struct RemarketingActionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RemarketingActionServiceClient<T>
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
        ) -> RemarketingActionServiceClient<InterceptedService<T, F>>
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
            RemarketingActionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates or updates remarketing actions. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [ConversionActionError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.RemarketingActionService/MutateRemarketingActions") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[SharedCriterionService.MutateSharedCriteria][google.ads.googleads.v10.services.SharedCriterionService.MutateSharedCriteria\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateSharedCriteriaRequest {
    /// Required. The ID of the customer whose shared criteria are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual shared criteria.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<SharedCriterionOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, remove) on an shared criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedCriterionOperation {
    /// The mutate operation.
    #[prost(oneof = "shared_criterion_operation::Operation", tags = "1, 3")]
    pub operation: ::core::option::Option<shared_criterion_operation::Operation>,
}
/// Nested message and enum types in `SharedCriterionOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateSharedCriterionResult>,
}
/// The result for the shared criterion mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateSharedCriterionResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated shared criterion with only mutable fields after mutate. The
    /// field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub shared_criterion: ::core::option::Option<super::resources::SharedCriterion>,
}
#[doc = r" Generated client implementations."]
pub mod shared_criterion_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage shared criteria."]
    #[derive(Debug, Clone)]
    pub struct SharedCriterionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SharedCriterionServiceClient<T>
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
        ) -> SharedCriterionServiceClient<InterceptedService<T, F>>
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
            SharedCriterionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates or removes shared criteria. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CriterionError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
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
                "/google.ads.googleads.v10.services.SharedCriterionService/MutateSharedCriteria",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[SharedSetService.MutateSharedSets][google.ads.googleads.v10.services.SharedSetService.MutateSharedSets\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateSharedSetsRequest {
    /// Required. The ID of the customer whose shared sets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual shared sets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<SharedSetOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation (create, update, remove) on an shared set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedSetOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "shared_set_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<shared_set_operation::Operation>,
}
/// Nested message and enum types in `SharedSetOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateSharedSetResult>,
}
/// The result for the shared set mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateSharedSetResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated shared set with only mutable fields after mutate. The field
    /// will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub shared_set: ::core::option::Option<super::resources::SharedSet>,
}
#[doc = r" Generated client implementations."]
pub mod shared_set_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage shared sets."]
    #[derive(Debug, Clone)]
    pub struct SharedSetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SharedSetServiceClient<T>
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
        ) -> SharedSetServiceClient<InterceptedService<T, F>>
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
            SharedSetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates, updates, or removes shared sets. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [IdError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperatorError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        #[doc = "   [SharedSetError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
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
                "/google.ads.googleads.v10.services.SharedSetService/MutateSharedSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[SmartCampaignSettingService.MutateSmartCampaignSetting][\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateSmartCampaignSettingsRequest {
    /// Required. The ID of the customer whose Smart campaign settings are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual Smart campaign settings.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<SmartCampaignSettingOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// A single operation to update Smart campaign settings for a campaign.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmartCampaignSettingOperation {
    /// Update operation: The Smart campaign setting must specify a valid
    /// resource name.
    #[prost(message, optional, tag = "1")]
    pub update: ::core::option::Option<super::resources::SmartCampaignSetting>,
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Response message for campaign mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateSmartCampaignSettingsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "1")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateSmartCampaignSettingResult>,
}
/// The result for the Smart campaign setting mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateSmartCampaignSettingResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The mutated Smart campaign setting with only mutable fields after mutate.
    /// The field will only be returned when response_content_type is set to
    /// "MUTABLE_RESOURCE".
    #[prost(message, optional, tag = "2")]
    pub smart_campaign_setting: ::core::option::Option<super::resources::SmartCampaignSetting>,
}
#[doc = r" Generated client implementations."]
pub mod smart_campaign_setting_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage Smart campaign settings."]
    #[derive(Debug, Clone)]
    pub struct SmartCampaignSettingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SmartCampaignSettingServiceClient<T>
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
        ) -> SmartCampaignSettingServiceClient<InterceptedService<T, F>>
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
            SmartCampaignSettingServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Updates Smart campaign settings for campaigns."]
        pub async fn mutate_smart_campaign_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateSmartCampaignSettingsRequest>,
        ) -> Result<tonic::Response<super::MutateSmartCampaignSettingsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.SmartCampaignSettingService/MutateSmartCampaignSettings") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[UserListService.MutateUserLists][google.ads.googleads.v10.services.UserListService.MutateUserLists\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateUserListsRequest {
    /// Required. The ID of the customer whose user lists are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual user lists.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<UserListOperation>,
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
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "user_list_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<user_list_operation::Operation>,
}
/// Nested message and enum types in `UserListOperation`.
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
        Remove(::prost::alloc::string::String),
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
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<MutateUserListResult>,
}
/// The result for the user list mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateUserListResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod user_list_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage user lists."]
    #[derive(Debug, Clone)]
    pub struct UserListServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> UserListServiceClient<T>
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
        ) -> UserListServiceClient<InterceptedService<T, F>>
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
            UserListServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates or updates user lists. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NotAllowlistedError]()"]
        #[doc = "   [NotEmptyError]()"]
        #[doc = "   [OperationAccessDeniedError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
        #[doc = "   [UserListError]()"]
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
                "/google.ads.googleads.v10.services.UserListService/MutateUserLists",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[GoogleAdsService.Search][google.ads.googleads.v10.services.GoogleAdsService.Search\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchGoogleAdsRequest {
    /// Required. The ID of the customer being queried.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The query string.
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// Token of the page to retrieve. If not specified, the first
    /// page of results will be returned. Use the value obtained from
    /// `next_page_token` in the previous response in order to request
    /// the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
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
/// Response message for \[GoogleAdsService.Search][google.ads.googleads.v10.services.GoogleAdsService.Search\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchGoogleAdsResponse {
    /// The list of rows that matched the query.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<GoogleAdsRow>,
    /// Pagination token used to retrieve the next page of results.
    /// Pass the content of this string as the `page_token` attribute of
    /// the next request. `next_page_token` is not returned for the last
    /// page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of results that match the query ignoring the LIMIT
    /// clause.
    #[prost(int64, tag = "3")]
    pub total_results_count: i64,
    /// FieldMask that represents what fields were requested by the user.
    #[prost(message, optional, tag = "5")]
    pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Summary row that contains summary of metrics in results.
    /// Summary of metrics means aggregation of metrics across all results,
    /// here aggregation could be sum, average, rate, etc.
    #[prost(message, optional, tag = "6")]
    pub summary_row: ::core::option::Option<GoogleAdsRow>,
}
/// Request message for \[GoogleAdsService.SearchStream][google.ads.googleads.v10.services.GoogleAdsService.SearchStream\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchGoogleAdsStreamRequest {
    /// Required. The ID of the customer being queried.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The query string.
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// Determines whether a summary row will be returned. By default, summary row
    /// is not returned. If requested, the summary row will be sent in a response
    /// by itself after all other query results are returned.
    #[prost(
        enumeration = "super::enums::summary_row_setting_enum::SummaryRowSetting",
        tag = "3"
    )]
    pub summary_row_setting: i32,
}
/// Response message for \[GoogleAdsService.SearchStream][google.ads.googleads.v10.services.GoogleAdsService.SearchStream\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchGoogleAdsStreamResponse {
    /// The list of rows that matched the query.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<GoogleAdsRow>,
    /// FieldMask that represents what fields were requested by the user.
    #[prost(message, optional, tag = "2")]
    pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Summary row that contains summary of metrics in results.
    /// Summary of metrics means aggregation of metrics across all results,
    /// here aggregation could be sum, average, rate, etc.
    #[prost(message, optional, tag = "3")]
    pub summary_row: ::core::option::Option<GoogleAdsRow>,
    /// The unique id of the request that is used for debugging purposes.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// A returned row from the query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleAdsRow {
    /// The account budget in the query.
    #[prost(message, optional, tag = "42")]
    pub account_budget: ::core::option::Option<super::resources::AccountBudget>,
    /// The account budget proposal referenced in the query.
    #[prost(message, optional, tag = "43")]
    pub account_budget_proposal: ::core::option::Option<super::resources::AccountBudgetProposal>,
    /// The AccountLink referenced in the query.
    #[prost(message, optional, tag = "143")]
    pub account_link: ::core::option::Option<super::resources::AccountLink>,
    /// The ad group referenced in the query.
    #[prost(message, optional, tag = "3")]
    pub ad_group: ::core::option::Option<super::resources::AdGroup>,
    /// The ad referenced in the query.
    #[prost(message, optional, tag = "16")]
    pub ad_group_ad: ::core::option::Option<super::resources::AdGroupAd>,
    /// The ad group ad asset combination view in the query.
    #[prost(message, optional, tag = "193")]
    pub ad_group_ad_asset_combination_view:
        ::core::option::Option<super::resources::AdGroupAdAssetCombinationView>,
    /// The ad group ad asset view in the query.
    #[prost(message, optional, tag = "131")]
    pub ad_group_ad_asset_view: ::core::option::Option<super::resources::AdGroupAdAssetView>,
    /// The ad group ad label referenced in the query.
    #[prost(message, optional, tag = "120")]
    pub ad_group_ad_label: ::core::option::Option<super::resources::AdGroupAdLabel>,
    /// The ad group asset referenced in the query.
    #[prost(message, optional, tag = "154")]
    pub ad_group_asset: ::core::option::Option<super::resources::AdGroupAsset>,
    /// The ad group audience view referenced in the query.
    #[prost(message, optional, tag = "57")]
    pub ad_group_audience_view: ::core::option::Option<super::resources::AdGroupAudienceView>,
    /// The bid modifier referenced in the query.
    #[prost(message, optional, tag = "24")]
    pub ad_group_bid_modifier: ::core::option::Option<super::resources::AdGroupBidModifier>,
    /// The criterion referenced in the query.
    #[prost(message, optional, tag = "17")]
    pub ad_group_criterion: ::core::option::Option<super::resources::AdGroupCriterion>,
    /// The ad group criterion customizer referenced in the query.
    #[prost(message, optional, tag = "187")]
    pub ad_group_criterion_customizer:
        ::core::option::Option<super::resources::AdGroupCriterionCustomizer>,
    /// The ad group criterion label referenced in the query.
    #[prost(message, optional, tag = "121")]
    pub ad_group_criterion_label: ::core::option::Option<super::resources::AdGroupCriterionLabel>,
    /// The ad group criterion simulation referenced in the query.
    #[prost(message, optional, tag = "110")]
    pub ad_group_criterion_simulation:
        ::core::option::Option<super::resources::AdGroupCriterionSimulation>,
    /// The ad group customizer referenced in the query.
    #[prost(message, optional, tag = "185")]
    pub ad_group_customizer: ::core::option::Option<super::resources::AdGroupCustomizer>,
    /// The ad group extension setting referenced in the query.
    #[prost(message, optional, tag = "112")]
    pub ad_group_extension_setting:
        ::core::option::Option<super::resources::AdGroupExtensionSetting>,
    /// The ad group feed referenced in the query.
    #[prost(message, optional, tag = "67")]
    pub ad_group_feed: ::core::option::Option<super::resources::AdGroupFeed>,
    /// The ad group label referenced in the query.
    #[prost(message, optional, tag = "115")]
    pub ad_group_label: ::core::option::Option<super::resources::AdGroupLabel>,
    /// The ad group simulation referenced in the query.
    #[prost(message, optional, tag = "107")]
    pub ad_group_simulation: ::core::option::Option<super::resources::AdGroupSimulation>,
    /// The ad parameter referenced in the query.
    #[prost(message, optional, tag = "130")]
    pub ad_parameter: ::core::option::Option<super::resources::AdParameter>,
    /// The age range view referenced in the query.
    #[prost(message, optional, tag = "48")]
    pub age_range_view: ::core::option::Option<super::resources::AgeRangeView>,
    /// The ad schedule view referenced in the query.
    #[prost(message, optional, tag = "89")]
    pub ad_schedule_view: ::core::option::Option<super::resources::AdScheduleView>,
    /// The domain category referenced in the query.
    #[prost(message, optional, tag = "91")]
    pub domain_category: ::core::option::Option<super::resources::DomainCategory>,
    /// The asset referenced in the query.
    #[prost(message, optional, tag = "105")]
    pub asset: ::core::option::Option<super::resources::Asset>,
    /// The asset field type view referenced in the query.
    #[prost(message, optional, tag = "168")]
    pub asset_field_type_view: ::core::option::Option<super::resources::AssetFieldTypeView>,
    /// The asset group asset referenced in the query.
    #[prost(message, optional, tag = "173")]
    pub asset_group_asset: ::core::option::Option<super::resources::AssetGroupAsset>,
    /// The asset group signal referenced in the query.
    #[prost(message, optional, tag = "191")]
    pub asset_group_signal: ::core::option::Option<super::resources::AssetGroupSignal>,
    /// The asset group listing group filter referenced in the query.
    #[prost(message, optional, tag = "182")]
    pub asset_group_listing_group_filter:
        ::core::option::Option<super::resources::AssetGroupListingGroupFilter>,
    /// The asset group product group view referenced in the query.
    #[prost(message, optional, tag = "189")]
    pub asset_group_product_group_view:
        ::core::option::Option<super::resources::AssetGroupProductGroupView>,
    /// The asset group referenced in the query.
    #[prost(message, optional, tag = "172")]
    pub asset_group: ::core::option::Option<super::resources::AssetGroup>,
    /// The asset set asset referenced in the query.
    #[prost(message, optional, tag = "180")]
    pub asset_set_asset: ::core::option::Option<super::resources::AssetSetAsset>,
    /// The asset set referenced in the query.
    #[prost(message, optional, tag = "179")]
    pub asset_set: ::core::option::Option<super::resources::AssetSet>,
    /// The batch job referenced in the query.
    #[prost(message, optional, tag = "139")]
    pub batch_job: ::core::option::Option<super::resources::BatchJob>,
    /// The bidding data exclusion referenced in the query.
    #[prost(message, optional, tag = "159")]
    pub bidding_data_exclusion: ::core::option::Option<super::resources::BiddingDataExclusion>,
    /// The bidding seasonality adjustment referenced in the query.
    #[prost(message, optional, tag = "160")]
    pub bidding_seasonality_adjustment:
        ::core::option::Option<super::resources::BiddingSeasonalityAdjustment>,
    /// The bidding strategy referenced in the query.
    #[prost(message, optional, tag = "18")]
    pub bidding_strategy: ::core::option::Option<super::resources::BiddingStrategy>,
    /// The bidding strategy simulation referenced in the query.
    #[prost(message, optional, tag = "158")]
    pub bidding_strategy_simulation:
        ::core::option::Option<super::resources::BiddingStrategySimulation>,
    /// The billing setup referenced in the query.
    #[prost(message, optional, tag = "41")]
    pub billing_setup: ::core::option::Option<super::resources::BillingSetup>,
    /// The call view referenced in the query.
    #[prost(message, optional, tag = "152")]
    pub call_view: ::core::option::Option<super::resources::CallView>,
    /// The campaign budget referenced in the query.
    #[prost(message, optional, tag = "19")]
    pub campaign_budget: ::core::option::Option<super::resources::CampaignBudget>,
    /// The campaign referenced in the query.
    #[prost(message, optional, tag = "2")]
    pub campaign: ::core::option::Option<super::resources::Campaign>,
    /// The campaign asset referenced in the query.
    #[prost(message, optional, tag = "142")]
    pub campaign_asset: ::core::option::Option<super::resources::CampaignAsset>,
    /// The campaign asset set referenced in the query.
    #[prost(message, optional, tag = "181")]
    pub campaign_asset_set: ::core::option::Option<super::resources::CampaignAssetSet>,
    /// The campaign audience view referenced in the query.
    #[prost(message, optional, tag = "69")]
    pub campaign_audience_view: ::core::option::Option<super::resources::CampaignAudienceView>,
    /// The campaign bid modifier referenced in the query.
    #[prost(message, optional, tag = "26")]
    pub campaign_bid_modifier: ::core::option::Option<super::resources::CampaignBidModifier>,
    /// The CampaignConversionGoal referenced in the query.
    #[prost(message, optional, tag = "175")]
    pub campaign_conversion_goal: ::core::option::Option<super::resources::CampaignConversionGoal>,
    /// The campaign criterion referenced in the query.
    #[prost(message, optional, tag = "20")]
    pub campaign_criterion: ::core::option::Option<super::resources::CampaignCriterion>,
    /// The campaign criterion simulation referenced in the query.
    #[prost(message, optional, tag = "111")]
    pub campaign_criterion_simulation:
        ::core::option::Option<super::resources::CampaignCriterionSimulation>,
    /// The campaign customizer referenced in the query.
    #[prost(message, optional, tag = "186")]
    pub campaign_customizer: ::core::option::Option<super::resources::CampaignCustomizer>,
    /// The campaign draft referenced in the query.
    #[prost(message, optional, tag = "49")]
    pub campaign_draft: ::core::option::Option<super::resources::CampaignDraft>,
    /// The campaign experiment referenced in the query.
    #[prost(message, optional, tag = "84")]
    pub campaign_experiment: ::core::option::Option<super::resources::CampaignExperiment>,
    /// The campaign extension setting referenced in the query.
    #[prost(message, optional, tag = "113")]
    pub campaign_extension_setting:
        ::core::option::Option<super::resources::CampaignExtensionSetting>,
    /// The campaign feed referenced in the query.
    #[prost(message, optional, tag = "63")]
    pub campaign_feed: ::core::option::Option<super::resources::CampaignFeed>,
    /// Campaign Group referenced in AWQL query.
    #[prost(message, optional, tag = "25")]
    pub campaign_group: ::core::option::Option<super::resources::CampaignGroup>,
    /// The campaign label referenced in the query.
    #[prost(message, optional, tag = "108")]
    pub campaign_label: ::core::option::Option<super::resources::CampaignLabel>,
    /// Campaign Shared Set referenced in AWQL query.
    #[prost(message, optional, tag = "30")]
    pub campaign_shared_set: ::core::option::Option<super::resources::CampaignSharedSet>,
    /// The campaign simulation referenced in the query.
    #[prost(message, optional, tag = "157")]
    pub campaign_simulation: ::core::option::Option<super::resources::CampaignSimulation>,
    /// The carrier constant referenced in the query.
    #[prost(message, optional, tag = "66")]
    pub carrier_constant: ::core::option::Option<super::resources::CarrierConstant>,
    /// The ChangeEvent referenced in the query.
    #[prost(message, optional, tag = "145")]
    pub change_event: ::core::option::Option<super::resources::ChangeEvent>,
    /// The ChangeStatus referenced in the query.
    #[prost(message, optional, tag = "37")]
    pub change_status: ::core::option::Option<super::resources::ChangeStatus>,
    /// The CombinedAudience referenced in the query.
    #[prost(message, optional, tag = "148")]
    pub combined_audience: ::core::option::Option<super::resources::CombinedAudience>,
    /// The Audience referenced in the query.
    #[prost(message, optional, tag = "190")]
    pub audience: ::core::option::Option<super::resources::Audience>,
    /// The conversion action referenced in the query.
    #[prost(message, optional, tag = "103")]
    pub conversion_action: ::core::option::Option<super::resources::ConversionAction>,
    /// The conversion custom variable referenced in the query.
    #[prost(message, optional, tag = "153")]
    pub conversion_custom_variable:
        ::core::option::Option<super::resources::ConversionCustomVariable>,
    /// The ConversionGoalCampaignConfig referenced in the query.
    #[prost(message, optional, tag = "177")]
    pub conversion_goal_campaign_config:
        ::core::option::Option<super::resources::ConversionGoalCampaignConfig>,
    /// The conversion value rule referenced in the query.
    #[prost(message, optional, tag = "164")]
    pub conversion_value_rule: ::core::option::Option<super::resources::ConversionValueRule>,
    /// The conversion value rule set referenced in the query.
    #[prost(message, optional, tag = "165")]
    pub conversion_value_rule_set: ::core::option::Option<super::resources::ConversionValueRuleSet>,
    /// The ClickView referenced in the query.
    #[prost(message, optional, tag = "122")]
    pub click_view: ::core::option::Option<super::resources::ClickView>,
    /// The currency constant referenced in the query.
    #[prost(message, optional, tag = "134")]
    pub currency_constant: ::core::option::Option<super::resources::CurrencyConstant>,
    /// The CustomAudience referenced in the query.
    #[prost(message, optional, tag = "147")]
    pub custom_audience: ::core::option::Option<super::resources::CustomAudience>,
    /// The CustomConversionGoal referenced in the query.
    #[prost(message, optional, tag = "176")]
    pub custom_conversion_goal: ::core::option::Option<super::resources::CustomConversionGoal>,
    /// The CustomInterest referenced in the query.
    #[prost(message, optional, tag = "104")]
    pub custom_interest: ::core::option::Option<super::resources::CustomInterest>,
    /// The customer referenced in the query.
    #[prost(message, optional, tag = "1")]
    pub customer: ::core::option::Option<super::resources::Customer>,
    /// The customer asset referenced in the query.
    #[prost(message, optional, tag = "155")]
    pub customer_asset: ::core::option::Option<super::resources::CustomerAsset>,
    /// The accessible bidding strategy referenced in the query.
    #[prost(message, optional, tag = "169")]
    pub accessible_bidding_strategy:
        ::core::option::Option<super::resources::AccessibleBiddingStrategy>,
    /// The customer customizer referenced in the query.
    #[prost(message, optional, tag = "184")]
    pub customer_customizer: ::core::option::Option<super::resources::CustomerCustomizer>,
    /// The CustomerManagerLink referenced in the query.
    #[prost(message, optional, tag = "61")]
    pub customer_manager_link: ::core::option::Option<super::resources::CustomerManagerLink>,
    /// The CustomerClientLink referenced in the query.
    #[prost(message, optional, tag = "62")]
    pub customer_client_link: ::core::option::Option<super::resources::CustomerClientLink>,
    /// The CustomerClient referenced in the query.
    #[prost(message, optional, tag = "70")]
    pub customer_client: ::core::option::Option<super::resources::CustomerClient>,
    /// The CustomerConversionGoal referenced in the query.
    #[prost(message, optional, tag = "174")]
    pub customer_conversion_goal: ::core::option::Option<super::resources::CustomerConversionGoal>,
    /// The customer extension setting referenced in the query.
    #[prost(message, optional, tag = "114")]
    pub customer_extension_setting:
        ::core::option::Option<super::resources::CustomerExtensionSetting>,
    /// The customer feed referenced in the query.
    #[prost(message, optional, tag = "64")]
    pub customer_feed: ::core::option::Option<super::resources::CustomerFeed>,
    /// The customer label referenced in the query.
    #[prost(message, optional, tag = "124")]
    pub customer_label: ::core::option::Option<super::resources::CustomerLabel>,
    /// The customer negative criterion referenced in the query.
    #[prost(message, optional, tag = "88")]
    pub customer_negative_criterion:
        ::core::option::Option<super::resources::CustomerNegativeCriterion>,
    /// The CustomerUserAccess referenced in the query.
    #[prost(message, optional, tag = "146")]
    pub customer_user_access: ::core::option::Option<super::resources::CustomerUserAccess>,
    /// The CustomerUserAccessInvitation referenced in the query.
    #[prost(message, optional, tag = "150")]
    pub customer_user_access_invitation:
        ::core::option::Option<super::resources::CustomerUserAccessInvitation>,
    /// The customizer attribute referenced in the query.
    #[prost(message, optional, tag = "178")]
    pub customizer_attribute: ::core::option::Option<super::resources::CustomizerAttribute>,
    /// The detail placement view referenced in the query.
    #[prost(message, optional, tag = "118")]
    pub detail_placement_view: ::core::option::Option<super::resources::DetailPlacementView>,
    /// The detailed demographic referenced in the query.
    #[prost(message, optional, tag = "166")]
    pub detailed_demographic: ::core::option::Option<super::resources::DetailedDemographic>,
    /// The display keyword view referenced in the query.
    #[prost(message, optional, tag = "47")]
    pub display_keyword_view: ::core::option::Option<super::resources::DisplayKeywordView>,
    /// The distance view referenced in the query.
    #[prost(message, optional, tag = "132")]
    pub distance_view: ::core::option::Option<super::resources::DistanceView>,
    /// The dynamic search ads search term view referenced in the query.
    #[prost(message, optional, tag = "106")]
    pub dynamic_search_ads_search_term_view:
        ::core::option::Option<super::resources::DynamicSearchAdsSearchTermView>,
    /// The expanded landing page view referenced in the query.
    #[prost(message, optional, tag = "128")]
    pub expanded_landing_page_view:
        ::core::option::Option<super::resources::ExpandedLandingPageView>,
    /// The extension feed item referenced in the query.
    #[prost(message, optional, tag = "85")]
    pub extension_feed_item: ::core::option::Option<super::resources::ExtensionFeedItem>,
    /// The feed referenced in the query.
    #[prost(message, optional, tag = "46")]
    pub feed: ::core::option::Option<super::resources::Feed>,
    /// The feed item referenced in the query.
    #[prost(message, optional, tag = "50")]
    pub feed_item: ::core::option::Option<super::resources::FeedItem>,
    /// The feed item set referenced in the query.
    #[prost(message, optional, tag = "149")]
    pub feed_item_set: ::core::option::Option<super::resources::FeedItemSet>,
    /// The feed item set link referenced in the query.
    #[prost(message, optional, tag = "151")]
    pub feed_item_set_link: ::core::option::Option<super::resources::FeedItemSetLink>,
    /// The feed item target referenced in the query.
    #[prost(message, optional, tag = "116")]
    pub feed_item_target: ::core::option::Option<super::resources::FeedItemTarget>,
    /// The feed mapping referenced in the query.
    #[prost(message, optional, tag = "58")]
    pub feed_mapping: ::core::option::Option<super::resources::FeedMapping>,
    /// The feed placeholder view referenced in the query.
    #[prost(message, optional, tag = "97")]
    pub feed_placeholder_view: ::core::option::Option<super::resources::FeedPlaceholderView>,
    /// The gender view referenced in the query.
    #[prost(message, optional, tag = "40")]
    pub gender_view: ::core::option::Option<super::resources::GenderView>,
    /// The geo target constant referenced in the query.
    #[prost(message, optional, tag = "23")]
    pub geo_target_constant: ::core::option::Option<super::resources::GeoTargetConstant>,
    /// The geographic view referenced in the query.
    #[prost(message, optional, tag = "125")]
    pub geographic_view: ::core::option::Option<super::resources::GeographicView>,
    /// The group placement view referenced in the query.
    #[prost(message, optional, tag = "119")]
    pub group_placement_view: ::core::option::Option<super::resources::GroupPlacementView>,
    /// The hotel group view referenced in the query.
    #[prost(message, optional, tag = "51")]
    pub hotel_group_view: ::core::option::Option<super::resources::HotelGroupView>,
    /// The hotel performance view referenced in the query.
    #[prost(message, optional, tag = "71")]
    pub hotel_performance_view: ::core::option::Option<super::resources::HotelPerformanceView>,
    /// The hotel reconciliation referenced in the query.
    #[prost(message, optional, tag = "188")]
    pub hotel_reconciliation: ::core::option::Option<super::resources::HotelReconciliation>,
    /// The income range view referenced in the query.
    #[prost(message, optional, tag = "138")]
    pub income_range_view: ::core::option::Option<super::resources::IncomeRangeView>,
    /// The keyword view referenced in the query.
    #[prost(message, optional, tag = "21")]
    pub keyword_view: ::core::option::Option<super::resources::KeywordView>,
    /// The keyword plan referenced in the query.
    #[prost(message, optional, tag = "32")]
    pub keyword_plan: ::core::option::Option<super::resources::KeywordPlan>,
    /// The keyword plan campaign referenced in the query.
    #[prost(message, optional, tag = "33")]
    pub keyword_plan_campaign: ::core::option::Option<super::resources::KeywordPlanCampaign>,
    /// The keyword plan campaign keyword referenced in the query.
    #[prost(message, optional, tag = "140")]
    pub keyword_plan_campaign_keyword:
        ::core::option::Option<super::resources::KeywordPlanCampaignKeyword>,
    /// The keyword plan ad group referenced in the query.
    #[prost(message, optional, tag = "35")]
    pub keyword_plan_ad_group: ::core::option::Option<super::resources::KeywordPlanAdGroup>,
    /// The keyword plan ad group referenced in the query.
    #[prost(message, optional, tag = "141")]
    pub keyword_plan_ad_group_keyword:
        ::core::option::Option<super::resources::KeywordPlanAdGroupKeyword>,
    /// The keyword theme constant referenced in the query.
    #[prost(message, optional, tag = "163")]
    pub keyword_theme_constant: ::core::option::Option<super::resources::KeywordThemeConstant>,
    /// The label referenced in the query.
    #[prost(message, optional, tag = "52")]
    pub label: ::core::option::Option<super::resources::Label>,
    /// The landing page view referenced in the query.
    #[prost(message, optional, tag = "126")]
    pub landing_page_view: ::core::option::Option<super::resources::LandingPageView>,
    /// The language constant referenced in the query.
    #[prost(message, optional, tag = "55")]
    pub language_constant: ::core::option::Option<super::resources::LanguageConstant>,
    /// The location view referenced in the query.
    #[prost(message, optional, tag = "123")]
    pub location_view: ::core::option::Option<super::resources::LocationView>,
    /// The managed placement view referenced in the query.
    #[prost(message, optional, tag = "53")]
    pub managed_placement_view: ::core::option::Option<super::resources::ManagedPlacementView>,
    /// The media file referenced in the query.
    #[prost(message, optional, tag = "90")]
    pub media_file: ::core::option::Option<super::resources::MediaFile>,
    /// The mobile app category constant referenced in the query.
    #[prost(message, optional, tag = "87")]
    pub mobile_app_category_constant:
        ::core::option::Option<super::resources::MobileAppCategoryConstant>,
    /// The mobile device constant referenced in the query.
    #[prost(message, optional, tag = "98")]
    pub mobile_device_constant: ::core::option::Option<super::resources::MobileDeviceConstant>,
    /// The offline user data job referenced in the query.
    #[prost(message, optional, tag = "137")]
    pub offline_user_data_job: ::core::option::Option<super::resources::OfflineUserDataJob>,
    /// The operating system version constant referenced in the query.
    #[prost(message, optional, tag = "86")]
    pub operating_system_version_constant:
        ::core::option::Option<super::resources::OperatingSystemVersionConstant>,
    /// The paid organic search term view referenced in the query.
    #[prost(message, optional, tag = "129")]
    pub paid_organic_search_term_view:
        ::core::option::Option<super::resources::PaidOrganicSearchTermView>,
    /// The parental status view referenced in the query.
    #[prost(message, optional, tag = "45")]
    pub parental_status_view: ::core::option::Option<super::resources::ParentalStatusView>,
    /// The Product Bidding Category referenced in the query.
    #[prost(message, optional, tag = "109")]
    pub product_bidding_category_constant:
        ::core::option::Option<super::resources::ProductBiddingCategoryConstant>,
    /// The product group view referenced in the query.
    #[prost(message, optional, tag = "54")]
    pub product_group_view: ::core::option::Option<super::resources::ProductGroupView>,
    /// The recommendation referenced in the query.
    #[prost(message, optional, tag = "22")]
    pub recommendation: ::core::option::Option<super::resources::Recommendation>,
    /// The search term view referenced in the query.
    #[prost(message, optional, tag = "68")]
    pub search_term_view: ::core::option::Option<super::resources::SearchTermView>,
    /// The shared set referenced in the query.
    #[prost(message, optional, tag = "29")]
    pub shared_criterion: ::core::option::Option<super::resources::SharedCriterion>,
    /// The shared set referenced in the query.
    #[prost(message, optional, tag = "27")]
    pub shared_set: ::core::option::Option<super::resources::SharedSet>,
    /// The Smart campaign setting referenced in the query.
    #[prost(message, optional, tag = "167")]
    pub smart_campaign_setting: ::core::option::Option<super::resources::SmartCampaignSetting>,
    /// The shopping performance view referenced in the query.
    #[prost(message, optional, tag = "117")]
    pub shopping_performance_view:
        ::core::option::Option<super::resources::ShoppingPerformanceView>,
    /// The Smart campaign search term view referenced in the query.
    #[prost(message, optional, tag = "170")]
    pub smart_campaign_search_term_view:
        ::core::option::Option<super::resources::SmartCampaignSearchTermView>,
    /// The AccountLink referenced in the query.
    #[prost(message, optional, tag = "144")]
    pub third_party_app_analytics_link:
        ::core::option::Option<super::resources::ThirdPartyAppAnalyticsLink>,
    /// The topic view referenced in the query.
    #[prost(message, optional, tag = "44")]
    pub topic_view: ::core::option::Option<super::resources::TopicView>,
    /// The experiment referenced in the query.
    #[prost(message, optional, tag = "133")]
    pub experiment: ::core::option::Option<super::resources::Experiment>,
    /// The experiment arm referenced in the query.
    #[prost(message, optional, tag = "183")]
    pub experiment_arm: ::core::option::Option<super::resources::ExperimentArm>,
    /// The user interest referenced in the query.
    #[prost(message, optional, tag = "59")]
    pub user_interest: ::core::option::Option<super::resources::UserInterest>,
    /// The life event referenced in the query.
    #[prost(message, optional, tag = "161")]
    pub life_event: ::core::option::Option<super::resources::LifeEvent>,
    /// The user list referenced in the query.
    #[prost(message, optional, tag = "38")]
    pub user_list: ::core::option::Option<super::resources::UserList>,
    /// The user location view referenced in the query.
    #[prost(message, optional, tag = "135")]
    pub user_location_view: ::core::option::Option<super::resources::UserLocationView>,
    /// The remarketing action referenced in the query.
    #[prost(message, optional, tag = "60")]
    pub remarketing_action: ::core::option::Option<super::resources::RemarketingAction>,
    /// The topic constant referenced in the query.
    #[prost(message, optional, tag = "31")]
    pub topic_constant: ::core::option::Option<super::resources::TopicConstant>,
    /// The video referenced in the query.
    #[prost(message, optional, tag = "39")]
    pub video: ::core::option::Option<super::resources::Video>,
    /// The webpage view referenced in the query.
    #[prost(message, optional, tag = "162")]
    pub webpage_view: ::core::option::Option<super::resources::WebpageView>,
    /// The lead form user submission referenced in the query.
    #[prost(message, optional, tag = "192")]
    pub lead_form_submission_data: ::core::option::Option<super::resources::LeadFormSubmissionData>,
    /// The metrics.
    #[prost(message, optional, tag = "4")]
    pub metrics: ::core::option::Option<super::common::Metrics>,
    /// The segments.
    #[prost(message, optional, tag = "102")]
    pub segments: ::core::option::Option<super::common::Segments>,
}
/// Request message for \[GoogleAdsService.Mutate][google.ads.googleads.v10.services.GoogleAdsService.Mutate\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateGoogleAdsRequest {
    /// Required. The ID of the customer whose resources are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual resources.
    #[prost(message, repeated, tag = "2")]
    pub mutate_operations: ::prost::alloc::vec::Vec<MutateOperation>,
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
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned post mutation. The mutable
    /// resource will only be returned if the resource has the appropriate response
    /// field. E.g. MutateCampaignResult.campaign.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "5"
    )]
    pub response_content_type: i32,
}
/// Response message for \[GoogleAdsService.Mutate][google.ads.googleads.v10.services.GoogleAdsService.Mutate\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateGoogleAdsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g., auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// All responses for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub mutate_operation_responses: ::prost::alloc::vec::Vec<MutateOperationResponse>,
}
/// A single operation (create, update, remove) on a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateOperation {
    /// The mutate operation.
    #[prost(
        oneof = "mutate_operation::Operation",
        tags = "17, 1, 56, 2, 77, 18, 3, 75, 19, 20, 21, 5, 49, 22, 23, 65, 78, 80, 62, 71, 72, 81, 58, 59, 6, 52, 73, 7, 8, 67, 13, 76, 24, 25, 26, 27, 9, 28, 10, 11, 12, 55, 69, 63, 64, 68, 57, 66, 79, 30, 31, 32, 34, 35, 70, 82, 83, 36, 37, 53, 54, 38, 39, 40, 44, 50, 51, 45, 48, 41, 42, 43, 14, 15, 61, 16"
    )]
    pub operation: ::core::option::Option<mutate_operation::Operation>,
}
/// Nested message and enum types in `MutateOperation`.
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
        /// An ad group asset mutate operation.
        #[prost(message, tag = "56")]
        AdGroupAssetOperation(super::AdGroupAssetOperation),
        /// An ad group bid modifier mutate operation.
        #[prost(message, tag = "2")]
        AdGroupBidModifierOperation(super::AdGroupBidModifierOperation),
        /// An ad group criterion customizer mutate operation.
        #[prost(message, tag = "77")]
        AdGroupCriterionCustomizerOperation(super::AdGroupCriterionCustomizerOperation),
        /// An ad group criterion label mutate operation.
        #[prost(message, tag = "18")]
        AdGroupCriterionLabelOperation(super::AdGroupCriterionLabelOperation),
        /// An ad group criterion mutate operation.
        #[prost(message, tag = "3")]
        AdGroupCriterionOperation(super::AdGroupCriterionOperation),
        /// An ad group customizer mutate operation.
        #[prost(message, tag = "75")]
        AdGroupCustomizerOperation(super::AdGroupCustomizerOperation),
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
        /// An asset group asset mutate operation.
        #[prost(message, tag = "65")]
        AssetGroupAssetOperation(super::AssetGroupAssetOperation),
        /// An asset group listing group filter mutate operation.
        #[prost(message, tag = "78")]
        AssetGroupListingGroupFilterOperation(super::AssetGroupListingGroupFilterOperation),
        /// An asset group signal mutate operation.
        #[prost(message, tag = "80")]
        AssetGroupSignalOperation(super::AssetGroupSignalOperation),
        /// An asset group mutate operation.
        #[prost(message, tag = "62")]
        AssetGroupOperation(super::AssetGroupOperation),
        /// An asset set asset mutate operation.
        #[prost(message, tag = "71")]
        AssetSetAssetOperation(super::AssetSetAssetOperation),
        /// An asset set mutate operation.
        #[prost(message, tag = "72")]
        AssetSetOperation(super::AssetSetOperation),
        /// An audience mutate operation.
        #[prost(message, tag = "81")]
        AudienceOperation(super::AudienceOperation),
        /// A bidding data exclusion mutate operation.
        #[prost(message, tag = "58")]
        BiddingDataExclusionOperation(super::BiddingDataExclusionOperation),
        /// A bidding seasonality adjustment mutate operation.
        #[prost(message, tag = "59")]
        BiddingSeasonalityAdjustmentOperation(super::BiddingSeasonalityAdjustmentOperation),
        /// A bidding strategy mutate operation.
        #[prost(message, tag = "6")]
        BiddingStrategyOperation(super::BiddingStrategyOperation),
        /// A campaign asset mutate operation.
        #[prost(message, tag = "52")]
        CampaignAssetOperation(super::CampaignAssetOperation),
        /// A campaign asset mutate operation.
        #[prost(message, tag = "73")]
        CampaignAssetSetOperation(super::CampaignAssetSetOperation),
        /// A campaign bid modifier mutate operation.
        #[prost(message, tag = "7")]
        CampaignBidModifierOperation(super::CampaignBidModifierOperation),
        /// A campaign budget mutate operation.
        #[prost(message, tag = "8")]
        CampaignBudgetOperation(super::CampaignBudgetOperation),
        /// A campaign conversion goal mutate operation.
        #[prost(message, tag = "67")]
        CampaignConversionGoalOperation(super::CampaignConversionGoalOperation),
        /// A campaign criterion mutate operation.
        #[prost(message, tag = "13")]
        CampaignCriterionOperation(super::CampaignCriterionOperation),
        /// An campaign customizer mutate operation.
        #[prost(message, tag = "76")]
        CampaignCustomizerOperation(super::CampaignCustomizerOperation),
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
        /// A campaign group mutate operation.
        #[prost(message, tag = "9")]
        CampaignGroupOperation(super::CampaignGroupOperation),
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
        /// A conversion custom variable mutate operation.
        #[prost(message, tag = "55")]
        ConversionCustomVariableOperation(super::ConversionCustomVariableOperation),
        /// A conversion goal campaign config mutate operation.
        #[prost(message, tag = "69")]
        ConversionGoalCampaignConfigOperation(super::ConversionGoalCampaignConfigOperation),
        /// A conversion value rule mutate operation.
        #[prost(message, tag = "63")]
        ConversionValueRuleOperation(super::ConversionValueRuleOperation),
        /// A conversion value rule set mutate operation.
        #[prost(message, tag = "64")]
        ConversionValueRuleSetOperation(super::ConversionValueRuleSetOperation),
        /// A custom conversion goal mutate operation.
        #[prost(message, tag = "68")]
        CustomConversionGoalOperation(super::CustomConversionGoalOperation),
        /// A customer asset mutate operation.
        #[prost(message, tag = "57")]
        CustomerAssetOperation(super::CustomerAssetOperation),
        /// A customer conversion goal mutate operation.
        #[prost(message, tag = "66")]
        CustomerConversionGoalOperation(super::CustomerConversionGoalOperation),
        /// An customer customizer mutate operation.
        #[prost(message, tag = "79")]
        CustomerCustomizerOperation(super::CustomerCustomizerOperation),
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
        /// An customizer attribute mutate operation.
        #[prost(message, tag = "70")]
        CustomizerAttributeOperation(super::CustomizerAttributeOperation),
        /// An experiment mutate operation.
        #[prost(message, tag = "82")]
        ExperimentOperation(super::ExperimentOperation),
        /// An experiment arm mutate operation.
        #[prost(message, tag = "83")]
        ExperimentArmOperation(super::ExperimentArmOperation),
        /// An extension feed item mutate operation.
        #[prost(message, tag = "36")]
        ExtensionFeedItemOperation(super::ExtensionFeedItemOperation),
        /// A feed item mutate operation.
        #[prost(message, tag = "37")]
        FeedItemOperation(super::FeedItemOperation),
        /// A feed item set mutate operation.
        #[prost(message, tag = "53")]
        FeedItemSetOperation(super::FeedItemSetOperation),
        /// A feed item set link mutate operation.
        #[prost(message, tag = "54")]
        FeedItemSetLinkOperation(super::FeedItemSetLinkOperation),
        /// A feed item target mutate operation.
        #[prost(message, tag = "38")]
        FeedItemTargetOperation(super::FeedItemTargetOperation),
        /// A feed mapping mutate operation.
        #[prost(message, tag = "39")]
        FeedMappingOperation(super::FeedMappingOperation),
        /// A feed mutate operation.
        #[prost(message, tag = "40")]
        FeedOperation(super::FeedOperation),
        /// A keyword plan ad group operation.
        #[prost(message, tag = "44")]
        KeywordPlanAdGroupOperation(super::KeywordPlanAdGroupOperation),
        /// A keyword plan ad group keyword operation.
        #[prost(message, tag = "50")]
        KeywordPlanAdGroupKeywordOperation(super::KeywordPlanAdGroupKeywordOperation),
        /// A keyword plan campaign keyword operation.
        #[prost(message, tag = "51")]
        KeywordPlanCampaignKeywordOperation(super::KeywordPlanCampaignKeywordOperation),
        /// A keyword plan campaign operation.
        #[prost(message, tag = "45")]
        KeywordPlanCampaignOperation(super::KeywordPlanCampaignOperation),
        /// A keyword plan operation.
        #[prost(message, tag = "48")]
        KeywordPlanOperation(super::KeywordPlanOperation),
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
        /// A Smart campaign setting mutate operation.
        #[prost(message, tag = "61")]
        SmartCampaignSettingOperation(super::SmartCampaignSettingOperation),
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
        tags = "17, 1, 56, 2, 77, 18, 3, 75, 19, 20, 21, 5, 22, 49, 23, 65, 78, 79, 62, 71, 72, 80, 58, 59, 6, 52, 73, 7, 8, 67, 13, 76, 24, 25, 26, 27, 9, 28, 10, 11, 12, 55, 69, 63, 64, 68, 57, 66, 74, 30, 31, 32, 34, 35, 70, 36, 37, 53, 54, 38, 39, 40, 44, 45, 50, 51, 48, 41, 42, 43, 14, 15, 61, 16"
    )]
    pub response: ::core::option::Option<mutate_operation_response::Response>,
}
/// Nested message and enum types in `MutateOperationResponse`.
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
        /// The result for the ad group asset mutate.
        #[prost(message, tag = "56")]
        AdGroupAssetResult(super::MutateAdGroupAssetResult),
        /// The result for the ad group bid modifier mutate.
        #[prost(message, tag = "2")]
        AdGroupBidModifierResult(super::MutateAdGroupBidModifierResult),
        /// The result for the ad group criterion customizer mutate.
        #[prost(message, tag = "77")]
        AdGroupCriterionCustomizerResult(super::MutateAdGroupCriterionCustomizerResult),
        /// The result for the ad group criterion label mutate.
        #[prost(message, tag = "18")]
        AdGroupCriterionLabelResult(super::MutateAdGroupCriterionLabelResult),
        /// The result for the ad group criterion mutate.
        #[prost(message, tag = "3")]
        AdGroupCriterionResult(super::MutateAdGroupCriterionResult),
        /// The result for the ad group customizer mutate.
        #[prost(message, tag = "75")]
        AdGroupCustomizerResult(super::MutateAdGroupCustomizerResult),
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
        /// The result for the asset group asset mutate.
        #[prost(message, tag = "65")]
        AssetGroupAssetResult(super::MutateAssetGroupAssetResult),
        /// The result for the asset group listing group filter mutate.
        #[prost(message, tag = "78")]
        AssetGroupListingGroupFilterResult(super::MutateAssetGroupListingGroupFilterResult),
        /// The result for the asset group signal mutate.
        #[prost(message, tag = "79")]
        AssetGroupSignalResult(super::MutateAssetGroupSignalResult),
        /// The result for the asset group mutate.
        #[prost(message, tag = "62")]
        AssetGroupResult(super::MutateAssetGroupResult),
        /// The result for the asset set asset mutate.
        #[prost(message, tag = "71")]
        AssetSetAssetResult(super::MutateAssetSetAssetResult),
        /// The result for the asset set mutate.
        #[prost(message, tag = "72")]
        AssetSetResult(super::MutateAssetSetResult),
        /// The result for the audience mutate.
        #[prost(message, tag = "80")]
        AudienceResult(super::MutateAudienceResult),
        /// The result for the bidding data exclusion mutate.
        #[prost(message, tag = "58")]
        BiddingDataExclusionResult(super::MutateBiddingDataExclusionsResult),
        /// The result for the bidding seasonality adjustment mutate.
        #[prost(message, tag = "59")]
        BiddingSeasonalityAdjustmentResult(super::MutateBiddingSeasonalityAdjustmentsResult),
        /// The result for the bidding strategy mutate.
        #[prost(message, tag = "6")]
        BiddingStrategyResult(super::MutateBiddingStrategyResult),
        /// The result for the campaign asset mutate.
        #[prost(message, tag = "52")]
        CampaignAssetResult(super::MutateCampaignAssetResult),
        /// The result for the campaign asset set mutate.
        #[prost(message, tag = "73")]
        CampaignAssetSetResult(super::MutateCampaignAssetSetResult),
        /// The result for the campaign bid modifier mutate.
        #[prost(message, tag = "7")]
        CampaignBidModifierResult(super::MutateCampaignBidModifierResult),
        /// The result for the campaign budget mutate.
        #[prost(message, tag = "8")]
        CampaignBudgetResult(super::MutateCampaignBudgetResult),
        /// The result for the campaign conversion goal mutate.
        #[prost(message, tag = "67")]
        CampaignConversionGoalResult(super::MutateCampaignConversionGoalResult),
        /// The result for the campaign criterion mutate.
        #[prost(message, tag = "13")]
        CampaignCriterionResult(super::MutateCampaignCriterionResult),
        /// The result for the campaign customizer mutate.
        #[prost(message, tag = "76")]
        CampaignCustomizerResult(super::MutateCampaignCustomizerResult),
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
        /// The result for the campaign group mutate.
        #[prost(message, tag = "9")]
        CampaignGroupResult(super::MutateCampaignGroupResult),
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
        /// The result for the conversion custom variable mutate.
        #[prost(message, tag = "55")]
        ConversionCustomVariableResult(super::MutateConversionCustomVariableResult),
        /// The result for the conversion goal campaign config mutate.
        #[prost(message, tag = "69")]
        ConversionGoalCampaignConfigResult(super::MutateConversionGoalCampaignConfigResult),
        /// The result for the conversion value rule mutate.
        #[prost(message, tag = "63")]
        ConversionValueRuleResult(super::MutateConversionValueRuleResult),
        /// The result for the conversion value rule set mutate.
        #[prost(message, tag = "64")]
        ConversionValueRuleSetResult(super::MutateConversionValueRuleSetResult),
        /// The result for the custom conversion goal mutate.
        #[prost(message, tag = "68")]
        CustomConversionGoalResult(super::MutateCustomConversionGoalResult),
        /// The result for the customer asset mutate.
        #[prost(message, tag = "57")]
        CustomerAssetResult(super::MutateCustomerAssetResult),
        /// The result for the customer conversion goal mutate.
        #[prost(message, tag = "66")]
        CustomerConversionGoalResult(super::MutateCustomerConversionGoalResult),
        /// The result for the customer customizer mutate.
        #[prost(message, tag = "74")]
        CustomerCustomizerResult(super::MutateCustomerCustomizerResult),
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
        /// The result for the customizer attribute mutate.
        #[prost(message, tag = "70")]
        CustomizerAttributeResult(super::MutateCustomizerAttributeResult),
        /// The result for the extension feed item mutate.
        #[prost(message, tag = "36")]
        ExtensionFeedItemResult(super::MutateExtensionFeedItemResult),
        /// The result for the feed item mutate.
        #[prost(message, tag = "37")]
        FeedItemResult(super::MutateFeedItemResult),
        /// The result for the feed item set mutate.
        #[prost(message, tag = "53")]
        FeedItemSetResult(super::MutateFeedItemSetResult),
        /// The result for the feed item set link mutate.
        #[prost(message, tag = "54")]
        FeedItemSetLinkResult(super::MutateFeedItemSetLinkResult),
        /// The result for the feed item target mutate.
        #[prost(message, tag = "38")]
        FeedItemTargetResult(super::MutateFeedItemTargetResult),
        /// The result for the feed mapping mutate.
        #[prost(message, tag = "39")]
        FeedMappingResult(super::MutateFeedMappingResult),
        /// The result for the feed mutate.
        #[prost(message, tag = "40")]
        FeedResult(super::MutateFeedResult),
        /// The result for the keyword plan ad group mutate.
        #[prost(message, tag = "44")]
        KeywordPlanAdGroupResult(super::MutateKeywordPlanAdGroupResult),
        /// The result for the keyword plan campaign mutate.
        #[prost(message, tag = "45")]
        KeywordPlanCampaignResult(super::MutateKeywordPlanCampaignResult),
        /// The result for the keyword plan ad group keyword mutate.
        #[prost(message, tag = "50")]
        KeywordPlanAdGroupKeywordResult(super::MutateKeywordPlanAdGroupKeywordResult),
        /// The result for the keyword plan campaign keyword mutate.
        #[prost(message, tag = "51")]
        KeywordPlanCampaignKeywordResult(super::MutateKeywordPlanCampaignKeywordResult),
        /// The result for the keyword plan mutate.
        #[prost(message, tag = "48")]
        KeywordPlanResult(super::MutateKeywordPlansResult),
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
        /// The result for the Smart campaign setting mutate.
        #[prost(message, tag = "61")]
        SmartCampaignSettingResult(super::MutateSmartCampaignSettingResult),
        /// The result for the user list mutate.
        #[prost(message, tag = "16")]
        UserListResult(super::MutateUserListResult),
    }
}
#[doc = r" Generated client implementations."]
pub mod google_ads_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to fetch data and metrics across resources."]
    #[derive(Debug, Clone)]
    pub struct GoogleAdsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GoogleAdsServiceClient<T>
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
        ) -> GoogleAdsServiceClient<InterceptedService<T, F>>
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
            GoogleAdsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns all rows that match the search query."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [ChangeEventError]()"]
        #[doc = "   [ChangeStatusError]()"]
        #[doc = "   [ClickViewError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QueryError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.GoogleAdsService/Search",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns all rows that match the search stream query."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [ChangeEventError]()"]
        #[doc = "   [ChangeStatusError]()"]
        #[doc = "   [ClickViewError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QueryError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.GoogleAdsService/SearchStream",
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
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AdCustomizerError]()"]
        #[doc = "   [AdError]()"]
        #[doc = "   [AdGroupAdError]()"]
        #[doc = "   [AdGroupCriterionError]()"]
        #[doc = "   [AdGroupError]()"]
        #[doc = "   [AssetError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [BiddingError]()"]
        #[doc = "   [CampaignBudgetError]()"]
        #[doc = "   [CampaignCriterionError]()"]
        #[doc = "   [CampaignError]()"]
        #[doc = "   [CampaignExperimentError]()"]
        #[doc = "   [CampaignSharedSetError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [ContextError]()"]
        #[doc = "   [ConversionActionError]()"]
        #[doc = "   [CriterionError]()"]
        #[doc = "   [CustomerFeedError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [DateRangeError]()"]
        #[doc = "   [DistinctError]()"]
        #[doc = "   [ExtensionFeedItemError]()"]
        #[doc = "   [ExtensionSettingError]()"]
        #[doc = "   [FeedAttributeReferenceError]()"]
        #[doc = "   [FeedError]()"]
        #[doc = "   [FeedItemError]()"]
        #[doc = "   [FeedItemSetError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [FunctionParsingError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [ImageError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [KeywordPlanAdGroupKeywordError]()"]
        #[doc = "   [KeywordPlanCampaignError]()"]
        #[doc = "   [KeywordPlanError]()"]
        #[doc = "   [LabelError]()"]
        #[doc = "   [ListOperationError]()"]
        #[doc = "   [MediaUploadError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [NullError]()"]
        #[doc = "   [OperationAccessDeniedError]()"]
        #[doc = "   [PolicyFindingError]()"]
        #[doc = "   [PolicyViolationError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        #[doc = "   [SettingError]()"]
        #[doc = "   [SharedSetError]()"]
        #[doc = "   [SizeLimitError]()"]
        #[doc = "   [StringFormatError]()"]
        #[doc = "   [StringLengthError]()"]
        #[doc = "   [UrlFieldError]()"]
        #[doc = "   [UserListError]()"]
        #[doc = "   [YoutubeVideoRegistrationError]()"]
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
                "/google.ads.googleads.v10.services.GoogleAdsService/Mutate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[BatchJobService.MutateBatchJob][google.ads.googleads.v10.services.BatchJobService.MutateBatchJob\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBatchJobRequest {
    /// Required. The ID of the customer for which to create a batch job.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The operation to perform on an individual batch job.
    #[prost(message, optional, tag = "2")]
    pub operation: ::core::option::Option<BatchJobOperation>,
}
/// A single operation on a batch job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchJobOperation {
    /// The mutate operation.
    #[prost(oneof = "batch_job_operation::Operation", tags = "1, 4")]
    pub operation: ::core::option::Option<batch_job_operation::Operation>,
}
/// Nested message and enum types in `BatchJobOperation`.
pub mod batch_job_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new batch job.
        #[prost(message, tag = "1")]
        Create(super::super::resources::BatchJob),
        /// Remove operation: The batch job must not have been run. A resource name
        /// for the removed batch job is expected, in this format:
        ///
        /// `customers/{customer_id}/batchJobs/{batch_job_id}`
        #[prost(string, tag = "4")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for \[BatchJobService.MutateBatchJob][google.ads.googleads.v10.services.BatchJobService.MutateBatchJob\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBatchJobResponse {
    /// The result for the mutate.
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<MutateBatchJobResult>,
}
/// The result for the batch job mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBatchJobResult {
    /// The resource name of the batch job.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// Request message for \[BatchJobService.RunBatchJob][google.ads.googleads.v10.services.BatchJobService.RunBatchJob\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunBatchJobRequest {
    /// Required. The resource name of the BatchJob to run.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// Request message for \[BatchJobService.AddBatchJobOperations][google.ads.googleads.v10.services.BatchJobService.AddBatchJobOperations\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddBatchJobOperationsRequest {
    /// Required. The resource name of the batch job.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// A token used to enforce sequencing.
    ///
    /// The first AddBatchJobOperations request for a batch job should not set
    /// sequence_token. Subsequent requests must set sequence_token to the value of
    /// next_sequence_token received in the previous AddBatchJobOperations
    /// response.
    #[prost(string, tag = "2")]
    pub sequence_token: ::prost::alloc::string::String,
    /// Required. The list of mutates being added.
    ///
    /// Operations can use negative integers as temp ids to signify dependencies
    /// between entities created in this batch job. For example, a customer with
    /// id = 1234 can create a campaign and an ad group in that same campaign by
    /// creating a campaign in the first operation with the resource name
    /// explicitly set to "customers/1234/campaigns/-1", and creating an ad group
    /// in the second operation with the campaign field also set to
    /// "customers/1234/campaigns/-1".
    #[prost(message, repeated, tag = "3")]
    pub mutate_operations: ::prost::alloc::vec::Vec<MutateOperation>,
}
/// Response message for \[BatchJobService.AddBatchJobOperations][google.ads.googleads.v10.services.BatchJobService.AddBatchJobOperations\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddBatchJobOperationsResponse {
    /// The total number of operations added so far for this batch job.
    #[prost(int64, tag = "1")]
    pub total_operations: i64,
    /// The sequence token to be used when calling AddBatchJobOperations again if
    /// more operations need to be added. The next AddBatchJobOperations request
    /// must set the sequence_token field to the value of this field.
    #[prost(string, tag = "2")]
    pub next_sequence_token: ::prost::alloc::string::String,
}
/// Request message for \[BatchJobService.ListBatchJobResults][google.ads.googleads.v10.services.BatchJobService.ListBatchJobResults\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBatchJobResultsRequest {
    /// Required. The resource name of the batch job whose results are being listed.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Token of the page to retrieve. If not specified, the first
    /// page of results will be returned. Use the value obtained from
    /// `next_page_token` in the previous response in order to request
    /// the next page of results.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Number of elements to retrieve in a single page.
    /// When a page request is too large, the server may decide to
    /// further limit the number of returned resources.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The response content type setting. Determines whether the mutable resource
    /// or just the resource name should be returned.
    #[prost(
        enumeration = "super::enums::response_content_type_enum::ResponseContentType",
        tag = "4"
    )]
    pub response_content_type: i32,
}
/// Response message for \[BatchJobService.ListBatchJobResults][google.ads.googleads.v10.services.BatchJobService.ListBatchJobResults\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBatchJobResultsResponse {
    /// The list of rows that matched the query.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<BatchJobResult>,
    /// Pagination token used to retrieve the next page of results.
    /// Pass the content of this string as the `page_token` attribute of
    /// the next request. `next_page_token` is not returned for the last
    /// page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// An individual batch job result.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchJobResult {
    /// Index of the mutate operation.
    #[prost(int64, tag = "1")]
    pub operation_index: i64,
    /// Response for the mutate.
    /// May be empty if errors occurred.
    #[prost(message, optional, tag = "2")]
    pub mutate_operation_response: ::core::option::Option<MutateOperationResponse>,
    /// Details of the errors when processing the operation.
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<super::super::super::super::rpc::Status>,
}
#[doc = r" Generated client implementations."]
pub mod batch_job_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage batch jobs."]
    #[derive(Debug, Clone)]
    pub struct BatchJobServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BatchJobServiceClient<T>
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
        ) -> BatchJobServiceClient<InterceptedService<T, F>>
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
            BatchJobServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Mutates a batch job."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        pub async fn mutate_batch_job(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateBatchJobRequest>,
        ) -> Result<tonic::Response<super::MutateBatchJobResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.BatchJobService/MutateBatchJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the results of the batch job. The job must be done."]
        #[doc = " Supports standard list paging."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [BatchJobError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn list_batch_job_results(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBatchJobResultsRequest>,
        ) -> Result<tonic::Response<super::ListBatchJobResultsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.BatchJobService/ListBatchJobResults",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Runs the batch job."]
        #[doc = ""]
        #[doc = " The Operation.metadata field type is BatchJobMetadata. When finished, the"]
        #[doc = " long running operation will not contain errors or a response. Instead, use"]
        #[doc = " ListBatchJobResults to get the results of the job."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [BatchJobError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn run_batch_job(
            &mut self,
            request: impl tonic::IntoRequest<super::RunBatchJobRequest>,
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
                "/google.ads.googleads.v10.services.BatchJobService/RunBatchJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Add operations to the batch job."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [BatchJobError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [ResourceCountLimitExceededError]()"]
        pub async fn add_batch_job_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::AddBatchJobOperationsRequest>,
        ) -> Result<tonic::Response<super::AddBatchJobOperationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.BatchJobService/AddBatchJobOperations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for billing setup mutate operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBillingSetupRequest {
    /// Required. Id of the customer to apply the billing setup mutate operation to.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The operation to perform.
    #[prost(message, optional, tag = "2")]
    pub operation: ::core::option::Option<BillingSetupOperation>,
}
/// A single operation on a billing setup, which describes the cancellation of an
/// existing billing setup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BillingSetupOperation {
    /// Only one of these operations can be set. "Update" operations are not
    /// supported.
    #[prost(oneof = "billing_setup_operation::Operation", tags = "2, 1")]
    pub operation: ::core::option::Option<billing_setup_operation::Operation>,
}
/// Nested message and enum types in `BillingSetupOperation`.
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
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for a billing setup operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBillingSetupResponse {
    /// A result that identifies the resource affected by the mutate request.
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<MutateBillingSetupResult>,
}
/// Result for a single billing setup mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBillingSetupResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod billing_setup_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
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
    #[derive(Debug, Clone)]
    pub struct BillingSetupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BillingSetupServiceClient<T>
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
        ) -> BillingSetupServiceClient<InterceptedService<T, F>>
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
            BillingSetupServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates a billing setup, or cancels an existing billing setup."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [BillingSetupError]()"]
        #[doc = "   [DateError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.BillingSetupService/MutateBillingSetup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[ConversionAdjustmentUploadService.UploadConversionAdjustments][google.ads.googleads.v10.services.ConversionAdjustmentUploadService.UploadConversionAdjustments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadConversionAdjustmentsRequest {
    /// Required. The ID of the customer performing the upload.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The conversion adjustments that are being uploaded.
    #[prost(message, repeated, tag = "2")]
    pub conversion_adjustments: ::prost::alloc::vec::Vec<ConversionAdjustment>,
    /// Required. If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried out
    /// in one transaction if and only if they are all valid. This should always be
    /// set to true.
    /// See
    /// <https://developers.google.com/google-ads/api/docs/best-practices/partial-failures>
    /// for more information about partial failure.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Response message for
/// \[ConversionAdjustmentUploadService.UploadConversionAdjustments][google.ads.googleads.v10.services.ConversionAdjustmentUploadService.UploadConversionAdjustments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadConversionAdjustmentsResponse {
    /// Errors that pertain to conversion adjustment failures in the partial
    /// failure mode. Returned when all errors occur inside the adjustments. If any
    /// errors occur outside the adjustments (e.g. auth errors), we return an RPC
    /// level error.
    /// See
    /// <https://developers.google.com/google-ads/api/docs/best-practices/partial-failures>
    /// for more information about partial failure.
    #[prost(message, optional, tag = "1")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// Returned for successfully processed conversion adjustments. Proto will be
    /// empty for rows that received an error. Results are not returned when
    /// validate_only is true.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<ConversionAdjustmentResult>,
}
/// A conversion adjustment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionAdjustment {
    /// For adjustments, uniquely identifies a conversion that was reported
    /// without an order ID specified. If the adjustment_type is ENHANCEMENT, this
    /// value is optional but may be set in addition to the order_id.
    #[prost(message, optional, tag = "12")]
    pub gclid_date_time_pair: ::core::option::Option<GclidDateTimePair>,
    /// The order ID of the conversion to be adjusted. If the conversion was
    /// reported with an order ID specified, that order ID must be used as the
    /// identifier here. The order ID is required for enhancements.
    #[prost(string, optional, tag = "13")]
    pub order_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the conversion action associated with this conversion
    /// adjustment. Note: Although this resource name consists of a customer id and
    /// a conversion action id, validation will ignore the customer id and use the
    /// conversion action id as the sole identifier of the conversion action.
    #[prost(string, optional, tag = "8")]
    pub conversion_action: ::core::option::Option<::prost::alloc::string::String>,
    /// The date time at which the adjustment occurred. Must be after the
    /// conversion_date_time. The timezone must be specified. The format is
    /// "yyyy-mm-dd hh:mm:ss+|-hh:mm", e.g. "2019-01-01 12:32:45-08:00".
    #[prost(string, optional, tag = "9")]
    pub adjustment_date_time: ::core::option::Option<::prost::alloc::string::String>,
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
    pub restatement_value: ::core::option::Option<RestatementValue>,
    /// The user identifiers to enhance the original conversion.
    /// ConversionAdjustmentUploadService only accepts user identifiers in
    /// enhancements. The maximum number of user identifiers for each
    /// enhancement is 5.
    #[prost(message, repeated, tag = "10")]
    pub user_identifiers: ::prost::alloc::vec::Vec<super::common::UserIdentifier>,
    /// The user agent to enhance the original conversion. This can be found in
    /// your user's HTTP request header when they convert on your web page.
    /// Example, "Mozilla/5.0 (iPhone; CPU iPhone OS 12_2 like Mac OS X)". User
    /// agent can only be specified in enhancements with user identifiers. This
    /// should match the user agent of the request that sent the original
    /// conversion so the conversion and its enhancement are either both attributed
    /// as same-device or both attributed as cross-device.
    #[prost(string, optional, tag = "11")]
    pub user_agent: ::core::option::Option<::prost::alloc::string::String>,
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
    #[prost(double, optional, tag = "3")]
    pub adjusted_value: ::core::option::Option<f64>,
    /// The currency of the restated value. If not provided, then the default
    /// currency from the conversion action is used, and if that is not set then
    /// the account currency is used. This is the ISO 4217 3-character currency
    /// code e.g. USD or EUR.
    #[prost(string, optional, tag = "4")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
}
/// Uniquely identifies a conversion that was reported without an order ID
/// specified.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GclidDateTimePair {
    /// Google click ID (gclid) associated with the original conversion for this
    /// adjustment.
    #[prost(string, optional, tag = "3")]
    pub gclid: ::core::option::Option<::prost::alloc::string::String>,
    /// The date time at which the original conversion for this adjustment
    /// occurred. The timezone must be specified. The format is "yyyy-mm-dd
    /// hh:mm:ss+|-hh:mm", e.g. "2019-01-01 12:32:45-08:00".
    #[prost(string, optional, tag = "4")]
    pub conversion_date_time: ::core::option::Option<::prost::alloc::string::String>,
}
/// Information identifying a successfully processed ConversionAdjustment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionAdjustmentResult {
    /// The gclid and conversion date time of the conversion.
    #[prost(message, optional, tag = "9")]
    pub gclid_date_time_pair: ::core::option::Option<GclidDateTimePair>,
    /// The order ID of the conversion to be adjusted.
    #[prost(string, tag = "10")]
    pub order_id: ::prost::alloc::string::String,
    /// Resource name of the conversion action associated with this conversion
    /// adjustment.
    #[prost(string, optional, tag = "7")]
    pub conversion_action: ::core::option::Option<::prost::alloc::string::String>,
    /// The date time at which the adjustment occurred. The format is
    /// "yyyy-mm-dd hh:mm:ss+|-hh:mm", e.g. "2019-01-01 12:32:45-08:00".
    #[prost(string, optional, tag = "8")]
    pub adjustment_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// The adjustment type.
    #[prost(
        enumeration = "super::enums::conversion_adjustment_type_enum::ConversionAdjustmentType",
        tag = "5"
    )]
    pub adjustment_type: i32,
}
#[doc = r" Generated client implementations."]
pub mod conversion_adjustment_upload_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to upload conversion adjustments."]
    #[derive(Debug, Clone)]
    pub struct ConversionAdjustmentUploadServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConversionAdjustmentUploadServiceClient<T>
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
        ) -> ConversionAdjustmentUploadServiceClient<InterceptedService<T, F>>
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
            ConversionAdjustmentUploadServiceClient::new(InterceptedService::new(
                inner,
                interceptor,
            ))
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
        #[doc = " Processes the given conversion adjustments."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [PartialFailureError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.ConversionAdjustmentUploadService/UploadConversionAdjustments") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[ConversionUploadService.UploadClickConversions][google.ads.googleads.v10.services.ConversionUploadService.UploadClickConversions\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadClickConversionsRequest {
    /// Required. The ID of the customer performing the upload.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The conversions that are being uploaded.
    #[prost(message, repeated, tag = "2")]
    pub conversions: ::prost::alloc::vec::Vec<ClickConversion>,
    /// Required. If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// This should always be set to true.
    /// See
    /// <https://developers.google.com/google-ads/api/docs/best-practices/partial-failures>
    /// for more information about partial failure.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Response message for \[ConversionUploadService.UploadClickConversions][google.ads.googleads.v10.services.ConversionUploadService.UploadClickConversions\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadClickConversionsResponse {
    /// Errors that pertain to conversion failures in the partial failure mode.
    /// Returned when all errors occur inside the conversions. If any errors occur
    /// outside the conversions (e.g. auth errors), we return an RPC level error.
    /// See
    /// <https://developers.google.com/google-ads/api/docs/best-practices/partial-failures>
    /// for more information about partial failure.
    #[prost(message, optional, tag = "1")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// Returned for successfully processed conversions. Proto will be empty for
    /// rows that received an error. Results are not returned when validate_only is
    /// true.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<ClickConversionResult>,
}
/// Request message for \[ConversionUploadService.UploadCallConversions][google.ads.googleads.v10.services.ConversionUploadService.UploadCallConversions\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadCallConversionsRequest {
    /// Required. The ID of the customer performing the upload.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The conversions that are being uploaded.
    #[prost(message, repeated, tag = "2")]
    pub conversions: ::prost::alloc::vec::Vec<CallConversion>,
    /// Required. If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// This should always be set to true.
    /// See
    /// <https://developers.google.com/google-ads/api/docs/best-practices/partial-failures>
    /// for more information about partial failure.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Response message for \[ConversionUploadService.UploadCallConversions][google.ads.googleads.v10.services.ConversionUploadService.UploadCallConversions\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadCallConversionsResponse {
    /// Errors that pertain to conversion failures in the partial failure mode.
    /// Returned when all errors occur inside the conversions. If any errors occur
    /// outside the conversions (e.g. auth errors), we return an RPC level error.
    /// See
    /// <https://developers.google.com/google-ads/api/docs/best-practices/partial-failures>
    /// for more information about partial failure.
    #[prost(message, optional, tag = "1")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// Returned for successfully processed conversions. Proto will be empty for
    /// rows that received an error. Results are not returned when validate_only is
    /// true.
    #[prost(message, repeated, tag = "2")]
    pub results: ::prost::alloc::vec::Vec<CallConversionResult>,
}
/// A click conversion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClickConversion {
    /// The Google click ID (gclid) associated with this conversion.
    #[prost(string, optional, tag = "9")]
    pub gclid: ::core::option::Option<::prost::alloc::string::String>,
    /// The click identifier for clicks associated with app conversions and
    /// originating from iOS devices starting with iOS14.
    #[prost(string, tag = "18")]
    pub gbraid: ::prost::alloc::string::String,
    /// The click identifier for clicks associated with web conversions and
    /// originating from iOS devices starting with iOS14.
    #[prost(string, tag = "19")]
    pub wbraid: ::prost::alloc::string::String,
    /// Resource name of the conversion action associated with this conversion.
    /// Note: Although this resource name consists of a customer id and a
    /// conversion action id, validation will ignore the customer id and use the
    /// conversion action id as the sole identifier of the conversion action.
    #[prost(string, optional, tag = "10")]
    pub conversion_action: ::core::option::Option<::prost::alloc::string::String>,
    /// The date time at which the conversion occurred. Must be after
    /// the click time. The timezone must be specified. The format is
    /// "yyyy-mm-dd hh:mm:ss+|-hh:mm", e.g. "2019-01-01 12:32:45-08:00".
    #[prost(string, optional, tag = "11")]
    pub conversion_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// The value of the conversion for the advertiser.
    #[prost(double, optional, tag = "12")]
    pub conversion_value: ::core::option::Option<f64>,
    /// Currency associated with the conversion value. This is the ISO 4217
    /// 3-character currency code. For example: USD, EUR.
    #[prost(string, optional, tag = "13")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    /// The order ID associated with the conversion. An order id can only be used
    /// for one conversion per conversion action.
    #[prost(string, optional, tag = "14")]
    pub order_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Additional data about externally attributed conversions. This field
    /// is required for conversions with an externally attributed conversion
    /// action, but should not be set otherwise.
    #[prost(message, optional, tag = "7")]
    pub external_attribution_data: ::core::option::Option<ExternalAttributionData>,
    /// The custom variables associated with this conversion.
    #[prost(message, repeated, tag = "15")]
    pub custom_variables: ::prost::alloc::vec::Vec<CustomVariable>,
    /// The cart data associated with this conversion.
    #[prost(message, optional, tag = "16")]
    pub cart_data: ::core::option::Option<CartData>,
    /// The user identifiers associated with this conversion. Only hashed_email and
    /// hashed_phone_number are supported for conversion uploads. The maximum
    /// number of user identifiers for each conversion is 5.
    #[prost(message, repeated, tag = "17")]
    pub user_identifiers: ::prost::alloc::vec::Vec<super::common::UserIdentifier>,
    /// The environment this conversion was recorded on. e.g. App or Web.
    #[prost(
        enumeration = "super::enums::conversion_environment_enum::ConversionEnvironment",
        tag = "20"
    )]
    pub conversion_environment: i32,
}
/// A call conversion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallConversion {
    /// The caller id from which this call was placed. Caller id is expected to be
    /// in E.164 format with preceding '+' sign. e.g. "+16502531234".
    #[prost(string, optional, tag = "7")]
    pub caller_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The date time at which the call occurred. The timezone must be specified.
    /// The format is "yyyy-mm-dd hh:mm:ss+|-hh:mm",
    /// e.g. "2019-01-01 12:32:45-08:00".
    #[prost(string, optional, tag = "8")]
    pub call_start_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the conversion action associated with this conversion.
    /// Note: Although this resource name consists of a customer id and a
    /// conversion action id, validation will ignore the customer id and use the
    /// conversion action id as the sole identifier of the conversion action.
    #[prost(string, optional, tag = "9")]
    pub conversion_action: ::core::option::Option<::prost::alloc::string::String>,
    /// The date time at which the conversion occurred. Must be after the call
    /// time. The timezone must be specified. The format is
    /// "yyyy-mm-dd hh:mm:ss+|-hh:mm", e.g. "2019-01-01 12:32:45-08:00".
    #[prost(string, optional, tag = "10")]
    pub conversion_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// The value of the conversion for the advertiser.
    #[prost(double, optional, tag = "11")]
    pub conversion_value: ::core::option::Option<f64>,
    /// Currency associated with the conversion value. This is the ISO 4217
    /// 3-character currency code. For example: USD, EUR.
    #[prost(string, optional, tag = "12")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    /// The custom variables associated with this conversion.
    #[prost(message, repeated, tag = "13")]
    pub custom_variables: ::prost::alloc::vec::Vec<CustomVariable>,
}
/// Contains additional information about externally attributed conversions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalAttributionData {
    /// Represents the fraction of the conversion that is attributed to the
    /// Google Ads click.
    #[prost(double, optional, tag = "3")]
    pub external_attribution_credit: ::core::option::Option<f64>,
    /// Specifies the attribution model name.
    #[prost(string, optional, tag = "4")]
    pub external_attribution_model: ::core::option::Option<::prost::alloc::string::String>,
}
/// Identifying information for a successfully processed ClickConversion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClickConversionResult {
    /// The Google Click ID (gclid) associated with this conversion.
    #[prost(string, optional, tag = "4")]
    pub gclid: ::core::option::Option<::prost::alloc::string::String>,
    /// The click identifier for clicks associated with app conversions and
    /// originating from iOS devices starting with iOS14.
    #[prost(string, tag = "8")]
    pub gbraid: ::prost::alloc::string::String,
    /// The click identifier for clicks associated with web conversions and
    /// originating from iOS devices starting with iOS14.
    #[prost(string, tag = "9")]
    pub wbraid: ::prost::alloc::string::String,
    /// Resource name of the conversion action associated with this conversion.
    #[prost(string, optional, tag = "5")]
    pub conversion_action: ::core::option::Option<::prost::alloc::string::String>,
    /// The date time at which the conversion occurred. The format is
    /// "yyyy-mm-dd hh:mm:ss+|-hh:mm", e.g. "2019-01-01 12:32:45-08:00".
    #[prost(string, optional, tag = "6")]
    pub conversion_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// The user identifiers associated with this conversion. Only hashed_email and
    /// hashed_phone_number are supported for conversion uploads. The maximum
    /// number of user identifiers for each conversion is 5.
    #[prost(message, repeated, tag = "7")]
    pub user_identifiers: ::prost::alloc::vec::Vec<super::common::UserIdentifier>,
}
/// Identifying information for a successfully processed CallConversionUpload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallConversionResult {
    /// The caller id from which this call was placed. Caller id is expected to be
    /// in E.164 format with preceding '+' sign.
    #[prost(string, optional, tag = "5")]
    pub caller_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The date time at which the call occurred. The format is
    /// "yyyy-mm-dd hh:mm:ss+|-hh:mm", e.g. "2019-01-01 12:32:45-08:00".
    #[prost(string, optional, tag = "6")]
    pub call_start_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the conversion action associated with this conversion.
    #[prost(string, optional, tag = "7")]
    pub conversion_action: ::core::option::Option<::prost::alloc::string::String>,
    /// The date time at which the conversion occurred. The format is
    /// "yyyy-mm-dd hh:mm:ss+|-hh:mm", e.g. "2019-01-01 12:32:45-08:00".
    #[prost(string, optional, tag = "8")]
    pub conversion_date_time: ::core::option::Option<::prost::alloc::string::String>,
}
/// A custom variable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomVariable {
    /// Resource name of the custom variable associated with this conversion.
    /// Note: Although this resource name consists of a customer id and a
    /// conversion custom variable id, validation will ignore the customer id and
    /// use the conversion custom variable id as the sole identifier of the
    /// conversion custom variable.
    #[prost(string, tag = "1")]
    pub conversion_custom_variable: ::prost::alloc::string::String,
    /// The value string of this custom variable.
    /// The value of the custom variable should not contain private customer data,
    /// such as email addresses or phone numbers.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Contains additional information about cart data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CartData {
    /// The Merchant Center ID where the items are uploaded.
    #[prost(int64, tag = "6")]
    pub merchant_id: i64,
    /// The country code associated with the feed where the items are uploaded.
    #[prost(string, tag = "2")]
    pub feed_country_code: ::prost::alloc::string::String,
    /// The language code associated with the feed where the items are uploaded.
    #[prost(string, tag = "3")]
    pub feed_language_code: ::prost::alloc::string::String,
    /// Sum of all transaction level discounts, such as free shipping and
    /// coupon discounts for the whole cart. The currency code is the same
    /// as that in the ClickConversion message.
    #[prost(double, tag = "4")]
    pub local_transaction_cost: f64,
    /// Data of the items purchased.
    #[prost(message, repeated, tag = "5")]
    pub items: ::prost::alloc::vec::Vec<cart_data::Item>,
}
/// Nested message and enum types in `CartData`.
pub mod cart_data {
    /// Contains data of the items purchased.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Item {
        /// The shopping id of the item. Must be equal to the Merchant Center product
        /// identifier.
        #[prost(string, tag = "1")]
        pub product_id: ::prost::alloc::string::String,
        /// Number of items sold.
        #[prost(int32, tag = "2")]
        pub quantity: i32,
        /// Unit price excluding tax, shipping, and any transaction
        /// level discounts. The currency code is the same as that in the
        /// ClickConversion message.
        #[prost(double, tag = "3")]
        pub unit_price: f64,
    }
}
#[doc = r" Generated client implementations."]
pub mod conversion_upload_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to upload conversions."]
    #[derive(Debug, Clone)]
    pub struct ConversionUploadServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConversionUploadServiceClient<T>
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
        ) -> ConversionUploadServiceClient<InterceptedService<T, F>>
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
            ConversionUploadServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Processes the given click conversions."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [ConversionUploadError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [PartialFailureError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.ConversionUploadService/UploadClickConversions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Processes the given call conversions."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [PartialFailureError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.ConversionUploadService/UploadCallConversions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CustomAudienceService.MutateCustomAudiences][google.ads.googleads.v10.services.CustomAudienceService.MutateCustomAudiences\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomAudiencesRequest {
    /// Required. The ID of the customer whose custom audiences are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual custom audiences.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CustomAudienceOperation>,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// A single operation (create, update) on a custom audience.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAudienceOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "custom_audience_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<custom_audience_operation::Operation>,
}
/// Nested message and enum types in `CustomAudienceOperation`.
pub mod custom_audience_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new custom
        /// audience.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CustomAudience),
        /// Update operation: The custom audience is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::CustomAudience),
        /// Remove operation: A resource name for the removed custom audience is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/customAudiences/{custom_audience_id}`
        #[prost(string, tag = "3")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for custom audience mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomAudiencesResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<MutateCustomAudienceResult>,
}
/// The result for the custom audience mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomAudienceResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod custom_audience_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage custom audiences."]
    #[derive(Debug, Clone)]
    pub struct CustomAudienceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomAudienceServiceClient<T>
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
        ) -> CustomAudienceServiceClient<InterceptedService<T, F>>
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
            CustomAudienceServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates or updates custom audiences. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CustomAudienceError]()"]
        #[doc = "   [CustomInterestError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [OperationAccessDeniedError]()"]
        #[doc = "   [PolicyViolationError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn mutate_custom_audiences(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCustomAudiencesRequest>,
        ) -> Result<tonic::Response<super::MutateCustomAudiencesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v10.services.CustomAudienceService/MutateCustomAudiences",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CustomInterestService.MutateCustomInterests][google.ads.googleads.v10.services.CustomInterestService.MutateCustomInterests\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomInterestsRequest {
    /// Required. The ID of the customer whose custom interests are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual custom interests.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CustomInterestOperation>,
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
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "custom_interest_operation::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<custom_interest_operation::Operation>,
}
/// Nested message and enum types in `CustomInterestOperation`.
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
    pub results: ::prost::alloc::vec::Vec<MutateCustomInterestResult>,
}
/// The result for the custom interest mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomInterestResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod custom_interest_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage custom interests."]
    #[derive(Debug, Clone)]
    pub struct CustomInterestServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomInterestServiceClient<T>
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
        ) -> CustomInterestServiceClient<InterceptedService<T, F>>
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
            CustomInterestServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates or updates custom interests. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CriterionError]()"]
        #[doc = "   [CustomInterestError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [PolicyViolationError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [StringLengthError]()"]
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
                "/google.ads.googleads.v10.services.CustomInterestService/MutateCustomInterests",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CustomerClientLinkService.MutateCustomerClientLink][google.ads.googleads.v10.services.CustomerClientLinkService.MutateCustomerClientLink\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerClientLinkRequest {
    /// Required. The ID of the customer whose customer link are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The operation to perform on the individual CustomerClientLink.
    #[prost(message, optional, tag = "2")]
    pub operation: ::core::option::Option<CustomerClientLinkOperation>,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// A single operation (create, update) on a CustomerClientLink.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerClientLinkOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "customer_client_link_operation::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<customer_client_link_operation::Operation>,
}
/// Nested message and enum types in `CustomerClientLinkOperation`.
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
    pub result: ::core::option::Option<MutateCustomerClientLinkResult>,
}
/// The result for a single customer client link mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerClientLinkResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod customer_client_link_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage customer client links."]
    #[derive(Debug, Clone)]
    pub struct CustomerClientLinkServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerClientLinkServiceClient<T>
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
        ) -> CustomerClientLinkServiceClient<InterceptedService<T, F>>
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
            CustomerClientLinkServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates or updates a customer client link. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [ManagerLinkError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [NewResourceCreationError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CustomerClientLinkService/MutateCustomerClientLink") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[CustomerManagerLinkService.MutateCustomerManagerLink][google.ads.googleads.v10.services.CustomerManagerLinkService.MutateCustomerManagerLink\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerManagerLinkRequest {
    /// Required. The ID of the customer whose customer manager links are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to perform on individual customer manager links.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<CustomerManagerLinkOperation>,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// Request message for \[CustomerManagerLinkService.MoveManagerLink][google.ads.googleads.v10.services.CustomerManagerLinkService.MoveManagerLink\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveManagerLinkRequest {
    /// Required. The ID of the client customer that is being moved.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The resource name of the previous CustomerManagerLink.
    /// The resource name has the form:
    /// `customers/{customer_id}/customerManagerLinks/{manager_customer_id}~{manager_link_id}`
    #[prost(string, tag = "2")]
    pub previous_customer_manager_link: ::prost::alloc::string::String,
    /// Required. The resource name of the new manager customer that the client wants to move
    /// to. Customer resource names have the format: "customers/{customer_id}"
    #[prost(string, tag = "3")]
    pub new_manager: ::prost::alloc::string::String,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
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
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "customer_manager_link_operation::Operation", tags = "2")]
    pub operation: ::core::option::Option<customer_manager_link_operation::Operation>,
}
/// Nested message and enum types in `CustomerManagerLinkOperation`.
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
    pub results: ::prost::alloc::vec::Vec<MutateCustomerManagerLinkResult>,
}
/// Response message for a CustomerManagerLink moveManagerLink.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveManagerLinkResponse {
    /// Returned for successful operations. Represents a CustomerManagerLink
    /// resource of the newly created link between client customer and new manager
    /// customer.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// The result for the customer manager link mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerManagerLinkResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod customer_manager_link_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage customer-manager links."]
    #[derive(Debug, Clone)]
    pub struct CustomerManagerLinkServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerManagerLinkServiceClient<T>
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
        ) -> CustomerManagerLinkServiceClient<InterceptedService<T, F>>
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
            CustomerManagerLinkServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Updates customer manager links. Operation statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [ManagerLinkError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CustomerManagerLinkService/MutateCustomerManagerLink") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Moves a client customer to a new manager customer."]
        #[doc = " This simplifies the complex request that requires two operations to move"]
        #[doc = " a client customer to a new manager. i.e:"]
        #[doc = " 1. Update operation with Status INACTIVE (previous manager) and,"]
        #[doc = " 2. Update operation with Status ACTIVE (new manager)."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.CustomerManagerLinkService/MoveManagerLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[CustomerUserAccessInvitation.MutateCustomerUserAccessInvitation][\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerUserAccessInvitationRequest {
    /// Required. The ID of the customer whose access invitation is being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The operation to perform on the access invitation
    #[prost(message, optional, tag = "2")]
    pub operation: ::core::option::Option<CustomerUserAccessInvitationOperation>,
}
/// A single operation (create or remove) on customer user access invitation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerUserAccessInvitationOperation {
    /// The mutate operation
    #[prost(
        oneof = "customer_user_access_invitation_operation::Operation",
        tags = "1, 2"
    )]
    pub operation: ::core::option::Option<customer_user_access_invitation_operation::Operation>,
}
/// Nested message and enum types in `CustomerUserAccessInvitationOperation`.
pub mod customer_user_access_invitation_operation {
    /// The mutate operation
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new access
        /// invitation.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CustomerUserAccessInvitation),
        /// Remove operation: A resource name for the revoke invitation is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/customerUserAccessInvitations/{invitation_id}`
        #[prost(string, tag = "2")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for access invitation mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerUserAccessInvitationResponse {
    /// Result for the mutate.
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<MutateCustomerUserAccessInvitationResult>,
}
/// The result for the access invitation mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerUserAccessInvitationResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod customer_user_access_invitation_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " This service manages the access invitation extended to users for a given"]
    #[doc = " customer."]
    #[derive(Debug, Clone)]
    pub struct CustomerUserAccessInvitationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerUserAccessInvitationServiceClient<T>
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
        ) -> CustomerUserAccessInvitationServiceClient<InterceptedService<T, F>>
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
            CustomerUserAccessInvitationServiceClient::new(InterceptedService::new(
                inner,
                interceptor,
            ))
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
        #[doc = " Creates or removes an access invitation."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AccessInvitationError]()"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn mutate_customer_user_access_invitation(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCustomerUserAccessInvitationRequest>,
        ) -> Result<tonic::Response<super::MutateCustomerUserAccessInvitationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CustomerUserAccessInvitationService/MutateCustomerUserAccessInvitation") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Mutate Request for
/// \[CustomerUserAccessService.MutateCustomerUserAccess][google.ads.googleads.v10.services.CustomerUserAccessService.MutateCustomerUserAccess\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerUserAccessRequest {
    /// Required. The ID of the customer being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The operation to perform on the customer
    #[prost(message, optional, tag = "2")]
    pub operation: ::core::option::Option<CustomerUserAccessOperation>,
}
/// A single operation (update, remove) on customer user access.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerUserAccessOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "customer_user_access_operation::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<customer_user_access_operation::Operation>,
}
/// Nested message and enum types in `CustomerUserAccessOperation`.
pub mod customer_user_access_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Update operation: The customer user access is expected to have a valid
        /// resource name.
        #[prost(message, tag = "1")]
        Update(super::super::resources::CustomerUserAccess),
        /// Remove operation: A resource name for the removed access is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/customerUserAccesses/{CustomerUserAccess.user_id}`
        #[prost(string, tag = "2")]
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for customer user access mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerUserAccessResponse {
    /// Result for the mutate.
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<MutateCustomerUserAccessResult>,
}
/// The result for the customer user access mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerUserAccessResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod customer_user_access_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " This service manages the permissions of a user on a given customer."]
    #[derive(Debug, Clone)]
    pub struct CustomerUserAccessServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerUserAccessServiceClient<T>
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
        ) -> CustomerUserAccessServiceClient<InterceptedService<T, F>>
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
            CustomerUserAccessServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Updates, removes permission of a user on a given customer. Operation"]
        #[doc = " statuses are returned."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CustomerUserAccessError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn mutate_customer_user_access(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCustomerUserAccessRequest>,
        ) -> Result<tonic::Response<super::MutateCustomerUserAccessResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.CustomerUserAccessService/MutateCustomerUserAccess") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[GeoTargetConstantService.SuggestGeoTargetConstants][google.ads.googleads.v10.services.GeoTargetConstantService.SuggestGeoTargetConstants\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestGeoTargetConstantsRequest {
    /// If possible, returned geo targets are translated using this locale. If not,
    /// en is used by default. This is also used as a hint for returned geo
    /// targets.
    #[prost(string, optional, tag = "6")]
    pub locale: ::core::option::Option<::prost::alloc::string::String>,
    /// Returned geo targets are restricted to this country code.
    #[prost(string, optional, tag = "7")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Required. A selector of geo target constants.
    #[prost(oneof = "suggest_geo_target_constants_request::Query", tags = "1, 2")]
    pub query: ::core::option::Option<suggest_geo_target_constants_request::Query>,
}
/// Nested message and enum types in `SuggestGeoTargetConstantsRequest`.
pub mod suggest_geo_target_constants_request {
    /// A list of location names.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocationNames {
        /// A list of location names.
        #[prost(string, repeated, tag = "2")]
        pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// A list of geo target constant resource names.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GeoTargets {
        /// A list of geo target constant resource names.
        #[prost(string, repeated, tag = "2")]
        pub geo_target_constants: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
/// Response message for \[GeoTargetConstantService.SuggestGeoTargetConstants][google.ads.googleads.v10.services.GeoTargetConstantService.SuggestGeoTargetConstants\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestGeoTargetConstantsResponse {
    /// Geo target constant suggestions.
    #[prost(message, repeated, tag = "1")]
    pub geo_target_constant_suggestions: ::prost::alloc::vec::Vec<GeoTargetConstantSuggestion>,
}
/// A geo target constant suggestion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoTargetConstantSuggestion {
    /// The language this GeoTargetConstantSuggestion is currently translated to.
    /// It affects the name of geo target fields. For example, if locale=en, then
    /// name=Spain. If locale=es, then name=España. The default locale will be
    /// returned if no translation exists for the locale in the request.
    #[prost(string, optional, tag = "6")]
    pub locale: ::core::option::Option<::prost::alloc::string::String>,
    /// Approximate user population that will be targeted, rounded to the
    /// nearest 100.
    #[prost(int64, optional, tag = "7")]
    pub reach: ::core::option::Option<i64>,
    /// If the request searched by location name, this is the location name that
    /// matched the geo target.
    #[prost(string, optional, tag = "8")]
    pub search_term: ::core::option::Option<::prost::alloc::string::String>,
    /// The GeoTargetConstant result.
    #[prost(message, optional, tag = "4")]
    pub geo_target_constant: ::core::option::Option<super::resources::GeoTargetConstant>,
    /// The list of parents of the geo target constant.
    #[prost(message, repeated, tag = "5")]
    pub geo_target_constant_parents: ::prost::alloc::vec::Vec<super::resources::GeoTargetConstant>,
}
#[doc = r" Generated client implementations."]
pub mod geo_target_constant_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to fetch geo target constants."]
    #[derive(Debug, Clone)]
    pub struct GeoTargetConstantServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GeoTargetConstantServiceClient<T>
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
        ) -> GeoTargetConstantServiceClient<InterceptedService<T, F>>
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
            GeoTargetConstantServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns GeoTargetConstant suggestions by location name or by resource name."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [GeoTargetConstantSuggestionError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.GeoTargetConstantService/SuggestGeoTargetConstants") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[GoogleAdsFieldService.GetGoogleAdsField][google.ads.googleads.v10.services.GoogleAdsFieldService.GetGoogleAdsField\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGoogleAdsFieldRequest {
    /// Required. The resource name of the field to get.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// Request message for \[GoogleAdsFieldService.SearchGoogleAdsFields][google.ads.googleads.v10.services.GoogleAdsFieldService.SearchGoogleAdsFields\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchGoogleAdsFieldsRequest {
    /// Required. The query string.
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    /// Token of the page to retrieve. If not specified, the first page of
    /// results will be returned. Use the value obtained from `next_page_token`
    /// in the previous response in order to request the next page of results.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Number of elements to retrieve in a single page.
    /// When too large a page is requested, the server may decide to further
    /// limit the number of returned resources.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// Response message for \[GoogleAdsFieldService.SearchGoogleAdsFields][google.ads.googleads.v10.services.GoogleAdsFieldService.SearchGoogleAdsFields\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchGoogleAdsFieldsResponse {
    /// The list of fields that matched the query.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<super::resources::GoogleAdsField>,
    /// Pagination token used to retrieve the next page of results. Pass the
    /// content of this string as the `page_token` attribute of the next request.
    /// `next_page_token` is not returned for the last page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of results that match the query ignoring the LIMIT clause.
    #[prost(int64, tag = "3")]
    pub total_results_count: i64,
}
#[doc = r" Generated client implementations."]
pub mod google_ads_field_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to fetch Google Ads API fields."]
    #[derive(Debug, Clone)]
    pub struct GoogleAdsFieldServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GoogleAdsFieldServiceClient<T>
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
        ) -> GoogleAdsFieldServiceClient<InterceptedService<T, F>>
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
            GoogleAdsFieldServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns just the requested field."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.GoogleAdsFieldService/GetGoogleAdsField",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns all fields that match the search query."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QueryError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.GoogleAdsFieldService/SearchGoogleAdsFields",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for fetching the invoices of a given billing setup that were
/// issued during a given month.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInvoicesRequest {
    /// Required. The ID of the customer to fetch invoices for.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The billing setup resource name of the requested invoices.
    ///
    /// `customers/{customer_id}/billingSetups/{billing_setup_id}`
    #[prost(string, tag = "2")]
    pub billing_setup: ::prost::alloc::string::String,
    /// Required. The issue year to retrieve invoices, in yyyy format. Only
    /// invoices issued in 2019 or later can be retrieved.
    #[prost(string, tag = "3")]
    pub issue_year: ::prost::alloc::string::String,
    /// Required. The issue month to retrieve invoices.
    #[prost(
        enumeration = "super::enums::month_of_year_enum::MonthOfYear",
        tag = "4"
    )]
    pub issue_month: i32,
}
/// Response message for \[InvoiceService.ListInvoices][google.ads.googleads.v10.services.InvoiceService.ListInvoices\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInvoicesResponse {
    /// The list of invoices that match the billing setup and time period.
    #[prost(message, repeated, tag = "1")]
    pub invoices: ::prost::alloc::vec::Vec<super::resources::Invoice>,
}
#[doc = r" Generated client implementations."]
pub mod invoice_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A service to fetch invoices issued for a billing setup during a given month."]
    #[derive(Debug, Clone)]
    pub struct InvoiceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> InvoiceServiceClient<T>
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
        ) -> InvoiceServiceClient<InterceptedService<T, F>>
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
            InvoiceServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns all invoices associated with a billing setup, for a given month."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [InvoiceError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.InvoiceService/ListInvoices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[KeywordPlanIdeaService.GenerateKeywordIdeas][google.ads.googleads.v10.services.KeywordPlanIdeaService.GenerateKeywordIdeas\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateKeywordIdeasRequest {
    /// The ID of the customer with the recommendation.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// The resource name of the language to target.
    /// Required
    #[prost(string, optional, tag = "14")]
    pub language: ::core::option::Option<::prost::alloc::string::String>,
    /// The resource names of the location to target.
    /// Max 10
    #[prost(string, repeated, tag = "15")]
    pub geo_target_constants: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If true, adult keywords will be included in response.
    /// The default value is false.
    #[prost(bool, tag = "10")]
    pub include_adult_keywords: bool,
    /// Token of the page to retrieve. If not specified, the first
    /// page of results will be returned. To request next page of results use the
    /// value obtained from `next_page_token` in the previous response.
    /// The request fields must match across pages.
    #[prost(string, tag = "12")]
    pub page_token: ::prost::alloc::string::String,
    /// Number of results to retrieve in a single page.
    /// A maximum of 10,000 results may be returned, if the page_size
    /// exceeds this, it is ignored.
    /// If unspecified, at most 10,000 results will be returned.
    /// The server may decide to further limit the number of returned resources.
    /// If the response contains fewer than 10,000 results it may not be assumed
    /// as last page of results.
    #[prost(int32, tag = "13")]
    pub page_size: i32,
    /// Targeting network.
    #[prost(
        enumeration = "super::enums::keyword_plan_network_enum::KeywordPlanNetwork",
        tag = "9"
    )]
    pub keyword_plan_network: i32,
    /// The keyword annotations to include in response.
    #[prost(
        enumeration = "super::enums::keyword_plan_keyword_annotation_enum::KeywordPlanKeywordAnnotation",
        repeated,
        tag = "17"
    )]
    pub keyword_annotation: ::prost::alloc::vec::Vec<i32>,
    /// The aggregate fields to include in response.
    #[prost(message, optional, tag = "16")]
    pub aggregate_metrics: ::core::option::Option<super::common::KeywordPlanAggregateMetrics>,
    /// The options for historical metrics data.
    #[prost(message, optional, tag = "18")]
    pub historical_metrics_options: ::core::option::Option<super::common::HistoricalMetricsOptions>,
    /// The type of seed to generate keyword ideas.
    #[prost(oneof = "generate_keyword_ideas_request::Seed", tags = "2, 3, 5, 11")]
    pub seed: ::core::option::Option<generate_keyword_ideas_request::Seed>,
}
/// Nested message and enum types in `GenerateKeywordIdeasRequest`.
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
        /// The site to generate ideas from, e.g. www.example.com.
        #[prost(message, tag = "11")]
        SiteSeed(super::SiteSeed),
    }
}
/// Keyword And Url Seed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordAndUrlSeed {
    /// The URL to crawl in order to generate keyword ideas.
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// Requires at least one keyword.
    #[prost(string, repeated, tag = "4")]
    pub keywords: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Keyword Seed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordSeed {
    /// Requires at least one keyword.
    #[prost(string, repeated, tag = "2")]
    pub keywords: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Site Seed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SiteSeed {
    /// The domain name of the site. If the customer requesting the ideas doesn't
    /// own the site provided only public information is returned.
    #[prost(string, optional, tag = "2")]
    pub site: ::core::option::Option<::prost::alloc::string::String>,
}
/// Url Seed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UrlSeed {
    /// The URL to crawl in order to generate keyword ideas.
    #[prost(string, optional, tag = "2")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
}
/// Response message for \[KeywordPlanIdeaService.GenerateKeywordIdeas][google.ads.googleads.v10.services.KeywordPlanIdeaService.GenerateKeywordIdeas\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateKeywordIdeaResponse {
    /// Results of generating keyword ideas.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<GenerateKeywordIdeaResult>,
    /// The aggregate metrics for all keyword ideas.
    #[prost(message, optional, tag = "4")]
    pub aggregate_metric_results:
        ::core::option::Option<super::common::KeywordPlanAggregateMetricResults>,
    /// Pagination token used to retrieve the next page of results.
    /// Pass the content of this string as the `page_token` attribute of
    /// the next request.
    /// `next_page_token` is not returned for the last page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of results available.
    #[prost(int64, tag = "3")]
    pub total_size: i64,
}
/// The result of generating keyword ideas.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateKeywordIdeaResult {
    /// Text of the keyword idea.
    /// As in Keyword Plan historical metrics, this text may not be an actual
    /// keyword, but the canonical form of multiple keywords.
    /// See KeywordPlanKeywordHistoricalMetrics message in KeywordPlanService.
    #[prost(string, optional, tag = "5")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    /// The historical metrics for the keyword.
    #[prost(message, optional, tag = "3")]
    pub keyword_idea_metrics: ::core::option::Option<super::common::KeywordPlanHistoricalMetrics>,
    /// The annotations for the keyword.
    /// The annotation data is only provided if requested.
    #[prost(message, optional, tag = "6")]
    pub keyword_annotations: ::core::option::Option<super::common::KeywordAnnotations>,
    /// The list of close variants from the requested keywords that
    /// are combined into this GenerateKeywordIdeaResult. See
    /// <https://support.google.com/google-ads/answer/9342105> for the
    /// definition of "close variants".
    #[prost(string, repeated, tag = "7")]
    pub close_variants: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// \[KeywordPlanIdeaService.GenerateKeywordHistoricalMetrics][google.ads.googleads.v10.services.KeywordPlanIdeaService.GenerateKeywordHistoricalMetrics\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateKeywordHistoricalMetricsRequest {
    /// The ID of the customer with the recommendation.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// A list of keywords to get historical metrics.
    /// Not all inputs will be returned as a result of near-exact deduplication.
    /// For example, if stats for "car" and "cars" are requested, only "car" will
    /// be returned.
    /// A maximum of 10,000 keywords can be used.
    #[prost(string, repeated, tag = "2")]
    pub keywords: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The options for historical metrics data.
    #[prost(message, optional, tag = "3")]
    pub historical_metrics_options: ::core::option::Option<super::common::HistoricalMetricsOptions>,
}
/// Response message for
/// \[KeywordPlanIdeaService.GenerateKeywordHistoricalMetrics][google.ads.googleads.v10.services.KeywordPlanIdeaService.GenerateKeywordHistoricalMetrics\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateKeywordHistoricalMetricsResponse {
    /// List of keywords and their historical metrics.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<GenerateKeywordHistoricalMetricsResult>,
}
/// The result of generating keyword historical metrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateKeywordHistoricalMetricsResult {
    /// The text of the query associated with one or more keywords.
    /// Note that we de-dupe your keywords list, eliminating close variants
    /// before returning the keywords as text. For example, if your request
    /// originally contained the keywords "car" and "cars", the returned search
    /// query will only contain "cars". The list of de-duped queries will be
    /// included in close_variants field.
    #[prost(string, optional, tag = "1")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    /// The list of close variants from the requested keywords whose stats
    /// are combined into this GenerateKeywordHistoricalMetricsResult.
    #[prost(string, repeated, tag = "3")]
    pub close_variants: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The historical metrics for text and its close variants
    #[prost(message, optional, tag = "2")]
    pub keyword_metrics: ::core::option::Option<super::common::KeywordPlanHistoricalMetrics>,
}
#[doc = r" Generated client implementations."]
pub mod keyword_plan_idea_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to generate keyword ideas."]
    #[derive(Debug, Clone)]
    pub struct KeywordPlanIdeaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> KeywordPlanIdeaServiceClient<T>
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
        ) -> KeywordPlanIdeaServiceClient<InterceptedService<T, F>>
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
            KeywordPlanIdeaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns a list of keyword ideas."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [KeywordPlanIdeaError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.KeywordPlanIdeaService/GenerateKeywordIdeas",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a list of keyword historical metrics."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn generate_keyword_historical_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateKeywordHistoricalMetricsRequest>,
        ) -> Result<tonic::Response<super::GenerateKeywordHistoricalMetricsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.KeywordPlanIdeaService/GenerateKeywordHistoricalMetrics") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[KeywordThemeConstantService.SuggestKeywordThemeConstants][google.ads.googleads.v10.services.KeywordThemeConstantService.SuggestKeywordThemeConstants\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestKeywordThemeConstantsRequest {
    /// The query text of a keyword theme that will be used to map to similar
    /// keyword themes. E.g. "plumber" or "roofer".
    #[prost(string, tag = "1")]
    pub query_text: ::prost::alloc::string::String,
    /// Upper-case, two-letter country code as defined by ISO-3166. This for
    /// refining the scope of the query, default to 'US' if not set.
    #[prost(string, tag = "2")]
    pub country_code: ::prost::alloc::string::String,
    /// The two letter language code for get corresponding keyword theme for
    /// refining the scope of the query, default to 'en' if not set.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// Response message for
/// \[KeywordThemeConstantService.SuggestKeywordThemeConstants][google.ads.googleads.v10.services.KeywordThemeConstantService.SuggestKeywordThemeConstants\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestKeywordThemeConstantsResponse {
    /// Smart Campaign keyword theme suggestions.
    #[prost(message, repeated, tag = "1")]
    pub keyword_theme_constants: ::prost::alloc::vec::Vec<super::resources::KeywordThemeConstant>,
}
#[doc = r" Generated client implementations."]
pub mod keyword_theme_constant_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to fetch Smart Campaign keyword themes."]
    #[derive(Debug, Clone)]
    pub struct KeywordThemeConstantServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> KeywordThemeConstantServiceClient<T>
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
        ) -> KeywordThemeConstantServiceClient<InterceptedService<T, F>>
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
            KeywordThemeConstantServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns KeywordThemeConstant suggestions by keyword themes."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn suggest_keyword_theme_constants(
            &mut self,
            request: impl tonic::IntoRequest<super::SuggestKeywordThemeConstantsRequest>,
        ) -> Result<tonic::Response<super::SuggestKeywordThemeConstantsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.KeywordThemeConstantService/SuggestKeywordThemeConstants") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[MerchantCenterLinkService.ListMerchantCenterLinks][google.ads.googleads.v10.services.MerchantCenterLinkService.ListMerchantCenterLinks\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMerchantCenterLinksRequest {
    /// Required. The ID of the customer onto which to apply the Merchant Center link list
    /// operation.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
}
/// Response message for \[MerchantCenterLinkService.ListMerchantCenterLinks][google.ads.googleads.v10.services.MerchantCenterLinkService.ListMerchantCenterLinks\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMerchantCenterLinksResponse {
    /// Merchant Center links available for the requested customer
    #[prost(message, repeated, tag = "1")]
    pub merchant_center_links: ::prost::alloc::vec::Vec<super::resources::MerchantCenterLink>,
}
/// Request message for \[MerchantCenterLinkService.GetMerchantCenterLink][google.ads.googleads.v10.services.MerchantCenterLinkService.GetMerchantCenterLink\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMerchantCenterLinkRequest {
    /// Required. Resource name of the Merchant Center link.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// Request message for \[MerchantCenterLinkService.MutateMerchantCenterLink][google.ads.googleads.v10.services.MerchantCenterLinkService.MutateMerchantCenterLink\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateMerchantCenterLinkRequest {
    /// Required. The ID of the customer being modified.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The operation to perform on the link
    #[prost(message, optional, tag = "2")]
    pub operation: ::core::option::Option<MerchantCenterLinkOperation>,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// A single update on a Merchant Center link.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerchantCenterLinkOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The operation to perform
    #[prost(oneof = "merchant_center_link_operation::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<merchant_center_link_operation::Operation>,
}
/// Nested message and enum types in `MerchantCenterLinkOperation`.
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
        Remove(::prost::alloc::string::String),
    }
}
/// Response message for Merchant Center link mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateMerchantCenterLinkResponse {
    /// Result for the mutate.
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<MutateMerchantCenterLinkResult>,
}
/// The result for the Merchant Center link mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateMerchantCenterLinkResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod merchant_center_link_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " This service allows management of links between Google Ads and Google"]
    #[doc = " Merchant Center."]
    #[derive(Debug, Clone)]
    pub struct MerchantCenterLinkServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MerchantCenterLinkServiceClient<T>
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
        ) -> MerchantCenterLinkServiceClient<InterceptedService<T, F>>
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
            MerchantCenterLinkServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns Merchant Center links available for this customer."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.MerchantCenterLinkService/ListMerchantCenterLinks") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the Merchant Center link in full detail."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.MerchantCenterLinkService/GetMerchantCenterLink") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates status or removes a Merchant Center link."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [FieldMaskError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.MerchantCenterLinkService/MutateMerchantCenterLink") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[OfflineUserDataJobService.CreateOfflineUserDataJob][google.ads.googleads.v10.services.OfflineUserDataJobService.CreateOfflineUserDataJob\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOfflineUserDataJobRequest {
    /// Required. The ID of the customer for which to create an offline user data job.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The offline user data job to be created.
    #[prost(message, optional, tag = "2")]
    pub job: ::core::option::Option<super::resources::OfflineUserDataJob>,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
    /// If true, match rate range for the offline user data job is calculated and
    /// made available in the resource.
    #[prost(bool, tag = "5")]
    pub enable_match_rate_range_preview: bool,
}
/// Response message for
/// \[OfflineUserDataJobService.CreateOfflineUserDataJob][google.ads.googleads.v10.services.OfflineUserDataJobService.CreateOfflineUserDataJob\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOfflineUserDataJobResponse {
    /// The resource name of the OfflineUserDataJob.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// Request message for \[OfflineUserDataJobService.RunOfflineUserDataJob][google.ads.googleads.v10.services.OfflineUserDataJobService.RunOfflineUserDataJob\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunOfflineUserDataJobRequest {
    /// Required. The resource name of the OfflineUserDataJob to run.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
}
/// Request message for
/// \[OfflineUserDataJobService.AddOfflineUserDataJobOperations][google.ads.googleads.v10.services.OfflineUserDataJobService.AddOfflineUserDataJobOperations\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOfflineUserDataJobOperationsRequest {
    /// Required. The resource name of the OfflineUserDataJob.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// True to enable partial failure for the offline user data job.
    #[prost(bool, optional, tag = "4")]
    pub enable_partial_failure: ::core::option::Option<bool>,
    /// True to enable warnings for the offline user data job. When enabled, a
    /// warning will not block the OfflineUserDataJobOperation, and will also
    /// return warning messages about malformed field values.
    #[prost(bool, optional, tag = "6")]
    pub enable_warnings: ::core::option::Option<bool>,
    /// Required. The list of operations to be done.
    #[prost(message, repeated, tag = "3")]
    pub operations: ::prost::alloc::vec::Vec<OfflineUserDataJobOperation>,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// Operation to be made for the AddOfflineUserDataJobOperationsRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfflineUserDataJobOperation {
    /// Operation to be made for the AddOfflineUserDataJobOperationsRequest.
    #[prost(oneof = "offline_user_data_job_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::core::option::Option<offline_user_data_job_operation::Operation>,
}
/// Nested message and enum types in `OfflineUserDataJobOperation`.
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
/// \[OfflineUserDataJobService.AddOfflineUserDataJobOperations][google.ads.googleads.v10.services.OfflineUserDataJobService.AddOfflineUserDataJobOperations\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOfflineUserDataJobOperationsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "1")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// Non blocking errors that pertain to operation failures in the warnings
    /// mode. Returned only when enable_warnings = true.
    #[prost(message, optional, tag = "2")]
    pub warning: ::core::option::Option<super::super::super::super::rpc::Status>,
}
#[doc = r" Generated client implementations."]
pub mod offline_user_data_job_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage offline user data jobs."]
    #[derive(Debug, Clone)]
    pub struct OfflineUserDataJobServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> OfflineUserDataJobServiceClient<T>
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
        ) -> OfflineUserDataJobServiceClient<InterceptedService<T, F>>
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
            OfflineUserDataJobServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates an offline user data job."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [NotAllowlistedError]()"]
        #[doc = "   [OfflineUserDataJobError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.OfflineUserDataJobService/CreateOfflineUserDataJob") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Adds operations to the offline user data job."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [OfflineUserDataJobError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.OfflineUserDataJobService/AddOfflineUserDataJobOperations") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Runs the offline user data job."]
        #[doc = ""]
        #[doc = " When finished, the long running operation will contain the processing"]
        #[doc = " result or failure information, if any."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [OfflineUserDataJobError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.OfflineUserDataJobService/RunOfflineUserDataJob") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for fetching all accessible payments accounts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPaymentsAccountsRequest {
    /// Required. The ID of the customer to apply the PaymentsAccount list operation to.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
}
/// Response message for \[PaymentsAccountService.ListPaymentsAccounts][google.ads.googleads.v10.services.PaymentsAccountService.ListPaymentsAccounts\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPaymentsAccountsResponse {
    /// The list of accessible payments accounts.
    #[prost(message, repeated, tag = "1")]
    pub payments_accounts: ::prost::alloc::vec::Vec<super::resources::PaymentsAccount>,
}
#[doc = r" Generated client implementations."]
pub mod payments_account_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to provide payments accounts that can be used to set up consolidated"]
    #[doc = " billing."]
    #[derive(Debug, Clone)]
    pub struct PaymentsAccountServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PaymentsAccountServiceClient<T>
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
        ) -> PaymentsAccountServiceClient<InterceptedService<T, F>>
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
            PaymentsAccountServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns all payments accounts associated with all managers"]
        #[doc = " between the login customer ID and specified serving customer in the"]
        #[doc = " hierarchy, inclusive."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [PaymentsAccountError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.PaymentsAccountService/ListPaymentsAccounts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[ReachPlanService.ListPlannableLocations][google.ads.googleads.v10.services.ReachPlanService.ListPlannableLocations\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPlannableLocationsRequest {}
/// The list of plannable locations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPlannableLocationsResponse {
    /// The list of locations available for planning.
    /// See
    /// <https://developers.google.com/google-ads/api/reference/data/geotargets>
    /// for sample locations.
    #[prost(message, repeated, tag = "1")]
    pub plannable_locations: ::prost::alloc::vec::Vec<PlannableLocation>,
}
/// A plannable location: country, metro region, province, etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlannableLocation {
    /// The location identifier.
    #[prost(string, optional, tag = "4")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// The unique location name in English.
    #[prost(string, optional, tag = "5")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The parent country (not present if location is a country).
    /// If present, will always be a GeoTargetConstant ID. Additional information
    /// such as country name is provided by
    /// \[ReachPlanService.ListPlannableLocations][google.ads.googleads.v10.services.ReachPlanService.ListPlannableLocations\] or
    /// \[GoogleAdsService.Search/SearchStream][\].
    #[prost(int64, optional, tag = "6")]
    pub parent_country_id: ::core::option::Option<i64>,
    /// The ISO-3166-1 alpha-2 country code that is associated with the location.
    #[prost(string, optional, tag = "7")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// The location's type. Location types correspond to target_type returned by
    /// searching location type in \[GoogleAdsService.Search/SearchStream][\].
    #[prost(string, optional, tag = "8")]
    pub location_type: ::core::option::Option<::prost::alloc::string::String>,
}
/// Request to list available products in a given location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPlannableProductsRequest {
    /// Required. The ID of the selected location for planning. To list the available
    /// plannable location IDs use \[ReachPlanService.ListPlannableLocations][google.ads.googleads.v10.services.ReachPlanService.ListPlannableLocations\].
    #[prost(string, tag = "2")]
    pub plannable_location_id: ::prost::alloc::string::String,
}
/// A response with all available products.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPlannableProductsResponse {
    /// The list of products available for planning and related targeting metadata.
    #[prost(message, repeated, tag = "1")]
    pub product_metadata: ::prost::alloc::vec::Vec<ProductMetadata>,
}
/// The metadata associated with an available plannable product.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductMetadata {
    /// The code associated with the ad product (for example: BUMPER,
    /// TRUEVIEW_IN_STREAM).
    /// To list the available plannable product codes use
    /// \[ReachPlanService.ListPlannableProducts][google.ads.googleads.v10.services.ReachPlanService.ListPlannableProducts\].
    #[prost(string, optional, tag = "4")]
    pub plannable_product_code: ::core::option::Option<::prost::alloc::string::String>,
    /// The name associated with the ad product.
    #[prost(string, tag = "3")]
    pub plannable_product_name: ::prost::alloc::string::String,
    /// The allowed plannable targeting for this product.
    #[prost(message, optional, tag = "2")]
    pub plannable_targeting: ::core::option::Option<PlannableTargeting>,
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
    pub age_ranges: ::prost::alloc::vec::Vec<i32>,
    /// Targetable genders for the ad product.
    #[prost(message, repeated, tag = "2")]
    pub genders: ::prost::alloc::vec::Vec<super::common::GenderInfo>,
    /// Targetable devices for the ad product.
    /// TABLET device targeting is automatically applied to reported metrics
    /// when MOBILE targeting is selected for CPM_MASTHEAD,
    /// GOOGLE_PREFERRED_BUMPER, and GOOGLE_PREFERRED_SHORT products.
    #[prost(message, repeated, tag = "3")]
    pub devices: ::prost::alloc::vec::Vec<super::common::DeviceInfo>,
    /// Targetable networks for the ad product.
    #[prost(
        enumeration = "super::enums::reach_plan_network_enum::ReachPlanNetwork",
        repeated,
        tag = "4"
    )]
    pub networks: ::prost::alloc::vec::Vec<i32>,
}
/// Request message for \[ReachPlanService.GenerateProductMixIdeas][google.ads.googleads.v10.services.ReachPlanService.GenerateProductMixIdeas\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateProductMixIdeasRequest {
    /// Required. The ID of the customer.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The ID of the location, this is one of the IDs returned by
    /// \[ReachPlanService.ListPlannableLocations][google.ads.googleads.v10.services.ReachPlanService.ListPlannableLocations\].
    #[prost(string, tag = "6")]
    pub plannable_location_id: ::prost::alloc::string::String,
    /// Required. Currency code.
    /// Three-character ISO 4217 currency code.
    #[prost(string, tag = "7")]
    pub currency_code: ::prost::alloc::string::String,
    /// Required. Total budget.
    /// Amount in micros. One million is equivalent to one unit.
    #[prost(int64, tag = "8")]
    pub budget_micros: i64,
    /// The preferences of the suggested product mix.
    /// An unset preference is interpreted as all possible values are allowed,
    /// unless explicitly specified.
    #[prost(message, optional, tag = "5")]
    pub preferences: ::core::option::Option<Preferences>,
}
/// Set of preferences about the planned mix.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Preferences {
    /// True if ad skippable.
    /// If not set, default is any value.
    #[prost(bool, optional, tag = "6")]
    pub is_skippable: ::core::option::Option<bool>,
    /// True if ad start with sound.
    /// If not set, default is any value.
    #[prost(bool, optional, tag = "7")]
    pub starts_with_sound: ::core::option::Option<bool>,
    /// The length of the ad.
    /// If not set, default is any value.
    #[prost(
        enumeration = "super::enums::reach_plan_ad_length_enum::ReachPlanAdLength",
        tag = "3"
    )]
    pub ad_length: i32,
    /// True if ad will only show on the top content.
    /// If not set, default is false.
    #[prost(bool, optional, tag = "8")]
    pub top_content_only: ::core::option::Option<bool>,
    /// True if the price is guaranteed. The cost of serving the ad is agreed
    /// upfront and not subject to an auction.
    /// If not set, default is any value.
    #[prost(bool, optional, tag = "9")]
    pub has_guaranteed_price: ::core::option::Option<bool>,
}
/// The suggested product mix.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateProductMixIdeasResponse {
    /// A list of products (ad formats) and the associated budget allocation idea.
    #[prost(message, repeated, tag = "1")]
    pub product_allocation: ::prost::alloc::vec::Vec<ProductAllocation>,
}
/// An allocation of a part of the budget on a given product.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductAllocation {
    /// Selected product for planning. The product codes returned are within the
    /// set of the ones returned by ListPlannableProducts when using the same
    /// location ID.
    #[prost(string, optional, tag = "3")]
    pub plannable_product_code: ::core::option::Option<::prost::alloc::string::String>,
    /// The value to be allocated for the suggested product in requested currency.
    /// Amount in micros. One million is equivalent to one unit.
    #[prost(int64, optional, tag = "4")]
    pub budget_micros: ::core::option::Option<i64>,
}
/// Request message for \[ReachPlanService.GenerateReachForecast][google.ads.googleads.v10.services.ReachPlanService.GenerateReachForecast\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateReachForecastRequest {
    /// Required. The ID of the customer.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// The currency code.
    /// Three-character ISO 4217 currency code.
    #[prost(string, optional, tag = "9")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Required. Campaign duration.
    #[prost(message, optional, tag = "3")]
    pub campaign_duration: ::core::option::Option<CampaignDuration>,
    /// Desired cookie frequency cap to be applied to each planned product.
    /// This is equivalent to the frequency cap exposed in Google Ads when creating
    /// a campaign, it represents the maximum number of times an ad can be shown to
    /// the same user.
    /// If not specified, no cap is applied.
    ///
    /// This field is deprecated in v4 and will eventually be removed.
    /// Please use cookie_frequency_cap_setting instead.
    #[prost(int32, optional, tag = "10")]
    pub cookie_frequency_cap: ::core::option::Option<i32>,
    /// Desired cookie frequency cap to be applied to each planned product.
    /// This is equivalent to the frequency cap exposed in Google Ads when creating
    /// a campaign, it represents the maximum number of times an ad can be shown to
    /// the same user during a specified time interval.
    /// If not specified, a default of 0 (no cap) is applied.
    ///
    /// This field replaces the deprecated cookie_frequency_cap field.
    #[prost(message, optional, tag = "8")]
    pub cookie_frequency_cap_setting: ::core::option::Option<FrequencyCap>,
    /// Desired minimum effective frequency (the number of times a person was
    /// exposed to the ad) for the reported reach metrics \[1-10\].
    /// This won't affect the targeting, but just the reporting.
    /// If not specified, a default of 1 is applied.
    ///
    /// This field cannot be combined with the effective_frequency_limit field.
    #[prost(int32, optional, tag = "11")]
    pub min_effective_frequency: ::core::option::Option<i32>,
    /// The highest minimum effective frequency (the number of times a person was
    /// exposed to the ad) value \[1-10\] to include in
    /// Forecast.effective_frequency_breakdowns.
    /// If not specified, Forecast.effective_frequency_breakdowns will not be
    /// provided.
    ///
    /// The effective frequency value provided here will also be used as the
    /// minimum effective frequency for the reported reach metrics.
    ///
    /// This field cannot be combined with the min_effective_frequency field.
    #[prost(message, optional, tag = "12")]
    pub effective_frequency_limit: ::core::option::Option<EffectiveFrequencyLimit>,
    /// The targeting to be applied to all products selected in the product mix.
    ///
    /// This is planned targeting: execution details might vary based on the
    /// advertising product, please consult an implementation specialist.
    ///
    /// See specific metrics for details on how targeting affects them.
    #[prost(message, optional, tag = "6")]
    pub targeting: ::core::option::Option<Targeting>,
    /// Required. The products to be forecast.
    /// The max number of allowed planned products is 15.
    #[prost(message, repeated, tag = "7")]
    pub planned_products: ::prost::alloc::vec::Vec<PlannedProduct>,
}
/// Effective frequency limit.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EffectiveFrequencyLimit {
    /// The highest effective frequency value to include in
    /// Forecast.effective_frequency_breakdowns.
    /// This field supports frequencies 1-10, inclusive.
    #[prost(int32, tag = "1")]
    pub effective_frequency_breakdown_limit: i32,
}
/// A rule specifying the maximum number of times an ad can be shown to a user
/// over a particular time period.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrequencyCap {
    /// Required. The number of impressions, inclusive.
    #[prost(int32, tag = "3")]
    pub impressions: i32,
    /// Required. The type of time unit.
    #[prost(
        enumeration = "super::enums::frequency_cap_time_unit_enum::FrequencyCapTimeUnit",
        tag = "2"
    )]
    pub time_unit: i32,
}
/// The targeting for which traffic metrics will be reported.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Targeting {
    /// Required. The ID of the selected location. Plannable location IDs can be
    /// obtained from \[ReachPlanService.ListPlannableLocations][google.ads.googleads.v10.services.ReachPlanService.ListPlannableLocations\].
    #[prost(string, optional, tag = "6")]
    pub plannable_location_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Targeted age range.
    /// An unset value is equivalent to targeting all ages.
    #[prost(
        enumeration = "super::enums::reach_plan_age_range_enum::ReachPlanAgeRange",
        tag = "2"
    )]
    pub age_range: i32,
    /// Targeted genders.
    /// An unset value is equivalent to targeting MALE and FEMALE.
    #[prost(message, repeated, tag = "3")]
    pub genders: ::prost::alloc::vec::Vec<super::common::GenderInfo>,
    /// Targeted devices.
    /// If not specified, targets all applicable devices. Applicable devices vary
    /// by product and region and can be obtained from
    /// \[ReachPlanService.ListPlannableProducts][google.ads.googleads.v10.services.ReachPlanService.ListPlannableProducts\].
    #[prost(message, repeated, tag = "4")]
    pub devices: ::prost::alloc::vec::Vec<super::common::DeviceInfo>,
    /// Targetable network for the ad product.
    /// If not specified, targets all applicable networks. Applicable networks vary
    /// by product and region and can be obtained from
    /// \[ReachPlanService.ListPlannableProducts][google.ads.googleads.v10.services.ReachPlanService.ListPlannableProducts\].
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
    ///
    /// This field cannot be combined with the date_range field.
    #[prost(int32, optional, tag = "2")]
    pub duration_in_days: ::core::option::Option<i32>,
    /// Date range of the campaign.
    /// Dates are in the yyyy-mm-dd format and inclusive.
    /// The end date must be < 1 year in the future and the
    /// date range must be <= 92 days long.
    ///
    /// This field cannot be combined with the duration_in_days field.
    #[prost(message, optional, tag = "3")]
    pub date_range: ::core::option::Option<super::common::DateRange>,
}
/// A product being planned for reach.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlannedProduct {
    /// Required. Selected product for planning.
    /// The code associated with the ad product (for example: Trueview, Bumper).
    /// To list the available plannable product codes use
    /// \[ReachPlanService.ListPlannableProducts][google.ads.googleads.v10.services.ReachPlanService.ListPlannableProducts\].
    #[prost(string, optional, tag = "3")]
    pub plannable_product_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Required. Maximum budget allocation in micros for the selected product.
    /// The value is specified in the selected planning currency_code.
    /// For example: 1 000 000$ = 1 000 000 000 000 micros.
    #[prost(int64, optional, tag = "4")]
    pub budget_micros: ::core::option::Option<i64>,
}
/// Response message containing the generated reach curve.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateReachForecastResponse {
    /// Reference on target audiences for this curve.
    #[prost(message, optional, tag = "1")]
    pub on_target_audience_metrics: ::core::option::Option<OnTargetAudienceMetrics>,
    /// The generated reach curve for the planned product mix.
    #[prost(message, optional, tag = "2")]
    pub reach_curve: ::core::option::Option<ReachCurve>,
}
/// The reach curve for the planned products.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReachCurve {
    /// All points on the reach curve.
    #[prost(message, repeated, tag = "1")]
    pub reach_forecasts: ::prost::alloc::vec::Vec<ReachForecast>,
}
/// A point on reach curve.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReachForecast {
    /// The cost in micros.
    #[prost(int64, tag = "5")]
    pub cost_micros: i64,
    /// Forecasted traffic metrics for this point.
    #[prost(message, optional, tag = "2")]
    pub forecast: ::core::option::Option<Forecast>,
    /// The forecasted allocation and traffic metrics for each planned product
    /// at this point on the reach curve.
    #[prost(message, repeated, tag = "4")]
    pub planned_product_reach_forecasts: ::prost::alloc::vec::Vec<PlannedProductReachForecast>,
}
/// Forecasted traffic metrics for the planned products and targeting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Forecast {
    /// Number of unique people reached at least
    /// GenerateReachForecastRequest.min_effective_frequency or
    /// GenerateReachForecastRequest.effective_frequency_limit times that exactly
    /// matches the Targeting.
    ///
    /// Note that a minimum number of unique people must be reached in order for
    /// data to be reported. If the minimum number is not met, the on_target_reach
    /// value will be rounded to 0.
    #[prost(int64, optional, tag = "5")]
    pub on_target_reach: ::core::option::Option<i64>,
    /// Total number of unique people reached at least
    /// GenerateReachForecastRequest.min_effective_frequency or
    /// GenerateReachForecastRequest.effective_frequency_limit times. This includes
    /// people that may fall outside the specified Targeting.
    ///
    /// Note that a minimum number of unique people must be reached in order for
    /// data to be reported. If the minimum number is not met, the total_reach
    /// value will be rounded to 0.
    #[prost(int64, optional, tag = "6")]
    pub total_reach: ::core::option::Option<i64>,
    /// Number of ad impressions that exactly matches the Targeting.
    #[prost(int64, optional, tag = "7")]
    pub on_target_impressions: ::core::option::Option<i64>,
    /// Total number of ad impressions. This includes impressions that may fall
    /// outside the specified Targeting, due to insufficient information on
    /// signed-in users.
    #[prost(int64, optional, tag = "8")]
    pub total_impressions: ::core::option::Option<i64>,
    /// Number of times the ad's impressions were considered viewable.
    /// See <https://support.google.com/google-ads/answer/7029393> for
    /// more information about what makes an ad viewable and how
    /// viewability is measured.
    #[prost(int64, optional, tag = "9")]
    pub viewable_impressions: ::core::option::Option<i64>,
    /// A list of effective frequency forecasts. The list is ordered starting with
    /// 1+ and ending with the value set in
    /// GenerateReachForecastRequest.effective_frequency_limit. If no
    /// effective_frequency_limit was set, this list will be empty.
    #[prost(message, repeated, tag = "10")]
    pub effective_frequency_breakdowns: ::prost::alloc::vec::Vec<EffectiveFrequencyBreakdown>,
}
/// The forecasted allocation and traffic metrics for a specific product
/// at a point on the reach curve.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlannedProductReachForecast {
    /// Selected product for planning. The product codes returned are within the
    /// set of the ones returned by ListPlannableProducts when using the same
    /// location ID.
    #[prost(string, tag = "1")]
    pub plannable_product_code: ::prost::alloc::string::String,
    /// The cost in micros. This may differ from the product's input allocation
    /// if one or more planned products cannot fulfill the budget because of
    /// limited inventory.
    #[prost(int64, tag = "2")]
    pub cost_micros: i64,
    /// Forecasted traffic metrics for this product.
    #[prost(message, optional, tag = "3")]
    pub planned_product_forecast: ::core::option::Option<PlannedProductForecast>,
}
/// Forecasted traffic metrics for a planned product.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlannedProductForecast {
    /// Number of unique people reached that exactly matches the Targeting.
    ///
    /// Note that a minimum number of unique people must be reached in order for
    /// data to be reported. If the minimum number is not met, the on_target_reach
    /// value will be rounded to 0.
    #[prost(int64, tag = "1")]
    pub on_target_reach: i64,
    /// Number of unique people reached. This includes people that may fall
    /// outside the specified Targeting.
    ///
    /// Note that a minimum number of unique people must be reached in order for
    /// data to be reported. If the minimum number is not met, the total_reach
    /// value will be rounded to 0.
    #[prost(int64, tag = "2")]
    pub total_reach: i64,
    /// Number of ad impressions that exactly matches the Targeting.
    #[prost(int64, tag = "3")]
    pub on_target_impressions: i64,
    /// Total number of ad impressions. This includes impressions that may fall
    /// outside the specified Targeting, due to insufficient information on
    /// signed-in users.
    #[prost(int64, tag = "4")]
    pub total_impressions: i64,
    /// Number of times the ad's impressions were considered viewable.
    /// See <https://support.google.com/google-ads/answer/7029393> for
    /// more information about what makes an ad viewable and how
    /// viewability is measured.
    #[prost(int64, optional, tag = "5")]
    pub viewable_impressions: ::core::option::Option<i64>,
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
    #[prost(int64, optional, tag = "3")]
    pub youtube_audience_size: ::core::option::Option<i64>,
    /// Reference audience size matching the considered targeting for Census.
    #[prost(int64, optional, tag = "4")]
    pub census_audience_size: ::core::option::Option<i64>,
}
/// A breakdown of the number of unique people reached at a given effective
/// frequency.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EffectiveFrequencyBreakdown {
    /// The effective frequency \[1-10\].
    #[prost(int32, tag = "1")]
    pub effective_frequency: i32,
    /// The number of unique people reached at least effective_frequency times that
    /// exactly matches the Targeting.
    ///
    /// Note that a minimum number of unique people must be reached in order for
    /// data to be reported. If the minimum number is not met, the on_target_reach
    /// value will be rounded to 0.
    #[prost(int64, tag = "2")]
    pub on_target_reach: i64,
    /// Total number of unique people reached at least effective_frequency times.
    /// This includes people that may fall outside the specified Targeting.
    ///
    /// Note that a minimum number of unique people must be reached in order for
    /// data to be reported. If the minimum number is not met, the total_reach
    /// value will be rounded to 0.
    #[prost(int64, tag = "3")]
    pub total_reach: i64,
}
#[doc = r" Generated client implementations."]
pub mod reach_plan_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Reach Plan Service gives users information about audience size that can"]
    #[doc = " be reached through advertisement on YouTube. In particular,"]
    #[doc = " GenerateReachForecast provides estimated number of people of specified"]
    #[doc = " demographics that can be reached by an ad in a given market by a campaign of"]
    #[doc = " certain duration with a defined budget."]
    #[derive(Debug, Clone)]
    pub struct ReachPlanServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ReachPlanServiceClient<T>
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
        ) -> ReachPlanServiceClient<InterceptedService<T, F>>
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
            ReachPlanServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns the list of plannable locations (for example, countries)."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.ReachPlanService/ListPlannableLocations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the list of per-location plannable YouTube ad formats with allowed"]
        #[doc = " targeting."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.ReachPlanService/ListPlannableProducts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Generates a product mix ideas given a set of preferences. This method"]
        #[doc = " helps the advertiser to obtain a good mix of ad formats and budget"]
        #[doc = " allocations based on its preferences."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [ReachPlanError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.ReachPlanService/GenerateProductMixIdeas",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Generates a reach forecast for a given targeting / product mix."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RangeError]()"]
        #[doc = "   [ReachPlanError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.ReachPlanService/GenerateReachForecast",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[RecommendationService.ApplyRecommendation][google.ads.googleads.v10.services.RecommendationService.ApplyRecommendation\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyRecommendationRequest {
    /// Required. The ID of the customer with the recommendation.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to apply recommendations.
    /// If partial_failure=false all recommendations should be of the same type
    /// There is a limit of 100 operations per request.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<ApplyRecommendationOperation>,
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
    pub resource_name: ::prost::alloc::string::String,
    /// Parameters to use when applying the recommendation.
    #[prost(
        oneof = "apply_recommendation_operation::ApplyParameters",
        tags = "2, 3, 4, 5, 10, 6, 7, 8, 9, 11, 12, 13"
    )]
    pub apply_parameters: ::core::option::Option<apply_recommendation_operation::ApplyParameters>,
}
/// Nested message and enum types in `ApplyRecommendationOperation`.
pub mod apply_recommendation_operation {
    /// Parameters to use when applying a campaign budget recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CampaignBudgetParameters {
        /// New budget amount to set for target budget resource. This is a required
        /// field.
        #[prost(int64, optional, tag = "2")]
        pub new_budget_amount_micros: ::core::option::Option<i64>,
    }
    /// Parameters to use when applying a text ad recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextAdParameters {
        /// New ad to add to recommended ad group. All necessary fields need to be
        /// set in this message. This is a required field.
        #[prost(message, optional, tag = "1")]
        pub ad: ::core::option::Option<super::super::resources::Ad>,
    }
    /// Parameters to use when applying keyword recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeywordParameters {
        /// The ad group resource to add keyword to. This is a required field.
        #[prost(string, optional, tag = "4")]
        pub ad_group: ::core::option::Option<::prost::alloc::string::String>,
        /// The match type of the keyword. This is a required field.
        #[prost(
            enumeration = "super::super::enums::keyword_match_type_enum::KeywordMatchType",
            tag = "2"
        )]
        pub match_type: i32,
        /// Optional, CPC bid to set for the keyword. If not set, keyword will use
        /// bid based on bidding strategy used by target ad group.
        #[prost(int64, optional, tag = "5")]
        pub cpc_bid_micros: ::core::option::Option<i64>,
    }
    /// Parameters to use when applying Target CPA recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TargetCpaOptInParameters {
        /// Average CPA to use for Target CPA bidding strategy. This is a required
        /// field.
        #[prost(int64, optional, tag = "3")]
        pub target_cpa_micros: ::core::option::Option<i64>,
        /// Optional, budget amount to set for the campaign.
        #[prost(int64, optional, tag = "4")]
        pub new_campaign_budget_amount_micros: ::core::option::Option<i64>,
    }
    /// Parameters to use when applying a Target ROAS opt-in recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TargetRoasOptInParameters {
        /// Average ROAS (revenue per unit of spend) to use for Target ROAS bidding
        /// strategy. The value is between 0.01 and 1000.0, inclusive. This is a
        /// required field, unless new_campaign_budget_amount_micros is set.
        #[prost(double, optional, tag = "1")]
        pub target_roas: ::core::option::Option<f64>,
        /// Optional, budget amount to set for the campaign.
        #[prost(int64, optional, tag = "2")]
        pub new_campaign_budget_amount_micros: ::core::option::Option<i64>,
    }
    /// Parameters to use when applying callout extension recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CalloutExtensionParameters {
        /// Callout extensions to be added. This is a required field.
        #[prost(message, repeated, tag = "1")]
        pub callout_extensions: ::prost::alloc::vec::Vec<super::super::common::CalloutFeedItem>,
    }
    /// Parameters to use when applying call extension recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CallExtensionParameters {
        /// Call extensions to be added. This is a required field.
        #[prost(message, repeated, tag = "1")]
        pub call_extensions: ::prost::alloc::vec::Vec<super::super::common::CallFeedItem>,
    }
    /// Parameters to use when applying sitelink extension recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SitelinkExtensionParameters {
        /// Sitelink extensions to be added. This is a required field.
        #[prost(message, repeated, tag = "1")]
        pub sitelink_extensions: ::prost::alloc::vec::Vec<super::super::common::SitelinkFeedItem>,
    }
    /// Parameters to use when applying move unused budget recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MoveUnusedBudgetParameters {
        /// Budget amount to move from excess budget to constrained budget. This is
        /// a required field.
        #[prost(int64, optional, tag = "2")]
        pub budget_micros_to_move: ::core::option::Option<i64>,
    }
    /// Parameters to use when applying a responsive search ad asset
    /// recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResponsiveSearchAdAssetParameters {
        /// Updated ad. The current ad's content will be replaced.
        #[prost(message, optional, tag = "1")]
        pub updated_ad: ::core::option::Option<super::super::resources::Ad>,
    }
    /// Parameters to use when applying a responsive search ad recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResponsiveSearchAdParameters {
        /// Required. New ad to add to recommended ad group.
        #[prost(message, optional, tag = "1")]
        pub ad: ::core::option::Option<super::super::resources::Ad>,
    }
    /// Parameters to use when applying a use broad match keyword recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UseBroadMatchKeywordParameters {
        /// New budget amount to set for target budget resource.
        #[prost(int64, optional, tag = "1")]
        pub new_budget_amount_micros: ::core::option::Option<i64>,
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
        /// Optional parameters to use when applying target ROAS opt-in
        /// recommendation.
        #[prost(message, tag = "10")]
        TargetRoasOptIn(TargetRoasOptInParameters),
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
        /// Parameters to use when applying a responsive search ad recommendation.
        #[prost(message, tag = "11")]
        ResponsiveSearchAd(ResponsiveSearchAdParameters),
        /// Parameters to use when applying a use broad match keyword recommendation.
        #[prost(message, tag = "12")]
        UseBroadMatchKeyword(UseBroadMatchKeywordParameters),
        /// Parameters to use when applying a responsive search ad recommendation.
        #[prost(message, tag = "13")]
        ResponsiveSearchAdAsset(ResponsiveSearchAdAssetParameters),
    }
}
/// Response message for \[RecommendationService.ApplyRecommendation][google.ads.googleads.v10.services.RecommendationService.ApplyRecommendation\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyRecommendationResponse {
    /// Results of operations to apply recommendations.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<ApplyRecommendationResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors)
    /// we return the RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The result of applying a recommendation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyRecommendationResult {
    /// Returned for successful applies.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// Request message for \[RecommendationService.DismissRecommendation][google.ads.googleads.v10.services.RecommendationService.DismissRecommendation\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DismissRecommendationRequest {
    /// Required. The ID of the customer with the recommendation.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to dismiss recommendations.
    /// If partial_failure=false all recommendations should be of the same type
    /// There is a limit of 100 operations per request.
    #[prost(message, repeated, tag = "3")]
    pub operations:
        ::prost::alloc::vec::Vec<dismiss_recommendation_request::DismissRecommendationOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, operations will be carried in a
    /// single transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "2")]
    pub partial_failure: bool,
}
/// Nested message and enum types in `DismissRecommendationRequest`.
pub mod dismiss_recommendation_request {
    /// Operation to dismiss a single recommendation identified by resource_name.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DismissRecommendationOperation {
        /// The resource name of the recommendation to dismiss.
        #[prost(string, tag = "1")]
        pub resource_name: ::prost::alloc::string::String,
    }
}
/// Response message for \[RecommendationService.DismissRecommendation][google.ads.googleads.v10.services.RecommendationService.DismissRecommendation\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DismissRecommendationResponse {
    /// Results of operations to dismiss recommendations.
    #[prost(message, repeated, tag = "1")]
    pub results:
        ::prost::alloc::vec::Vec<dismiss_recommendation_response::DismissRecommendationResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors)
    /// we return the RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// Nested message and enum types in `DismissRecommendationResponse`.
pub mod dismiss_recommendation_response {
    /// The result of dismissing a recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DismissRecommendationResult {
        /// Returned for successful dismissals.
        #[prost(string, tag = "1")]
        pub resource_name: ::prost::alloc::string::String,
    }
}
#[doc = r" Generated client implementations."]
pub mod recommendation_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage recommendations."]
    #[derive(Debug, Clone)]
    pub struct RecommendationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RecommendationServiceClient<T>
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
        ) -> RecommendationServiceClient<InterceptedService<T, F>>
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
            RecommendationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Applies given recommendations with corresponding apply parameters."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [DatabaseError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RecommendationError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [UrlFieldError]()"]
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
                "/google.ads.googleads.v10.services.RecommendationService/ApplyRecommendation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Dismisses given recommendations."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RecommendationError]()"]
        #[doc = "   [RequestError]()"]
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
                "/google.ads.googleads.v10.services.RecommendationService/DismissRecommendation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[SmartCampaignSuggestService.SuggestSmartCampaignBudgets][\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestSmartCampaignBudgetOptionsRequest {
    /// Required. The ID of the customer whose budget options are to be suggested.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. For first time campaign creation use SuggestionInfo, for
    /// subsequent updates on BudgetOptions based on an already created campaign
    /// use that campaign.
    #[prost(
        oneof = "suggest_smart_campaign_budget_options_request::SuggestionData",
        tags = "2, 3"
    )]
    pub suggestion_data:
        ::core::option::Option<suggest_smart_campaign_budget_options_request::SuggestionData>,
}
/// Nested message and enum types in `SuggestSmartCampaignBudgetOptionsRequest`.
pub mod suggest_smart_campaign_budget_options_request {
    /// Required. For first time campaign creation use SuggestionInfo, for
    /// subsequent updates on BudgetOptions based on an already created campaign
    /// use that campaign.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SuggestionData {
        /// Required. The resource name of the campaign to get suggestion for.
        #[prost(string, tag = "2")]
        Campaign(::prost::alloc::string::String),
        /// Required. Information needed to get budget options
        #[prost(message, tag = "3")]
        SuggestionInfo(super::SmartCampaignSuggestionInfo),
    }
}
/// Information needed to get suggestion for Smart Campaign. More information
/// provided will help the system to derive better suggestions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmartCampaignSuggestionInfo {
    /// Optional. Landing page URL of the campaign.
    #[prost(string, tag = "1")]
    pub final_url: ::prost::alloc::string::String,
    /// Optional. The two letter advertising language for the Smart campaign to be
    /// constructed, default to 'en' if not set.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. The business ad schedule.
    #[prost(message, repeated, tag = "6")]
    pub ad_schedules: ::prost::alloc::vec::Vec<super::common::AdScheduleInfo>,
    /// Optional. Smart campaign keyword themes. This field may greatly improve suggestion
    /// accuracy and we recommend always setting it if possible.
    #[prost(message, repeated, tag = "7")]
    pub keyword_themes: ::prost::alloc::vec::Vec<super::common::KeywordThemeInfo>,
    /// The business settings to consider when generating suggestions.
    /// Settings are automatically extracted from the business when provided.
    /// Otherwise, these settings must be specified explicitly.
    #[prost(
        oneof = "smart_campaign_suggestion_info::BusinessSetting",
        tags = "8, 2"
    )]
    pub business_setting: ::core::option::Option<smart_campaign_suggestion_info::BusinessSetting>,
    /// The geo target of the campaign, either a list of locations or
    /// a single proximity shall be specified.
    #[prost(oneof = "smart_campaign_suggestion_info::GeoTarget", tags = "4, 5")]
    pub geo_target: ::core::option::Option<smart_campaign_suggestion_info::GeoTarget>,
}
/// Nested message and enum types in `SmartCampaignSuggestionInfo`.
pub mod smart_campaign_suggestion_info {
    /// A list of locations.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocationList {
        /// Required. Locations.
        #[prost(message, repeated, tag = "1")]
        pub locations: ::prost::alloc::vec::Vec<super::super::common::LocationInfo>,
    }
    /// A context that describes a business.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BusinessContext {
        /// Optional. The name of the business.
        #[prost(string, tag = "1")]
        pub business_name: ::prost::alloc::string::String,
    }
    /// The business settings to consider when generating suggestions.
    /// Settings are automatically extracted from the business when provided.
    /// Otherwise, these settings must be specified explicitly.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BusinessSetting {
        /// Optional. Context describing the business to advertise.
        #[prost(message, tag = "8")]
        BusinessContext(BusinessContext),
        /// Optional. The ID of the Business Profile location.
        /// The location ID can be fetched by Business Profile API with its form:
        /// accounts/{accountId}/locations/{locationId}. The last {locationId}
        /// component from the Business Profile API represents the
        /// business_location_id. See the [Business Profile API]
        /// (<https://developers.google.com/my-business/reference/rest/v4/accounts.locations>)
        #[prost(int64, tag = "2")]
        BusinessLocationId(i64),
    }
    /// The geo target of the campaign, either a list of locations or
    /// a single proximity shall be specified.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GeoTarget {
        /// Optional. The targeting geo location by locations.
        #[prost(message, tag = "4")]
        LocationList(LocationList),
        /// Optional. The targeting geo location by proximity.
        #[prost(message, tag = "5")]
        Proximity(super::super::common::ProximityInfo),
    }
}
/// Response message for
/// \[SmartCampaignSuggestService.SuggestSmartCampaignBudgets][\]. Depending on
/// whether the system could suggest the options, either all of the options or
/// none of them might be returned.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestSmartCampaignBudgetOptionsResponse {
    /// Optional. The lowest budget option.
    #[prost(message, optional, tag = "1")]
    pub low: ::core::option::Option<suggest_smart_campaign_budget_options_response::BudgetOption>,
    /// Optional. The recommended budget option.
    #[prost(message, optional, tag = "2")]
    pub recommended:
        ::core::option::Option<suggest_smart_campaign_budget_options_response::BudgetOption>,
    /// Optional. The highest budget option.
    #[prost(message, optional, tag = "3")]
    pub high: ::core::option::Option<suggest_smart_campaign_budget_options_response::BudgetOption>,
}
/// Nested message and enum types in `SuggestSmartCampaignBudgetOptionsResponse`.
pub mod suggest_smart_campaign_budget_options_response {
    /// Performance metrics for a given budget option.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Metrics {
        /// The estimated min daily clicks.
        #[prost(int64, tag = "1")]
        pub min_daily_clicks: i64,
        /// The estimated max daily clicks.
        #[prost(int64, tag = "2")]
        pub max_daily_clicks: i64,
    }
    /// Smart Campaign budget option.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BudgetOption {
        /// The amount of the budget, in the local currency for the account.
        /// Amount is specified in micros, where one million is equivalent to one
        /// currency unit.
        #[prost(int64, tag = "1")]
        pub daily_amount_micros: i64,
        /// Metrics pertaining to the suggested budget, could be empty if there is
        /// not enough information to derive the estimates.
        #[prost(message, optional, tag = "2")]
        pub metrics: ::core::option::Option<Metrics>,
    }
}
/// Request message for
/// \[SmartCampaignSuggestService.SuggestSmartCampaignAd][google.ads.googleads.v10.services.SmartCampaignSuggestService.SuggestSmartCampaignAd\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestSmartCampaignAdRequest {
    /// Required. The ID of the customer.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. Inputs used to suggest a Smart campaign ad.
    /// Required fields: final_url, language_code, keyword_themes.
    /// Optional but recommended fields to improve the quality of the suggestion:
    /// business_setting and geo_target.
    #[prost(message, optional, tag = "2")]
    pub suggestion_info: ::core::option::Option<SmartCampaignSuggestionInfo>,
}
/// Response message for
/// \[SmartCampaignSuggestService.SuggestSmartCampaignAd][google.ads.googleads.v10.services.SmartCampaignSuggestService.SuggestSmartCampaignAd\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestSmartCampaignAdResponse {
    /// Optional. Ad info includes 3 creative headlines and 2 creative descriptions.
    #[prost(message, optional, tag = "1")]
    pub ad_info: ::core::option::Option<super::common::SmartCampaignAdInfo>,
}
/// Request message for
/// \[SmartCampaignSuggestService.SuggestKeywordThemes][google.ads.googleads.v10.services.SmartCampaignSuggestService.SuggestKeywordThemes\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestKeywordThemesRequest {
    /// Required. The ID of the customer.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. Information to get keyword theme suggestions.
    /// Required fields:
    /// * suggestion_info.final_url
    /// * suggestion_info.language_code
    /// * suggestion_info.geo_target
    ///
    /// Recommended fields:
    /// * suggestion_info.business_setting
    #[prost(message, optional, tag = "2")]
    pub suggestion_info: ::core::option::Option<SmartCampaignSuggestionInfo>,
}
/// Response message for
/// \[SmartCampaignSuggestService.SuggestKeywordThemes][google.ads.googleads.v10.services.SmartCampaignSuggestService.SuggestKeywordThemes\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestKeywordThemesResponse {
    /// Smart campaign keyword theme suggestions.
    #[prost(message, repeated, tag = "1")]
    pub keyword_themes: ::prost::alloc::vec::Vec<super::resources::KeywordThemeConstant>,
}
#[doc = r" Generated client implementations."]
pub mod smart_campaign_suggest_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to get suggestions for Smart Campaigns."]
    #[derive(Debug, Clone)]
    pub struct SmartCampaignSuggestServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SmartCampaignSuggestServiceClient<T>
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
        ) -> SmartCampaignSuggestServiceClient<InterceptedService<T, F>>
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
            SmartCampaignSuggestServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns BudgetOption suggestions."]
        pub async fn suggest_smart_campaign_budget_options(
            &mut self,
            request: impl tonic::IntoRequest<super::SuggestSmartCampaignBudgetOptionsRequest>,
        ) -> Result<tonic::Response<super::SuggestSmartCampaignBudgetOptionsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.SmartCampaignSuggestService/SuggestSmartCampaignBudgetOptions") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Suggests a Smart campaign ad compatible with the Ad family of resources,"]
        #[doc = " based on data points such as targeting and the business to advertise."]
        pub async fn suggest_smart_campaign_ad(
            &mut self,
            request: impl tonic::IntoRequest<super::SuggestSmartCampaignAdRequest>,
        ) -> Result<tonic::Response<super::SuggestSmartCampaignAdResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.SmartCampaignSuggestService/SuggestSmartCampaignAd") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Suggests keyword themes to advertise on."]
        pub async fn suggest_keyword_themes(
            &mut self,
            request: impl tonic::IntoRequest<super::SuggestKeywordThemesRequest>,
        ) -> Result<tonic::Response<super::SuggestKeywordThemesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.SmartCampaignSuggestService/SuggestKeywordThemes") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[ThirdPartyAppAnalyticsLinkService.RegenerateShareableLinkId][google.ads.googleads.v10.services.ThirdPartyAppAnalyticsLinkService.RegenerateShareableLinkId\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegenerateShareableLinkIdRequest {
    /// Resource name of the third party app analytics link.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// Response message for
/// \[ThirdPartyAppAnalyticsLinkService.RegenerateShareableLinkId][google.ads.googleads.v10.services.ThirdPartyAppAnalyticsLinkService.RegenerateShareableLinkId\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegenerateShareableLinkIdResponse {}
#[doc = r" Generated client implementations."]
pub mod third_party_app_analytics_link_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " This service allows management of links between Google Ads and third party"]
    #[doc = " app analytics."]
    #[derive(Debug, Clone)]
    pub struct ThirdPartyAppAnalyticsLinkServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ThirdPartyAppAnalyticsLinkServiceClient<T>
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
        ) -> ThirdPartyAppAnalyticsLinkServiceClient<InterceptedService<T, F>>
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
            ThirdPartyAppAnalyticsLinkServiceClient::new(InterceptedService::new(
                inner,
                interceptor,
            ))
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
        #[doc = " Regenerate ThirdPartyAppAnalyticsLink.shareable_link_id that should be"]
        #[doc = " provided to the third party when setting up app analytics."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        pub async fn regenerate_shareable_link_id(
            &mut self,
            request: impl tonic::IntoRequest<super::RegenerateShareableLinkIdRequest>,
        ) -> Result<tonic::Response<super::RegenerateShareableLinkIdResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.ads.googleads.v10.services.ThirdPartyAppAnalyticsLinkService/RegenerateShareableLinkId") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[UserDataService.UploadUserData][google.ads.googleads.v10.services.UserDataService.UploadUserData\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadUserDataRequest {
    /// Required. The ID of the customer for which to update the user data.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The list of operations to be done.
    #[prost(message, repeated, tag = "3")]
    pub operations: ::prost::alloc::vec::Vec<UserDataOperation>,
    /// Metadata of the request.
    #[prost(oneof = "upload_user_data_request::Metadata", tags = "2")]
    pub metadata: ::core::option::Option<upload_user_data_request::Metadata>,
}
/// Nested message and enum types in `UploadUserDataRequest`.
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
    #[prost(oneof = "user_data_operation::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<user_data_operation::Operation>,
}
/// Nested message and enum types in `UserDataOperation`.
pub mod user_data_operation {
    /// Operation to be made for the UploadUserDataRequest.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// The list of user data to be appended to the user list.
        #[prost(message, tag = "1")]
        Create(super::super::common::UserData),
        /// The list of user data to be removed from the user list.
        #[prost(message, tag = "2")]
        Remove(super::super::common::UserData),
    }
}
/// Response message for \[UserDataService.UploadUserData][google.ads.googleads.v10.services.UserDataService.UploadUserData\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadUserDataResponse {
    /// The date time at which the request was received by API, formatted as
    /// "yyyy-mm-dd hh:mm:ss+|-hh:mm", e.g. "2019-01-01 12:32:45-08:00".
    #[prost(string, optional, tag = "3")]
    pub upload_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// Number of upload data operations received by API.
    #[prost(int32, optional, tag = "4")]
    pub received_operations_count: ::core::option::Option<i32>,
}
#[doc = r" Generated client implementations."]
pub mod user_data_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to manage user data uploads."]
    #[derive(Debug, Clone)]
    pub struct UserDataServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> UserDataServiceClient<T>
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
        ) -> UserDataServiceClient<InterceptedService<T, F>>
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
            UserDataServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Uploads the given user data."]
        #[doc = ""]
        #[doc = " List of thrown errors:"]
        #[doc = "   [AuthenticationError]()"]
        #[doc = "   [AuthorizationError]()"]
        #[doc = "   [CollectionSizeError]()"]
        #[doc = "   [FieldError]()"]
        #[doc = "   [HeaderError]()"]
        #[doc = "   [InternalError]()"]
        #[doc = "   [MutateError]()"]
        #[doc = "   [OfflineUserDataJobError]()"]
        #[doc = "   [QuotaError]()"]
        #[doc = "   [RequestError]()"]
        #[doc = "   [UserDataError]()"]
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
                "/google.ads.googleads.v10.services.UserDataService/UploadUserData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
