/// Represents a view of BiddingStrategies owned by and shared with the customer.
///
/// In contrast to BiddingStrategy, this resource includes strategies owned by
/// managers of the customer and shared with this customer - in addition to
/// strategies owned by this customer. This resource does not provide metrics and
/// only exposes a limited subset of the BiddingStrategy attributes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessibleBiddingStrategy {
    /// Output only. The resource name of the accessible bidding strategy.
    /// AccessibleBiddingStrategy resource names have the form:
    ///
    /// `customers/{customer_id}/accessibleBiddingStrategies/{bidding_strategy_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the bidding strategy.
    #[prost(int64, tag = "2")]
    pub id: i64,
    /// Output only. The name of the bidding strategy.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The type of the bidding strategy.
    #[prost(
        enumeration = "super::enums::bidding_strategy_type_enum::BiddingStrategyType",
        tag = "4"
    )]
    pub r#type: i32,
    /// Output only. The ID of the Customer which owns the bidding strategy.
    #[prost(int64, tag = "5")]
    pub owner_customer_id: i64,
    /// Output only. descriptive_name of the Customer which owns the bidding strategy.
    #[prost(string, tag = "6")]
    pub owner_descriptive_name: ::prost::alloc::string::String,
    /// The bidding scheme.
    ///
    /// Only one can be set.
    #[prost(
        oneof = "accessible_bidding_strategy::Scheme",
        tags = "7, 8, 9, 10, 11, 12"
    )]
    pub scheme: ::core::option::Option<accessible_bidding_strategy::Scheme>,
}
/// Nested message and enum types in `AccessibleBiddingStrategy`.
pub mod accessible_bidding_strategy {
    /// An automated bidding strategy to help get the most conversion value for
    /// your campaigns while spending your budget.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MaximizeConversionValue {
        /// Output only. The target return on ad spend (ROAS) option. If set, the bid strategy
        /// will maximize revenue while averaging the target return on ad spend. If
        /// the target ROAS is high, the bid strategy may not be able to spend the
        /// full budget. If the target ROAS is not set, the bid strategy will aim to
        /// achieve the highest possible ROAS for the budget.
        #[prost(double, tag = "1")]
        pub target_roas: f64,
    }
    /// An automated bidding strategy to help get the most conversions for your
    /// campaigns while spending your budget.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MaximizeConversions {
        /// Output only. The target cost per acquisition (CPA) option. This is the average amount
        /// that you would like to spend per acquisition.
        #[prost(int64, tag = "1")]
        pub target_cpa: i64,
    }
    /// An automated bid strategy that sets bids to help get as many conversions as
    /// possible at the target cost-per-acquisition (CPA) you set.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TargetCpa {
        /// Output only. Average CPA target.
        /// This target should be greater than or equal to minimum billable unit
        /// based on the currency for the account.
        #[prost(int64, optional, tag = "1")]
        pub target_cpa_micros: ::core::option::Option<i64>,
    }
    /// An automated bidding strategy that sets bids so that a certain percentage
    /// of search ads are shown at the top of the first page (or other targeted
    /// location).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TargetImpressionShare {
        /// Output only. The targeted location on the search results page.
        #[prost(
            enumeration = "super::super::enums::target_impression_share_location_enum::TargetImpressionShareLocation",
            tag = "1"
        )]
        pub location: i32,
        /// The desired fraction of ads to be shown in the targeted location in
        /// micros. E.g. 1% equals 10,000.
        #[prost(int64, optional, tag = "2")]
        pub location_fraction_micros: ::core::option::Option<i64>,
        /// Output only. The highest CPC bid the automated bidding system is permitted to specify.
        /// This is a required field entered by the advertiser that sets the ceiling
        /// and specified in local micros.
        #[prost(int64, optional, tag = "3")]
        pub cpc_bid_ceiling_micros: ::core::option::Option<i64>,
    }
    /// An automated bidding strategy that helps you maximize revenue while
    /// averaging a specific target return on ad spend (ROAS).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TargetRoas {
        /// Output only. The desired revenue (based on conversion data) per unit of spend.
        #[prost(double, optional, tag = "1")]
        pub target_roas: ::core::option::Option<f64>,
    }
    /// An automated bid strategy that sets your bids to help get as many clicks
    /// as possible within your budget.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TargetSpend {
        /// Output only. The spend target under which to maximize clicks.
        /// A TargetSpend bidder will attempt to spend the smaller of this value
        /// or the natural throttling spend amount.
        /// If not specified, the budget is used as the spend target.
        /// This field is deprecated and should no longer be used. See
        /// <https://ads-developers.googleblog.com/2020/05/reminder-about-sunset-creation-of.html>
        /// for details.
        #[deprecated]
        #[prost(int64, optional, tag = "1")]
        pub target_spend_micros: ::core::option::Option<i64>,
        /// Output only. Maximum bid limit that can be set by the bid strategy.
        /// The limit applies to all keywords managed by the strategy.
        #[prost(int64, optional, tag = "2")]
        pub cpc_bid_ceiling_micros: ::core::option::Option<i64>,
    }
    /// The bidding scheme.
    ///
    /// Only one can be set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Scheme {
        /// Output only. An automated bidding strategy to help get the most conversion value for
        /// your campaigns while spending your budget.
        #[prost(message, tag = "7")]
        MaximizeConversionValue(MaximizeConversionValue),
        /// Output only. An automated bidding strategy to help get the most conversions for your
        /// campaigns while spending your budget.
        #[prost(message, tag = "8")]
        MaximizeConversions(MaximizeConversions),
        /// Output only. A bidding strategy that sets bids to help get as many conversions as
        /// possible at the target cost-per-acquisition (CPA) you set.
        #[prost(message, tag = "9")]
        TargetCpa(TargetCpa),
        /// Output only. A bidding strategy that automatically optimizes towards a desired
        /// percentage of impressions.
        #[prost(message, tag = "10")]
        TargetImpressionShare(TargetImpressionShare),
        /// Output only. A bidding strategy that helps you maximize revenue while averaging a
        /// specific target Return On Ad Spend (ROAS).
        #[prost(message, tag = "11")]
        TargetRoas(TargetRoas),
        /// Output only. A bid strategy that sets your bids to help get as many clicks as
        /// possible within your budget.
        #[prost(message, tag = "12")]
        TargetSpend(TargetSpend),
    }
}
// Proto file describing the AccountBudget resource.

/// An account-level budget. It contains information about the budget itself,
/// as well as the most recently approved changes to the budget and proposed
/// changes that are pending approval. The proposed changes that are pending
/// approval, if any, are found in 'pending_proposal'.  Effective details about
/// the budget are found in fields prefixed 'approved_', 'adjusted_' and those
/// without a prefix.  Since some effective details may differ from what the user
/// had originally requested (e.g. spending limit), these differences are
/// juxtaposed via 'proposed_', 'approved_', and possibly 'adjusted_' fields.
///
/// This resource is mutated using AccountBudgetProposal and cannot be mutated
/// directly. A budget may have at most one pending proposal at any given time.
/// It is read through pending_proposal.
///
/// Once approved, a budget may be subject to adjustments, such as credit
/// adjustments.  Adjustments create differences between the 'approved' and
/// 'adjusted' fields, which would otherwise be identical.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountBudget {
    /// Output only. The resource name of the account-level budget.
    /// AccountBudget resource names have the form:
    ///
    /// `customers/{customer_id}/accountBudgets/{account_budget_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the account-level budget.
    #[prost(int64, optional, tag = "23")]
    pub id: ::core::option::Option<i64>,
    /// Output only. The resource name of the billing setup associated with this account-level
    /// budget.  BillingSetup resource names have the form:
    ///
    /// `customers/{customer_id}/billingSetups/{billing_setup_id}`
    #[prost(string, optional, tag = "24")]
    pub billing_setup: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The status of this account-level budget.
    #[prost(
        enumeration = "super::enums::account_budget_status_enum::AccountBudgetStatus",
        tag = "4"
    )]
    pub status: i32,
    /// Output only. The name of the account-level budget.
    #[prost(string, optional, tag = "25")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The proposed start time of the account-level budget in
    /// yyyy-MM-dd HH:mm:ss format.  If a start time type of NOW was proposed,
    /// this is the time of request.
    #[prost(string, optional, tag = "26")]
    pub proposed_start_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The approved start time of the account-level budget in yyyy-MM-dd HH:mm:ss
    /// format.
    ///
    /// For example, if a new budget is approved after the proposed start time,
    /// the approved start time is the time of approval.
    #[prost(string, optional, tag = "27")]
    pub approved_start_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The total adjustments amount.
    ///
    /// An example of an adjustment is courtesy credits.
    #[prost(int64, tag = "33")]
    pub total_adjustments_micros: i64,
    /// Output only. The value of Ads that have been served, in micros.
    ///
    /// This includes overdelivery costs, in which case a credit might be
    /// automatically applied to the budget (see total_adjustments_micros).
    #[prost(int64, tag = "34")]
    pub amount_served_micros: i64,
    /// Output only. A purchase order number is a value that helps users reference this budget
    /// in their monthly invoices.
    #[prost(string, optional, tag = "35")]
    pub purchase_order_number: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Notes associated with the budget.
    #[prost(string, optional, tag = "36")]
    pub notes: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The pending proposal to modify this budget, if applicable.
    #[prost(message, optional, tag = "22")]
    pub pending_proposal: ::core::option::Option<account_budget::PendingAccountBudgetProposal>,
    /// The proposed end time of the account-level budget.
    #[prost(oneof = "account_budget::ProposedEndTime", tags = "28, 9")]
    pub proposed_end_time: ::core::option::Option<account_budget::ProposedEndTime>,
    /// The approved end time of the account-level budget.
    ///
    /// For example, if a budget's end time is updated and the proposal is approved
    /// after the proposed end time, the approved end time is the time of approval.
    #[prost(oneof = "account_budget::ApprovedEndTime", tags = "29, 11")]
    pub approved_end_time: ::core::option::Option<account_budget::ApprovedEndTime>,
    /// The proposed spending limit.
    #[prost(oneof = "account_budget::ProposedSpendingLimit", tags = "30, 13")]
    pub proposed_spending_limit: ::core::option::Option<account_budget::ProposedSpendingLimit>,
    /// The approved spending limit.
    ///
    /// For example, if the amount already spent by the account exceeds the
    /// proposed spending limit at the time the proposal is approved, the approved
    /// spending limit is set to the amount already spent.
    #[prost(oneof = "account_budget::ApprovedSpendingLimit", tags = "31, 15")]
    pub approved_spending_limit: ::core::option::Option<account_budget::ApprovedSpendingLimit>,
    /// The spending limit after adjustments have been applied.  Adjustments are
    /// stored in total_adjustments_micros.
    ///
    /// This value has the final say on how much the account is allowed to spend.
    #[prost(oneof = "account_budget::AdjustedSpendingLimit", tags = "32, 17")]
    pub adjusted_spending_limit: ::core::option::Option<account_budget::AdjustedSpendingLimit>,
}
/// Nested message and enum types in `AccountBudget`.
pub mod account_budget {
    /// A pending proposal associated with the enclosing account-level budget,
    /// if applicable.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PendingAccountBudgetProposal {
        /// Output only. The resource name of the proposal.
        /// AccountBudgetProposal resource names have the form:
        ///
        /// `customers/{customer_id}/accountBudgetProposals/{account_budget_proposal_id}`
        #[prost(string, optional, tag = "12")]
        pub account_budget_proposal: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. The type of this proposal, e.g. END to end the budget associated
        /// with this proposal.
        #[prost(
            enumeration = "super::super::enums::account_budget_proposal_type_enum::AccountBudgetProposalType",
            tag = "2"
        )]
        pub proposal_type: i32,
        /// Output only. The name to assign to the account-level budget.
        #[prost(string, optional, tag = "13")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. The start time in yyyy-MM-dd HH:mm:ss format.
        #[prost(string, optional, tag = "14")]
        pub start_date_time: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. A purchase order number is a value that helps users reference this budget
        /// in their monthly invoices.
        #[prost(string, optional, tag = "17")]
        pub purchase_order_number: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. Notes associated with this budget.
        #[prost(string, optional, tag = "18")]
        pub notes: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. The time when this account-level budget proposal was created.
        /// Formatted as yyyy-MM-dd HH:mm:ss.
        #[prost(string, optional, tag = "19")]
        pub creation_date_time: ::core::option::Option<::prost::alloc::string::String>,
        /// The end time of the account-level budget.
        #[prost(oneof = "pending_account_budget_proposal::EndTime", tags = "15, 6")]
        pub end_time: ::core::option::Option<pending_account_budget_proposal::EndTime>,
        /// The spending limit.
        #[prost(
            oneof = "pending_account_budget_proposal::SpendingLimit",
            tags = "16, 8"
        )]
        pub spending_limit: ::core::option::Option<pending_account_budget_proposal::SpendingLimit>,
    }
    /// Nested message and enum types in `PendingAccountBudgetProposal`.
    pub mod pending_account_budget_proposal {
        /// The end time of the account-level budget.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum EndTime {
            /// Output only. The end time in yyyy-MM-dd HH:mm:ss format.
            #[prost(string, tag = "15")]
            EndDateTime(::prost::alloc::string::String),
            /// Output only. The end time as a well-defined type, e.g. FOREVER.
            #[prost(
                enumeration = "super::super::super::enums::time_type_enum::TimeType",
                tag = "6"
            )]
            EndTimeType(i32),
        }
        /// The spending limit.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum SpendingLimit {
            /// Output only. The spending limit in micros.  One million is equivalent to
            /// one unit.
            #[prost(int64, tag = "16")]
            SpendingLimitMicros(i64),
            /// Output only. The spending limit as a well-defined type, e.g. INFINITE.
            #[prost(
                enumeration = "super::super::super::enums::spending_limit_type_enum::SpendingLimitType",
                tag = "8"
            )]
            SpendingLimitType(i32),
        }
    }
    /// The proposed end time of the account-level budget.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ProposedEndTime {
        /// Output only. The proposed end time in yyyy-MM-dd HH:mm:ss format.
        #[prost(string, tag = "28")]
        ProposedEndDateTime(::prost::alloc::string::String),
        /// Output only. The proposed end time as a well-defined type, e.g. FOREVER.
        #[prost(
            enumeration = "super::super::enums::time_type_enum::TimeType",
            tag = "9"
        )]
        ProposedEndTimeType(i32),
    }
    /// The approved end time of the account-level budget.
    ///
    /// For example, if a budget's end time is updated and the proposal is approved
    /// after the proposed end time, the approved end time is the time of approval.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ApprovedEndTime {
        /// Output only. The approved end time in yyyy-MM-dd HH:mm:ss format.
        #[prost(string, tag = "29")]
        ApprovedEndDateTime(::prost::alloc::string::String),
        /// Output only. The approved end time as a well-defined type, e.g. FOREVER.
        #[prost(
            enumeration = "super::super::enums::time_type_enum::TimeType",
            tag = "11"
        )]
        ApprovedEndTimeType(i32),
    }
    /// The proposed spending limit.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ProposedSpendingLimit {
        /// Output only. The proposed spending limit in micros.  One million is equivalent to
        /// one unit.
        #[prost(int64, tag = "30")]
        ProposedSpendingLimitMicros(i64),
        /// Output only. The proposed spending limit as a well-defined type, e.g. INFINITE.
        #[prost(
            enumeration = "super::super::enums::spending_limit_type_enum::SpendingLimitType",
            tag = "13"
        )]
        ProposedSpendingLimitType(i32),
    }
    /// The approved spending limit.
    ///
    /// For example, if the amount already spent by the account exceeds the
    /// proposed spending limit at the time the proposal is approved, the approved
    /// spending limit is set to the amount already spent.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ApprovedSpendingLimit {
        /// Output only. The approved spending limit in micros.  One million is equivalent to
        /// one unit.  This will only be populated if the proposed spending limit
        /// is finite, and will always be greater than or equal to the
        /// proposed spending limit.
        #[prost(int64, tag = "31")]
        ApprovedSpendingLimitMicros(i64),
        /// Output only. The approved spending limit as a well-defined type, e.g. INFINITE.  This
        /// will only be populated if the approved spending limit is INFINITE.
        #[prost(
            enumeration = "super::super::enums::spending_limit_type_enum::SpendingLimitType",
            tag = "15"
        )]
        ApprovedSpendingLimitType(i32),
    }
    /// The spending limit after adjustments have been applied.  Adjustments are
    /// stored in total_adjustments_micros.
    ///
    /// This value has the final say on how much the account is allowed to spend.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AdjustedSpendingLimit {
        /// Output only. The adjusted spending limit in micros.  One million is equivalent to
        /// one unit.
        ///
        /// If the approved spending limit is finite, the adjusted
        /// spending limit may vary depending on the types of adjustments applied
        /// to this budget, if applicable.
        ///
        /// The different kinds of adjustments are described here:
        /// <https://support.google.com/google-ads/answer/1704323>
        ///
        /// For example, a debit adjustment reduces how much the account is
        /// allowed to spend.
        #[prost(int64, tag = "32")]
        AdjustedSpendingLimitMicros(i64),
        /// Output only. The adjusted spending limit as a well-defined type, e.g. INFINITE.
        /// This will only be populated if the adjusted spending limit is INFINITE,
        /// which is guaranteed to be true if the approved spending limit is
        /// INFINITE.
        #[prost(
            enumeration = "super::super::enums::spending_limit_type_enum::SpendingLimitType",
            tag = "17"
        )]
        AdjustedSpendingLimitType(i32),
    }
}
// Proto file describing the AccountBudgetProposal resource.

/// An account-level budget proposal.
///
/// All fields prefixed with 'proposed' may not necessarily be applied directly.
/// For example, proposed spending limits may be adjusted before their
/// application.  This is true if the 'proposed' field has an 'approved'
/// counterpart, e.g. spending limits.
///
/// Please note that the proposal type (proposal_type) changes which fields are
/// required and which must remain empty.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountBudgetProposal {
    /// Immutable. The resource name of the proposal.
    /// AccountBudgetProposal resource names have the form:
    ///
    /// `customers/{customer_id}/accountBudgetProposals/{account_budget_proposal_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the proposal.
    #[prost(int64, optional, tag = "25")]
    pub id: ::core::option::Option<i64>,
    /// Immutable. The resource name of the billing setup associated with this proposal.
    #[prost(string, optional, tag = "26")]
    pub billing_setup: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The resource name of the account-level budget associated with this
    /// proposal.
    #[prost(string, optional, tag = "27")]
    pub account_budget: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The type of this proposal, e.g. END to end the budget associated with this
    /// proposal.
    #[prost(
        enumeration = "super::enums::account_budget_proposal_type_enum::AccountBudgetProposalType",
        tag = "4"
    )]
    pub proposal_type: i32,
    /// Output only. The status of this proposal.
    /// When a new proposal is created, the status defaults to PENDING.
    #[prost(
        enumeration = "super::enums::account_budget_proposal_status_enum::AccountBudgetProposalStatus",
        tag = "15"
    )]
    pub status: i32,
    /// Immutable. The name to assign to the account-level budget.
    #[prost(string, optional, tag = "28")]
    pub proposed_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The approved start date time in yyyy-mm-dd hh:mm:ss format.
    #[prost(string, optional, tag = "30")]
    pub approved_start_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. A purchase order number is a value that enables the user to help them
    /// reference this budget in their monthly invoices.
    #[prost(string, optional, tag = "35")]
    pub proposed_purchase_order_number: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. Notes associated with this budget.
    #[prost(string, optional, tag = "36")]
    pub proposed_notes: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The date time when this account-level budget proposal was created, which is
    /// not the same as its approval date time, if applicable.
    #[prost(string, optional, tag = "37")]
    pub creation_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The date time when this account-level budget was approved, if applicable.
    #[prost(string, optional, tag = "38")]
    pub approval_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// The proposed start date time of the account-level budget, which cannot be
    /// in the past.
    #[prost(oneof = "account_budget_proposal::ProposedStartTime", tags = "29, 7")]
    pub proposed_start_time: ::core::option::Option<account_budget_proposal::ProposedStartTime>,
    /// The proposed end date time of the account-level budget, which cannot be in
    /// the past.
    #[prost(oneof = "account_budget_proposal::ProposedEndTime", tags = "31, 9")]
    pub proposed_end_time: ::core::option::Option<account_budget_proposal::ProposedEndTime>,
    /// The approved end date time of the account-level budget.
    #[prost(oneof = "account_budget_proposal::ApprovedEndTime", tags = "32, 22")]
    pub approved_end_time: ::core::option::Option<account_budget_proposal::ApprovedEndTime>,
    /// The proposed spending limit.
    #[prost(
        oneof = "account_budget_proposal::ProposedSpendingLimit",
        tags = "33, 11"
    )]
    pub proposed_spending_limit:
        ::core::option::Option<account_budget_proposal::ProposedSpendingLimit>,
    /// The approved spending limit.
    #[prost(
        oneof = "account_budget_proposal::ApprovedSpendingLimit",
        tags = "34, 24"
    )]
    pub approved_spending_limit:
        ::core::option::Option<account_budget_proposal::ApprovedSpendingLimit>,
}
/// Nested message and enum types in `AccountBudgetProposal`.
pub mod account_budget_proposal {
    /// The proposed start date time of the account-level budget, which cannot be
    /// in the past.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ProposedStartTime {
        /// Immutable. The proposed start date time in yyyy-mm-dd hh:mm:ss format.
        #[prost(string, tag = "29")]
        ProposedStartDateTime(::prost::alloc::string::String),
        /// Immutable. The proposed start date time as a well-defined type, e.g. NOW.
        #[prost(
            enumeration = "super::super::enums::time_type_enum::TimeType",
            tag = "7"
        )]
        ProposedStartTimeType(i32),
    }
    /// The proposed end date time of the account-level budget, which cannot be in
    /// the past.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ProposedEndTime {
        /// Immutable. The proposed end date time in yyyy-mm-dd hh:mm:ss format.
        #[prost(string, tag = "31")]
        ProposedEndDateTime(::prost::alloc::string::String),
        /// Immutable. The proposed end date time as a well-defined type, e.g. FOREVER.
        #[prost(
            enumeration = "super::super::enums::time_type_enum::TimeType",
            tag = "9"
        )]
        ProposedEndTimeType(i32),
    }
    /// The approved end date time of the account-level budget.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ApprovedEndTime {
        /// Output only. The approved end date time in yyyy-mm-dd hh:mm:ss format.
        #[prost(string, tag = "32")]
        ApprovedEndDateTime(::prost::alloc::string::String),
        /// Output only. The approved end date time as a well-defined type, e.g. FOREVER.
        #[prost(
            enumeration = "super::super::enums::time_type_enum::TimeType",
            tag = "22"
        )]
        ApprovedEndTimeType(i32),
    }
    /// The proposed spending limit.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ProposedSpendingLimit {
        /// Immutable. The proposed spending limit in micros.  One million is equivalent to
        /// one unit.
        #[prost(int64, tag = "33")]
        ProposedSpendingLimitMicros(i64),
        /// Immutable. The proposed spending limit as a well-defined type, e.g. INFINITE.
        #[prost(
            enumeration = "super::super::enums::spending_limit_type_enum::SpendingLimitType",
            tag = "11"
        )]
        ProposedSpendingLimitType(i32),
    }
    /// The approved spending limit.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ApprovedSpendingLimit {
        /// Output only. The approved spending limit in micros.  One million is equivalent to
        /// one unit.
        #[prost(int64, tag = "34")]
        ApprovedSpendingLimitMicros(i64),
        /// Output only. The approved spending limit as a well-defined type, e.g. INFINITE.
        #[prost(
            enumeration = "super::super::enums::spending_limit_type_enum::SpendingLimitType",
            tag = "24"
        )]
        ApprovedSpendingLimitType(i32),
    }
}
/// Represents the data sharing connection between a Google Ads account and
/// another account
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLink {
    /// Immutable. Resource name of the account link.
    /// AccountLink resource names have the form:
    /// `customers/{customer_id}/accountLinks/{account_link_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the link.
    /// This field is read only.
    #[prost(int64, optional, tag = "8")]
    pub account_link_id: ::core::option::Option<i64>,
    /// The status of the link.
    #[prost(
        enumeration = "super::enums::account_link_status_enum::AccountLinkStatus",
        tag = "3"
    )]
    pub status: i32,
    /// Output only. The type of the linked account.
    #[prost(
        enumeration = "super::enums::linked_account_type_enum::LinkedAccountType",
        tag = "4"
    )]
    pub r#type: i32,
    /// An account linked to this Google Ads account.
    #[prost(oneof = "account_link::LinkedAccount", tags = "5, 6, 7, 9")]
    pub linked_account: ::core::option::Option<account_link::LinkedAccount>,
}
/// Nested message and enum types in `AccountLink`.
pub mod account_link {
    /// An account linked to this Google Ads account.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LinkedAccount {
        /// Immutable. A third party app analytics link.
        #[prost(message, tag = "5")]
        ThirdPartyAppAnalytics(super::ThirdPartyAppAnalyticsLinkIdentifier),
        /// Output only. Data partner link.
        #[prost(message, tag = "6")]
        DataPartner(super::DataPartnerLinkIdentifier),
        /// Output only. Google Ads link.
        #[prost(message, tag = "7")]
        GoogleAds(super::GoogleAdsLinkIdentifier),
        /// Output only. Hotel link
        #[prost(message, tag = "9")]
        HotelCenter(super::HotelCenterLinkIdentifier),
    }
}
/// The identifiers of a Third Party App Analytics Link.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThirdPartyAppAnalyticsLinkIdentifier {
    /// Immutable. The ID of the app analytics provider.
    /// This field should not be empty when creating a new third
    /// party app analytics link. It is unable to be modified after the creation of
    /// the link.
    #[prost(int64, optional, tag = "4")]
    pub app_analytics_provider_id: ::core::option::Option<i64>,
    /// Immutable. A string that uniquely identifies a mobile application from which the data
    /// was collected to the Google Ads API. For iOS, the ID string is the 9 digit
    /// string that appears at the end of an App Store URL (e.g., "422689480" for
    /// "Gmail" whose App Store link is
    /// <https://apps.apple.com/us/app/gmail-email-by-google/id422689480>). For
    /// Android, the ID string is the application's package name (e.g.,
    /// "com.google.android.gm" for "Gmail" given Google Play link
    /// <https://play.google.com/store/apps/details?id=com.google.android.gm>)
    /// This field should not be empty when creating a new third
    /// party app analytics link. It is unable to be modified after the creation of
    /// the link.
    #[prost(string, optional, tag = "5")]
    pub app_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The vendor of the app.
    /// This field should not be empty when creating a new third
    /// party app analytics link. It is unable to be modified after the creation of
    /// the link.
    #[prost(
        enumeration = "super::enums::mobile_app_vendor_enum::MobileAppVendor",
        tag = "3"
    )]
    pub app_vendor: i32,
}
/// The identifier for Data Partner account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataPartnerLinkIdentifier {
    /// Immutable. The customer ID of the Data partner account.
    /// This field is required and should not be empty when creating a new
    /// data partner link. It is unable to be modified after the creation of
    /// the link.
    #[prost(int64, optional, tag = "1")]
    pub data_partner_id: ::core::option::Option<i64>,
}
/// The identifier for Hotel account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelCenterLinkIdentifier {
    /// Output only. The hotel center id of the hotel account.
    #[prost(int64, tag = "1")]
    pub hotel_center_id: i64,
}
/// The identifier for Google Ads account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleAdsLinkIdentifier {
    /// Immutable. The resource name of the Google Ads account.
    /// This field is required and should not be empty when creating a new
    /// Google Ads link. It is unable to be modified after the creation of
    /// the link.
    #[prost(string, optional, tag = "3")]
    pub customer: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the ad type.

/// An ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ad {
    /// Immutable. The resource name of the ad.
    /// Ad resource names have the form:
    ///
    /// `customers/{customer_id}/ads/{ad_id}`
    #[prost(string, tag = "37")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the ad.
    #[prost(int64, optional, tag = "40")]
    pub id: ::core::option::Option<i64>,
    /// The list of possible final URLs after all cross-domain redirects for the
    /// ad.
    #[prost(string, repeated, tag = "41")]
    pub final_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of final app URLs that will be used on mobile if the user has the
    /// specific app installed.
    #[prost(message, repeated, tag = "35")]
    pub final_app_urls: ::prost::alloc::vec::Vec<super::common::FinalAppUrl>,
    /// The list of possible final mobile URLs after all cross-domain redirects
    /// for the ad.
    #[prost(string, repeated, tag = "42")]
    pub final_mobile_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The URL template for constructing a tracking URL.
    #[prost(string, optional, tag = "43")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
    /// The suffix to use when constructing a final URL.
    #[prost(string, optional, tag = "44")]
    pub final_url_suffix: ::core::option::Option<::prost::alloc::string::String>,
    /// The list of mappings that can be used to substitute custom parameter tags
    /// in a `tracking_url_template`, `final_urls`, or `mobile_final_urls`.
    /// For mutates, please use url custom parameter operations.
    #[prost(message, repeated, tag = "10")]
    pub url_custom_parameters: ::prost::alloc::vec::Vec<super::common::CustomParameter>,
    /// The URL that appears in the ad description for some ad formats.
    #[prost(string, optional, tag = "45")]
    pub display_url: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The type of ad.
    #[prost(enumeration = "super::enums::ad_type_enum::AdType", tag = "5")]
    pub r#type: i32,
    /// Output only. Indicates if this ad was automatically added by Google Ads and not by a
    /// user. For example, this could happen when ads are automatically created as
    /// suggestions for new ads based on knowledge of how existing ads are
    /// performing.
    #[prost(bool, optional, tag = "46")]
    pub added_by_google_ads: ::core::option::Option<bool>,
    /// The device preference for the ad. You can only specify a preference for
    /// mobile devices. When this preference is set the ad will be preferred over
    /// other ads when being displayed on a mobile device. The ad can still be
    /// displayed on other device types, e.g. if no other ads are available.
    /// If unspecified (no device preference), all devices are targeted.
    /// This is only supported by some ad types.
    #[prost(enumeration = "super::enums::device_enum::Device", tag = "20")]
    pub device_preference: i32,
    /// Additional URLs for the ad that are tagged with a unique identifier that
    /// can be referenced from other fields in the ad.
    #[prost(message, repeated, tag = "26")]
    pub url_collections: ::prost::alloc::vec::Vec<super::common::UrlCollection>,
    /// Immutable. The name of the ad. This is only used to be able to identify the ad. It
    /// does not need to be unique and does not affect the served ad. The name
    /// field is currently only supported for DisplayUploadAd, ImageAd,
    /// ShoppingComparisonListingAd and VideoAd.
    #[prost(string, optional, tag = "47")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. If this ad is system managed, then this field will indicate the source.
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::system_managed_resource_source_enum::SystemManagedResourceSource",
        tag = "27"
    )]
    pub system_managed_resource_source: i32,
    /// Details pertinent to the ad type. Exactly one value must be set.
    #[prost(
        oneof = "ad::AdData",
        tags = "6, 7, 49, 14, 15, 17, 18, 21, 22, 24, 39, 25, 28, 29, 30, 31, 32, 33, 34, 36, 48, 50, 51, 52"
    )]
    pub ad_data: ::core::option::Option<ad::AdData>,
}
/// Nested message and enum types in `Ad`.
pub mod ad {
    /// Details pertinent to the ad type. Exactly one value must be set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AdData {
        /// Immutable. Details pertaining to a text ad.
        #[prost(message, tag = "6")]
        TextAd(super::super::common::TextAdInfo),
        /// Details pertaining to an expanded text ad.
        #[prost(message, tag = "7")]
        ExpandedTextAd(super::super::common::ExpandedTextAdInfo),
        /// Details pertaining to a call ad.
        #[prost(message, tag = "49")]
        CallAd(super::super::common::CallAdInfo),
        /// Immutable. Details pertaining to an Expanded Dynamic Search Ad.
        /// This type of ad has its headline, final URLs, and display URL
        /// auto-generated at serving time according to domain name specific
        /// information provided by `dynamic_search_ads_setting` linked at the
        /// campaign level.
        #[prost(message, tag = "14")]
        ExpandedDynamicSearchAd(super::super::common::ExpandedDynamicSearchAdInfo),
        /// Details pertaining to a hotel ad.
        #[prost(message, tag = "15")]
        HotelAd(super::super::common::HotelAdInfo),
        /// Details pertaining to a Smart Shopping ad.
        #[prost(message, tag = "17")]
        ShoppingSmartAd(super::super::common::ShoppingSmartAdInfo),
        /// Details pertaining to a Shopping product ad.
        #[prost(message, tag = "18")]
        ShoppingProductAd(super::super::common::ShoppingProductAdInfo),
        /// Immutable. Details pertaining to a Gmail ad.
        #[prost(message, tag = "21")]
        GmailAd(super::super::common::GmailAdInfo),
        /// Immutable. Details pertaining to an Image ad.
        #[prost(message, tag = "22")]
        ImageAd(super::super::common::ImageAdInfo),
        /// Details pertaining to a Video ad.
        #[prost(message, tag = "24")]
        VideoAd(super::super::common::VideoAdInfo),
        /// Details pertaining to a Video responsive ad.
        #[prost(message, tag = "39")]
        VideoResponsiveAd(super::super::common::VideoResponsiveAdInfo),
        /// Details pertaining to a responsive search ad.
        #[prost(message, tag = "25")]
        ResponsiveSearchAd(super::super::common::ResponsiveSearchAdInfo),
        /// Details pertaining to a legacy responsive display ad.
        #[prost(message, tag = "28")]
        LegacyResponsiveDisplayAd(super::super::common::LegacyResponsiveDisplayAdInfo),
        /// Details pertaining to an app ad.
        #[prost(message, tag = "29")]
        AppAd(super::super::common::AppAdInfo),
        /// Immutable. Details pertaining to a legacy app install ad.
        #[prost(message, tag = "30")]
        LegacyAppInstallAd(super::super::common::LegacyAppInstallAdInfo),
        /// Details pertaining to a responsive display ad.
        #[prost(message, tag = "31")]
        ResponsiveDisplayAd(super::super::common::ResponsiveDisplayAdInfo),
        /// Details pertaining to a local ad.
        #[prost(message, tag = "32")]
        LocalAd(super::super::common::LocalAdInfo),
        /// Details pertaining to a display upload ad.
        #[prost(message, tag = "33")]
        DisplayUploadAd(super::super::common::DisplayUploadAdInfo),
        /// Details pertaining to an app engagement ad.
        #[prost(message, tag = "34")]
        AppEngagementAd(super::super::common::AppEngagementAdInfo),
        /// Details pertaining to a Shopping Comparison Listing ad.
        #[prost(message, tag = "36")]
        ShoppingComparisonListingAd(super::super::common::ShoppingComparisonListingAdInfo),
        /// Details pertaining to a Smart campaign ad.
        #[prost(message, tag = "48")]
        SmartCampaignAd(super::super::common::SmartCampaignAdInfo),
        /// Details pertaining to an app pre-registration ad.
        #[prost(message, tag = "50")]
        AppPreRegistrationAd(super::super::common::AppPreRegistrationAdInfo),
        /// Details pertaining to a discovery multi asset ad.
        #[prost(message, tag = "51")]
        DiscoveryMultiAssetAd(super::super::common::DiscoveryMultiAssetAdInfo),
        /// Details pertaining to a discovery carousel ad.
        #[prost(message, tag = "52")]
        DiscoveryCarouselAd(super::super::common::DiscoveryCarouselAdInfo),
    }
}
// Proto file describing the ad group resource.

/// An ad group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroup {
    /// Immutable. The resource name of the ad group.
    /// Ad group resource names have the form:
    ///
    /// `customers/{customer_id}/adGroups/{ad_group_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the ad group.
    #[prost(int64, optional, tag = "34")]
    pub id: ::core::option::Option<i64>,
    /// The name of the ad group.
    ///
    /// This field is required and should not be empty when creating new ad
    /// groups.
    ///
    /// It must contain fewer than 255 UTF-8 full-width characters.
    ///
    /// It must not contain any null (code point 0x0), NL line feed
    /// (code point 0xA) or carriage return (code point 0xD) characters.
    #[prost(string, optional, tag = "35")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The status of the ad group.
    #[prost(
        enumeration = "super::enums::ad_group_status_enum::AdGroupStatus",
        tag = "5"
    )]
    pub status: i32,
    /// Immutable. The type of the ad group.
    #[prost(
        enumeration = "super::enums::ad_group_type_enum::AdGroupType",
        tag = "12"
    )]
    pub r#type: i32,
    /// The ad rotation mode of the ad group.
    #[prost(
        enumeration = "super::enums::ad_group_ad_rotation_mode_enum::AdGroupAdRotationMode",
        tag = "22"
    )]
    pub ad_rotation_mode: i32,
    /// Output only. For draft or experiment ad groups, this field is the resource name of the
    /// base ad group from which this ad group was created. If a draft or
    /// experiment ad group does not have a base ad group, then this field is null.
    ///
    /// For base ad groups, this field equals the ad group resource name.
    ///
    /// This field is read-only.
    #[prost(string, optional, tag = "36")]
    pub base_ad_group: ::core::option::Option<::prost::alloc::string::String>,
    /// The URL template for constructing a tracking URL.
    #[prost(string, optional, tag = "37")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
    /// The list of mappings used to substitute custom parameter tags in a
    /// `tracking_url_template`, `final_urls`, or `mobile_final_urls`.
    #[prost(message, repeated, tag = "6")]
    pub url_custom_parameters: ::prost::alloc::vec::Vec<super::common::CustomParameter>,
    /// Immutable. The campaign to which the ad group belongs.
    #[prost(string, optional, tag = "38")]
    pub campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// The maximum CPC (cost-per-click) bid.
    #[prost(int64, optional, tag = "39")]
    pub cpc_bid_micros: ::core::option::Option<i64>,
    /// Output only. Value will be same as that of the CPC (cost-per-click) bid value when the
    /// bidding strategy is one of manual cpc, enhanced cpc, page one promoted or
    /// target outrank share, otherwise the value will be null.
    #[prost(int64, optional, tag = "57")]
    pub effective_cpc_bid_micros: ::core::option::Option<i64>,
    /// The maximum CPM (cost-per-thousand viewable impressions) bid.
    #[prost(int64, optional, tag = "40")]
    pub cpm_bid_micros: ::core::option::Option<i64>,
    /// The target CPA (cost-per-acquisition). If the ad group's campaign
    /// bidding strategy is TargetCpa or MaximizeConversions (with its target_cpa
    /// field set), then this field overrides the target CPA specified in the
    /// campaign's bidding strategy.
    /// Otherwise, this value is ignored.
    #[prost(int64, optional, tag = "41")]
    pub target_cpa_micros: ::core::option::Option<i64>,
    /// Output only. The CPV (cost-per-view) bid.
    #[prost(int64, optional, tag = "42")]
    pub cpv_bid_micros: ::core::option::Option<i64>,
    /// Average amount in micros that the advertiser is willing to pay for every
    /// thousand times the ad is shown.
    #[prost(int64, optional, tag = "43")]
    pub target_cpm_micros: ::core::option::Option<i64>,
    /// The target ROAS (return-on-ad-spend) override. If the ad group's campaign
    /// bidding strategy is TargetRoas or MaximizeConversionValue (with its
    /// target_roas field set), then this field overrides the target ROAS specified
    /// in the campaign's bidding strategy.
    /// Otherwise, this value is ignored.
    #[prost(double, optional, tag = "44")]
    pub target_roas: ::core::option::Option<f64>,
    /// The percent cpc bid amount, expressed as a fraction of the advertised price
    /// for some good or service. The valid range for the fraction is [0,1) and the
    /// value stored here is 1,000,000 * \[fraction\].
    #[prost(int64, optional, tag = "45")]
    pub percent_cpc_bid_micros: ::core::option::Option<i64>,
    /// Settings for the Display Campaign Optimizer, initially termed "Explorer".
    #[prost(message, optional, tag = "21")]
    pub explorer_auto_optimizer_setting:
        ::core::option::Option<super::common::ExplorerAutoOptimizerSetting>,
    /// Allows advertisers to specify a targeting dimension on which to place
    /// absolute bids. This is only applicable for campaigns that target only the
    /// display network and not search.
    #[prost(
        enumeration = "super::enums::targeting_dimension_enum::TargetingDimension",
        tag = "23"
    )]
    pub display_custom_bid_dimension: i32,
    /// URL template for appending params to Final URL.
    #[prost(string, optional, tag = "46")]
    pub final_url_suffix: ::core::option::Option<::prost::alloc::string::String>,
    /// Setting for targeting related features.
    #[prost(message, optional, tag = "25")]
    pub targeting_setting: ::core::option::Option<super::common::TargetingSetting>,
    /// Immutable. Setting for audience related features.
    #[prost(message, optional, tag = "56")]
    pub audience_setting: ::core::option::Option<ad_group::AudienceSetting>,
    /// Output only. The effective target CPA (cost-per-acquisition).
    /// This field is read-only.
    #[prost(int64, optional, tag = "47")]
    pub effective_target_cpa_micros: ::core::option::Option<i64>,
    /// Output only. Source of the effective target CPA.
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::bidding_source_enum::BiddingSource",
        tag = "29"
    )]
    pub effective_target_cpa_source: i32,
    /// Output only. The effective target ROAS (return-on-ad-spend).
    /// This field is read-only.
    #[prost(double, optional, tag = "48")]
    pub effective_target_roas: ::core::option::Option<f64>,
    /// Output only. Source of the effective target ROAS.
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::bidding_source_enum::BiddingSource",
        tag = "32"
    )]
    pub effective_target_roas_source: i32,
    /// Output only. The resource names of labels attached to this ad group.
    #[prost(string, repeated, tag = "49")]
    pub labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The asset field types that should be excluded from this ad group. Asset
    /// links with these field types will not be inherited by this ad group from
    /// the upper levels.
    #[prost(
        enumeration = "super::enums::asset_field_type_enum::AssetFieldType",
        repeated,
        tag = "54"
    )]
    pub excluded_parent_asset_field_types: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `AdGroup`.
pub mod ad_group {
    /// Settings for the audience targeting.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AudienceSetting {
        /// Immutable. If true, this ad group uses an Audience resource for audience targeting.
        /// If false, this ad group may use audience segment criteria instead.
        #[prost(bool, tag = "1")]
        pub use_audience_grouped: bool,
    }
}
// Proto file describing the ad group ad resource.

/// An ad group ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAd {
    /// Immutable. The resource name of the ad.
    /// Ad group ad resource names have the form:
    ///
    /// `customers/{customer_id}/adGroupAds/{ad_group_id}~{ad_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The status of the ad.
    #[prost(
        enumeration = "super::enums::ad_group_ad_status_enum::AdGroupAdStatus",
        tag = "3"
    )]
    pub status: i32,
    /// Immutable. The ad group to which the ad belongs.
    #[prost(string, optional, tag = "9")]
    pub ad_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The ad.
    #[prost(message, optional, tag = "5")]
    pub ad: ::core::option::Option<Ad>,
    /// Output only. Policy information for the ad.
    #[prost(message, optional, tag = "6")]
    pub policy_summary: ::core::option::Option<AdGroupAdPolicySummary>,
    /// Output only. Overall ad strength for this ad group ad.
    #[prost(enumeration = "super::enums::ad_strength_enum::AdStrength", tag = "7")]
    pub ad_strength: i32,
    /// Output only. A list of recommendations to improve the ad strength. For example, a
    /// recommendation could be "Your headlines are a little too similar.
    /// Try adding more distinct headlines.".
    #[prost(string, repeated, tag = "13")]
    pub action_items: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The resource names of labels attached to this ad group ad.
    #[prost(string, repeated, tag = "10")]
    pub labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Contains policy information for an ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAdPolicySummary {
    /// Output only. The list of policy findings for this ad.
    #[prost(message, repeated, tag = "1")]
    pub policy_topic_entries: ::prost::alloc::vec::Vec<super::common::PolicyTopicEntry>,
    /// Output only. Where in the review process this ad is.
    #[prost(
        enumeration = "super::enums::policy_review_status_enum::PolicyReviewStatus",
        tag = "2"
    )]
    pub review_status: i32,
    /// Output only. The overall approval status of this ad, calculated based on the status of
    /// its individual policy topic entries.
    #[prost(
        enumeration = "super::enums::policy_approval_status_enum::PolicyApprovalStatus",
        tag = "3"
    )]
    pub approval_status: i32,
}
// Proto file describing the asset combination view resource.

/// A view on the usage of ad group ad asset combination.
/// Now we only support AdGroupAdAssetCombinationView for Responsive Search Ads,
/// with more ad types planned for the future.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAdAssetCombinationView {
    /// Output only. The resource name of the ad group ad asset combination view. The
    /// combination ID is 128 bits long, where the upper 64 bits are stored in
    /// asset_combination_id_high, and the lower 64 bits are stored in
    /// asset_combination_id_low.
    /// AdGroupAd Asset Combination view resource names have the form:
    /// `customers/{customer_id}/adGroupAdAssetCombinationViews/{AdGroupAd.ad_group_id}~{AdGroupAd.ad.ad_id}~{AssetCombination.asset_combination_id_low}~{AssetCombination.asset_combination_id_high}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. Served assets.
    #[prost(message, repeated, tag = "2")]
    pub served_assets: ::prost::alloc::vec::Vec<super::common::AssetUsage>,
    /// Output only. The status between the asset combination and the latest version of the ad.
    /// If true, the asset combination is linked to the latest version of the ad.
    /// If false, it means the link once existed but has been removed and is no
    /// longer present in the latest version of the ad.
    #[prost(bool, optional, tag = "3")]
    pub enabled: ::core::option::Option<bool>,
}
// Proto file describing the ad group ad asset view resource.

/// A link between an AdGroupAd and an Asset.
/// Currently we only support AdGroupAdAssetView for AppAds.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAdAssetView {
    /// Output only. The resource name of the ad group ad asset view.
    /// Ad group ad asset view resource names have the form (Before V4):
    ///
    /// `customers/{customer_id}/adGroupAdAssets/{AdGroupAdAsset.ad_group_id}~{AdGroupAdAsset.ad.ad_id}~{AdGroupAdAsset.asset_id}~{AdGroupAdAsset.field_type}`
    ///
    /// Ad group ad asset view resource names have the form (Beginning from V4):
    ///
    /// `customers/{customer_id}/adGroupAdAssetViews/{AdGroupAdAsset.ad_group_id}~{AdGroupAdAsset.ad_id}~{AdGroupAdAsset.asset_id}~{AdGroupAdAsset.field_type}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ad group ad to which the asset is linked.
    #[prost(string, optional, tag = "9")]
    pub ad_group_ad: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The asset which is linked to the ad group ad.
    #[prost(string, optional, tag = "10")]
    pub asset: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Role that the asset takes in the ad.
    #[prost(
        enumeration = "super::enums::asset_field_type_enum::AssetFieldType",
        tag = "2"
    )]
    pub field_type: i32,
    /// Output only. The status between the asset and the latest version of the ad. If true, the
    /// asset is linked to the latest version of the ad. If false, it means the
    /// link once existed but has been removed and is no longer present in the
    /// latest version of the ad.
    #[prost(bool, optional, tag = "8")]
    pub enabled: ::core::option::Option<bool>,
    /// Output only. Policy information for the ad group ad asset.
    #[prost(message, optional, tag = "3")]
    pub policy_summary: ::core::option::Option<AdGroupAdAssetPolicySummary>,
    /// Output only. Performance of an asset linkage.
    #[prost(
        enumeration = "super::enums::asset_performance_label_enum::AssetPerformanceLabel",
        tag = "4"
    )]
    pub performance_label: i32,
}
/// Contains policy information for an ad group ad asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAdAssetPolicySummary {
    /// Output only. The list of policy findings for the ad group ad asset.
    #[prost(message, repeated, tag = "1")]
    pub policy_topic_entries: ::prost::alloc::vec::Vec<super::common::PolicyTopicEntry>,
    /// Output only. Where in the review process this ad group ad asset is.
    #[prost(
        enumeration = "super::enums::policy_review_status_enum::PolicyReviewStatus",
        tag = "2"
    )]
    pub review_status: i32,
    /// Output only. The overall approval status of this ad group ad asset, calculated based on
    /// the status of its individual policy topic entries.
    #[prost(
        enumeration = "super::enums::policy_approval_status_enum::PolicyApprovalStatus",
        tag = "3"
    )]
    pub approval_status: i32,
}
// Proto file describing the ad group ad label resource.

/// A relationship between an ad group ad and a label.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAdLabel {
    /// Immutable. The resource name of the ad group ad label.
    /// Ad group ad label resource names have the form:
    /// `customers/{customer_id}/adGroupAdLabels/{ad_group_id}~{ad_id}~{label_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The ad group ad to which the label is attached.
    #[prost(string, optional, tag = "4")]
    pub ad_group_ad: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The label assigned to the ad group ad.
    #[prost(string, optional, tag = "5")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the AdGroupAsset resource.

/// A link between an ad group and an asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAsset {
    /// Immutable. The resource name of the ad group asset.
    /// AdGroupAsset resource names have the form:
    ///
    /// `customers/{customer_id}/adGroupAssets/{ad_group_id}~{asset_id}~{field_type}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Required. Immutable. The ad group to which the asset is linked.
    #[prost(string, tag = "2")]
    pub ad_group: ::prost::alloc::string::String,
    /// Required. Immutable. The asset which is linked to the ad group.
    #[prost(string, tag = "3")]
    pub asset: ::prost::alloc::string::String,
    /// Required. Immutable. Role that the asset takes under the linked ad group.
    #[prost(
        enumeration = "super::enums::asset_field_type_enum::AssetFieldType",
        tag = "4"
    )]
    pub field_type: i32,
    /// Status of the ad group asset.
    #[prost(
        enumeration = "super::enums::asset_link_status_enum::AssetLinkStatus",
        tag = "5"
    )]
    pub status: i32,
}
// Proto file describing the ad group audience view resource.

/// An ad group audience view.
/// Includes performance data from interests and remarketing lists for Display
/// Network and YouTube Network ads, and remarketing lists for search ads (RLSA),
/// aggregated at the audience level.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAudienceView {
    /// Output only. The resource name of the ad group audience view.
    /// Ad group audience view resource names have the form:
    ///
    /// `customers/{customer_id}/adGroupAudienceViews/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
// Proto file describing the ad group bid modifier resource.

/// Represents an ad group bid modifier.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupBidModifier {
    /// Immutable. The resource name of the ad group bid modifier.
    /// Ad group bid modifier resource names have the form:
    ///
    /// `customers/{customer_id}/adGroupBidModifiers/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The ad group to which this criterion belongs.
    #[prost(string, optional, tag = "13")]
    pub ad_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The ID of the criterion to bid modify.
    ///
    /// This field is ignored for mutates.
    #[prost(int64, optional, tag = "14")]
    pub criterion_id: ::core::option::Option<i64>,
    /// The modifier for the bid when the criterion matches. The modifier must be
    /// in the range: 0.1 - 10.0. The range is 1.0 - 6.0 for PreferredContent.
    /// Use 0 to opt out of a Device type.
    #[prost(double, optional, tag = "15")]
    pub bid_modifier: ::core::option::Option<f64>,
    /// Output only. The base ad group from which this draft/trial adgroup bid modifier was
    /// created. If ad_group is a base ad group then this field will be equal to
    /// ad_group. If the ad group was created in the draft or trial and has no
    /// corresponding base ad group, then this field will be null.
    /// This field is readonly.
    #[prost(string, optional, tag = "16")]
    pub base_ad_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Bid modifier source.
    #[prost(
        enumeration = "super::enums::bid_modifier_source_enum::BidModifierSource",
        tag = "10"
    )]
    pub bid_modifier_source: i32,
    /// The criterion of this ad group bid modifier.
    ///
    /// Required in create operations starting in V5.
    #[prost(
        oneof = "ad_group_bid_modifier::Criterion",
        tags = "5, 6, 7, 8, 11, 12, 17"
    )]
    pub criterion: ::core::option::Option<ad_group_bid_modifier::Criterion>,
}
/// Nested message and enum types in `AdGroupBidModifier`.
pub mod ad_group_bid_modifier {
    /// The criterion of this ad group bid modifier.
    ///
    /// Required in create operations starting in V5.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Criterion {
        /// Immutable. Criterion for hotel date selection (default dates vs. user selected).
        #[prost(message, tag = "5")]
        HotelDateSelectionType(super::super::common::HotelDateSelectionTypeInfo),
        /// Immutable. Criterion for number of days prior to the stay the booking is being made.
        #[prost(message, tag = "6")]
        HotelAdvanceBookingWindow(super::super::common::HotelAdvanceBookingWindowInfo),
        /// Immutable. Criterion for length of hotel stay in nights.
        #[prost(message, tag = "7")]
        HotelLengthOfStay(super::super::common::HotelLengthOfStayInfo),
        /// Immutable. Criterion for day of the week the booking is for.
        #[prost(message, tag = "8")]
        HotelCheckInDay(super::super::common::HotelCheckInDayInfo),
        /// Immutable. A device criterion.
        #[prost(message, tag = "11")]
        Device(super::super::common::DeviceInfo),
        /// Immutable. A preferred content criterion.
        #[prost(message, tag = "12")]
        PreferredContent(super::super::common::PreferredContentInfo),
        /// Immutable. Criterion for a hotel check-in date range.
        #[prost(message, tag = "17")]
        HotelCheckInDateRange(super::super::common::HotelCheckInDateRangeInfo),
    }
}
// Proto file describing the ad group criterion resource.

/// An ad group criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCriterion {
    /// Immutable. The resource name of the ad group criterion.
    /// Ad group criterion resource names have the form:
    ///
    /// `customers/{customer_id}/adGroupCriteria/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the criterion.
    ///
    /// This field is ignored for mutates.
    #[prost(int64, optional, tag = "56")]
    pub criterion_id: ::core::option::Option<i64>,
    /// Output only. The display name of the criterion.
    ///
    /// This field is ignored for mutates.
    #[prost(string, tag = "77")]
    pub display_name: ::prost::alloc::string::String,
    /// The status of the criterion.
    ///
    /// This is the status of the ad group criterion entity, set by the client.
    /// Note: UI reports may incorporate additional information that affects
    /// whether a criterion is eligible to run. In some cases a criterion that's
    /// REMOVED in the API can still show as enabled in the UI.
    /// For example, campaigns by default show to users of all age ranges unless
    /// excluded. The UI will show each age range as "enabled", since they're
    /// eligible to see the ads; but AdGroupCriterion.status will show "removed",
    /// since no positive criterion was added.
    #[prost(
        enumeration = "super::enums::ad_group_criterion_status_enum::AdGroupCriterionStatus",
        tag = "3"
    )]
    pub status: i32,
    /// Output only. Information regarding the quality of the criterion.
    #[prost(message, optional, tag = "4")]
    pub quality_info: ::core::option::Option<ad_group_criterion::QualityInfo>,
    /// Immutable. The ad group to which the criterion belongs.
    #[prost(string, optional, tag = "57")]
    pub ad_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The type of the criterion.
    #[prost(
        enumeration = "super::enums::criterion_type_enum::CriterionType",
        tag = "25"
    )]
    pub r#type: i32,
    /// Immutable. Whether to target (`false`) or exclude (`true`) the criterion.
    ///
    /// This field is immutable. To switch a criterion from positive to negative,
    /// remove then re-add it.
    #[prost(bool, optional, tag = "58")]
    pub negative: ::core::option::Option<bool>,
    /// Output only. Serving status of the criterion.
    #[prost(
        enumeration = "super::enums::criterion_system_serving_status_enum::CriterionSystemServingStatus",
        tag = "52"
    )]
    pub system_serving_status: i32,
    /// Output only. Approval status of the criterion.
    #[prost(
        enumeration = "super::enums::ad_group_criterion_approval_status_enum::AdGroupCriterionApprovalStatus",
        tag = "53"
    )]
    pub approval_status: i32,
    /// Output only. List of disapproval reasons of the criterion.
    ///
    /// The different reasons for disapproving a criterion can be found here:
    /// <https://support.google.com/adspolicy/answer/6008942>
    ///
    /// This field is read-only.
    #[prost(string, repeated, tag = "59")]
    pub disapproval_reasons: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The resource names of labels attached to this ad group criterion.
    #[prost(string, repeated, tag = "60")]
    pub labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The modifier for the bid when the criterion matches. The modifier must be
    /// in the range: 0.1 - 10.0. Most targetable criteria types support modifiers.
    #[prost(double, optional, tag = "61")]
    pub bid_modifier: ::core::option::Option<f64>,
    /// The CPC (cost-per-click) bid.
    #[prost(int64, optional, tag = "62")]
    pub cpc_bid_micros: ::core::option::Option<i64>,
    /// The CPM (cost-per-thousand viewable impressions) bid.
    #[prost(int64, optional, tag = "63")]
    pub cpm_bid_micros: ::core::option::Option<i64>,
    /// The CPV (cost-per-view) bid.
    #[prost(int64, optional, tag = "64")]
    pub cpv_bid_micros: ::core::option::Option<i64>,
    /// The CPC bid amount, expressed as a fraction of the advertised price
    /// for some good or service. The valid range for the fraction is [0,1) and the
    /// value stored here is 1,000,000 * \[fraction\].
    #[prost(int64, optional, tag = "65")]
    pub percent_cpc_bid_micros: ::core::option::Option<i64>,
    /// Output only. The effective CPC (cost-per-click) bid.
    #[prost(int64, optional, tag = "66")]
    pub effective_cpc_bid_micros: ::core::option::Option<i64>,
    /// Output only. The effective CPM (cost-per-thousand viewable impressions) bid.
    #[prost(int64, optional, tag = "67")]
    pub effective_cpm_bid_micros: ::core::option::Option<i64>,
    /// Output only. The effective CPV (cost-per-view) bid.
    #[prost(int64, optional, tag = "68")]
    pub effective_cpv_bid_micros: ::core::option::Option<i64>,
    /// Output only. The effective Percent CPC bid amount.
    #[prost(int64, optional, tag = "69")]
    pub effective_percent_cpc_bid_micros: ::core::option::Option<i64>,
    /// Output only. Source of the effective CPC bid.
    #[prost(
        enumeration = "super::enums::bidding_source_enum::BiddingSource",
        tag = "21"
    )]
    pub effective_cpc_bid_source: i32,
    /// Output only. Source of the effective CPM bid.
    #[prost(
        enumeration = "super::enums::bidding_source_enum::BiddingSource",
        tag = "22"
    )]
    pub effective_cpm_bid_source: i32,
    /// Output only. Source of the effective CPV bid.
    #[prost(
        enumeration = "super::enums::bidding_source_enum::BiddingSource",
        tag = "23"
    )]
    pub effective_cpv_bid_source: i32,
    /// Output only. Source of the effective Percent CPC bid.
    #[prost(
        enumeration = "super::enums::bidding_source_enum::BiddingSource",
        tag = "35"
    )]
    pub effective_percent_cpc_bid_source: i32,
    /// Output only. Estimates for criterion bids at various positions.
    #[prost(message, optional, tag = "10")]
    pub position_estimates: ::core::option::Option<ad_group_criterion::PositionEstimates>,
    /// The list of possible final URLs after all cross-domain redirects for the
    /// ad.
    #[prost(string, repeated, tag = "70")]
    pub final_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The list of possible final mobile URLs after all cross-domain redirects.
    #[prost(string, repeated, tag = "71")]
    pub final_mobile_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// URL template for appending params to final URL.
    #[prost(string, optional, tag = "72")]
    pub final_url_suffix: ::core::option::Option<::prost::alloc::string::String>,
    /// The URL template for constructing a tracking URL.
    #[prost(string, optional, tag = "73")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
    /// The list of mappings used to substitute custom parameter tags in a
    /// `tracking_url_template`, `final_urls`, or `mobile_final_urls`.
    #[prost(message, repeated, tag = "14")]
    pub url_custom_parameters: ::prost::alloc::vec::Vec<super::common::CustomParameter>,
    /// The ad group criterion.
    ///
    /// Exactly one must be set.
    #[prost(
        oneof = "ad_group_criterion::Criterion",
        tags = "27, 28, 29, 30, 32, 36, 37, 38, 39, 42, 40, 41, 43, 45, 46, 47, 48, 49, 74, 75, 79"
    )]
    pub criterion: ::core::option::Option<ad_group_criterion::Criterion>,
}
/// Nested message and enum types in `AdGroupCriterion`.
pub mod ad_group_criterion {
    /// A container for ad group criterion quality information.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QualityInfo {
        /// Output only. The quality score.
        ///
        /// This field may not be populated if Google does not have enough
        /// information to determine a value.
        #[prost(int32, optional, tag = "5")]
        pub quality_score: ::core::option::Option<i32>,
        /// Output only. The performance of the ad compared to other advertisers.
        #[prost(
            enumeration = "super::super::enums::quality_score_bucket_enum::QualityScoreBucket",
            tag = "2"
        )]
        pub creative_quality_score: i32,
        /// Output only. The quality score of the landing page.
        #[prost(
            enumeration = "super::super::enums::quality_score_bucket_enum::QualityScoreBucket",
            tag = "3"
        )]
        pub post_click_quality_score: i32,
        /// Output only. The click-through rate compared to that of other advertisers.
        #[prost(
            enumeration = "super::super::enums::quality_score_bucket_enum::QualityScoreBucket",
            tag = "4"
        )]
        pub search_predicted_ctr: i32,
    }
    /// Estimates for criterion bids at various positions.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PositionEstimates {
        /// Output only. The estimate of the CPC bid required for ad to be shown on first
        /// page of search results.
        #[prost(int64, optional, tag = "6")]
        pub first_page_cpc_micros: ::core::option::Option<i64>,
        /// Output only. The estimate of the CPC bid required for ad to be displayed in first
        /// position, at the top of the first page of search results.
        #[prost(int64, optional, tag = "7")]
        pub first_position_cpc_micros: ::core::option::Option<i64>,
        /// Output only. The estimate of the CPC bid required for ad to be displayed at the top
        /// of the first page of search results.
        #[prost(int64, optional, tag = "8")]
        pub top_of_page_cpc_micros: ::core::option::Option<i64>,
        /// Output only. Estimate of how many clicks per week you might get by changing your
        /// keyword bid to the value in first_position_cpc_micros.
        #[prost(int64, optional, tag = "9")]
        pub estimated_add_clicks_at_first_position_cpc: ::core::option::Option<i64>,
        /// Output only. Estimate of how your cost per week might change when changing your
        /// keyword bid to the value in first_position_cpc_micros.
        #[prost(int64, optional, tag = "10")]
        pub estimated_add_cost_at_first_position_cpc: ::core::option::Option<i64>,
    }
    /// The ad group criterion.
    ///
    /// Exactly one must be set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Criterion {
        /// Immutable. Keyword.
        #[prost(message, tag = "27")]
        Keyword(super::super::common::KeywordInfo),
        /// Immutable. Placement.
        #[prost(message, tag = "28")]
        Placement(super::super::common::PlacementInfo),
        /// Immutable. Mobile app category.
        #[prost(message, tag = "29")]
        MobileAppCategory(super::super::common::MobileAppCategoryInfo),
        /// Immutable. Mobile application.
        #[prost(message, tag = "30")]
        MobileApplication(super::super::common::MobileApplicationInfo),
        /// Immutable. Listing group.
        #[prost(message, tag = "32")]
        ListingGroup(super::super::common::ListingGroupInfo),
        /// Immutable. Age range.
        #[prost(message, tag = "36")]
        AgeRange(super::super::common::AgeRangeInfo),
        /// Immutable. Gender.
        #[prost(message, tag = "37")]
        Gender(super::super::common::GenderInfo),
        /// Immutable. Income range.
        #[prost(message, tag = "38")]
        IncomeRange(super::super::common::IncomeRangeInfo),
        /// Immutable. Parental status.
        #[prost(message, tag = "39")]
        ParentalStatus(super::super::common::ParentalStatusInfo),
        /// Immutable. User List.
        #[prost(message, tag = "42")]
        UserList(super::super::common::UserListInfo),
        /// Immutable. YouTube Video.
        #[prost(message, tag = "40")]
        YoutubeVideo(super::super::common::YouTubeVideoInfo),
        /// Immutable. YouTube Channel.
        #[prost(message, tag = "41")]
        YoutubeChannel(super::super::common::YouTubeChannelInfo),
        /// Immutable. Topic.
        #[prost(message, tag = "43")]
        Topic(super::super::common::TopicInfo),
        /// Immutable. User Interest.
        #[prost(message, tag = "45")]
        UserInterest(super::super::common::UserInterestInfo),
        /// Immutable. Webpage
        #[prost(message, tag = "46")]
        Webpage(super::super::common::WebpageInfo),
        /// Immutable. App Payment Model.
        #[prost(message, tag = "47")]
        AppPaymentModel(super::super::common::AppPaymentModelInfo),
        /// Immutable. Custom Affinity.
        #[prost(message, tag = "48")]
        CustomAffinity(super::super::common::CustomAffinityInfo),
        /// Immutable. Custom Intent.
        #[prost(message, tag = "49")]
        CustomIntent(super::super::common::CustomIntentInfo),
        /// Immutable. Custom Audience.
        #[prost(message, tag = "74")]
        CustomAudience(super::super::common::CustomAudienceInfo),
        /// Immutable. Combined Audience.
        #[prost(message, tag = "75")]
        CombinedAudience(super::super::common::CombinedAudienceInfo),
        /// Immutable. Audience.
        #[prost(message, tag = "79")]
        Audience(super::super::common::AudienceInfo),
    }
}
/// A customizer value for the associated CustomizerAttribute at the
/// AdGroupCriterion level.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCriterionCustomizer {
    /// Immutable. The resource name of the ad group criterion customizer.
    /// Ad group criterion customizer resource names have the form:
    ///
    /// `customers/{customer_id}/adGroupCriterionCustomizers/{ad_group_id}~{criterion_id}~{customizer_attribute_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The ad group criterion to which the customizer attribute is linked.
    /// It must be a keyword criterion.
    #[prost(string, optional, tag = "2")]
    pub ad_group_criterion: ::core::option::Option<::prost::alloc::string::String>,
    /// Required. Immutable. The customizer attribute which is linked to the ad group criterion.
    #[prost(string, tag = "3")]
    pub customizer_attribute: ::prost::alloc::string::String,
    /// Output only. The status of the ad group criterion customizer.
    #[prost(
        enumeration = "super::enums::customizer_value_status_enum::CustomizerValueStatus",
        tag = "4"
    )]
    pub status: i32,
    /// Required. The value to associate with the customizer attribute at this level. The
    /// value must be of the type specified for the CustomizerAttribute.
    #[prost(message, optional, tag = "5")]
    pub value: ::core::option::Option<super::common::CustomizerValue>,
}
// Proto file describing the ad group criterion label resource.

/// A relationship between an ad group criterion and a label.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCriterionLabel {
    /// Immutable. The resource name of the ad group criterion label.
    /// Ad group criterion label resource names have the form:
    /// `customers/{customer_id}/adGroupCriterionLabels/{ad_group_id}~{criterion_id}~{label_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The ad group criterion to which the label is attached.
    #[prost(string, optional, tag = "4")]
    pub ad_group_criterion: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The label assigned to the ad group criterion.
    #[prost(string, optional, tag = "5")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the ad group criterion simulation resource.

/// An ad group criterion simulation. Supported combinations of advertising
/// channel type, criterion type, simulation type, and simulation modification
/// method are detailed below respectively. Hotel AdGroupCriterion simulation
/// operations starting in V5.
///
/// 1. DISPLAY - KEYWORD - CPC_BID - UNIFORM
/// 2. SEARCH - KEYWORD - CPC_BID - UNIFORM
/// 3. SHOPPING - LISTING_GROUP - CPC_BID - UNIFORM
/// 4. HOTEL - LISTING_GROUP - CPC_BID - UNIFORM
/// 5. HOTEL - LISTING_GROUP - PERCENT_CPC_BID - UNIFORM
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCriterionSimulation {
    /// Output only. The resource name of the ad group criterion simulation.
    /// Ad group criterion simulation resource names have the form:
    ///
    /// `customers/{customer_id}/adGroupCriterionSimulations/{ad_group_id}~{criterion_id}~{type}~{modification_method}~{start_date}~{end_date}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. AdGroup ID of the simulation.
    #[prost(int64, optional, tag = "9")]
    pub ad_group_id: ::core::option::Option<i64>,
    /// Output only. Criterion ID of the simulation.
    #[prost(int64, optional, tag = "10")]
    pub criterion_id: ::core::option::Option<i64>,
    /// Output only. The field that the simulation modifies.
    #[prost(
        enumeration = "super::enums::simulation_type_enum::SimulationType",
        tag = "4"
    )]
    pub r#type: i32,
    /// Output only. How the simulation modifies the field.
    #[prost(
        enumeration = "super::enums::simulation_modification_method_enum::SimulationModificationMethod",
        tag = "5"
    )]
    pub modification_method: i32,
    /// Output only. First day on which the simulation is based, in YYYY-MM-DD format.
    #[prost(string, optional, tag = "11")]
    pub start_date: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Last day on which the simulation is based, in YYYY-MM-DD format.
    #[prost(string, optional, tag = "12")]
    pub end_date: ::core::option::Option<::prost::alloc::string::String>,
    /// List of simulation points.
    #[prost(oneof = "ad_group_criterion_simulation::PointList", tags = "8, 13")]
    pub point_list: ::core::option::Option<ad_group_criterion_simulation::PointList>,
}
/// Nested message and enum types in `AdGroupCriterionSimulation`.
pub mod ad_group_criterion_simulation {
    /// List of simulation points.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PointList {
        /// Output only. Simulation points if the simulation type is CPC_BID.
        #[prost(message, tag = "8")]
        CpcBidPointList(super::super::common::CpcBidSimulationPointList),
        /// Output only. Simulation points if the simulation type is PERCENT_CPC_BID.
        #[prost(message, tag = "13")]
        PercentCpcBidPointList(super::super::common::PercentCpcBidSimulationPointList),
    }
}
/// A customizer value for the associated CustomizerAttribute at the AdGroup
/// level.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCustomizer {
    /// Immutable. The resource name of the ad group customizer.
    /// Ad group customizer resource names have the form:
    ///
    /// `customers/{customer_id}/adGroupCustomizers/{ad_group_id}~{customizer_attribute_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The ad group to which the customizer attribute is linked.
    #[prost(string, tag = "2")]
    pub ad_group: ::prost::alloc::string::String,
    /// Required. Immutable. The customizer attribute which is linked to the ad group.
    #[prost(string, tag = "3")]
    pub customizer_attribute: ::prost::alloc::string::String,
    /// Output only. The status of the ad group customizer.
    #[prost(
        enumeration = "super::enums::customizer_value_status_enum::CustomizerValueStatus",
        tag = "4"
    )]
    pub status: i32,
    /// Required. The value to associate with the customizer attribute at this level. The
    /// value must be of the type specified for the CustomizerAttribute.
    #[prost(message, optional, tag = "5")]
    pub value: ::core::option::Option<super::common::CustomizerValue>,
}
// Proto file describing the AdGroupExtensionSetting resource.

/// An ad group extension setting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupExtensionSetting {
    /// Immutable. The resource name of the ad group extension setting.
    /// AdGroupExtensionSetting resource names have the form:
    ///
    /// `customers/{customer_id}/adGroupExtensionSettings/{ad_group_id}~{extension_type}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The extension type of the ad group extension setting.
    #[prost(
        enumeration = "super::enums::extension_type_enum::ExtensionType",
        tag = "2"
    )]
    pub extension_type: i32,
    /// Immutable. The resource name of the ad group. The linked extension feed items will
    /// serve under this ad group.
    /// AdGroup resource names have the form:
    ///
    /// `customers/{customer_id}/adGroups/{ad_group_id}`
    #[prost(string, optional, tag = "6")]
    pub ad_group: ::core::option::Option<::prost::alloc::string::String>,
    /// The resource names of the extension feed items to serve under the ad group.
    /// ExtensionFeedItem resource names have the form:
    ///
    /// `customers/{customer_id}/extensionFeedItems/{feed_item_id}`
    #[prost(string, repeated, tag = "7")]
    pub extension_feed_items: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The device for which the extensions will serve. Optional.
    #[prost(
        enumeration = "super::enums::extension_setting_device_enum::ExtensionSettingDevice",
        tag = "5"
    )]
    pub device: i32,
}
// Proto file describing the AdGroupFeed resource.

/// An ad group feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupFeed {
    /// Immutable. The resource name of the ad group feed.
    /// Ad group feed resource names have the form:
    ///
    /// `customers/{customer_id}/adGroupFeeds/{ad_group_id}~{feed_id}
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The feed being linked to the ad group.
    #[prost(string, optional, tag = "7")]
    pub feed: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The ad group being linked to the feed.
    #[prost(string, optional, tag = "8")]
    pub ad_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Indicates which placeholder types the feed may populate under the connected
    /// ad group. Required.
    #[prost(
        enumeration = "super::enums::placeholder_type_enum::PlaceholderType",
        repeated,
        tag = "4"
    )]
    pub placeholder_types: ::prost::alloc::vec::Vec<i32>,
    /// Matching function associated with the AdGroupFeed.
    /// The matching function is used to filter the set of feed items selected.
    /// Required.
    #[prost(message, optional, tag = "5")]
    pub matching_function: ::core::option::Option<super::common::MatchingFunction>,
    /// Output only. Status of the ad group feed.
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::feed_link_status_enum::FeedLinkStatus",
        tag = "6"
    )]
    pub status: i32,
}
// Proto file describing the ad group label resource.

/// A relationship between an ad group and a label.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupLabel {
    /// Immutable. The resource name of the ad group label.
    /// Ad group label resource names have the form:
    /// `customers/{customer_id}/adGroupLabels/{ad_group_id}~{label_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The ad group to which the label is attached.
    #[prost(string, optional, tag = "4")]
    pub ad_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The label assigned to the ad group.
    #[prost(string, optional, tag = "5")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the ad group simulation resource.

/// An ad group simulation. Supported combinations of advertising
/// channel type, simulation type and simulation modification method is
/// detailed below respectively.
///
/// 1. SEARCH - CPC_BID - DEFAULT
/// 2. SEARCH - CPC_BID - UNIFORM
/// 3. SEARCH - TARGET_CPA - UNIFORM
/// 4. SEARCH - TARGET_ROAS - UNIFORM
/// 5. DISPLAY - CPC_BID - DEFAULT
/// 6. DISPLAY - CPC_BID - UNIFORM
/// 7. DISPLAY - TARGET_CPA - UNIFORM
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupSimulation {
    /// Output only. The resource name of the ad group simulation.
    /// Ad group simulation resource names have the form:
    ///
    /// `customers/{customer_id}/adGroupSimulations/{ad_group_id}~{type}~{modification_method}~{start_date}~{end_date}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. Ad group id of the simulation.
    #[prost(int64, optional, tag = "12")]
    pub ad_group_id: ::core::option::Option<i64>,
    /// Output only. The field that the simulation modifies.
    #[prost(
        enumeration = "super::enums::simulation_type_enum::SimulationType",
        tag = "3"
    )]
    pub r#type: i32,
    /// Output only. How the simulation modifies the field.
    #[prost(
        enumeration = "super::enums::simulation_modification_method_enum::SimulationModificationMethod",
        tag = "4"
    )]
    pub modification_method: i32,
    /// Output only. First day on which the simulation is based, in YYYY-MM-DD format.
    #[prost(string, optional, tag = "13")]
    pub start_date: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Last day on which the simulation is based, in YYYY-MM-DD format
    #[prost(string, optional, tag = "14")]
    pub end_date: ::core::option::Option<::prost::alloc::string::String>,
    /// List of simulation points.
    #[prost(oneof = "ad_group_simulation::PointList", tags = "8, 10, 9, 11")]
    pub point_list: ::core::option::Option<ad_group_simulation::PointList>,
}
/// Nested message and enum types in `AdGroupSimulation`.
pub mod ad_group_simulation {
    /// List of simulation points.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PointList {
        /// Output only. Simulation points if the simulation type is CPC_BID.
        #[prost(message, tag = "8")]
        CpcBidPointList(super::super::common::CpcBidSimulationPointList),
        /// Output only. Simulation points if the simulation type is CPV_BID.
        #[prost(message, tag = "10")]
        CpvBidPointList(super::super::common::CpvBidSimulationPointList),
        /// Output only. Simulation points if the simulation type is TARGET_CPA.
        #[prost(message, tag = "9")]
        TargetCpaPointList(super::super::common::TargetCpaSimulationPointList),
        /// Output only. Simulation points if the simulation type is TARGET_ROAS.
        #[prost(message, tag = "11")]
        TargetRoasPointList(super::super::common::TargetRoasSimulationPointList),
    }
}
// Proto file describing the ad parameter resource.

/// An ad parameter that is used to update numeric values (such as prices or
/// inventory levels) in any text line of an ad (including URLs). There can
/// be a maximum of two AdParameters per ad group criterion. (One with
/// parameter_index = 1 and one with parameter_index = 2.)
/// In the ad the parameters are referenced by a placeholder of the form
/// "{param#:value}". E.g. "{param1:$17}"
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdParameter {
    /// Immutable. The resource name of the ad parameter.
    /// Ad parameter resource names have the form:
    ///
    /// `customers/{customer_id}/adParameters/{ad_group_id}~{criterion_id}~{parameter_index}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The ad group criterion that this ad parameter belongs to.
    #[prost(string, optional, tag = "5")]
    pub ad_group_criterion: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The unique index of this ad parameter. Must be either 1 or 2.
    #[prost(int64, optional, tag = "6")]
    pub parameter_index: ::core::option::Option<i64>,
    /// Numeric value to insert into the ad text. The following restrictions
    ///  apply:
    ///  - Can use comma or period as a separator, with an optional period or
    ///    comma (respectively) for fractional values. For example, 1,000,000.00
    ///    and 2.000.000,10 are valid.
    ///  - Can be prepended or appended with a currency symbol. For example,
    ///    $99.99 is valid.
    ///  - Can be prepended or appended with a currency code. For example, 99.99USD
    ///    and EUR200 are valid.
    ///  - Can use '%'. For example, 1.0% and 1,0% are valid.
    ///  - Can use plus or minus. For example, -10.99 and 25+ are valid.
    ///  - Can use '/' between two numbers. For example 4/1 and 0.95/0.45 are
    ///    valid.
    #[prost(string, optional, tag = "7")]
    pub insertion_text: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the ad schedule view resource.

/// An ad schedule view summarizes the performance of campaigns by
/// AdSchedule criteria.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdScheduleView {
    /// Output only. The resource name of the ad schedule view.
    /// AdSchedule view resource names have the form:
    ///
    /// `customers/{customer_id}/adScheduleViews/{campaign_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
// Proto file describing the age range view resource.

/// An age range view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgeRangeView {
    /// Output only. The resource name of the age range view.
    /// Age range view resource names have the form:
    ///
    /// `customers/{customer_id}/ageRangeViews/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
// Proto file describing the asset resource.

/// Asset is a part of an ad which can be shared across multiple ads.
/// It can be an image (ImageAsset), a video (YoutubeVideoAsset), etc.
/// Assets are immutable and cannot be removed. To stop an asset from serving,
/// remove the asset from the entity that is using it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// Immutable. The resource name of the asset.
    /// Asset resource names have the form:
    ///
    /// `customers/{customer_id}/assets/{asset_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the asset.
    #[prost(int64, optional, tag = "11")]
    pub id: ::core::option::Option<i64>,
    /// Optional name of the asset.
    #[prost(string, optional, tag = "12")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Type of the asset.
    #[prost(enumeration = "super::enums::asset_type_enum::AssetType", tag = "4")]
    pub r#type: i32,
    /// A list of possible final URLs after all cross domain redirects.
    #[prost(string, repeated, tag = "14")]
    pub final_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of possible final mobile URLs after all cross domain redirects.
    #[prost(string, repeated, tag = "16")]
    pub final_mobile_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// URL template for constructing a tracking URL.
    #[prost(string, optional, tag = "17")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of mappings to be used for substituting URL custom parameter tags in
    /// the tracking_url_template, final_urls, and/or final_mobile_urls.
    #[prost(message, repeated, tag = "18")]
    pub url_custom_parameters: ::prost::alloc::vec::Vec<super::common::CustomParameter>,
    /// URL template for appending params to landing page URLs served with parallel
    /// tracking.
    #[prost(string, optional, tag = "19")]
    pub final_url_suffix: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Policy information for the asset.
    #[prost(message, optional, tag = "13")]
    pub policy_summary: ::core::option::Option<AssetPolicySummary>,
    /// The specific type of the asset.
    #[prost(
        oneof = "asset::AssetData",
        tags = "5, 6, 7, 8, 9, 10, 15, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37"
    )]
    pub asset_data: ::core::option::Option<asset::AssetData>,
}
/// Nested message and enum types in `Asset`.
pub mod asset {
    /// The specific type of the asset.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AssetData {
        /// Immutable. A YouTube video asset.
        #[prost(message, tag = "5")]
        YoutubeVideoAsset(super::super::common::YoutubeVideoAsset),
        /// Immutable. A media bundle asset.
        #[prost(message, tag = "6")]
        MediaBundleAsset(super::super::common::MediaBundleAsset),
        /// Output only. An image asset.
        #[prost(message, tag = "7")]
        ImageAsset(super::super::common::ImageAsset),
        /// Immutable. A text asset.
        #[prost(message, tag = "8")]
        TextAsset(super::super::common::TextAsset),
        /// A lead form asset.
        #[prost(message, tag = "9")]
        LeadFormAsset(super::super::common::LeadFormAsset),
        /// A book on google asset.
        #[prost(message, tag = "10")]
        BookOnGoogleAsset(super::super::common::BookOnGoogleAsset),
        /// A promotion asset.
        #[prost(message, tag = "15")]
        PromotionAsset(super::super::common::PromotionAsset),
        /// A callout asset.
        #[prost(message, tag = "20")]
        CalloutAsset(super::super::common::CalloutAsset),
        /// A structured snippet asset.
        #[prost(message, tag = "21")]
        StructuredSnippetAsset(super::super::common::StructuredSnippetAsset),
        /// A sitelink asset.
        #[prost(message, tag = "22")]
        SitelinkAsset(super::super::common::SitelinkAsset),
        /// A page feed asset.
        #[prost(message, tag = "23")]
        PageFeedAsset(super::super::common::PageFeedAsset),
        /// A dynamic education asset.
        #[prost(message, tag = "24")]
        DynamicEducationAsset(super::super::common::DynamicEducationAsset),
        /// A mobile app asset.
        #[prost(message, tag = "25")]
        MobileAppAsset(super::super::common::MobileAppAsset),
        /// A hotel callout asset.
        #[prost(message, tag = "26")]
        HotelCalloutAsset(super::super::common::HotelCalloutAsset),
        /// A call asset.
        #[prost(message, tag = "27")]
        CallAsset(super::super::common::CallAsset),
        /// A price asset.
        #[prost(message, tag = "28")]
        PriceAsset(super::super::common::PriceAsset),
        /// Immutable. A call to action asset.
        #[prost(message, tag = "29")]
        CallToActionAsset(super::super::common::CallToActionAsset),
        /// A dynamic real estate asset.
        #[prost(message, tag = "30")]
        DynamicRealEstateAsset(super::super::common::DynamicRealEstateAsset),
        /// A dynamic custom asset.
        #[prost(message, tag = "31")]
        DynamicCustomAsset(super::super::common::DynamicCustomAsset),
        /// A dynamic hotels and rentals asset.
        #[prost(message, tag = "32")]
        DynamicHotelsAndRentalsAsset(super::super::common::DynamicHotelsAndRentalsAsset),
        /// A dynamic flights asset.
        #[prost(message, tag = "33")]
        DynamicFlightsAsset(super::super::common::DynamicFlightsAsset),
        /// Immutable. A discovery carousel card asset.
        #[prost(message, tag = "34")]
        DiscoveryCarouselCardAsset(super::super::common::DiscoveryCarouselCardAsset),
        /// A dynamic travel asset.
        #[prost(message, tag = "35")]
        DynamicTravelAsset(super::super::common::DynamicTravelAsset),
        /// A dynamic local asset.
        #[prost(message, tag = "36")]
        DynamicLocalAsset(super::super::common::DynamicLocalAsset),
        /// A dynamic jobs asset.
        #[prost(message, tag = "37")]
        DynamicJobsAsset(super::super::common::DynamicJobsAsset),
    }
}
/// Contains policy information for an asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetPolicySummary {
    /// Output only. The list of policy findings for this asset.
    #[prost(message, repeated, tag = "1")]
    pub policy_topic_entries: ::prost::alloc::vec::Vec<super::common::PolicyTopicEntry>,
    /// Output only. Where in the review process this asset is.
    #[prost(
        enumeration = "super::enums::policy_review_status_enum::PolicyReviewStatus",
        tag = "2"
    )]
    pub review_status: i32,
    /// Output only. The overall approval status of this asset, calculated based on the status
    /// of its individual policy topic entries.
    #[prost(
        enumeration = "super::enums::policy_approval_status_enum::PolicyApprovalStatus",
        tag = "3"
    )]
    pub approval_status: i32,
}
// Proto file describing the AssetFieldTypeView resource.

/// An asset field type view.
/// This view reports non-overcounted metrics for each asset field type when the
/// asset is used as extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetFieldTypeView {
    /// Output only. The resource name of the asset field type view.
    /// Asset field type view resource names have the form:
    ///
    /// `customers/{customer_id}/assetFieldTypeViews/{field_type}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The asset field type of the asset field type view.
    #[prost(
        enumeration = "super::enums::asset_field_type_enum::AssetFieldType",
        tag = "3"
    )]
    pub field_type: i32,
}
/// An asset group.
/// AssetGroupAsset is used to link an asset to the asset group.
/// AssetGroupSignal is used to associate a signal to an asset group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetGroup {
    /// Immutable. The resource name of the asset group.
    /// Asset group resource names have the form:
    ///
    /// `customers/{customer_id}/assetGroups/{asset_group_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the asset group.
    #[prost(int64, tag = "9")]
    pub id: i64,
    /// Immutable. The campaign with which this asset group is associated.
    /// The asset which is linked to the asset group.
    #[prost(string, tag = "2")]
    pub campaign: ::prost::alloc::string::String,
    /// Required. Name of the asset group. Required. It must have a minimum length of 1 and
    /// maximum length of 128. It must be unique under a campaign.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// A list of final URLs after all cross domain redirects. In performance max,
    /// by default, the urls are eligible for expansion unless opted out.
    #[prost(string, repeated, tag = "4")]
    pub final_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of final mobile URLs after all cross domain redirects. In
    /// performance max, by default, the urls are eligible for expansion
    /// unless opted out.
    #[prost(string, repeated, tag = "5")]
    pub final_mobile_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The status of the asset group.
    #[prost(
        enumeration = "super::enums::asset_group_status_enum::AssetGroupStatus",
        tag = "6"
    )]
    pub status: i32,
    /// First part of text that may appear appended to the url displayed in
    /// the ad.
    #[prost(string, tag = "7")]
    pub path1: ::prost::alloc::string::String,
    /// Second part of text that may appear appended to the url displayed in
    /// the ad. This field can only be set when path1 is set.
    #[prost(string, tag = "8")]
    pub path2: ::prost::alloc::string::String,
}
/// AssetGroupAsset is the link between an asset and an asset group.
/// Adding an AssetGroupAsset links an asset with an asset group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetGroupAsset {
    /// Immutable. The resource name of the asset group asset.
    /// Asset group asset resource name have the form:
    ///
    /// `customers/{customer_id}/assetGroupAssets/{asset_group_id}~{asset_id}~{field_type}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The asset group which this asset group asset is linking.
    #[prost(string, tag = "2")]
    pub asset_group: ::prost::alloc::string::String,
    /// Immutable. The asset which this asset group asset is linking.
    #[prost(string, tag = "3")]
    pub asset: ::prost::alloc::string::String,
    /// The description of the placement of the asset within the asset group. E.g.:
    /// HEADLINE, YOUTUBE_VIDEO etc
    #[prost(
        enumeration = "super::enums::asset_field_type_enum::AssetFieldType",
        tag = "4"
    )]
    pub field_type: i32,
    /// The status of the link between an asset and asset group.
    #[prost(
        enumeration = "super::enums::asset_link_status_enum::AssetLinkStatus",
        tag = "5"
    )]
    pub status: i32,
    /// Output only. The performance of this asset group asset.
    #[prost(
        enumeration = "super::enums::asset_performance_label_enum::AssetPerformanceLabel",
        tag = "6"
    )]
    pub performance_label: i32,
    /// Output only. The policy information for this asset group asset.
    #[prost(message, optional, tag = "7")]
    pub policy_summary: ::core::option::Option<super::common::PolicySummary>,
}
/// AssetGroupListingGroupFilter represents a listing group filter tree node in
/// an asset group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetGroupListingGroupFilter {
    /// Immutable. The resource name of the asset group listing group filter.
    /// Asset group listing group filter resource name have the form:
    ///
    /// `customers/{customer_id}/assetGroupListingGroupFilters/{asset_group_id}~{listing_group_filter_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The asset group which this asset group listing group filter is part of.
    #[prost(string, tag = "2")]
    pub asset_group: ::prost::alloc::string::String,
    /// Output only. The ID of the ListingGroupFilter.
    #[prost(int64, tag = "3")]
    pub id: i64,
    /// Immutable. Type of a listing group filter node.
    #[prost(
        enumeration = "super::enums::listing_group_filter_type_enum::ListingGroupFilterType",
        tag = "4"
    )]
    pub r#type: i32,
    /// Immutable. The vertical the current node tree represents. All nodes in the same tree
    /// must belong to the same vertical.
    #[prost(
        enumeration = "super::enums::listing_group_filter_vertical_enum::ListingGroupFilterVertical",
        tag = "5"
    )]
    pub vertical: i32,
    /// Dimension value with which this listing group is refining its parent.
    /// Undefined for the root group.
    #[prost(message, optional, tag = "6")]
    pub case_value: ::core::option::Option<ListingGroupFilterDimension>,
    /// Immutable. Resource name of the parent listing group subdivision. Null for the root
    /// listing group filter node.
    #[prost(string, tag = "7")]
    pub parent_listing_group_filter: ::prost::alloc::string::String,
}
/// Listing dimensions for the asset group listing group filter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupFilterDimension {
    /// Dimension of one of the types below is always present.
    #[prost(
        oneof = "listing_group_filter_dimension::Dimension",
        tags = "1, 2, 3, 4, 5, 6, 7"
    )]
    pub dimension: ::core::option::Option<listing_group_filter_dimension::Dimension>,
}
/// Nested message and enum types in `ListingGroupFilterDimension`.
pub mod listing_group_filter_dimension {
    /// One element of a bidding category at a certain level. Top-level categories
    /// are at level 1, their children at level 2, and so on. We currently support
    /// up to 5 levels. The user must specify a dimension type that indicates the
    /// level of the category. All cases of the same subdivision must have the same
    /// dimension type (category level).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProductBiddingCategory {
        /// ID of the product bidding category.
        ///
        /// This ID is equivalent to the google_product_category ID as described in
        /// this article: <https://support.google.com/merchants/answer/6324436>
        #[prost(int64, optional, tag = "1")]
        pub id: ::core::option::Option<i64>,
        /// Indicates the level of the category in the taxonomy.
        #[prost(
            enumeration = "super::super::enums::listing_group_filter_bidding_category_level_enum::ListingGroupFilterBiddingCategoryLevel",
            tag = "2"
        )]
        pub level: i32,
    }
    /// Brand of the product.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProductBrand {
        /// String value of the product brand.
        #[prost(string, optional, tag = "1")]
        pub value: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// Locality of a product offer.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProductChannel {
        /// Value of the locality.
        #[prost(
            enumeration = "super::super::enums::listing_group_filter_product_channel_enum::ListingGroupFilterProductChannel",
            tag = "1"
        )]
        pub channel: i32,
    }
    /// Condition of a product offer.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProductCondition {
        /// Value of the condition.
        #[prost(
            enumeration = "super::super::enums::listing_group_filter_product_condition_enum::ListingGroupFilterProductCondition",
            tag = "1"
        )]
        pub condition: i32,
    }
    /// Custom attribute of a product offer.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProductCustomAttribute {
        /// String value of the product custom attribute.
        #[prost(string, optional, tag = "1")]
        pub value: ::core::option::Option<::prost::alloc::string::String>,
        /// Indicates the index of the custom attribute.
        #[prost(
            enumeration = "super::super::enums::listing_group_filter_custom_attribute_index_enum::ListingGroupFilterCustomAttributeIndex",
            tag = "2"
        )]
        pub index: i32,
    }
    /// Item id of a product offer.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProductItemId {
        /// Value of the id.
        #[prost(string, optional, tag = "1")]
        pub value: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// Type of a product offer.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProductType {
        /// Value of the type.
        #[prost(string, optional, tag = "1")]
        pub value: ::core::option::Option<::prost::alloc::string::String>,
        /// Level of the type.
        #[prost(
            enumeration = "super::super::enums::listing_group_filter_product_type_level_enum::ListingGroupFilterProductTypeLevel",
            tag = "2"
        )]
        pub level: i32,
    }
    /// Dimension of one of the types below is always present.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Dimension {
        /// Bidding category of a product offer.
        #[prost(message, tag = "1")]
        ProductBiddingCategory(ProductBiddingCategory),
        /// Brand of a product offer.
        #[prost(message, tag = "2")]
        ProductBrand(ProductBrand),
        /// Locality of a product offer.
        #[prost(message, tag = "3")]
        ProductChannel(ProductChannel),
        /// Condition of a product offer.
        #[prost(message, tag = "4")]
        ProductCondition(ProductCondition),
        /// Custom attribute of a product offer.
        #[prost(message, tag = "5")]
        ProductCustomAttribute(ProductCustomAttribute),
        /// Item id of a product offer.
        #[prost(message, tag = "6")]
        ProductItemId(ProductItemId),
        /// Type of a product offer.
        #[prost(message, tag = "7")]
        ProductType(ProductType),
    }
}
// Proto file describing the AssetGroupProductGroupView resource.

/// An asset group product group view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetGroupProductGroupView {
    /// Output only. The resource name of the asset group product group view.
    /// Asset group product group view resource names have the form:
    ///
    /// `customers/{customer_id}/assetGroupProductGroupViews/{asset_group_id}~{listing_group_filter_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The asset group associated with the listing group filter.
    #[prost(string, tag = "2")]
    pub asset_group: ::prost::alloc::string::String,
    /// Output only. The resource name of the asset group listing group filter.
    #[prost(string, tag = "4")]
    pub asset_group_listing_group_filter: ::prost::alloc::string::String,
}
/// AssetGroupSignal represents a signal in an asset group. The existence of a
/// signal tells the performance max campaign who's most likely to convert.
/// Performance Max uses the signal to look for new people with similar or
/// stronger intent to find conversions across Search, Display, Video, and more.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetGroupSignal {
    /// Immutable. The resource name of the asset group signal.
    /// Asset group signal resource name have the form:
    ///
    /// `customers/{customer_id}/assetGroupSignals/{asset_group_id}~{signal_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The asset group which this asset group signal belongs to.
    #[prost(string, tag = "2")]
    pub asset_group: ::prost::alloc::string::String,
    /// Immutable. The signal(audience criterion) to be used by the performance max campaign.
    #[prost(message, optional, tag = "3")]
    pub audience: ::core::option::Option<super::common::AudienceInfo>,
}
/// An asset set representing a collection of assets.
/// Use AssetSetAsset to link an asset to the asset set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSet {
    /// Output only. The ID of the asset set.
    #[prost(int64, tag = "6")]
    pub id: i64,
    /// Immutable. The resource name of the asset set.
    /// Asset set resource names have the form:
    ///
    /// `customers/{customer_id}/assetSets/{asset_set_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Required. Name of the asset set. Required. It must have a minimum length of 1 and
    /// maximum length of 128.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. The type of the asset set. Required.
    #[prost(
        enumeration = "super::enums::asset_set_type_enum::AssetSetType",
        tag = "3"
    )]
    pub r#type: i32,
    /// Output only. The status of the asset set. Read-only.
    #[prost(
        enumeration = "super::enums::asset_set_status_enum::AssetSetStatus",
        tag = "4"
    )]
    pub status: i32,
    /// Merchant ID and Feed Label from Google Merchant Center.
    #[prost(message, optional, tag = "5")]
    pub merchant_center_feed: ::core::option::Option<asset_set::MerchantCenterFeed>,
}
/// Nested message and enum types in `AssetSet`.
pub mod asset_set {
    /// Merchant ID and Feed Label from Google Merchant Center.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MerchantCenterFeed {
        /// Required. Merchant ID from Google Merchant Center
        #[prost(int64, tag = "1")]
        pub merchant_id: i64,
        /// Optional. Feed Label from Google Merchant Center.
        #[prost(string, optional, tag = "2")]
        pub feed_label: ::core::option::Option<::prost::alloc::string::String>,
    }
}
/// AssetSetAsset is the link between an asset and an asset set.
/// Adding an AssetSetAsset links an asset with an asset set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSetAsset {
    /// Immutable. The resource name of the asset set asset.
    /// Asset set asset resource names have the form:
    ///
    /// `customers/{customer_id}/assetSetAssets/{asset_set_id}~{asset_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The asset set which this asset set asset is linking to.
    #[prost(string, tag = "2")]
    pub asset_set: ::prost::alloc::string::String,
    /// Immutable. The asset which this asset set asset is linking to.
    #[prost(string, tag = "3")]
    pub asset: ::prost::alloc::string::String,
    /// Output only. The status of the asset set asset. Read-only.
    #[prost(
        enumeration = "super::enums::asset_set_asset_status_enum::AssetSetAssetStatus",
        tag = "4"
    )]
    pub status: i32,
}
// Proto file describing the Audience resource.

/// Audience is an effective targeting option that allows you to
/// intersect different segment attributes, such as detailed demographics and
/// affinities, to create audiences that represent sections of your target
/// segments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Audience {
    /// Immutable. The resource name of the audience.
    /// Audience names have the form:
    ///
    /// `customers/{customer_id}/audiences/{audience_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. ID of the audience.
    #[prost(int64, tag = "2")]
    pub id: i64,
    /// Output only. Status of this audience. Indicates whether the audience
    /// is enabled or removed.
    #[prost(
        enumeration = "super::enums::audience_status_enum::AudienceStatus",
        tag = "3"
    )]
    pub status: i32,
    /// Required. Name of the audience. It should be unique across all
    /// audiences. It must have a minimum length of 1 and
    /// maximum length of 255.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// Description of this audience.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Positive dimensions specifying the audience composition.
    #[prost(message, repeated, tag = "6")]
    pub dimensions: ::prost::alloc::vec::Vec<super::common::AudienceDimension>,
    /// Negative dimension specifying the audience composition.
    #[prost(message, optional, tag = "7")]
    pub exclusion_dimension: ::core::option::Option<super::common::AudienceExclusionDimension>,
}
// Proto file describing the batch job resource.

/// A list of mutates being processed asynchronously. The mutates are uploaded
/// by the user. The mutates themselves aren't readable and the results of the
/// job can only be read using BatchJobService.ListBatchJobResults.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchJob {
    /// Immutable. The resource name of the batch job.
    /// Batch job resource names have the form:
    ///
    /// `customers/{customer_id}/batchJobs/{batch_job_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. ID of this batch job.
    #[prost(int64, optional, tag = "7")]
    pub id: ::core::option::Option<i64>,
    /// Output only. The next sequence token to use when adding operations. Only set when the
    /// batch job status is PENDING.
    #[prost(string, optional, tag = "8")]
    pub next_add_sequence_token: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Contains additional information about this batch job.
    #[prost(message, optional, tag = "4")]
    pub metadata: ::core::option::Option<batch_job::BatchJobMetadata>,
    /// Output only. Status of this batch job.
    #[prost(
        enumeration = "super::enums::batch_job_status_enum::BatchJobStatus",
        tag = "5"
    )]
    pub status: i32,
    /// Output only. The resource name of the long-running operation that can be used to poll
    /// for completion. Only set when the batch job status is RUNNING or DONE.
    #[prost(string, optional, tag = "9")]
    pub long_running_operation: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `BatchJob`.
pub mod batch_job {
    /// Additional information about the batch job. This message is also used as
    /// metadata returned in batch job Long Running Operations.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BatchJobMetadata {
        /// Output only. The time when this batch job was created.
        /// Formatted as yyyy-mm-dd hh:mm:ss. Example: "2018-03-05 09:15:00"
        #[prost(string, optional, tag = "8")]
        pub creation_date_time: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. The time when this batch job started running.
        /// Formatted as yyyy-mm-dd hh:mm:ss. Example: "2018-03-05 09:15:30"
        #[prost(string, optional, tag = "7")]
        pub start_date_time: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. The time when this batch job was completed.
        /// Formatted as yyyy-MM-dd HH:mm:ss. Example: "2018-03-05 09:16:00"
        #[prost(string, optional, tag = "9")]
        pub completion_date_time: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. The fraction (between 0.0 and 1.0) of mutates that have been processed.
        /// This is empty if the job hasn't started running yet.
        #[prost(double, optional, tag = "10")]
        pub estimated_completion_ratio: ::core::option::Option<f64>,
        /// Output only. The number of mutate operations in the batch job.
        #[prost(int64, optional, tag = "11")]
        pub operation_count: ::core::option::Option<i64>,
        /// Output only. The number of mutate operations executed by the batch job.
        /// Present only if the job has started running.
        #[prost(int64, optional, tag = "12")]
        pub executed_operation_count: ::core::option::Option<i64>,
    }
}
/// Represents a bidding data exclusion.
///
/// See "About data exclusions" at
/// <https://support.google.com/google-ads/answer/10370710.>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingDataExclusion {
    /// Immutable. The resource name of the data exclusion.
    /// Data exclusion resource names have the form:
    ///
    /// `customers/{customer_id}/biddingDataExclusions/{data_exclusion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the data exclusion.
    #[prost(int64, tag = "2")]
    pub data_exclusion_id: i64,
    /// The scope of the data exclusion.
    #[prost(
        enumeration = "super::enums::seasonality_event_scope_enum::SeasonalityEventScope",
        tag = "3"
    )]
    pub scope: i32,
    /// Output only. The status of the data exclusion.
    #[prost(
        enumeration = "super::enums::seasonality_event_status_enum::SeasonalityEventStatus",
        tag = "4"
    )]
    pub status: i32,
    /// Required. The inclusive start time of the data exclusion in yyyy-MM-dd HH:mm:ss
    /// format.
    ///
    /// A data exclusion is backward looking and should be used for events that
    /// start in the past and end either in the past or future.
    #[prost(string, tag = "5")]
    pub start_date_time: ::prost::alloc::string::String,
    /// Required. The exclusive end time of the data exclusion in yyyy-MM-dd HH:mm:ss format.
    ///
    /// The length of [start_date_time, end_date_time) interval must be
    /// within (0, 14 days].
    #[prost(string, tag = "6")]
    pub end_date_time: ::prost::alloc::string::String,
    /// The name of the data exclusion. The name can be at most 255
    /// characters.
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
    /// The description of the data exclusion. The description can be at
    /// most 2048 characters.
    #[prost(string, tag = "8")]
    pub description: ::prost::alloc::string::String,
    /// If not specified, all devices will be included in this exclusion.
    /// Otherwise, only the specified targeted devices will be included in this
    /// exclusion.
    #[prost(enumeration = "super::enums::device_enum::Device", repeated, tag = "9")]
    pub devices: ::prost::alloc::vec::Vec<i32>,
    /// The data exclusion will apply to the campaigns listed when the scope of
    /// this exclusion is CAMPAIGN. The maximum number of campaigns per event is
    /// 2000.
    /// Note: a data exclusion with both advertising_channel_types and
    /// campaign_ids is not supported.
    #[prost(string, repeated, tag = "10")]
    pub campaigns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The data_exclusion will apply to all the campaigns under the listed
    /// channels retroactively as well as going forward when the scope of this
    /// exclusion is CHANNEL.
    /// The supported advertising channel types are DISPLAY, SEARCH and SHOPPING.
    /// Note: a data exclusion with both advertising_channel_types and
    /// campaign_ids is not supported.
    #[prost(
        enumeration = "super::enums::advertising_channel_type_enum::AdvertisingChannelType",
        repeated,
        tag = "11"
    )]
    pub advertising_channel_types: ::prost::alloc::vec::Vec<i32>,
}
/// Represents a bidding seasonality adjustment.
///
/// See "About seasonality adjustments" at
/// <https://support.google.com/google-ads/answer/10369906.>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingSeasonalityAdjustment {
    /// Immutable. The resource name of the seasonality adjustment.
    /// Seasonality adjustment resource names have the form:
    ///
    /// `customers/{customer_id}/biddingSeasonalityAdjustments/{seasonality_adjustment_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the seasonality adjustment.
    #[prost(int64, tag = "2")]
    pub seasonality_adjustment_id: i64,
    /// The scope of the seasonality adjustment.
    #[prost(
        enumeration = "super::enums::seasonality_event_scope_enum::SeasonalityEventScope",
        tag = "3"
    )]
    pub scope: i32,
    /// Output only. The status of the seasonality adjustment.
    #[prost(
        enumeration = "super::enums::seasonality_event_status_enum::SeasonalityEventStatus",
        tag = "4"
    )]
    pub status: i32,
    /// Required. The inclusive start time of the seasonality adjustment in yyyy-MM-dd
    /// HH:mm:ss format.
    ///
    /// A seasonality adjustment is forward looking and should be used for events
    /// that start and end in the future.
    #[prost(string, tag = "5")]
    pub start_date_time: ::prost::alloc::string::String,
    /// Required. The exclusive end time of the seasonality adjustment in yyyy-MM-dd HH:mm:ss
    /// format.
    ///
    /// The length of [start_date_time, end_date_time) interval must be
    /// within (0, 14 days].
    #[prost(string, tag = "6")]
    pub end_date_time: ::prost::alloc::string::String,
    /// The name of the seasonality adjustment. The name can be at most 255
    /// characters.
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
    /// The description of the seasonality adjustment. The description can be at
    /// most 2048 characters.
    #[prost(string, tag = "8")]
    pub description: ::prost::alloc::string::String,
    /// If not specified, all devices will be included in this adjustment.
    /// Otherwise, only the specified targeted devices will be included in this
    /// adjustment.
    #[prost(enumeration = "super::enums::device_enum::Device", repeated, tag = "9")]
    pub devices: ::prost::alloc::vec::Vec<i32>,
    /// Conversion rate modifier estimated based on expected conversion rate
    /// changes. When this field is unset or set to 1.0 no adjustment will be
    /// applied to traffic. The allowed range is 0.1 to 10.0.
    #[prost(double, tag = "10")]
    pub conversion_rate_modifier: f64,
    /// The seasonality adjustment will apply to the campaigns listed when the
    /// scope of this adjustment is CAMPAIGN. The maximum number of campaigns per
    /// event is 2000.
    /// Note: a seasonality adjustment with both advertising_channel_types and
    /// campaign_ids is not supported.
    #[prost(string, repeated, tag = "11")]
    pub campaigns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The seasonality adjustment will apply to all the campaigns under the listed
    /// channels retroactively as well as going forward when the scope of this
    /// adjustment is CHANNEL.
    /// The supported advertising channel types are DISPLAY, SEARCH and SHOPPING.
    /// Note: a seasonality adjustment with both advertising_channel_types and
    /// campaign_ids is not supported.
    #[prost(
        enumeration = "super::enums::advertising_channel_type_enum::AdvertisingChannelType",
        repeated,
        tag = "12"
    )]
    pub advertising_channel_types: ::prost::alloc::vec::Vec<i32>,
}
// Proto file describing the BiddingStrategy resource

/// A bidding strategy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingStrategy {
    /// Immutable. The resource name of the bidding strategy.
    /// Bidding strategy resource names have the form:
    ///
    /// `customers/{customer_id}/biddingStrategies/{bidding_strategy_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the bidding strategy.
    #[prost(int64, optional, tag = "16")]
    pub id: ::core::option::Option<i64>,
    /// The name of the bidding strategy.
    /// All bidding strategies within an account must be named distinctly.
    ///
    /// The length of this string should be between 1 and 255, inclusive,
    /// in UTF-8 bytes, (trimmed).
    #[prost(string, optional, tag = "17")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The status of the bidding strategy.
    ///
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::bidding_strategy_status_enum::BiddingStrategyStatus",
        tag = "15"
    )]
    pub status: i32,
    /// Output only. The type of the bidding strategy.
    /// Create a bidding strategy by setting the bidding scheme.
    ///
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::bidding_strategy_type_enum::BiddingStrategyType",
        tag = "5"
    )]
    pub r#type: i32,
    /// Immutable. The currency used by the bidding strategy (ISO 4217 three-letter code).
    ///
    /// For bidding strategies in manager customers, this currency can be set on
    /// creation and defaults to the manager customer's currency. For serving
    /// customers, this field cannot be set; all strategies in a serving customer
    /// implicitly use the serving customer's currency. In all cases the
    /// effective_currency_code field returns the currency used by the strategy.
    #[prost(string, tag = "23")]
    pub currency_code: ::prost::alloc::string::String,
    /// Output only. The currency used by the bidding strategy (ISO 4217 three-letter code).
    ///
    /// For bidding strategies in manager customers, this is the currency set by
    /// the advertiser when creating the strategy. For serving customers, this is
    /// the customer's currency_code.
    ///
    /// Bidding strategy metrics are reported in this currency.
    ///
    /// This field is read-only.
    #[prost(string, optional, tag = "20")]
    pub effective_currency_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The number of campaigns attached to this bidding strategy.
    ///
    /// This field is read-only.
    #[prost(int64, optional, tag = "18")]
    pub campaign_count: ::core::option::Option<i64>,
    /// Output only. The number of non-removed campaigns attached to this bidding strategy.
    ///
    /// This field is read-only.
    #[prost(int64, optional, tag = "19")]
    pub non_removed_campaign_count: ::core::option::Option<i64>,
    /// The bidding scheme.
    ///
    /// Only one can be set.
    #[prost(oneof = "bidding_strategy::Scheme", tags = "7, 21, 22, 9, 48, 11, 12")]
    pub scheme: ::core::option::Option<bidding_strategy::Scheme>,
}
/// Nested message and enum types in `BiddingStrategy`.
pub mod bidding_strategy {
    /// The bidding scheme.
    ///
    /// Only one can be set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Scheme {
        /// A bidding strategy that raises bids for clicks that seem more likely to
        /// lead to a conversion and lowers them for clicks where they seem less
        /// likely.
        #[prost(message, tag = "7")]
        EnhancedCpc(super::super::common::EnhancedCpc),
        /// An automated bidding strategy to help get the most conversion value for
        /// your campaigns while spending your budget.
        #[prost(message, tag = "21")]
        MaximizeConversionValue(super::super::common::MaximizeConversionValue),
        /// An automated bidding strategy to help get the most conversions for your
        /// campaigns while spending your budget.
        #[prost(message, tag = "22")]
        MaximizeConversions(super::super::common::MaximizeConversions),
        /// A bidding strategy that sets bids to help get as many conversions as
        /// possible at the target cost-per-acquisition (CPA) you set.
        #[prost(message, tag = "9")]
        TargetCpa(super::super::common::TargetCpa),
        /// A bidding strategy that automatically optimizes towards a desired
        /// percentage of impressions.
        #[prost(message, tag = "48")]
        TargetImpressionShare(super::super::common::TargetImpressionShare),
        /// A bidding strategy that helps you maximize revenue while averaging a
        /// specific target Return On Ad Spend (ROAS).
        #[prost(message, tag = "11")]
        TargetRoas(super::super::common::TargetRoas),
        /// A bid strategy that sets your bids to help get as many clicks as
        /// possible within your budget.
        #[prost(message, tag = "12")]
        TargetSpend(super::super::common::TargetSpend),
    }
}
// Proto file describing the bidding strategy simulation resource.

/// A bidding strategy simulation. Supported combinations of simulation type
/// and simulation modification method are detailed below respectively.
///
/// 1. TARGET_CPA - UNIFORM
/// 2. TARGET_ROAS - UNIFORM
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingStrategySimulation {
    /// Output only. The resource name of the bidding strategy simulation.
    /// Bidding strategy simulation resource names have the form:
    ///
    /// `customers/{customer_id}/biddingStrategySimulations/{bidding_strategy_id}~{type}~{modification_method}~{start_date}~{end_date}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. Bidding strategy shared set id of the simulation.
    #[prost(int64, tag = "2")]
    pub bidding_strategy_id: i64,
    /// Output only. The field that the simulation modifies.
    #[prost(
        enumeration = "super::enums::simulation_type_enum::SimulationType",
        tag = "3"
    )]
    pub r#type: i32,
    /// Output only. How the simulation modifies the field.
    #[prost(
        enumeration = "super::enums::simulation_modification_method_enum::SimulationModificationMethod",
        tag = "4"
    )]
    pub modification_method: i32,
    /// Output only. First day on which the simulation is based, in YYYY-MM-DD format.
    #[prost(string, tag = "5")]
    pub start_date: ::prost::alloc::string::String,
    /// Output only. Last day on which the simulation is based, in YYYY-MM-DD format
    #[prost(string, tag = "6")]
    pub end_date: ::prost::alloc::string::String,
    /// List of simulation points.
    #[prost(oneof = "bidding_strategy_simulation::PointList", tags = "7, 8")]
    pub point_list: ::core::option::Option<bidding_strategy_simulation::PointList>,
}
/// Nested message and enum types in `BiddingStrategySimulation`.
pub mod bidding_strategy_simulation {
    /// List of simulation points.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PointList {
        /// Output only. Simulation points if the simulation type is TARGET_CPA.
        #[prost(message, tag = "7")]
        TargetCpaPointList(super::super::common::TargetCpaSimulationPointList),
        /// Output only. Simulation points if the simulation type is TARGET_ROAS.
        #[prost(message, tag = "8")]
        TargetRoasPointList(super::super::common::TargetRoasSimulationPointList),
    }
}
// Proto file describing the BillingSetup resource.

/// A billing setup, which associates a payments account and an advertiser. A
/// billing setup is specific to one advertiser.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BillingSetup {
    /// Immutable. The resource name of the billing setup.
    /// BillingSetup resource names have the form:
    ///
    /// `customers/{customer_id}/billingSetups/{billing_setup_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the billing setup.
    #[prost(int64, optional, tag = "15")]
    pub id: ::core::option::Option<i64>,
    /// Output only. The status of the billing setup.
    #[prost(
        enumeration = "super::enums::billing_setup_status_enum::BillingSetupStatus",
        tag = "3"
    )]
    pub status: i32,
    /// Immutable. The resource name of the payments account associated with this billing
    /// setup. Payments resource names have the form:
    ///
    /// `customers/{customer_id}/paymentsAccounts/{payments_account_id}`
    /// When setting up billing, this is used to signup with an existing payments
    /// account (and then payments_account_info should not be set).
    /// When getting a billing setup, this and payments_account_info will be
    /// populated.
    #[prost(string, optional, tag = "18")]
    pub payments_account: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The payments account information associated with this billing setup.
    /// When setting up billing, this is used to signup with a new payments account
    /// (and then payments_account should not be set).
    /// When getting a billing setup, this and payments_account will be
    /// populated.
    #[prost(message, optional, tag = "12")]
    pub payments_account_info: ::core::option::Option<billing_setup::PaymentsAccountInfo>,
    /// When creating a new billing setup, this is when the setup should take
    /// effect. NOW is the only acceptable start time if the customer doesn't have
    /// any approved setups.
    ///
    /// When fetching an existing billing setup, this is the requested start time.
    /// However, if the setup was approved (see status) after the requested start
    /// time, then this is the approval time.
    #[prost(oneof = "billing_setup::StartTime", tags = "16, 10")]
    pub start_time: ::core::option::Option<billing_setup::StartTime>,
    /// When the billing setup ends / ended. This is either FOREVER or the start
    /// time of the next scheduled billing setup.
    #[prost(oneof = "billing_setup::EndTime", tags = "17, 14")]
    pub end_time: ::core::option::Option<billing_setup::EndTime>,
}
/// Nested message and enum types in `BillingSetup`.
pub mod billing_setup {
    /// Container of payments account information for this billing.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PaymentsAccountInfo {
        /// Output only. A 16 digit id used to identify the payments account associated with the
        /// billing setup.
        ///
        /// This must be passed as a string with dashes, e.g. "1234-5678-9012-3456".
        #[prost(string, optional, tag = "6")]
        pub payments_account_id: ::core::option::Option<::prost::alloc::string::String>,
        /// Immutable. The name of the payments account associated with the billing setup.
        ///
        /// This enables the user to specify a meaningful name for a payments account
        /// to aid in reconciling monthly invoices.
        ///
        /// This name will be printed in the monthly invoices.
        #[prost(string, optional, tag = "7")]
        pub payments_account_name: ::core::option::Option<::prost::alloc::string::String>,
        /// Immutable. A 12 digit id used to identify the payments profile associated with the
        /// billing setup.
        ///
        /// This must be passed in as a string with dashes, e.g. "1234-5678-9012".
        #[prost(string, optional, tag = "8")]
        pub payments_profile_id: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. The name of the payments profile associated with the billing setup.
        #[prost(string, optional, tag = "9")]
        pub payments_profile_name: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. A secondary payments profile id present in uncommon situations, e.g.
        /// when a sequential liability agreement has been arranged.
        #[prost(string, optional, tag = "10")]
        pub secondary_payments_profile_id: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// When creating a new billing setup, this is when the setup should take
    /// effect. NOW is the only acceptable start time if the customer doesn't have
    /// any approved setups.
    ///
    /// When fetching an existing billing setup, this is the requested start time.
    /// However, if the setup was approved (see status) after the requested start
    /// time, then this is the approval time.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StartTime {
        /// Immutable. The start date time in yyyy-MM-dd or yyyy-MM-dd HH:mm:ss format. Only a
        /// future time is allowed.
        #[prost(string, tag = "16")]
        StartDateTime(::prost::alloc::string::String),
        /// Immutable. The start time as a type. Only NOW is allowed.
        #[prost(
            enumeration = "super::super::enums::time_type_enum::TimeType",
            tag = "10"
        )]
        StartTimeType(i32),
    }
    /// When the billing setup ends / ended. This is either FOREVER or the start
    /// time of the next scheduled billing setup.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EndTime {
        /// Output only. The end date time in yyyy-MM-dd or yyyy-MM-dd HH:mm:ss format.
        #[prost(string, tag = "17")]
        EndDateTime(::prost::alloc::string::String),
        /// Output only. The end time as a type.  The only possible value is FOREVER.
        #[prost(
            enumeration = "super::super::enums::time_type_enum::TimeType",
            tag = "14"
        )]
        EndTimeType(i32),
    }
}
// Proto file describing the call view resource.

/// A call view that includes data for call tracking of call-only ads or call
/// extensions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallView {
    /// Output only. The resource name of the call view.
    /// Call view resource names have the form:
    ///
    /// `customers/{customer_id}/callViews/{call_detail_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. Country code of the caller.
    #[prost(string, tag = "2")]
    pub caller_country_code: ::prost::alloc::string::String,
    /// Output only. Area code of the caller. Null if the call duration is shorter than 15
    /// seconds.
    #[prost(string, tag = "3")]
    pub caller_area_code: ::prost::alloc::string::String,
    /// Output only. The advertiser-provided call duration in seconds.
    #[prost(int64, tag = "4")]
    pub call_duration_seconds: i64,
    /// Output only. The advertiser-provided call start date time.
    #[prost(string, tag = "5")]
    pub start_call_date_time: ::prost::alloc::string::String,
    /// Output only. The advertiser-provided call end date time.
    #[prost(string, tag = "6")]
    pub end_call_date_time: ::prost::alloc::string::String,
    /// Output only. The call tracking display location.
    #[prost(
        enumeration = "super::enums::call_tracking_display_location_enum::CallTrackingDisplayLocation",
        tag = "7"
    )]
    pub call_tracking_display_location: i32,
    /// Output only. The type of the call.
    #[prost(enumeration = "super::enums::call_type_enum::CallType", tag = "8")]
    pub r#type: i32,
    /// Output only. The status of the call.
    #[prost(
        enumeration = "super::enums::google_voice_call_status_enum::GoogleVoiceCallStatus",
        tag = "9"
    )]
    pub call_status: i32,
}
// Proto file describing the Campaign resource.

/// A campaign.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Campaign {
    /// Immutable. The resource name of the campaign.
    /// Campaign resource names have the form:
    ///
    /// `customers/{customer_id}/campaigns/{campaign_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the campaign.
    #[prost(int64, optional, tag = "59")]
    pub id: ::core::option::Option<i64>,
    /// The name of the campaign.
    ///
    /// This field is required and should not be empty when creating new
    /// campaigns.
    ///
    /// It must not contain any null (code point 0x0), NL line feed
    /// (code point 0xA) or carriage return (code point 0xD) characters.
    #[prost(string, optional, tag = "58")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The status of the campaign.
    ///
    /// When a new campaign is added, the status defaults to ENABLED.
    #[prost(
        enumeration = "super::enums::campaign_status_enum::CampaignStatus",
        tag = "5"
    )]
    pub status: i32,
    /// Output only. The ad serving status of the campaign.
    #[prost(
        enumeration = "super::enums::campaign_serving_status_enum::CampaignServingStatus",
        tag = "21"
    )]
    pub serving_status: i32,
    /// The ad serving optimization status of the campaign.
    #[prost(
        enumeration = "super::enums::ad_serving_optimization_status_enum::AdServingOptimizationStatus",
        tag = "8"
    )]
    pub ad_serving_optimization_status: i32,
    /// Immutable. The primary serving target for ads within the campaign.
    /// The targeting options can be refined in `network_settings`.
    ///
    /// This field is required and should not be empty when creating new
    /// campaigns.
    ///
    /// Can be set only when creating campaigns.
    /// After the campaign is created, the field can not be changed.
    #[prost(
        enumeration = "super::enums::advertising_channel_type_enum::AdvertisingChannelType",
        tag = "9"
    )]
    pub advertising_channel_type: i32,
    /// Immutable. Optional refinement to `advertising_channel_type`.
    /// Must be a valid sub-type of the parent channel type.
    ///
    /// Can be set only when creating campaigns.
    /// After campaign is created, the field can not be changed.
    #[prost(
        enumeration = "super::enums::advertising_channel_sub_type_enum::AdvertisingChannelSubType",
        tag = "10"
    )]
    pub advertising_channel_sub_type: i32,
    /// The URL template for constructing a tracking URL.
    #[prost(string, optional, tag = "60")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
    /// The list of mappings used to substitute custom parameter tags in a
    /// `tracking_url_template`, `final_urls`, or `mobile_final_urls`.
    #[prost(message, repeated, tag = "12")]
    pub url_custom_parameters: ::prost::alloc::vec::Vec<super::common::CustomParameter>,
    /// Settings for Real-Time Bidding, a feature only available for campaigns
    /// targeting the Ad Exchange network.
    #[prost(message, optional, tag = "39")]
    pub real_time_bidding_setting: ::core::option::Option<super::common::RealTimeBiddingSetting>,
    /// The network settings for the campaign.
    #[prost(message, optional, tag = "14")]
    pub network_settings: ::core::option::Option<campaign::NetworkSettings>,
    /// Immutable. The hotel setting for the campaign.
    #[prost(message, optional, tag = "32")]
    pub hotel_setting: ::core::option::Option<campaign::HotelSettingInfo>,
    /// The setting for controlling Dynamic Search Ads (DSA).
    #[prost(message, optional, tag = "33")]
    pub dynamic_search_ads_setting: ::core::option::Option<campaign::DynamicSearchAdsSetting>,
    /// The setting for controlling Shopping campaigns.
    #[prost(message, optional, tag = "36")]
    pub shopping_setting: ::core::option::Option<campaign::ShoppingSetting>,
    /// Setting for targeting related features.
    #[prost(message, optional, tag = "43")]
    pub targeting_setting: ::core::option::Option<super::common::TargetingSetting>,
    /// Immutable. Setting for audience related features.
    #[prost(message, optional, tag = "73")]
    pub audience_setting: ::core::option::Option<campaign::AudienceSetting>,
    /// The setting for ads geotargeting.
    #[prost(message, optional, tag = "47")]
    pub geo_target_type_setting: ::core::option::Option<campaign::GeoTargetTypeSetting>,
    /// The setting for local campaign.
    #[prost(message, optional, tag = "50")]
    pub local_campaign_setting: ::core::option::Option<campaign::LocalCampaignSetting>,
    /// The setting related to App Campaign.
    #[prost(message, optional, tag = "51")]
    pub app_campaign_setting: ::core::option::Option<campaign::AppCampaignSetting>,
    /// Output only. The resource names of labels attached to this campaign.
    #[prost(string, repeated, tag = "61")]
    pub labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The type of campaign: normal, draft, or experiment.
    #[prost(
        enumeration = "super::enums::campaign_experiment_type_enum::CampaignExperimentType",
        tag = "17"
    )]
    pub experiment_type: i32,
    /// Output only. The resource name of the base campaign of a draft or experiment campaign.
    /// For base campaigns, this is equal to `resource_name`.
    ///
    /// This field is read-only.
    #[prost(string, optional, tag = "56")]
    pub base_campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// The budget of the campaign.
    #[prost(string, optional, tag = "62")]
    pub campaign_budget: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The type of bidding strategy.
    ///
    /// A bidding strategy can be created by setting either the bidding scheme to
    /// create a standard bidding strategy or the `bidding_strategy` field to
    /// create a portfolio bidding strategy.
    ///
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::bidding_strategy_type_enum::BiddingStrategyType",
        tag = "22"
    )]
    pub bidding_strategy_type: i32,
    /// Output only. Resource name of AccessibleBiddingStrategy, a read-only view of the
    /// unrestricted attributes of the attached portfolio bidding
    /// strategy identified by 'bidding_strategy'. Empty, if the campaign does not
    /// use a portfolio strategy.
    /// Unrestricted strategy attributes are available to all customers
    /// with whom the strategy is shared and are read from the
    /// AccessibleBiddingStrategy resource. In contrast, restricted attributes are
    /// only available to the owner customer of the strategy and their managers.
    /// Restricted attributes can only be read from the BiddingStrategy resource.
    #[prost(string, tag = "71")]
    pub accessible_bidding_strategy: ::prost::alloc::string::String,
    /// The date when campaign started in serving customer's timezone in YYYY-MM-DD
    /// format.
    #[prost(string, optional, tag = "63")]
    pub start_date: ::core::option::Option<::prost::alloc::string::String>,
    /// The campaign group this campaign belongs to.
    #[prost(string, optional, tag = "76")]
    pub campaign_group: ::core::option::Option<::prost::alloc::string::String>,
    /// The last day of the campaign in serving customer's timezone in YYYY-MM-DD
    /// format.
    #[prost(string, optional, tag = "64")]
    pub end_date: ::core::option::Option<::prost::alloc::string::String>,
    /// Suffix used to append query parameters to landing pages that are served
    /// with parallel tracking.
    #[prost(string, optional, tag = "65")]
    pub final_url_suffix: ::core::option::Option<::prost::alloc::string::String>,
    /// A list that limits how often each user will see this campaign's ads.
    #[prost(message, repeated, tag = "40")]
    pub frequency_caps: ::prost::alloc::vec::Vec<super::common::FrequencyCapEntry>,
    /// Output only. 3-Tier Brand Safety setting for the campaign.
    #[prost(
        enumeration = "super::enums::brand_safety_suitability_enum::BrandSafetySuitability",
        tag = "42"
    )]
    pub video_brand_safety_suitability: i32,
    /// Describes how unbranded pharma ads will be displayed.
    #[prost(message, optional, tag = "44")]
    pub vanity_pharma: ::core::option::Option<campaign::VanityPharma>,
    /// Selective optimization setting for this campaign, which includes a set of
    /// conversion actions to optimize this campaign towards.
    #[prost(message, optional, tag = "45")]
    pub selective_optimization: ::core::option::Option<campaign::SelectiveOptimization>,
    /// Optimization goal setting for this campaign, which includes a set of
    /// optimization goal types.
    #[prost(message, optional, tag = "54")]
    pub optimization_goal_setting: ::core::option::Option<campaign::OptimizationGoalSetting>,
    /// Output only. Campaign-level settings for tracking information.
    #[prost(message, optional, tag = "46")]
    pub tracking_setting: ::core::option::Option<campaign::TrackingSetting>,
    /// Payment mode for the campaign.
    #[prost(
        enumeration = "super::enums::payment_mode_enum::PaymentMode",
        tag = "52"
    )]
    pub payment_mode: i32,
    /// Output only. Optimization score of the campaign.
    ///
    /// Optimization score is an estimate of how well a campaign is set to perform.
    /// It ranges from 0% (0.0) to 100% (1.0), with 100% indicating that the
    /// campaign is performing at full potential. This field is null for unscored
    /// campaigns.
    ///
    /// See "About optimization score" at
    /// <https://support.google.com/google-ads/answer/9061546.>
    ///
    /// This field is read-only.
    #[prost(double, optional, tag = "66")]
    pub optimization_score: ::core::option::Option<f64>,
    /// The asset field types that should be excluded from this campaign. Asset
    /// links with these field types will not be inherited by this campaign from
    /// the upper level.
    #[prost(
        enumeration = "super::enums::asset_field_type_enum::AssetFieldType",
        repeated,
        tag = "69"
    )]
    pub excluded_parent_asset_field_types: ::prost::alloc::vec::Vec<i32>,
    /// Represents opting out of URL expansion to more targeted URLs. If opted out
    /// (true), only the final URLs in the asset group or URLs specified in the
    /// advertiser's Google Merchant Center or business data feeds are targeted.
    /// If opted in (false), the entire domain will be targeted. This field can
    /// only be set for Performance Max campaigns, where the default value is
    /// false.
    #[prost(bool, optional, tag = "72")]
    pub url_expansion_opt_out: ::core::option::Option<bool>,
    /// The bidding strategy for the campaign.
    ///
    /// Must be either portfolio (created via BiddingStrategy service) or
    /// standard, that is embedded into the campaign.
    #[prost(
        oneof = "campaign::CampaignBiddingStrategy",
        tags = "67, 49, 24, 25, 37, 30, 31, 26, 48, 29, 27, 34, 41"
    )]
    pub campaign_bidding_strategy: ::core::option::Option<campaign::CampaignBiddingStrategy>,
}
/// Nested message and enum types in `Campaign`.
pub mod campaign {
    /// The network settings for the campaign.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NetworkSettings {
        /// Whether ads will be served with google.com search results.
        #[prost(bool, optional, tag = "5")]
        pub target_google_search: ::core::option::Option<bool>,
        /// Whether ads will be served on partner sites in the Google Search Network
        /// (requires `target_google_search` to also be `true`).
        #[prost(bool, optional, tag = "6")]
        pub target_search_network: ::core::option::Option<bool>,
        /// Whether ads will be served on specified placements in the Google Display
        /// Network. Placements are specified using the Placement criterion.
        #[prost(bool, optional, tag = "7")]
        pub target_content_network: ::core::option::Option<bool>,
        /// Whether ads will be served on the Google Partner Network.
        /// This is available only to some select Google partner accounts.
        #[prost(bool, optional, tag = "8")]
        pub target_partner_search_network: ::core::option::Option<bool>,
    }
    /// Campaign-level settings for hotel ads.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HotelSettingInfo {
        /// Immutable. The linked Hotel Center account.
        #[prost(int64, optional, tag = "2")]
        pub hotel_center_id: ::core::option::Option<i64>,
    }
    /// The setting for controlling Dynamic Search Ads (DSA).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DynamicSearchAdsSetting {
        /// Required. The Internet domain name that this setting represents, e.g., "google.com"
        /// or "www.google.com".
        #[prost(string, tag = "6")]
        pub domain_name: ::prost::alloc::string::String,
        /// Required. The language code specifying the language of the domain, e.g., "en".
        #[prost(string, tag = "7")]
        pub language_code: ::prost::alloc::string::String,
        /// Whether the campaign uses advertiser supplied URLs exclusively.
        #[prost(bool, optional, tag = "8")]
        pub use_supplied_urls_only: ::core::option::Option<bool>,
        /// The list of page feeds associated with the campaign.
        #[prost(string, repeated, tag = "9")]
        pub feeds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// The setting for Shopping campaigns. Defines the universe of products that
    /// can be advertised by the campaign, and how this campaign interacts with
    /// other Shopping campaigns.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ShoppingSetting {
        /// Immutable. ID of the Merchant Center account.
        /// This field is required for create operations. This field is immutable for
        /// Shopping campaigns.
        #[prost(int64, optional, tag = "5")]
        pub merchant_id: ::core::option::Option<i64>,
        /// Sales country of products to include in the campaign.
        /// This field is required for Shopping campaigns.
        /// This field is optional for non-Shopping campaigns, but it must be equal
        /// to 'ZZ' if set.
        #[prost(string, optional, tag = "6")]
        pub sales_country: ::core::option::Option<::prost::alloc::string::String>,
        /// Priority of the campaign. Campaigns with numerically higher priorities
        /// take precedence over those with lower priorities.
        /// This field is required for Shopping campaigns, with values between 0 and
        /// 2, inclusive.
        /// This field is optional for Smart Shopping campaigns, but must be equal to
        /// 3 if set.
        #[prost(int32, optional, tag = "7")]
        pub campaign_priority: ::core::option::Option<i32>,
        /// Whether to include local products.
        #[prost(bool, optional, tag = "8")]
        pub enable_local: ::core::option::Option<bool>,
        /// Immutable. Whether to target Vehicle Listing inventory.
        #[prost(bool, tag = "9")]
        pub use_vehicle_inventory: bool,
    }
    /// Campaign-level settings for tracking information.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TrackingSetting {
        /// Output only. The url used for dynamic tracking.
        #[prost(string, optional, tag = "2")]
        pub tracking_url: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// Represents a collection of settings related to ads geotargeting.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GeoTargetTypeSetting {
        /// The setting used for positive geotargeting in this particular campaign.
        #[prost(
            enumeration = "super::super::enums::positive_geo_target_type_enum::PositiveGeoTargetType",
            tag = "1"
        )]
        pub positive_geo_target_type: i32,
        /// The setting used for negative geotargeting in this particular campaign.
        #[prost(
            enumeration = "super::super::enums::negative_geo_target_type_enum::NegativeGeoTargetType",
            tag = "2"
        )]
        pub negative_geo_target_type: i32,
    }
    /// Campaign setting for local campaigns.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocalCampaignSetting {
        /// The location source type for this local campaign.
        #[prost(
            enumeration = "super::super::enums::location_source_type_enum::LocationSourceType",
            tag = "1"
        )]
        pub location_source_type: i32,
    }
    /// Campaign-level settings for App Campaigns.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AppCampaignSetting {
        /// Represents the goal which the bidding strategy of this app campaign
        /// should optimize towards.
        #[prost(
            enumeration = "super::super::enums::app_campaign_bidding_strategy_goal_type_enum::AppCampaignBiddingStrategyGoalType",
            tag = "1"
        )]
        pub bidding_strategy_goal_type: i32,
        /// Immutable. A string that uniquely identifies a mobile application.
        #[prost(string, optional, tag = "4")]
        pub app_id: ::core::option::Option<::prost::alloc::string::String>,
        /// Immutable. The application store that distributes this specific app.
        #[prost(
            enumeration = "super::super::enums::app_campaign_app_store_enum::AppCampaignAppStore",
            tag = "3"
        )]
        pub app_store: i32,
    }
    /// Describes how unbranded pharma ads will be displayed.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VanityPharma {
        /// The display mode for vanity pharma URLs.
        #[prost(
            enumeration = "super::super::enums::vanity_pharma_display_url_mode_enum::VanityPharmaDisplayUrlMode",
            tag = "1"
        )]
        pub vanity_pharma_display_url_mode: i32,
        /// The text that will be displayed in display URL of the text ad when
        /// website description is the selected display mode for vanity pharma URLs.
        #[prost(
            enumeration = "super::super::enums::vanity_pharma_text_enum::VanityPharmaText",
            tag = "2"
        )]
        pub vanity_pharma_text: i32,
    }
    /// Selective optimization setting for this campaign, which includes a set of
    /// conversion actions to optimize this campaign towards.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SelectiveOptimization {
        /// The selected set of conversion actions for optimizing this campaign.
        #[prost(string, repeated, tag = "2")]
        pub conversion_actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Optimization goal setting for this campaign, which includes a set of
    /// optimization goal types.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OptimizationGoalSetting {
        /// The list of optimization goal types.
        #[prost(
            enumeration = "super::super::enums::optimization_goal_type_enum::OptimizationGoalType",
            repeated,
            tag = "1"
        )]
        pub optimization_goal_types: ::prost::alloc::vec::Vec<i32>,
    }
    /// Settings for the audience targeting.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AudienceSetting {
        /// Immutable. If true, this campaign uses an Audience resource for audience targeting.
        /// If false, this campaign may use audience segment criteria instead.
        #[prost(bool, optional, tag = "1")]
        pub use_audience_grouped: ::core::option::Option<bool>,
    }
    /// The bidding strategy for the campaign.
    ///
    /// Must be either portfolio (created via BiddingStrategy service) or
    /// standard, that is embedded into the campaign.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CampaignBiddingStrategy {
        /// Portfolio bidding strategy used by campaign.
        #[prost(string, tag = "67")]
        BiddingStrategy(::prost::alloc::string::String),
        /// Commission is an automatic bidding strategy in which the advertiser pays
        /// a certain portion of the conversion value.
        #[prost(message, tag = "49")]
        Commission(super::super::common::Commission),
        /// Standard Manual CPC bidding strategy.
        /// Manual click-based bidding where user pays per click.
        #[prost(message, tag = "24")]
        ManualCpc(super::super::common::ManualCpc),
        /// Standard Manual CPM bidding strategy.
        /// Manual impression-based bidding where user pays per thousand
        /// impressions.
        #[prost(message, tag = "25")]
        ManualCpm(super::super::common::ManualCpm),
        /// Output only. A bidding strategy that pays a configurable amount per video view.
        #[prost(message, tag = "37")]
        ManualCpv(super::super::common::ManualCpv),
        /// Standard Maximize Conversions bidding strategy that automatically
        /// maximizes number of conversions while spending your budget.
        #[prost(message, tag = "30")]
        MaximizeConversions(super::super::common::MaximizeConversions),
        /// Standard Maximize Conversion Value bidding strategy that automatically
        /// sets bids to maximize revenue while spending your budget.
        #[prost(message, tag = "31")]
        MaximizeConversionValue(super::super::common::MaximizeConversionValue),
        /// Standard Target CPA bidding strategy that automatically sets bids to
        /// help get as many conversions as possible at the target
        /// cost-per-acquisition (CPA) you set.
        #[prost(message, tag = "26")]
        TargetCpa(super::super::common::TargetCpa),
        /// Target Impression Share bidding strategy. An automated bidding strategy
        /// that sets bids to achieve a desired percentage of impressions.
        #[prost(message, tag = "48")]
        TargetImpressionShare(super::super::common::TargetImpressionShare),
        /// Standard Target ROAS bidding strategy that automatically maximizes
        /// revenue while averaging a specific target return on ad spend (ROAS).
        #[prost(message, tag = "29")]
        TargetRoas(super::super::common::TargetRoas),
        /// Standard Target Spend bidding strategy that automatically sets your bids
        /// to help get as many clicks as possible within your budget.
        #[prost(message, tag = "27")]
        TargetSpend(super::super::common::TargetSpend),
        /// Standard Percent Cpc bidding strategy where bids are a fraction of the
        /// advertised price for some good or service.
        #[prost(message, tag = "34")]
        PercentCpc(super::super::common::PercentCpc),
        /// A bidding strategy that automatically optimizes cost per thousand
        /// impressions.
        #[prost(message, tag = "41")]
        TargetCpm(super::super::common::TargetCpm),
    }
}
// Proto file describing the CampaignAsset resource.

/// A link between a Campaign and an Asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignAsset {
    /// Immutable. The resource name of the campaign asset.
    /// CampaignAsset resource names have the form:
    ///
    /// `customers/{customer_id}/campaignAssets/{campaign_id}~{asset_id}~{field_type}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The campaign to which the asset is linked.
    #[prost(string, optional, tag = "6")]
    pub campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The asset which is linked to the campaign.
    #[prost(string, optional, tag = "7")]
    pub asset: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. Role that the asset takes under the linked campaign.
    /// Required.
    #[prost(
        enumeration = "super::enums::asset_field_type_enum::AssetFieldType",
        tag = "4"
    )]
    pub field_type: i32,
    /// Status of the campaign asset.
    #[prost(
        enumeration = "super::enums::asset_link_status_enum::AssetLinkStatus",
        tag = "5"
    )]
    pub status: i32,
}
// Proto file describing the CampaignAsset resource.

/// CampaignAssetSet is the linkage between a campaign and an asset set.
/// Adding a CampaignAssetSet links an asset set with a campaign.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignAssetSet {
    /// Immutable. The resource name of the campaign asset set.
    /// Asset set asset resource names have the form:
    ///
    /// `customers/{customer_id}/campaignAssetSets/{campaign_id}~{asset_set_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The campaign to which this asset set is linked.
    #[prost(string, tag = "2")]
    pub campaign: ::prost::alloc::string::String,
    /// Immutable. The asset set which is linked to the campaign.
    #[prost(string, tag = "3")]
    pub asset_set: ::prost::alloc::string::String,
    /// Output only. The status of the campaign asset set asset. Read-only.
    #[prost(
        enumeration = "super::enums::asset_set_link_status_enum::AssetSetLinkStatus",
        tag = "4"
    )]
    pub status: i32,
}
// Proto file describing the campaign audience view resource.

/// A campaign audience view.
/// Includes performance data from interests and remarketing lists for Display
/// Network and YouTube Network ads, and remarketing lists for search ads (RLSA),
/// aggregated by campaign and audience criterion. This view only includes
/// audiences attached at the campaign level.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignAudienceView {
    /// Output only. The resource name of the campaign audience view.
    /// Campaign audience view resource names have the form:
    ///
    /// `customers/{customer_id}/campaignAudienceViews/{campaign_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
// Proto file describing the Campaign Bid Modifier resource.

/// Represents a bid-modifiable only criterion at the campaign level.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignBidModifier {
    /// Immutable. The resource name of the campaign bid modifier.
    /// Campaign bid modifier resource names have the form:
    ///
    /// `customers/{customer_id}/campaignBidModifiers/{campaign_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The campaign to which this criterion belongs.
    #[prost(string, optional, tag = "6")]
    pub campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The ID of the criterion to bid modify.
    ///
    /// This field is ignored for mutates.
    #[prost(int64, optional, tag = "7")]
    pub criterion_id: ::core::option::Option<i64>,
    /// The modifier for the bid when the criterion matches.
    #[prost(double, optional, tag = "8")]
    pub bid_modifier: ::core::option::Option<f64>,
    /// The criterion of this campaign bid modifier.
    ///
    /// Required in create operations starting in V5.
    #[prost(oneof = "campaign_bid_modifier::Criterion", tags = "5")]
    pub criterion: ::core::option::Option<campaign_bid_modifier::Criterion>,
}
/// Nested message and enum types in `CampaignBidModifier`.
pub mod campaign_bid_modifier {
    /// The criterion of this campaign bid modifier.
    ///
    /// Required in create operations starting in V5.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Criterion {
        /// Immutable. Criterion for interaction type. Only supported for search campaigns.
        #[prost(message, tag = "5")]
        InteractionType(super::super::common::InteractionTypeInfo),
    }
}
// Proto file describing the Budget resource.

/// A campaign budget.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignBudget {
    /// Immutable. The resource name of the campaign budget.
    /// Campaign budget resource names have the form:
    ///
    /// `customers/{customer_id}/campaignBudgets/{campaign_budget_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the campaign budget.
    ///
    /// A campaign budget is created using the CampaignBudgetService create
    /// operation and is assigned a budget ID. A budget ID can be shared across
    /// different campaigns; the system will then allocate the campaign budget
    /// among different campaigns to get optimum results.
    #[prost(int64, optional, tag = "19")]
    pub id: ::core::option::Option<i64>,
    /// The name of the campaign budget.
    ///
    /// When creating a campaign budget through CampaignBudgetService, every
    /// explicitly shared campaign budget must have a non-null, non-empty name.
    /// Campaign budgets that are not explicitly shared derive their name from the
    /// attached campaign's name.
    ///
    /// The length of this string must be between 1 and 255, inclusive,
    /// in UTF-8 bytes, (trimmed).
    #[prost(string, optional, tag = "20")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The amount of the budget, in the local currency for the account.
    /// Amount is specified in micros, where one million is equivalent to one
    /// currency unit. Monthly spend is capped at 30.4 times this amount.
    #[prost(int64, optional, tag = "21")]
    pub amount_micros: ::core::option::Option<i64>,
    /// The lifetime amount of the budget, in the local currency for the account.
    /// Amount is specified in micros, where one million is equivalent to one
    /// currency unit.
    #[prost(int64, optional, tag = "22")]
    pub total_amount_micros: ::core::option::Option<i64>,
    /// Output only. The status of this campaign budget. This field is read-only.
    #[prost(
        enumeration = "super::enums::budget_status_enum::BudgetStatus",
        tag = "6"
    )]
    pub status: i32,
    /// The delivery method that determines the rate at which the campaign budget
    /// is spent.
    ///
    /// Defaults to STANDARD if unspecified in a create operation.
    #[prost(
        enumeration = "super::enums::budget_delivery_method_enum::BudgetDeliveryMethod",
        tag = "7"
    )]
    pub delivery_method: i32,
    /// Specifies whether the budget is explicitly shared. Defaults to true if
    /// unspecified in a create operation.
    ///
    /// If true, the budget was created with the purpose of sharing
    /// across one or more campaigns.
    ///
    /// If false, the budget was created with the intention of only being used
    /// with a single campaign. The budget's name and status will stay in sync
    /// with the campaign's name and status. Attempting to share the budget with a
    /// second campaign will result in an error.
    ///
    /// A non-shared budget can become an explicitly shared. The same operation
    /// must also assign the budget a name.
    ///
    /// A shared campaign budget can never become non-shared.
    #[prost(bool, optional, tag = "23")]
    pub explicitly_shared: ::core::option::Option<bool>,
    /// Output only. The number of campaigns actively using the budget.
    ///
    /// This field is read-only.
    #[prost(int64, optional, tag = "24")]
    pub reference_count: ::core::option::Option<i64>,
    /// Output only. Indicates whether there is a recommended budget for this campaign budget.
    ///
    /// This field is read-only.
    #[prost(bool, optional, tag = "25")]
    pub has_recommended_budget: ::core::option::Option<bool>,
    /// Output only. The recommended budget amount. If no recommendation is available, this will
    /// be set to the budget amount.
    /// Amount is specified in micros, where one million is equivalent to one
    /// currency unit.
    ///
    /// This field is read-only.
    #[prost(int64, optional, tag = "26")]
    pub recommended_budget_amount_micros: ::core::option::Option<i64>,
    /// Immutable. Period over which to spend the budget. Defaults to DAILY if not specified.
    #[prost(
        enumeration = "super::enums::budget_period_enum::BudgetPeriod",
        tag = "13"
    )]
    pub period: i32,
    /// Output only. The estimated change in weekly clicks if the recommended budget is applied.
    ///
    /// This field is read-only.
    #[prost(int64, optional, tag = "27")]
    pub recommended_budget_estimated_change_weekly_clicks: ::core::option::Option<i64>,
    /// Output only. The estimated change in weekly cost in micros if the recommended budget is
    /// applied. One million is equivalent to one currency unit.
    ///
    /// This field is read-only.
    #[prost(int64, optional, tag = "28")]
    pub recommended_budget_estimated_change_weekly_cost_micros: ::core::option::Option<i64>,
    /// Output only. The estimated change in weekly interactions if the recommended budget is
    /// applied.
    ///
    /// This field is read-only.
    #[prost(int64, optional, tag = "29")]
    pub recommended_budget_estimated_change_weekly_interactions: ::core::option::Option<i64>,
    /// Output only. The estimated change in weekly views if the recommended budget is applied.
    ///
    /// This field is read-only.
    #[prost(int64, optional, tag = "30")]
    pub recommended_budget_estimated_change_weekly_views: ::core::option::Option<i64>,
    /// Immutable. The type of the campaign budget.
    #[prost(enumeration = "super::enums::budget_type_enum::BudgetType", tag = "18")]
    pub r#type: i32,
}
/// The biddability setting for the specified campaign only for all
/// conversion actions with a matching category and origin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignConversionGoal {
    /// Immutable. The resource name of the campaign conversion goal.
    /// Campaign conversion goal resource names have the form:
    ///
    /// `customers/{customer_id}/campaignConversionGoals/{campaign_id}~{category}~{origin}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The campaign with which this campaign conversion goal is associated.
    #[prost(string, tag = "2")]
    pub campaign: ::prost::alloc::string::String,
    /// The conversion category of this campaign conversion goal.
    #[prost(
        enumeration = "super::enums::conversion_action_category_enum::ConversionActionCategory",
        tag = "3"
    )]
    pub category: i32,
    /// The conversion origin of this campaign conversion goal.
    #[prost(
        enumeration = "super::enums::conversion_origin_enum::ConversionOrigin",
        tag = "4"
    )]
    pub origin: i32,
    /// The biddability of the campaign conversion goal.
    #[prost(bool, tag = "5")]
    pub biddable: bool,
}
// Proto file describing the Campaign Criterion resource.

/// A campaign criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignCriterion {
    /// Immutable. The resource name of the campaign criterion.
    /// Campaign criterion resource names have the form:
    ///
    /// `customers/{customer_id}/campaignCriteria/{campaign_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The campaign to which the criterion belongs.
    #[prost(string, optional, tag = "37")]
    pub campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The ID of the criterion.
    ///
    /// This field is ignored during mutate.
    #[prost(int64, optional, tag = "38")]
    pub criterion_id: ::core::option::Option<i64>,
    /// Output only. The display name of the criterion.
    ///
    /// This field is ignored for mutates.
    #[prost(string, tag = "43")]
    pub display_name: ::prost::alloc::string::String,
    /// The modifier for the bids when the criterion matches. The modifier must be
    /// in the range: 0.1 - 10.0. Most targetable criteria types support modifiers.
    /// Use 0 to opt out of a Device type.
    #[prost(float, optional, tag = "39")]
    pub bid_modifier: ::core::option::Option<f32>,
    /// Immutable. Whether to target (`false`) or exclude (`true`) the criterion.
    #[prost(bool, optional, tag = "40")]
    pub negative: ::core::option::Option<bool>,
    /// Output only. The type of the criterion.
    #[prost(
        enumeration = "super::enums::criterion_type_enum::CriterionType",
        tag = "6"
    )]
    pub r#type: i32,
    /// The status of the criterion.
    #[prost(
        enumeration = "super::enums::campaign_criterion_status_enum::CampaignCriterionStatus",
        tag = "35"
    )]
    pub status: i32,
    /// The campaign criterion.
    ///
    /// Exactly one must be set.
    #[prost(
        oneof = "campaign_criterion::Criterion",
        tags = "8, 9, 10, 11, 12, 13, 15, 16, 17, 18, 19, 22, 20, 21, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 36, 41, 42, 45"
    )]
    pub criterion: ::core::option::Option<campaign_criterion::Criterion>,
}
/// Nested message and enum types in `CampaignCriterion`.
pub mod campaign_criterion {
    /// The campaign criterion.
    ///
    /// Exactly one must be set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Criterion {
        /// Immutable. Keyword.
        #[prost(message, tag = "8")]
        Keyword(super::super::common::KeywordInfo),
        /// Immutable. Placement.
        #[prost(message, tag = "9")]
        Placement(super::super::common::PlacementInfo),
        /// Immutable. Mobile app category.
        #[prost(message, tag = "10")]
        MobileAppCategory(super::super::common::MobileAppCategoryInfo),
        /// Immutable. Mobile application.
        #[prost(message, tag = "11")]
        MobileApplication(super::super::common::MobileApplicationInfo),
        /// Immutable. Location.
        #[prost(message, tag = "12")]
        Location(super::super::common::LocationInfo),
        /// Immutable. Device.
        #[prost(message, tag = "13")]
        Device(super::super::common::DeviceInfo),
        /// Immutable. Ad Schedule.
        #[prost(message, tag = "15")]
        AdSchedule(super::super::common::AdScheduleInfo),
        /// Immutable. Age range.
        #[prost(message, tag = "16")]
        AgeRange(super::super::common::AgeRangeInfo),
        /// Immutable. Gender.
        #[prost(message, tag = "17")]
        Gender(super::super::common::GenderInfo),
        /// Immutable. Income range.
        #[prost(message, tag = "18")]
        IncomeRange(super::super::common::IncomeRangeInfo),
        /// Immutable. Parental status.
        #[prost(message, tag = "19")]
        ParentalStatus(super::super::common::ParentalStatusInfo),
        /// Immutable. User List.
        #[prost(message, tag = "22")]
        UserList(super::super::common::UserListInfo),
        /// Immutable. YouTube Video.
        #[prost(message, tag = "20")]
        YoutubeVideo(super::super::common::YouTubeVideoInfo),
        /// Immutable. YouTube Channel.
        #[prost(message, tag = "21")]
        YoutubeChannel(super::super::common::YouTubeChannelInfo),
        /// Immutable. Proximity.
        #[prost(message, tag = "23")]
        Proximity(super::super::common::ProximityInfo),
        /// Immutable. Topic.
        #[prost(message, tag = "24")]
        Topic(super::super::common::TopicInfo),
        /// Immutable. Listing scope.
        #[prost(message, tag = "25")]
        ListingScope(super::super::common::ListingScopeInfo),
        /// Immutable. Language.
        #[prost(message, tag = "26")]
        Language(super::super::common::LanguageInfo),
        /// Immutable. IpBlock.
        #[prost(message, tag = "27")]
        IpBlock(super::super::common::IpBlockInfo),
        /// Immutable. ContentLabel.
        #[prost(message, tag = "28")]
        ContentLabel(super::super::common::ContentLabelInfo),
        /// Immutable. Carrier.
        #[prost(message, tag = "29")]
        Carrier(super::super::common::CarrierInfo),
        /// Immutable. User Interest.
        #[prost(message, tag = "30")]
        UserInterest(super::super::common::UserInterestInfo),
        /// Immutable. Webpage.
        #[prost(message, tag = "31")]
        Webpage(super::super::common::WebpageInfo),
        /// Immutable. Operating system version.
        #[prost(message, tag = "32")]
        OperatingSystemVersion(super::super::common::OperatingSystemVersionInfo),
        /// Immutable. Mobile Device.
        #[prost(message, tag = "33")]
        MobileDevice(super::super::common::MobileDeviceInfo),
        /// Immutable. Location Group
        #[prost(message, tag = "34")]
        LocationGroup(super::super::common::LocationGroupInfo),
        /// Immutable. Custom Affinity.
        #[prost(message, tag = "36")]
        CustomAffinity(super::super::common::CustomAffinityInfo),
        /// Immutable. Custom Audience
        #[prost(message, tag = "41")]
        CustomAudience(super::super::common::CustomAudienceInfo),
        /// Immutable. Combined Audience.
        #[prost(message, tag = "42")]
        CombinedAudience(super::super::common::CombinedAudienceInfo),
        /// Immutable. Smart Campaign Keyword Theme.
        #[prost(message, tag = "45")]
        KeywordTheme(super::super::common::KeywordThemeInfo),
    }
}
// Proto file describing the campaign criterion simulation resource.

/// A campaign criterion simulation. Supported combinations of advertising
/// channel type, criterion ids, simulation type and simulation modification
/// method is detailed below respectively.
///
/// 1. SEARCH - 30000,30001,30002 - BID_MODIFIER - UNIFORM
/// 2. DISPLAY - 30001 - BID_MODIFIER - UNIFORM
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignCriterionSimulation {
    /// Output only. The resource name of the campaign criterion simulation.
    /// Campaign criterion simulation resource names have the form:
    ///
    /// `customers/{customer_id}/campaignCriterionSimulations/{campaign_id}~{criterion_id}~{type}~{modification_method}~{start_date}~{end_date}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. Campaign ID of the simulation.
    #[prost(int64, optional, tag = "9")]
    pub campaign_id: ::core::option::Option<i64>,
    /// Output only. Criterion ID of the simulation.
    #[prost(int64, optional, tag = "10")]
    pub criterion_id: ::core::option::Option<i64>,
    /// Output only. The field that the simulation modifies.
    #[prost(
        enumeration = "super::enums::simulation_type_enum::SimulationType",
        tag = "4"
    )]
    pub r#type: i32,
    /// Output only. How the simulation modifies the field.
    #[prost(
        enumeration = "super::enums::simulation_modification_method_enum::SimulationModificationMethod",
        tag = "5"
    )]
    pub modification_method: i32,
    /// Output only. First day on which the simulation is based, in YYYY-MM-DD format.
    #[prost(string, optional, tag = "11")]
    pub start_date: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Last day on which the simulation is based, in YYYY-MM-DD format.
    #[prost(string, optional, tag = "12")]
    pub end_date: ::core::option::Option<::prost::alloc::string::String>,
    /// List of simulation points.
    #[prost(oneof = "campaign_criterion_simulation::PointList", tags = "8")]
    pub point_list: ::core::option::Option<campaign_criterion_simulation::PointList>,
}
/// Nested message and enum types in `CampaignCriterionSimulation`.
pub mod campaign_criterion_simulation {
    /// List of simulation points.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PointList {
        /// Output only. Simulation points if the simulation type is BID_MODIFIER.
        #[prost(message, tag = "8")]
        BidModifierPointList(super::super::common::BidModifierSimulationPointList),
    }
}
/// A customizer value for the associated CustomizerAttribute at the Campaign
/// level.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignCustomizer {
    /// Immutable. The resource name of the campaign customizer.
    /// Campaign customizer resource names have the form:
    ///
    /// `customers/{customer_id}/campaignCustomizers/{campaign_id}~{customizer_attribute_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The campaign to which the customizer attribute is linked.
    #[prost(string, tag = "2")]
    pub campaign: ::prost::alloc::string::String,
    /// Required. Immutable. The customizer attribute which is linked to the campaign.
    #[prost(string, tag = "3")]
    pub customizer_attribute: ::prost::alloc::string::String,
    /// Output only. The status of the campaign customizer.
    #[prost(
        enumeration = "super::enums::customizer_value_status_enum::CustomizerValueStatus",
        tag = "4"
    )]
    pub status: i32,
    /// Required. The value to associate with the customizer attribute at this level. The
    /// value must be of the type specified for the CustomizerAttribute.
    #[prost(message, optional, tag = "5")]
    pub value: ::core::option::Option<super::common::CustomizerValue>,
}
// Proto file describing the Campaign Draft resource.

/// A campaign draft.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignDraft {
    /// Immutable. The resource name of the campaign draft.
    /// Campaign draft resource names have the form:
    ///
    /// `customers/{customer_id}/campaignDrafts/{base_campaign_id}~{draft_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the draft.
    ///
    /// This field is read-only.
    #[prost(int64, optional, tag = "9")]
    pub draft_id: ::core::option::Option<i64>,
    /// Immutable. The base campaign to which the draft belongs.
    #[prost(string, optional, tag = "10")]
    pub base_campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// The name of the campaign draft.
    ///
    /// This field is required and should not be empty when creating new
    /// campaign drafts.
    ///
    /// It must not contain any null (code point 0x0), NL line feed
    /// (code point 0xA) or carriage return (code point 0xD) characters.
    #[prost(string, optional, tag = "11")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Resource name of the Campaign that results from overlaying the draft
    /// changes onto the base campaign.
    ///
    /// This field is read-only.
    #[prost(string, optional, tag = "12")]
    pub draft_campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The status of the campaign draft. This field is read-only.
    ///
    /// When a new campaign draft is added, the status defaults to PROPOSED.
    #[prost(
        enumeration = "super::enums::campaign_draft_status_enum::CampaignDraftStatus",
        tag = "6"
    )]
    pub status: i32,
    /// Output only. Whether there is an experiment based on this draft currently serving.
    #[prost(bool, optional, tag = "13")]
    pub has_experiment_running: ::core::option::Option<bool>,
    /// Output only. The resource name of the long-running operation that can be used to poll
    /// for completion of draft promotion. This is only set if the draft promotion
    /// is in progress or finished.
    #[prost(string, optional, tag = "14")]
    pub long_running_operation: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the Campaign Experiment resource.

/// An A/B experiment that compares the performance of the base campaign
/// (the control) and a variation of that campaign (the experiment).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignExperiment {
    /// Immutable. The resource name of the campaign experiment.
    /// Campaign experiment resource names have the form:
    ///
    /// `customers/{customer_id}/campaignExperiments/{campaign_experiment_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the campaign experiment.
    ///
    /// This field is read-only.
    #[prost(int64, optional, tag = "13")]
    pub id: ::core::option::Option<i64>,
    /// Immutable. The campaign draft with staged changes to the base campaign.
    #[prost(string, optional, tag = "14")]
    pub campaign_draft: ::core::option::Option<::prost::alloc::string::String>,
    /// The name of the campaign experiment.
    ///
    /// This field is required when creating new campaign experiments
    /// and must not conflict with the name of another non-removed
    /// campaign experiment or campaign.
    ///
    /// It must not contain any null (code point 0x0), NL line feed
    /// (code point 0xA) or carriage return (code point 0xD) characters.
    #[prost(string, optional, tag = "15")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The description of the experiment.
    #[prost(string, optional, tag = "16")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. Share of traffic directed to experiment as a percent (must be between 1 and
    /// 99 inclusive. Base campaign receives the remainder of the traffic
    /// (100 - traffic_split_percent). Required for create.
    #[prost(int64, optional, tag = "17")]
    pub traffic_split_percent: ::core::option::Option<i64>,
    /// Immutable. Determines the behavior of the traffic split.
    #[prost(
        enumeration = "super::enums::campaign_experiment_traffic_split_type_enum::CampaignExperimentTrafficSplitType",
        tag = "7"
    )]
    pub traffic_split_type: i32,
    /// Output only. The experiment campaign, as opposed to the base campaign.
    #[prost(string, optional, tag = "18")]
    pub experiment_campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The status of the campaign experiment. This field is read-only.
    #[prost(
        enumeration = "super::enums::campaign_experiment_status_enum::CampaignExperimentStatus",
        tag = "9"
    )]
    pub status: i32,
    /// Output only. The resource name of the long-running operation that can be used to poll
    /// for completion of experiment create or promote. The most recent long
    /// running operation is returned.
    #[prost(string, optional, tag = "19")]
    pub long_running_operation: ::core::option::Option<::prost::alloc::string::String>,
    /// Date when the campaign experiment starts. By default, the experiment starts
    /// now or on the campaign's start date, whichever is later. If this field is
    /// set, then the experiment starts at the beginning of the specified date in
    /// the customer's time zone. Cannot be changed once the experiment starts.
    ///
    /// Format: YYYY-MM-DD
    /// Example: 2019-03-14
    #[prost(string, optional, tag = "20")]
    pub start_date: ::core::option::Option<::prost::alloc::string::String>,
    /// The last day of the campaign experiment. By default, the experiment ends on
    /// the campaign's end date. If this field is set, then the experiment ends at
    /// the end of the specified date in the customer's time zone.
    ///
    /// Format: YYYY-MM-DD
    /// Example: 2019-04-18
    #[prost(string, optional, tag = "21")]
    pub end_date: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the CampaignExtensionSetting resource.

/// A campaign extension setting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignExtensionSetting {
    /// Immutable. The resource name of the campaign extension setting.
    /// CampaignExtensionSetting resource names have the form:
    ///
    /// `customers/{customer_id}/campaignExtensionSettings/{campaign_id}~{extension_type}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The extension type of the customer extension setting.
    #[prost(
        enumeration = "super::enums::extension_type_enum::ExtensionType",
        tag = "2"
    )]
    pub extension_type: i32,
    /// Immutable. The resource name of the campaign. The linked extension feed items will
    /// serve under this campaign.
    /// Campaign resource names have the form:
    ///
    /// `customers/{customer_id}/campaigns/{campaign_id}`
    #[prost(string, optional, tag = "6")]
    pub campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// The resource names of the extension feed items to serve under the campaign.
    /// ExtensionFeedItem resource names have the form:
    ///
    /// `customers/{customer_id}/extensionFeedItems/{feed_item_id}`
    #[prost(string, repeated, tag = "7")]
    pub extension_feed_items: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The device for which the extensions will serve. Optional.
    #[prost(
        enumeration = "super::enums::extension_setting_device_enum::ExtensionSettingDevice",
        tag = "5"
    )]
    pub device: i32,
}
// Proto file describing the CampaignFeed resource.

/// A campaign feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignFeed {
    /// Immutable. The resource name of the campaign feed.
    /// Campaign feed resource names have the form:
    ///
    /// `customers/{customer_id}/campaignFeeds/{campaign_id}~{feed_id}
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The feed to which the CampaignFeed belongs.
    #[prost(string, optional, tag = "7")]
    pub feed: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The campaign to which the CampaignFeed belongs.
    #[prost(string, optional, tag = "8")]
    pub campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// Indicates which placeholder types the feed may populate under the connected
    /// campaign. Required.
    #[prost(
        enumeration = "super::enums::placeholder_type_enum::PlaceholderType",
        repeated,
        tag = "4"
    )]
    pub placeholder_types: ::prost::alloc::vec::Vec<i32>,
    /// Matching function associated with the CampaignFeed.
    /// The matching function is used to filter the set of feed items selected.
    /// Required.
    #[prost(message, optional, tag = "5")]
    pub matching_function: ::core::option::Option<super::common::MatchingFunction>,
    /// Output only. Status of the campaign feed.
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::feed_link_status_enum::FeedLinkStatus",
        tag = "6"
    )]
    pub status: i32,
}
// Proto file describing the Campaign group resource.

/// A campaign group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignGroup {
    /// Immutable. The resource name of the campaign group.
    /// Campaign group resource names have the form:
    ///
    /// `customers/{customer_id}/campaignGroups/{campaign_group_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the campaign group.
    #[prost(int64, tag = "3")]
    pub id: i64,
    /// The name of the campaign group.
    ///
    /// This field is required and should not be empty when creating new campaign
    /// groups.
    ///
    /// It must not contain any null (code point 0x0), NL line feed
    /// (code point 0xA) or carriage return (code point 0xD) characters.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// The status of the campaign group.
    ///
    /// When a new campaign group is added, the status defaults to ENABLED.
    #[prost(
        enumeration = "super::enums::campaign_group_status_enum::CampaignGroupStatus",
        tag = "5"
    )]
    pub status: i32,
}
// Proto file describing the campaign label resource.

/// Represents a relationship between a campaign and a label.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignLabel {
    /// Immutable. Name of the resource.
    /// Campaign label resource names have the form:
    /// `customers/{customer_id}/campaignLabels/{campaign_id}~{label_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The campaign to which the label is attached.
    #[prost(string, optional, tag = "4")]
    pub campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The label assigned to the campaign.
    #[prost(string, optional, tag = "5")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the CampaignSharedSet resource.

/// CampaignSharedSets are used for managing the shared sets associated with a
/// campaign.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignSharedSet {
    /// Immutable. The resource name of the campaign shared set.
    /// Campaign shared set resource names have the form:
    ///
    /// `customers/{customer_id}/campaignSharedSets/{campaign_id}~{shared_set_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The campaign to which the campaign shared set belongs.
    #[prost(string, optional, tag = "5")]
    pub campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The shared set associated with the campaign. This may be a negative keyword
    /// shared set of another customer. This customer should be a manager of the
    /// other customer, otherwise the campaign shared set will exist but have no
    /// serving effect. Only negative keyword shared sets can be associated with
    /// Shopping campaigns. Only negative placement shared sets can be associated
    /// with Display mobile app campaigns.
    #[prost(string, optional, tag = "6")]
    pub shared_set: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The status of this campaign shared set. Read only.
    #[prost(
        enumeration = "super::enums::campaign_shared_set_status_enum::CampaignSharedSetStatus",
        tag = "2"
    )]
    pub status: i32,
}
// Proto file describing the campaign simulation resource.

/// A campaign simulation. Supported combinations of advertising
/// channel type, simulation type and simulation modification
/// method is detailed below respectively.
///
/// SEARCH - CPC_BID - UNIFORM
/// SEARCH - CPC_BID - SCALING
/// SEARCH - TARGET_CPA - UNIFORM
/// SEARCH - TARGET_CPA - SCALING
/// SEARCH - TARGET_ROAS - UNIFORM
/// SEARCH - TARGET_IMPRESSION_SHARE - UNIFORM
/// SEARCH - BUDGET - UNIFORM
/// SHOPPING - BUDGET - UNIFORM
/// SHOPPING - TARGET_ROAS - UNIFORM
/// MULTIPLE - TARGET_CPA - UNIFORM
/// OWNED_AND_OPERATED - TARGET_CPA - DEFAULT
/// DISPLAY - TARGET_CPA - UNIFORM
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignSimulation {
    /// Output only. The resource name of the campaign simulation.
    /// Campaign simulation resource names have the form:
    ///
    /// `customers/{customer_id}/campaignSimulations/{campaign_id}~{type}~{modification_method}~{start_date}~{end_date}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. Campaign id of the simulation.
    #[prost(int64, tag = "2")]
    pub campaign_id: i64,
    /// Output only. The field that the simulation modifies.
    #[prost(
        enumeration = "super::enums::simulation_type_enum::SimulationType",
        tag = "3"
    )]
    pub r#type: i32,
    /// Output only. How the simulation modifies the field.
    #[prost(
        enumeration = "super::enums::simulation_modification_method_enum::SimulationModificationMethod",
        tag = "4"
    )]
    pub modification_method: i32,
    /// Output only. First day on which the simulation is based, in YYYY-MM-DD format.
    #[prost(string, tag = "5")]
    pub start_date: ::prost::alloc::string::String,
    /// Output only. Last day on which the simulation is based, in YYYY-MM-DD format
    #[prost(string, tag = "6")]
    pub end_date: ::prost::alloc::string::String,
    /// List of simulation points.
    #[prost(oneof = "campaign_simulation::PointList", tags = "7, 8, 9, 10, 11")]
    pub point_list: ::core::option::Option<campaign_simulation::PointList>,
}
/// Nested message and enum types in `CampaignSimulation`.
pub mod campaign_simulation {
    /// List of simulation points.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PointList {
        /// Output only. Simulation points if the simulation type is CPC_BID.
        #[prost(message, tag = "7")]
        CpcBidPointList(super::super::common::CpcBidSimulationPointList),
        /// Output only. Simulation points if the simulation type is TARGET_CPA.
        #[prost(message, tag = "8")]
        TargetCpaPointList(super::super::common::TargetCpaSimulationPointList),
        /// Output only. Simulation points if the simulation type is TARGET_ROAS.
        #[prost(message, tag = "9")]
        TargetRoasPointList(super::super::common::TargetRoasSimulationPointList),
        /// Output only. Simulation points if the simulation type is TARGET_IMPRESSION_SHARE.
        #[prost(message, tag = "10")]
        TargetImpressionSharePointList(
            super::super::common::TargetImpressionShareSimulationPointList,
        ),
        /// Output only. Simulation points if the simulation type is BUDGET.
        #[prost(message, tag = "11")]
        BudgetPointList(super::super::common::BudgetSimulationPointList),
    }
}
// Proto file describing the Carrier constant resource.

/// A carrier criterion that can be used in campaign targeting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CarrierConstant {
    /// Output only. The resource name of the carrier criterion.
    /// Carrier criterion resource names have the form:
    ///
    /// `carrierConstants/{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the carrier criterion.
    #[prost(int64, optional, tag = "5")]
    pub id: ::core::option::Option<i64>,
    /// Output only. The full name of the carrier in English.
    #[prost(string, optional, tag = "6")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The country code of the country where the carrier is located, e.g., "AR",
    /// "FR", etc.
    #[prost(string, optional, tag = "7")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the CustomerAsset resource.

/// A link between a customer and an asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerAsset {
    /// Immutable. The resource name of the customer asset.
    /// CustomerAsset resource names have the form:
    ///
    /// `customers/{customer_id}/customerAssets/{asset_id}~{field_type}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Required. Immutable. The asset which is linked to the customer.
    #[prost(string, tag = "2")]
    pub asset: ::prost::alloc::string::String,
    /// Required. Immutable. Role that the asset takes for the customer link.
    #[prost(
        enumeration = "super::enums::asset_field_type_enum::AssetFieldType",
        tag = "3"
    )]
    pub field_type: i32,
    /// Status of the customer asset.
    #[prost(
        enumeration = "super::enums::asset_link_status_enum::AssetLinkStatus",
        tag = "4"
    )]
    pub status: i32,
}
// Proto file describing the Feed resource.

/// A feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feed {
    /// Immutable. The resource name of the feed.
    /// Feed resource names have the form:
    ///
    /// `customers/{customer_id}/feeds/{feed_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the feed.
    /// This field is read-only.
    #[prost(int64, optional, tag = "11")]
    pub id: ::core::option::Option<i64>,
    /// Immutable. Name of the feed. Required.
    #[prost(string, optional, tag = "12")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The Feed's attributes. Required on CREATE, unless
    /// system_feed_generation_data is provided, in which case Google Ads will
    /// update the feed with the correct attributes.
    /// Disallowed on UPDATE. Use attribute_operations to add new attributes.
    #[prost(message, repeated, tag = "4")]
    pub attributes: ::prost::alloc::vec::Vec<FeedAttribute>,
    /// The list of operations changing the feed attributes. Attributes can only
    /// be added, not removed.
    #[prost(message, repeated, tag = "9")]
    pub attribute_operations: ::prost::alloc::vec::Vec<FeedAttributeOperation>,
    /// Immutable. Specifies who manages the FeedAttributes for the Feed.
    #[prost(enumeration = "super::enums::feed_origin_enum::FeedOrigin", tag = "5")]
    pub origin: i32,
    /// Output only. Status of the feed.
    /// This field is read-only.
    #[prost(enumeration = "super::enums::feed_status_enum::FeedStatus", tag = "8")]
    pub status: i32,
    /// The system data for the Feed. This data specifies information for
    /// generating the feed items of the system generated feed.
    #[prost(oneof = "feed::SystemFeedGenerationData", tags = "6, 7")]
    pub system_feed_generation_data: ::core::option::Option<feed::SystemFeedGenerationData>,
}
/// Nested message and enum types in `Feed`.
pub mod feed {
    /// Data used to configure a location feed populated from Business Profile.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlacesLocationFeedData {
        /// Immutable. Required authentication token (from OAuth API) for the email.
        /// This field can only be specified in a create request. All its subfields
        /// are not selectable.
        #[prost(message, optional, tag = "1")]
        pub oauth_info: ::core::option::Option<places_location_feed_data::OAuthInfo>,
        /// Email address of a Business Profile or email address of a
        /// manager of the Business Profile. Required.
        #[prost(string, optional, tag = "7")]
        pub email_address: ::core::option::Option<::prost::alloc::string::String>,
        /// Plus page ID of the managed business whose locations should be used. If
        /// this field is not set, then all businesses accessible by the user
        /// (specified by email_address) are used.
        /// This field is mutate-only and is not selectable.
        #[prost(string, tag = "8")]
        pub business_account_id: ::prost::alloc::string::String,
        /// Used to filter Business Profile listings by business name. If
        /// business_name_filter is set, only listings with a matching business name
        /// are candidates to be sync'd into FeedItems.
        #[prost(string, optional, tag = "9")]
        pub business_name_filter: ::core::option::Option<::prost::alloc::string::String>,
        /// Used to filter Business Profile listings by categories. If entries
        /// exist in category_filters, only listings that belong to any of the
        /// categories are candidates to be sync'd into FeedItems. If no entries
        /// exist in category_filters, then all listings are candidates for syncing.
        #[prost(string, repeated, tag = "11")]
        pub category_filters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Used to filter Business Profile listings by labels. If entries exist in
        /// label_filters, only listings that has any of the labels set are
        /// candidates to be synchronized into FeedItems. If no entries exist in
        /// label_filters, then all listings are candidates for syncing.
        #[prost(string, repeated, tag = "12")]
        pub label_filters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Nested message and enum types in `PlacesLocationFeedData`.
    pub mod places_location_feed_data {
        /// Data used for authorization using OAuth.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct OAuthInfo {
            /// The HTTP method used to obtain authorization.
            #[prost(string, optional, tag = "4")]
            pub http_method: ::core::option::Option<::prost::alloc::string::String>,
            /// The HTTP request URL used to obtain authorization.
            #[prost(string, optional, tag = "5")]
            pub http_request_url: ::core::option::Option<::prost::alloc::string::String>,
            /// The HTTP authorization header used to obtain authorization.
            #[prost(string, optional, tag = "6")]
            pub http_authorization_header: ::core::option::Option<::prost::alloc::string::String>,
        }
    }
    /// Data used to configure an affiliate location feed populated with the
    /// specified chains.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AffiliateLocationFeedData {
        /// The list of chains that the affiliate location feed will sync the
        /// locations from.
        #[prost(int64, repeated, tag = "3")]
        pub chain_ids: ::prost::alloc::vec::Vec<i64>,
        /// The relationship the chains have with the advertiser.
        #[prost(
            enumeration = "super::super::enums::affiliate_location_feed_relationship_type_enum::AffiliateLocationFeedRelationshipType",
            tag = "2"
        )]
        pub relationship_type: i32,
    }
    /// The system data for the Feed. This data specifies information for
    /// generating the feed items of the system generated feed.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SystemFeedGenerationData {
        /// Data used to configure a location feed populated from Business Profile.
        #[prost(message, tag = "6")]
        PlacesLocationFeedData(PlacesLocationFeedData),
        /// Data used to configure an affiliate location feed populated with
        /// the specified chains.
        #[prost(message, tag = "7")]
        AffiliateLocationFeedData(AffiliateLocationFeedData),
    }
}
/// FeedAttributes define the types of data expected to be present in a Feed. A
/// single FeedAttribute specifies the expected type of the FeedItemAttributes
/// with the same FeedAttributeId. Optionally, a FeedAttribute can be marked as
/// being part of a FeedItem's unique key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedAttribute {
    /// ID of the attribute.
    #[prost(int64, optional, tag = "5")]
    pub id: ::core::option::Option<i64>,
    /// The name of the attribute. Required.
    #[prost(string, optional, tag = "6")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Data type for feed attribute. Required.
    #[prost(
        enumeration = "super::enums::feed_attribute_type_enum::FeedAttributeType",
        tag = "3"
    )]
    pub r#type: i32,
    /// Indicates that data corresponding to this attribute is part of a
    /// FeedItem's unique key. It defaults to false if it is unspecified. Note
    /// that a unique key is not required in a Feed's schema, in which case the
    /// FeedItems must be referenced by their feed_item_id.
    #[prost(bool, optional, tag = "7")]
    pub is_part_of_key: ::core::option::Option<bool>,
}
/// Operation to be performed on a feed attribute list in a mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedAttributeOperation {
    /// Output only. Type of list operation to perform.
    #[prost(enumeration = "feed_attribute_operation::Operator", tag = "1")]
    pub operator: i32,
    /// Output only. The feed attribute being added to the list.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<FeedAttribute>,
}
/// Nested message and enum types in `FeedAttributeOperation`.
pub mod feed_attribute_operation {
    /// The operator.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operator {
        /// Unspecified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Add the attribute to the existing attributes.
        Add = 2,
    }
}
// Proto file describing the FeedItem resource.

/// A feed item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItem {
    /// Immutable. The resource name of the feed item.
    /// Feed item resource names have the form:
    ///
    /// `customers/{customer_id}/feedItems/{feed_id}~{feed_item_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The feed to which this feed item belongs.
    #[prost(string, optional, tag = "11")]
    pub feed: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The ID of this feed item.
    #[prost(int64, optional, tag = "12")]
    pub id: ::core::option::Option<i64>,
    /// Start time in which this feed item is effective and can begin serving. The
    /// time is in the customer's time zone.
    /// The format is "YYYY-MM-DD HH:MM:SS".
    /// Examples: "2018-03-05 09:15:00" or "2018-02-01 14:34:30"
    #[prost(string, optional, tag = "13")]
    pub start_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// End time in which this feed item is no longer effective and will stop
    /// serving. The time is in the customer's time zone.
    /// The format is "YYYY-MM-DD HH:MM:SS".
    /// Examples: "2018-03-05 09:15:00" or "2018-02-01 14:34:30"
    #[prost(string, optional, tag = "14")]
    pub end_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// The feed item's attribute values.
    #[prost(message, repeated, tag = "6")]
    pub attribute_values: ::prost::alloc::vec::Vec<FeedItemAttributeValue>,
    /// Geo targeting restriction specifies the type of location that can be used
    /// for targeting.
    #[prost(
        enumeration = "super::enums::geo_targeting_restriction_enum::GeoTargetingRestriction",
        tag = "7"
    )]
    pub geo_targeting_restriction: i32,
    /// The list of mappings used to substitute custom parameter tags in a
    /// `tracking_url_template`, `final_urls`, or `mobile_final_urls`.
    #[prost(message, repeated, tag = "8")]
    pub url_custom_parameters: ::prost::alloc::vec::Vec<super::common::CustomParameter>,
    /// Output only. Status of the feed item.
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::feed_item_status_enum::FeedItemStatus",
        tag = "9"
    )]
    pub status: i32,
    /// Output only. List of info about a feed item's validation and approval state for active
    /// feed mappings. There will be an entry in the list for each type of feed
    /// mapping associated with the feed, e.g. a feed with a sitelink and a call
    /// feed mapping would cause every feed item associated with that feed to have
    /// an entry in this list for both sitelink and call.
    /// This field is read-only.
    #[prost(message, repeated, tag = "10")]
    pub policy_infos: ::prost::alloc::vec::Vec<FeedItemPlaceholderPolicyInfo>,
}
/// A feed item attribute value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemAttributeValue {
    /// Id of the feed attribute for which the value is associated with.
    #[prost(int64, optional, tag = "11")]
    pub feed_attribute_id: ::core::option::Option<i64>,
    /// Int64 value. Should be set if feed_attribute_id refers to a feed attribute
    /// of type INT64.
    #[prost(int64, optional, tag = "12")]
    pub integer_value: ::core::option::Option<i64>,
    /// Bool value. Should be set if feed_attribute_id refers to a feed attribute
    /// of type BOOLEAN.
    #[prost(bool, optional, tag = "13")]
    pub boolean_value: ::core::option::Option<bool>,
    /// String value. Should be set if feed_attribute_id refers to a feed attribute
    /// of type STRING, URL or DATE_TIME.
    /// For STRING the maximum length is 1500 characters. For URL the maximum
    /// length is 2076 characters. For DATE_TIME the string must be in the format
    /// "YYYYMMDD HHMMSS".
    #[prost(string, optional, tag = "14")]
    pub string_value: ::core::option::Option<::prost::alloc::string::String>,
    /// Double value. Should be set if feed_attribute_id refers to a feed attribute
    /// of type DOUBLE.
    #[prost(double, optional, tag = "15")]
    pub double_value: ::core::option::Option<f64>,
    /// Price value. Should be set if feed_attribute_id refers to a feed attribute
    /// of type PRICE.
    #[prost(message, optional, tag = "6")]
    pub price_value: ::core::option::Option<super::common::Money>,
    /// Repeated int64 value. Should be set if feed_attribute_id refers to a feed
    /// attribute of type INT64_LIST.
    #[prost(int64, repeated, tag = "16")]
    pub integer_values: ::prost::alloc::vec::Vec<i64>,
    /// Repeated bool value. Should be set if feed_attribute_id refers to a feed
    /// attribute of type BOOLEAN_LIST.
    #[prost(bool, repeated, tag = "17")]
    pub boolean_values: ::prost::alloc::vec::Vec<bool>,
    /// Repeated string value. Should be set if feed_attribute_id refers to a feed
    /// attribute of type STRING_LIST, URL_LIST or DATE_TIME_LIST.
    /// For STRING_LIST and URL_LIST the total size of the list in bytes may not
    /// exceed 3000. For DATE_TIME_LIST the number of elements may not exceed 200.
    ///
    /// For STRING_LIST the maximum length of each string element is 1500
    /// characters. For URL_LIST the maximum length is 2076 characters. For
    /// DATE_TIME the format of the string must be the same as start and end time
    /// for the feed item.
    #[prost(string, repeated, tag = "18")]
    pub string_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Repeated double value. Should be set if feed_attribute_id refers to a feed
    /// attribute of type DOUBLE_LIST.
    #[prost(double, repeated, tag = "19")]
    pub double_values: ::prost::alloc::vec::Vec<f64>,
}
/// Policy, validation, and quality approval info for a feed item for the
/// specified placeholder type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemPlaceholderPolicyInfo {
    /// Output only. The placeholder type.
    #[prost(
        enumeration = "super::enums::placeholder_type_enum::PlaceholderType",
        tag = "10"
    )]
    pub placeholder_type_enum: i32,
    /// Output only. The FeedMapping that contains the placeholder type.
    #[prost(string, optional, tag = "11")]
    pub feed_mapping_resource_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Where the placeholder type is in the review process.
    #[prost(
        enumeration = "super::enums::policy_review_status_enum::PolicyReviewStatus",
        tag = "3"
    )]
    pub review_status: i32,
    /// Output only. The overall approval status of the placeholder type, calculated based on
    /// the status of its individual policy topic entries.
    #[prost(
        enumeration = "super::enums::policy_approval_status_enum::PolicyApprovalStatus",
        tag = "4"
    )]
    pub approval_status: i32,
    /// Output only. The list of policy findings for the placeholder type.
    #[prost(message, repeated, tag = "5")]
    pub policy_topic_entries: ::prost::alloc::vec::Vec<super::common::PolicyTopicEntry>,
    /// Output only. The validation status of the palceholder type.
    #[prost(
        enumeration = "super::enums::feed_item_validation_status_enum::FeedItemValidationStatus",
        tag = "6"
    )]
    pub validation_status: i32,
    /// Output only. List of placeholder type validation errors.
    #[prost(message, repeated, tag = "7")]
    pub validation_errors: ::prost::alloc::vec::Vec<FeedItemValidationError>,
    /// Output only. Placeholder type quality evaluation approval status.
    #[prost(
        enumeration = "super::enums::feed_item_quality_approval_status_enum::FeedItemQualityApprovalStatus",
        tag = "8"
    )]
    pub quality_approval_status: i32,
    /// Output only. List of placeholder type quality evaluation disapproval reasons.
    #[prost(
        enumeration = "super::enums::feed_item_quality_disapproval_reason_enum::FeedItemQualityDisapprovalReason",
        repeated,
        packed = "false",
        tag = "9"
    )]
    pub quality_disapproval_reasons: ::prost::alloc::vec::Vec<i32>,
}
/// Stores a validation error and the set of offending feed attributes which
/// together are responsible for causing a feed item validation error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemValidationError {
    /// Output only. Error code indicating what validation error was triggered. The description
    /// of the error can be found in the 'description' field.
    #[prost(
        enumeration = "super::errors::feed_item_validation_error_enum::FeedItemValidationError",
        tag = "1"
    )]
    pub validation_error: i32,
    /// Output only. The description of the validation error.
    #[prost(string, optional, tag = "6")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Set of feed attributes in the feed item flagged during validation. If
    /// empty, no specific feed attributes can be associated with the error
    /// (e.g. error across the entire feed item).
    #[prost(int64, repeated, packed = "false", tag = "7")]
    pub feed_attribute_ids: ::prost::alloc::vec::Vec<i64>,
    /// Output only. Any extra information related to this error which is not captured by
    /// validation_error and feed_attribute_id (e.g. placeholder field IDs when
    /// feed_attribute_id is not mapped). Note that extra_info is not localized.
    #[prost(string, optional, tag = "8")]
    pub extra_info: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the Change Event resource.

/// Describes the granular change of returned resource of certain resource types.
/// Changes made through UI, API and new versions of Editor
/// by external users (including external users, and internal users that can be
/// shown externally) in the past 30 days will be shown. The change shows the old
/// values of the changed fields before the change and the new values right after
/// the change. ChangeEvent could have up to 3 minutes delay to reflect a new
/// change.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeEvent {
    /// Output only. The resource name of the change event.
    /// Change event resource names have the form:
    ///
    /// `customers/{customer_id}/changeEvents/{timestamp_micros}~{command_index}~{mutate_index}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. Time at which the change was committed on this resource.
    #[prost(string, tag = "2")]
    pub change_date_time: ::prost::alloc::string::String,
    /// Output only. The type of the changed resource. This dictates what resource
    /// will be set in old_resource and new_resource.
    #[prost(
        enumeration = "super::enums::change_event_resource_type_enum::ChangeEventResourceType",
        tag = "3"
    )]
    pub change_resource_type: i32,
    /// Output only. The Simply resource this change occurred on.
    #[prost(string, tag = "4")]
    pub change_resource_name: ::prost::alloc::string::String,
    /// Output only. Where the change was made through.
    #[prost(
        enumeration = "super::enums::change_client_type_enum::ChangeClientType",
        tag = "5"
    )]
    pub client_type: i32,
    /// Output only. The email of the user who made this change.
    #[prost(string, tag = "6")]
    pub user_email: ::prost::alloc::string::String,
    /// Output only. The old resource before the change. Only changed fields will be populated.
    #[prost(message, optional, tag = "7")]
    pub old_resource: ::core::option::Option<change_event::ChangedResource>,
    /// Output only. The new resource after the change. Only changed fields will be populated.
    #[prost(message, optional, tag = "8")]
    pub new_resource: ::core::option::Option<change_event::ChangedResource>,
    /// Output only. The operation on the changed resource.
    #[prost(
        enumeration = "super::enums::resource_change_operation_enum::ResourceChangeOperation",
        tag = "9"
    )]
    pub resource_change_operation: i32,
    /// Output only. A list of fields that are changed in the returned resource.
    #[prost(message, optional, tag = "10")]
    pub changed_fields: ::core::option::Option<::prost_types::FieldMask>,
    /// Output only. The Campaign affected by this change.
    #[prost(string, tag = "11")]
    pub campaign: ::prost::alloc::string::String,
    /// Output only. The AdGroup affected by this change.
    #[prost(string, tag = "12")]
    pub ad_group: ::prost::alloc::string::String,
    /// Output only. The Feed affected by this change.
    #[prost(string, tag = "13")]
    pub feed: ::prost::alloc::string::String,
    /// Output only. The FeedItem affected by this change.
    #[prost(string, tag = "14")]
    pub feed_item: ::prost::alloc::string::String,
    /// Output only. The Asset affected by this change.
    #[prost(string, tag = "20")]
    pub asset: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ChangeEvent`.
pub mod change_event {
    /// A wrapper proto presenting all supported resources.
    /// Only the resource of the change_resource_type will be set.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChangedResource {
        /// Output only. Set if change_resource_type == AD.
        #[prost(message, optional, tag = "1")]
        pub ad: ::core::option::Option<super::Ad>,
        /// Output only. Set if change_resource_type == AD_GROUP.
        #[prost(message, optional, tag = "2")]
        pub ad_group: ::core::option::Option<super::AdGroup>,
        /// Output only. Set if change_resource_type == AD_GROUP_CRITERION.
        #[prost(message, optional, tag = "3")]
        pub ad_group_criterion: ::core::option::Option<super::AdGroupCriterion>,
        /// Output only. Set if change_resource_type == CAMPAIGN.
        #[prost(message, optional, tag = "4")]
        pub campaign: ::core::option::Option<super::Campaign>,
        /// Output only. Set if change_resource_type == CAMPAIGN_BUDGET.
        #[prost(message, optional, tag = "5")]
        pub campaign_budget: ::core::option::Option<super::CampaignBudget>,
        /// Output only. Set if change_resource_type == AD_GROUP_BID_MODIFIER.
        #[prost(message, optional, tag = "6")]
        pub ad_group_bid_modifier: ::core::option::Option<super::AdGroupBidModifier>,
        /// Output only. Set if change_resource_type == CAMPAIGN_CRITERION.
        #[prost(message, optional, tag = "7")]
        pub campaign_criterion: ::core::option::Option<super::CampaignCriterion>,
        /// Output only. Set if change_resource_type == FEED.
        #[prost(message, optional, tag = "8")]
        pub feed: ::core::option::Option<super::Feed>,
        /// Output only. Set if change_resource_type == FEED_ITEM.
        #[prost(message, optional, tag = "9")]
        pub feed_item: ::core::option::Option<super::FeedItem>,
        /// Output only. Set if change_resource_type == CAMPAIGN_FEED.
        #[prost(message, optional, tag = "10")]
        pub campaign_feed: ::core::option::Option<super::CampaignFeed>,
        /// Output only. Set if change_resource_type == AD_GROUP_FEED.
        #[prost(message, optional, tag = "11")]
        pub ad_group_feed: ::core::option::Option<super::AdGroupFeed>,
        /// Output only. Set if change_resource_type == AD_GROUP_AD.
        #[prost(message, optional, tag = "12")]
        pub ad_group_ad: ::core::option::Option<super::AdGroupAd>,
        /// Output only. Set if change_resource_type == ASSET.
        #[prost(message, optional, tag = "13")]
        pub asset: ::core::option::Option<super::Asset>,
        /// Output only. Set if change_resource_type == CUSTOMER_ASSET.
        #[prost(message, optional, tag = "14")]
        pub customer_asset: ::core::option::Option<super::CustomerAsset>,
        /// Output only. Set if change_resource_type == CAMPAIGN_ASSET.
        #[prost(message, optional, tag = "15")]
        pub campaign_asset: ::core::option::Option<super::CampaignAsset>,
        /// Output only. Set if change_resource_type == AD_GROUP_ASSET.
        #[prost(message, optional, tag = "16")]
        pub ad_group_asset: ::core::option::Option<super::AdGroupAsset>,
        /// Output only. Set if change_resource_type == ASSET_SET.
        #[prost(message, optional, tag = "17")]
        pub asset_set: ::core::option::Option<super::AssetSet>,
        /// Output only. Set if change_resource_type == ASSET_SET_ASSET.
        #[prost(message, optional, tag = "18")]
        pub asset_set_asset: ::core::option::Option<super::AssetSetAsset>,
        /// Output only. Set if change_resource_type == CAMPAIGN_ASSET_SET.
        #[prost(message, optional, tag = "19")]
        pub campaign_asset_set: ::core::option::Option<super::CampaignAssetSet>,
    }
}
// Proto file describing the Change Status resource.

/// Describes the status of returned resource. ChangeStatus could have up to 3
/// minutes delay to reflect a new change.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeStatus {
    /// Output only. The resource name of the change status.
    /// Change status resource names have the form:
    ///
    /// `customers/{customer_id}/changeStatus/{change_status_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. Time at which the most recent change has occurred on this resource.
    #[prost(string, optional, tag = "24")]
    pub last_change_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Represents the type of the changed resource. This dictates what fields
    /// will be set. For example, for AD_GROUP, campaign and ad_group fields will
    /// be set.
    #[prost(
        enumeration = "super::enums::change_status_resource_type_enum::ChangeStatusResourceType",
        tag = "4"
    )]
    pub resource_type: i32,
    /// Output only. The Campaign affected by this change.
    #[prost(string, optional, tag = "17")]
    pub campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The AdGroup affected by this change.
    #[prost(string, optional, tag = "18")]
    pub ad_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Represents the status of the changed resource.
    #[prost(
        enumeration = "super::enums::change_status_operation_enum::ChangeStatusOperation",
        tag = "8"
    )]
    pub resource_status: i32,
    /// Output only. The AdGroupAd affected by this change.
    #[prost(string, optional, tag = "25")]
    pub ad_group_ad: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The AdGroupCriterion affected by this change.
    #[prost(string, optional, tag = "26")]
    pub ad_group_criterion: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The CampaignCriterion affected by this change.
    #[prost(string, optional, tag = "27")]
    pub campaign_criterion: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The Feed affected by this change.
    #[prost(string, optional, tag = "28")]
    pub feed: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The FeedItem affected by this change.
    #[prost(string, optional, tag = "29")]
    pub feed_item: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The AdGroupFeed affected by this change.
    #[prost(string, optional, tag = "30")]
    pub ad_group_feed: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The CampaignFeed affected by this change.
    #[prost(string, optional, tag = "31")]
    pub campaign_feed: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The AdGroupBidModifier affected by this change.
    #[prost(string, optional, tag = "32")]
    pub ad_group_bid_modifier: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The SharedSet affected by this change.
    #[prost(string, tag = "33")]
    pub shared_set: ::prost::alloc::string::String,
    /// Output only. The CampaignSharedSet affected by this change.
    #[prost(string, tag = "34")]
    pub campaign_shared_set: ::prost::alloc::string::String,
    /// Output only. The Asset affected by this change.
    #[prost(string, tag = "35")]
    pub asset: ::prost::alloc::string::String,
    /// Output only. The CustomerAsset affected by this change.
    #[prost(string, tag = "36")]
    pub customer_asset: ::prost::alloc::string::String,
    /// Output only. The CampaignAsset affected by this change.
    #[prost(string, tag = "37")]
    pub campaign_asset: ::prost::alloc::string::String,
    /// Output only. The AdGroupAsset affected by this change.
    #[prost(string, tag = "38")]
    pub ad_group_asset: ::prost::alloc::string::String,
}
// Proto file describing the ClickView resource.

/// A click view with metrics aggregated at each click level, including both
/// valid and invalid clicks. For non-Search campaigns, metrics.clicks
/// represents the number of valid and invalid interactions.
/// Queries including ClickView must have a filter limiting the results to one
/// day and can be requested for dates back to 90 days before the time of the
/// request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClickView {
    /// Output only. The resource name of the click view.
    /// Click view resource names have the form:
    ///
    /// `customers/{customer_id}/clickViews/{date (yyyy-MM-dd)}~{gclid}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The Google Click ID.
    #[prost(string, optional, tag = "8")]
    pub gclid: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The location criteria matching the area of interest associated with the
    /// impression.
    #[prost(message, optional, tag = "3")]
    pub area_of_interest: ::core::option::Option<super::common::ClickLocation>,
    /// Output only. The location criteria matching the location of presence associated with the
    /// impression.
    #[prost(message, optional, tag = "4")]
    pub location_of_presence: ::core::option::Option<super::common::ClickLocation>,
    /// Output only. Page number in search results where the ad was shown.
    #[prost(int64, optional, tag = "9")]
    pub page_number: ::core::option::Option<i64>,
    /// Output only. The associated ad.
    #[prost(string, optional, tag = "10")]
    pub ad_group_ad: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The associated campaign location target, if one exists.
    #[prost(string, optional, tag = "11")]
    pub campaign_location_target: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The associated user list, if one exists.
    #[prost(string, optional, tag = "12")]
    pub user_list: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The associated keyword, if one exists and the click corresponds to the
    /// SEARCH channel.
    #[prost(string, tag = "13")]
    pub keyword: ::prost::alloc::string::String,
    /// Output only. Basic information about the associated keyword, if it exists.
    #[prost(message, optional, tag = "14")]
    pub keyword_info: ::core::option::Option<super::common::KeywordInfo>,
}
// Proto file describing the Combined Audience resource.

/// Describe a resource for combined audiences which includes different
/// audiences.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CombinedAudience {
    /// Immutable. The resource name of the combined audience.
    /// Combined audience names have the form:
    ///
    /// `customers/{customer_id}/combinedAudience/{combined_audience_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. ID of the combined audience.
    #[prost(int64, tag = "2")]
    pub id: i64,
    /// Output only. Status of this combined audience. Indicates whether the combined audience
    /// is enabled or removed.
    #[prost(
        enumeration = "super::enums::combined_audience_status_enum::CombinedAudienceStatus",
        tag = "3"
    )]
    pub status: i32,
    /// Output only. Name of the combined audience. It should be unique across all combined
    /// audiences.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Description of this combined audience.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
}
// Proto file describing the Conversion Action resource.

/// A conversion action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionAction {
    /// Immutable. The resource name of the conversion action.
    /// Conversion action resource names have the form:
    ///
    /// `customers/{customer_id}/conversionActions/{conversion_action_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the conversion action.
    #[prost(int64, optional, tag = "21")]
    pub id: ::core::option::Option<i64>,
    /// The name of the conversion action.
    ///
    /// This field is required and should not be empty when creating new
    /// conversion actions.
    #[prost(string, optional, tag = "22")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The status of this conversion action for conversion event accrual.
    #[prost(
        enumeration = "super::enums::conversion_action_status_enum::ConversionActionStatus",
        tag = "4"
    )]
    pub status: i32,
    /// Immutable. The type of this conversion action.
    #[prost(
        enumeration = "super::enums::conversion_action_type_enum::ConversionActionType",
        tag = "5"
    )]
    pub r#type: i32,
    /// Output only. The conversion origin of this conversion action.
    #[prost(
        enumeration = "super::enums::conversion_origin_enum::ConversionOrigin",
        tag = "30"
    )]
    pub origin: i32,
    /// If a conversion action's primary_for_goal bit is false, the conversion
    /// action is non-biddable for all campaigns regardless of their customer
    /// conversion goal or campaign conversion goal.
    /// However, custom conversion goals do not respect primary_for_goal, so if
    /// a campaign has a custom conversion goal configured with a
    /// primary_for_goal = false conversion action, that conversion action is
    /// still biddable.
    /// By default, primary_for_goal will be true if not set. In V9,
    /// primary_for_goal can only be set to false after creation via an 'update'
    /// operation because it's not declared as optional.
    #[prost(bool, optional, tag = "31")]
    pub primary_for_goal: ::core::option::Option<bool>,
    /// The category of conversions reported for this conversion action.
    #[prost(
        enumeration = "super::enums::conversion_action_category_enum::ConversionActionCategory",
        tag = "6"
    )]
    pub category: i32,
    /// Output only. The resource name of the conversion action owner customer, or null if this
    /// is a system-defined conversion action.
    #[prost(string, optional, tag = "23")]
    pub owner_customer: ::core::option::Option<::prost::alloc::string::String>,
    /// Whether this conversion action should be included in the "conversions"
    /// metric.
    #[prost(bool, optional, tag = "24")]
    pub include_in_conversions_metric: ::core::option::Option<bool>,
    /// The maximum number of days that may elapse between an interaction
    /// (e.g., a click) and a conversion event.
    #[prost(int64, optional, tag = "25")]
    pub click_through_lookback_window_days: ::core::option::Option<i64>,
    /// The maximum number of days which may elapse between an impression and a
    /// conversion without an interaction.
    #[prost(int64, optional, tag = "26")]
    pub view_through_lookback_window_days: ::core::option::Option<i64>,
    /// Settings related to the value for conversion events associated with this
    /// conversion action.
    #[prost(message, optional, tag = "11")]
    pub value_settings: ::core::option::Option<conversion_action::ValueSettings>,
    /// How to count conversion events for the conversion action.
    #[prost(
        enumeration = "super::enums::conversion_action_counting_type_enum::ConversionActionCountingType",
        tag = "12"
    )]
    pub counting_type: i32,
    /// Settings related to this conversion action's attribution model.
    #[prost(message, optional, tag = "13")]
    pub attribution_model_settings:
        ::core::option::Option<conversion_action::AttributionModelSettings>,
    /// Output only. The snippets used for tracking conversions.
    #[prost(message, repeated, tag = "14")]
    pub tag_snippets: ::prost::alloc::vec::Vec<super::common::TagSnippet>,
    /// The phone call duration in seconds after which a conversion should be
    /// reported for this conversion action.
    ///
    /// The value must be between 0 and 10000, inclusive.
    #[prost(int64, optional, tag = "27")]
    pub phone_call_duration_seconds: ::core::option::Option<i64>,
    /// App ID for an app conversion action.
    #[prost(string, optional, tag = "28")]
    pub app_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Mobile app vendor for an app conversion action.
    #[prost(
        enumeration = "super::enums::mobile_app_vendor_enum::MobileAppVendor",
        tag = "17"
    )]
    pub mobile_app_vendor: i32,
    /// Output only. Firebase settings for Firebase conversion types.
    #[prost(message, optional, tag = "18")]
    pub firebase_settings: ::core::option::Option<conversion_action::FirebaseSettings>,
    /// Output only. Third Party App Analytics settings for third party conversion types.
    #[prost(message, optional, tag = "19")]
    pub third_party_app_analytics_settings:
        ::core::option::Option<conversion_action::ThirdPartyAppAnalyticsSettings>,
}
/// Nested message and enum types in `ConversionAction`.
pub mod conversion_action {
    /// Settings related to this conversion action's attribution model.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AttributionModelSettings {
        /// The attribution model type of this conversion action.
        #[prost(
            enumeration = "super::super::enums::attribution_model_enum::AttributionModel",
            tag = "1"
        )]
        pub attribution_model: i32,
        /// Output only. The status of the data-driven attribution model for the conversion
        /// action.
        #[prost(
            enumeration = "super::super::enums::data_driven_model_status_enum::DataDrivenModelStatus",
            tag = "2"
        )]
        pub data_driven_model_status: i32,
    }
    /// Settings related to the value for conversion events associated with this
    /// conversion action.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ValueSettings {
        /// The value to use when conversion events for this conversion action are
        /// sent with an invalid, disallowed or missing value, or when
        /// this conversion action is configured to always use the default value.
        #[prost(double, optional, tag = "4")]
        pub default_value: ::core::option::Option<f64>,
        /// The currency code to use when conversion events for this conversion
        /// action are sent with an invalid or missing currency code, or when this
        /// conversion action is configured to always use the default value.
        #[prost(string, optional, tag = "5")]
        pub default_currency_code: ::core::option::Option<::prost::alloc::string::String>,
        /// Controls whether the default value and default currency code are used in
        /// place of the value and currency code specified in conversion events for
        /// this conversion action.
        #[prost(bool, optional, tag = "6")]
        pub always_use_default_value: ::core::option::Option<bool>,
    }
    /// Settings related to a third party app analytics conversion action.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ThirdPartyAppAnalyticsSettings {
        /// Output only. The event name of a third-party app analytics conversion.
        #[prost(string, optional, tag = "2")]
        pub event_name: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. Name of the third-party app analytics provider.
        #[prost(string, tag = "3")]
        pub provider_name: ::prost::alloc::string::String,
    }
    /// Settings related to a Firebase conversion action.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FirebaseSettings {
        /// Output only. The event name of a Firebase conversion.
        #[prost(string, optional, tag = "3")]
        pub event_name: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. The Firebase project ID of the conversion.
        #[prost(string, optional, tag = "4")]
        pub project_id: ::core::option::Option<::prost::alloc::string::String>,
    }
}
// Proto file describing the Conversion Custom Variable resource.

/// A conversion custom variable
/// See "About custom variables for conversions" at
/// <https://support.google.com/google-ads/answer/9964350>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionCustomVariable {
    /// Immutable. The resource name of the conversion custom variable.
    /// Conversion custom variable resource names have the form:
    ///
    /// `customers/{customer_id}/conversionCustomVariables/{conversion_custom_variable_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the conversion custom variable.
    #[prost(int64, tag = "2")]
    pub id: i64,
    /// Required. The name of the conversion custom variable.
    /// Name should be unique. The maximum length of name is 100 characters.
    /// There should not be any extra spaces before and after.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. The tag of the conversion custom variable. It is used in the event snippet
    /// and sent to Google Ads along with conversion pings. For conversion uploads
    /// in Google Ads API, the resource name of the conversion custom variable is
    /// used.
    /// Tag should be unique. The maximum size of tag is 100 bytes.
    /// There should not be any extra spaces before and after.
    /// Currently only lowercase letters, numbers and underscores are allowed in
    /// the tag.
    #[prost(string, tag = "4")]
    pub tag: ::prost::alloc::string::String,
    /// The status of the conversion custom variable for conversion event accrual.
    #[prost(
        enumeration = "super::enums::conversion_custom_variable_status_enum::ConversionCustomVariableStatus",
        tag = "5"
    )]
    pub status: i32,
    /// Output only. The resource name of the customer that owns the conversion custom variable.
    #[prost(string, tag = "6")]
    pub owner_customer: ::prost::alloc::string::String,
}
/// Conversion goal settings for a Campaign.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionGoalCampaignConfig {
    /// Immutable. The resource name of the conversion goal campaign config.
    /// Conversion goal campaign config resource names have the form:
    ///
    /// `customers/{customer_id}/conversionGoalCampaignConfigs/{campaign_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The campaign with which this conversion goal campaign config is associated.
    #[prost(string, tag = "2")]
    pub campaign: ::prost::alloc::string::String,
    /// The level of goal config the campaign is using.
    #[prost(
        enumeration = "super::enums::goal_config_level_enum::GoalConfigLevel",
        tag = "3"
    )]
    pub goal_config_level: i32,
    /// The custom conversion goal the campaign is using for optimization.
    #[prost(string, tag = "4")]
    pub custom_conversion_goal: ::prost::alloc::string::String,
}
// Proto file describing the Conversion Value Rule resource.

/// A conversion value rule
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionValueRule {
    /// Immutable. The resource name of the conversion value rule.
    /// Conversion value rule resource names have the form:
    ///
    /// `customers/{customer_id}/conversionValueRules/{conversion_value_rule_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the conversion value rule.
    #[prost(int64, tag = "2")]
    pub id: i64,
    /// Action applied when the rule is triggered.
    #[prost(message, optional, tag = "3")]
    pub action: ::core::option::Option<conversion_value_rule::ValueRuleAction>,
    /// Condition for Geo location that must be satisfied for the value rule to
    /// apply.
    #[prost(message, optional, tag = "4")]
    pub geo_location_condition:
        ::core::option::Option<conversion_value_rule::ValueRuleGeoLocationCondition>,
    /// Condition for device type that must be satisfied for the value rule to
    /// apply.
    #[prost(message, optional, tag = "5")]
    pub device_condition: ::core::option::Option<conversion_value_rule::ValueRuleDeviceCondition>,
    /// Condition for audience that must be satisfied for the value rule to apply.
    #[prost(message, optional, tag = "6")]
    pub audience_condition:
        ::core::option::Option<conversion_value_rule::ValueRuleAudienceCondition>,
    /// Output only. The resource name of the conversion value rule's owner customer.
    /// When the value rule is inherited from a manager
    /// customer, owner_customer will be the resource name of the manager whereas
    /// the customer in the resource_name will be of the requesting serving
    /// customer.
    /// ** Read-only **
    #[prost(string, tag = "7")]
    pub owner_customer: ::prost::alloc::string::String,
    /// The status of the conversion value rule.
    #[prost(
        enumeration = "super::enums::conversion_value_rule_status_enum::ConversionValueRuleStatus",
        tag = "8"
    )]
    pub status: i32,
}
/// Nested message and enum types in `ConversionValueRule`.
pub mod conversion_value_rule {
    /// Action applied when rule is applied.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ValueRuleAction {
        /// Specifies applied operation.
        #[prost(
            enumeration = "super::super::enums::value_rule_operation_enum::ValueRuleOperation",
            tag = "1"
        )]
        pub operation: i32,
        /// Specifies applied value.
        #[prost(double, tag = "2")]
        pub value: f64,
    }
    /// Condition on Geo dimension.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ValueRuleGeoLocationCondition {
        /// Geo locations that advertisers want to exclude.
        #[prost(string, repeated, tag = "1")]
        pub excluded_geo_target_constants: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Excluded Geo location match type.
        #[prost(
            enumeration = "super::super::enums::value_rule_geo_location_match_type_enum::ValueRuleGeoLocationMatchType",
            tag = "2"
        )]
        pub excluded_geo_match_type: i32,
        /// Geo locations that advertisers want to include.
        #[prost(string, repeated, tag = "3")]
        pub geo_target_constants: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Included Geo location match type.
        #[prost(
            enumeration = "super::super::enums::value_rule_geo_location_match_type_enum::ValueRuleGeoLocationMatchType",
            tag = "4"
        )]
        pub geo_match_type: i32,
    }
    /// Condition on Device dimension.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ValueRuleDeviceCondition {
        /// Value for device type condition.
        #[prost(
            enumeration = "super::super::enums::value_rule_device_type_enum::ValueRuleDeviceType",
            repeated,
            tag = "1"
        )]
        pub device_types: ::prost::alloc::vec::Vec<i32>,
    }
    /// Condition on Audience dimension.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ValueRuleAudienceCondition {
        /// User Lists.
        #[prost(string, repeated, tag = "1")]
        pub user_lists: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// User Interests.
        #[prost(string, repeated, tag = "2")]
        pub user_interests: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
// Proto file describing the Conversion Value Rule Set resource.

/// A conversion value rule set
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionValueRuleSet {
    /// Immutable. The resource name of the conversion value rule set.
    /// Conversion value rule set resource names have the form:
    ///
    /// `customers/{customer_id}/conversionValueRuleSets/{conversion_value_rule_set_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the conversion value rule set.
    #[prost(int64, tag = "2")]
    pub id: i64,
    /// Resource names of rules within the rule set.
    #[prost(string, repeated, tag = "3")]
    pub conversion_value_rules: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Defines dimensions for Value Rule conditions. The condition types of value
    /// rules within this value rule set must be of these dimensions. The first
    /// entry in this list is the primary dimension of the included value rules.
    /// When using value rule primary dimension segmentation, conversion values
    /// will be segmented into the values adjusted by value rules and the original
    /// values, if some value rules apply.
    #[prost(
        enumeration = "super::enums::value_rule_set_dimension_enum::ValueRuleSetDimension",
        repeated,
        tag = "4"
    )]
    pub dimensions: ::prost::alloc::vec::Vec<i32>,
    /// Output only. The resource name of the conversion value rule set's owner customer.
    /// When the value rule set is inherited from a manager
    /// customer, owner_customer will be the resource name of the manager whereas
    /// the customer in the resource_name will be of the requesting serving
    /// customer.
    /// ** Read-only **
    #[prost(string, tag = "5")]
    pub owner_customer: ::prost::alloc::string::String,
    /// Immutable. Defines the scope where the conversion value rule set is attached.
    #[prost(
        enumeration = "super::enums::value_rule_set_attachment_type_enum::ValueRuleSetAttachmentType",
        tag = "6"
    )]
    pub attachment_type: i32,
    /// The resource name of the campaign when the conversion value rule
    /// set is attached to a campaign.
    #[prost(string, tag = "7")]
    pub campaign: ::prost::alloc::string::String,
    /// Output only. The status of the conversion value rule set.
    /// ** Read-only **
    #[prost(
        enumeration = "super::enums::conversion_value_rule_set_status_enum::ConversionValueRuleSetStatus",
        tag = "8"
    )]
    pub status: i32,
    /// Immutable. The conversion action categories of the conversion value rule set.
    #[prost(
        enumeration = "super::enums::conversion_action_category_enum::ConversionActionCategory",
        repeated,
        packed = "false",
        tag = "9"
    )]
    pub conversion_action_categories: ::prost::alloc::vec::Vec<i32>,
}
// Proto file describing the Currency Constant resource.

/// A currency constant.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrencyConstant {
    /// Output only. The resource name of the currency constant.
    /// Currency constant resource names have the form:
    ///
    /// `currencyConstants/{code}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. ISO 4217 three-letter currency code, e.g. "USD"
    #[prost(string, optional, tag = "6")]
    pub code: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Full English name of the currency.
    #[prost(string, optional, tag = "7")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Standard symbol for describing this currency, e.g. '$' for US Dollars.
    #[prost(string, optional, tag = "8")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The billable unit for this currency. Billed amounts should be multiples of
    /// this value.
    #[prost(int64, optional, tag = "9")]
    pub billable_unit_micros: ::core::option::Option<i64>,
}
// Proto file describing the Custom Audience resource.

/// A custom audience. This is a list of users by interest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAudience {
    /// Immutable. The resource name of the custom audience.
    /// Custom audience resource names have the form:
    ///
    /// `customers/{customer_id}/customAudiences/{custom_audience_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. ID of the custom audience.
    #[prost(int64, tag = "2")]
    pub id: i64,
    /// Output only. Status of this custom audience. Indicates whether the custom audience is
    /// enabled or removed.
    #[prost(
        enumeration = "super::enums::custom_audience_status_enum::CustomAudienceStatus",
        tag = "3"
    )]
    pub status: i32,
    /// Name of the custom audience. It should be unique for all custom audiences
    /// created by a customer.
    /// This field is required for creating operations.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// Type of the custom audience.
    /// ("INTEREST" OR "PURCHASE_INTENT" is not allowed for newly created custom
    /// audience but kept for existing audiences)
    #[prost(
        enumeration = "super::enums::custom_audience_type_enum::CustomAudienceType",
        tag = "5"
    )]
    pub r#type: i32,
    /// Description of this custom audience.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    /// List of custom audience members that this custom audience is composed of.
    /// Members can be added during CustomAudience creation. If members are
    /// presented in UPDATE operation, existing members will be overridden.
    #[prost(message, repeated, tag = "7")]
    pub members: ::prost::alloc::vec::Vec<CustomAudienceMember>,
}
/// A member of custom audience. A member can be a KEYWORD, URL,
/// PLACE_CATEGORY or APP. It can only be created or removed but not changed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAudienceMember {
    /// The type of custom audience member, KEYWORD, URL, PLACE_CATEGORY or APP.
    #[prost(
        enumeration = "super::enums::custom_audience_member_type_enum::CustomAudienceMemberType",
        tag = "1"
    )]
    pub member_type: i32,
    /// The CustomAudienceMember value. One field is populated depending on the
    /// member type.
    #[prost(oneof = "custom_audience_member::Value", tags = "2, 3, 4, 5")]
    pub value: ::core::option::Option<custom_audience_member::Value>,
}
/// Nested message and enum types in `CustomAudienceMember`.
pub mod custom_audience_member {
    /// The CustomAudienceMember value. One field is populated depending on the
    /// member type.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// A keyword or keyword phrase — at most 10 words and 80 characters.
        /// Languages with double-width characters such as Chinese, Japanese,
        /// or Korean, are allowed 40 characters, which describes the user's
        /// interests or actions.
        #[prost(string, tag = "2")]
        Keyword(::prost::alloc::string::String),
        /// An HTTP URL, protocol-included — at most 2048 characters, which includes
        /// contents users have interests in.
        #[prost(string, tag = "3")]
        Url(::prost::alloc::string::String),
        /// A place type described by a place category users visit.
        #[prost(int64, tag = "4")]
        PlaceCategory(i64),
        /// A package name of Android apps which users installed such as
        /// com.google.example.
        #[prost(string, tag = "5")]
        App(::prost::alloc::string::String),
    }
}
/// Custom conversion goal that can make arbitrary conversion actions biddable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomConversionGoal {
    /// Immutable. The resource name of the custom conversion goal.
    /// Custom conversion goal resource names have the form:
    ///
    /// `customers/{customer_id}/customConversionGoals/{goal_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The ID for this custom conversion goal.
    #[prost(int64, tag = "2")]
    pub id: i64,
    /// The name for this custom conversion goal.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Conversion actions that the custom conversion goal makes biddable.
    #[prost(string, repeated, tag = "4")]
    pub conversion_actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The status of the custom conversion goal.
    #[prost(
        enumeration = "super::enums::custom_conversion_goal_status_enum::CustomConversionGoalStatus",
        tag = "5"
    )]
    pub status: i32,
}
// Proto file describing the Custom Interest resource.

/// A custom interest. This is a list of users by interest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomInterest {
    /// Immutable. The resource name of the custom interest.
    /// Custom interest resource names have the form:
    ///
    /// `customers/{customer_id}/customInterests/{custom_interest_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. Id of the custom interest.
    #[prost(int64, optional, tag = "8")]
    pub id: ::core::option::Option<i64>,
    /// Status of this custom interest. Indicates whether the custom interest is
    /// enabled or removed.
    #[prost(
        enumeration = "super::enums::custom_interest_status_enum::CustomInterestStatus",
        tag = "3"
    )]
    pub status: i32,
    /// Name of the custom interest. It should be unique across the same custom
    /// affinity audience.
    /// This field is required for create operations.
    #[prost(string, optional, tag = "9")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Type of the custom interest, CUSTOM_AFFINITY or CUSTOM_INTENT.
    /// By default the type is set to CUSTOM_AFFINITY.
    #[prost(
        enumeration = "super::enums::custom_interest_type_enum::CustomInterestType",
        tag = "5"
    )]
    pub r#type: i32,
    /// Description of this custom interest audience.
    #[prost(string, optional, tag = "10")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// List of custom interest members that this custom interest is composed of.
    /// Members can be added during CustomInterest creation. If members are
    /// presented in UPDATE operation, existing members will be overridden.
    #[prost(message, repeated, tag = "7")]
    pub members: ::prost::alloc::vec::Vec<CustomInterestMember>,
}
/// A member of custom interest audience. A member can be a keyword or url.
/// It is immutable, that is, it can only be created or removed but not changed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomInterestMember {
    /// The type of custom interest member, KEYWORD or URL.
    #[prost(
        enumeration = "super::enums::custom_interest_member_type_enum::CustomInterestMemberType",
        tag = "1"
    )]
    pub member_type: i32,
    /// Keyword text when member_type is KEYWORD or URL string when
    /// member_type is URL.
    #[prost(string, optional, tag = "3")]
    pub parameter: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the Customer resource.

/// A customer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Customer {
    /// Immutable. The resource name of the customer.
    /// Customer resource names have the form:
    ///
    /// `customers/{customer_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the customer.
    #[prost(int64, optional, tag = "19")]
    pub id: ::core::option::Option<i64>,
    /// Optional, non-unique descriptive name of the customer.
    #[prost(string, optional, tag = "20")]
    pub descriptive_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The currency in which the account operates.
    /// A subset of the currency codes from the ISO 4217 standard is
    /// supported.
    #[prost(string, optional, tag = "21")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The local timezone ID of the customer.
    #[prost(string, optional, tag = "22")]
    pub time_zone: ::core::option::Option<::prost::alloc::string::String>,
    /// The URL template for constructing a tracking URL out of parameters.
    #[prost(string, optional, tag = "23")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
    /// The URL template for appending params to the final URL
    #[prost(string, optional, tag = "24")]
    pub final_url_suffix: ::core::option::Option<::prost::alloc::string::String>,
    /// Whether auto-tagging is enabled for the customer.
    #[prost(bool, optional, tag = "25")]
    pub auto_tagging_enabled: ::core::option::Option<bool>,
    /// Output only. Whether the Customer has a Partners program badge. If the Customer is not
    /// associated with the Partners program, this will be false. For more
    /// information, see <https://support.google.com/partners/answer/3125774.>
    #[prost(bool, optional, tag = "26")]
    pub has_partners_badge: ::core::option::Option<bool>,
    /// Output only. Whether the customer is a manager.
    #[prost(bool, optional, tag = "27")]
    pub manager: ::core::option::Option<bool>,
    /// Output only. Whether the customer is a test account.
    #[prost(bool, optional, tag = "28")]
    pub test_account: ::core::option::Option<bool>,
    /// Call reporting setting for a customer.
    #[prost(message, optional, tag = "10")]
    pub call_reporting_setting: ::core::option::Option<CallReportingSetting>,
    /// Output only. Conversion tracking setting for a customer.
    #[prost(message, optional, tag = "14")]
    pub conversion_tracking_setting: ::core::option::Option<ConversionTrackingSetting>,
    /// Output only. Remarketing setting for a customer.
    #[prost(message, optional, tag = "15")]
    pub remarketing_setting: ::core::option::Option<RemarketingSetting>,
    /// Output only. Reasons why the customer is not eligible to use PaymentMode.CONVERSIONS. If
    /// the list is empty, the customer is eligible. This field is read-only.
    #[prost(
        enumeration = "super::enums::customer_pay_per_conversion_eligibility_failure_reason_enum::CustomerPayPerConversionEligibilityFailureReason",
        repeated,
        packed = "false",
        tag = "16"
    )]
    pub pay_per_conversion_eligibility_failure_reasons: ::prost::alloc::vec::Vec<i32>,
    /// Output only. Optimization score of the customer.
    ///
    /// Optimization score is an estimate of how well a customer's campaigns are
    /// set to perform. It ranges from 0% (0.0) to 100% (1.0). This field is null
    /// for all manager customers, and for unscored non-manager customers.
    ///
    /// See "About optimization score" at
    /// <https://support.google.com/google-ads/answer/9061546.>
    ///
    /// This field is read-only.
    #[prost(double, optional, tag = "29")]
    pub optimization_score: ::core::option::Option<f64>,
    /// Output only. Optimization score weight of the customer.
    ///
    /// Optimization score weight can be used to compare/aggregate optimization
    /// scores across multiple non-manager customers. The aggregate optimization
    /// score of a manager is computed as the sum over all of their customers of
    /// `Customer.optimization_score * Customer.optimization_score_weight`. This
    /// field is 0 for all manager customers, and for unscored non-manager
    /// customers.
    ///
    /// This field is read-only.
    #[prost(double, tag = "30")]
    pub optimization_score_weight: f64,
    /// Output only. The status of the customer.
    #[prost(
        enumeration = "super::enums::customer_status_enum::CustomerStatus",
        tag = "36"
    )]
    pub status: i32,
}
/// Call reporting setting for a customer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallReportingSetting {
    /// Enable reporting of phone call events by redirecting them via Google
    /// System.
    #[prost(bool, optional, tag = "10")]
    pub call_reporting_enabled: ::core::option::Option<bool>,
    /// Whether to enable call conversion reporting.
    #[prost(bool, optional, tag = "11")]
    pub call_conversion_reporting_enabled: ::core::option::Option<bool>,
    /// Customer-level call conversion action to attribute a call conversion to.
    /// If not set a default conversion action is used. Only in effect when
    /// call_conversion_reporting_enabled is set to true.
    #[prost(string, optional, tag = "12")]
    pub call_conversion_action: ::core::option::Option<::prost::alloc::string::String>,
}
/// A collection of customer-wide settings related to Google Ads Conversion
/// Tracking.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionTrackingSetting {
    /// Output only. The conversion tracking id used for this account. This id is automatically
    /// assigned after any conversion tracking feature is used. If the customer
    /// doesn't use conversion tracking, this is 0. This field is read-only.
    #[prost(int64, optional, tag = "3")]
    pub conversion_tracking_id: ::core::option::Option<i64>,
    /// Output only. The conversion tracking id of the customer's manager. This is set when the
    /// customer is opted into cross account conversion tracking, and it overrides
    /// conversion_tracking_id. This field can only be managed through the Google
    /// Ads UI. This field is read-only.
    #[prost(int64, optional, tag = "4")]
    pub cross_account_conversion_tracking_id: ::core::option::Option<i64>,
    /// Output only. Whether the customer has accepted customer data terms. If using
    /// cross-account conversion tracking, this value is inherited from the
    /// manager. This field is read-only. For more
    /// information, see <https://support.google.com/adspolicy/answer/7475709.>
    #[prost(bool, tag = "5")]
    pub accepted_customer_data_terms: bool,
    /// Output only. Conversion tracking status. It indicates whether the customer is using
    /// conversion tracking, and who is the conversion tracking owner of this
    /// customer. If this customer is using cross-account conversion tracking,
    /// the value returned will differ based on the `login-customer-id` of the
    /// request.
    #[prost(
        enumeration = "super::enums::conversion_tracking_status_enum::ConversionTrackingStatus",
        tag = "6"
    )]
    pub conversion_tracking_status: i32,
    /// Output only. Whether the customer is opted-in for enhanced conversions
    /// for leads. If using cross-account conversion tracking, this value is
    /// inherited from the manager. This field is read-only.
    #[prost(bool, tag = "7")]
    pub enhanced_conversions_for_leads_enabled: bool,
    /// Output only. The resource name of the customer where conversions are created and
    /// managed. This field is read-only.
    #[prost(string, tag = "8")]
    pub google_ads_conversion_customer: ::prost::alloc::string::String,
}
/// Remarketing setting for a customer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemarketingSetting {
    /// Output only. The Google tag.
    #[prost(string, optional, tag = "2")]
    pub google_global_site_tag: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the CustomerClient resource.

/// A link between the given customer and a client customer. CustomerClients only
/// exist for manager customers. All direct and indirect client customers are
/// included, as well as the manager itself.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerClient {
    /// Output only. The resource name of the customer client.
    /// CustomerClient resource names have the form:
    /// `customers/{customer_id}/customerClients/{client_customer_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The resource name of the client-customer which is linked to
    /// the given customer. Read only.
    #[prost(string, optional, tag = "12")]
    pub client_customer: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Specifies whether this is a
    /// [hidden account](<https://support.google.com/google-ads/answer/7519830>).
    /// Read only.
    #[prost(bool, optional, tag = "13")]
    pub hidden: ::core::option::Option<bool>,
    /// Output only. Distance between given customer and client. For self link, the level value
    /// will be 0. Read only.
    #[prost(int64, optional, tag = "14")]
    pub level: ::core::option::Option<i64>,
    /// Output only. Common Locale Data Repository (CLDR) string representation of the
    /// time zone of the client, e.g. America/Los_Angeles. Read only.
    #[prost(string, optional, tag = "15")]
    pub time_zone: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Identifies if the client is a test account. Read only.
    #[prost(bool, optional, tag = "16")]
    pub test_account: ::core::option::Option<bool>,
    /// Output only. Identifies if the client is a manager. Read only.
    #[prost(bool, optional, tag = "17")]
    pub manager: ::core::option::Option<bool>,
    /// Output only. Descriptive name for the client. Read only.
    #[prost(string, optional, tag = "18")]
    pub descriptive_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Currency code (e.g. 'USD', 'EUR') for the client. Read only.
    #[prost(string, optional, tag = "19")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The ID of the client customer. Read only.
    #[prost(int64, optional, tag = "20")]
    pub id: ::core::option::Option<i64>,
    /// Output only. The resource names of the labels owned by the requesting customer that are
    /// applied to the client customer.
    /// Label resource names have the form:
    ///
    /// `customers/{customer_id}/labels/{label_id}`
    #[prost(string, repeated, tag = "21")]
    pub applied_labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The status of the client customer. Read only.
    #[prost(
        enumeration = "super::enums::customer_status_enum::CustomerStatus",
        tag = "22"
    )]
    pub status: i32,
}
// Proto file describing the CustomerClientLink resource.

/// Represents customer client link relationship.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerClientLink {
    /// Immutable. Name of the resource.
    /// CustomerClientLink resource names have the form:
    /// `customers/{customer_id}/customerClientLinks/{client_customer_id}~{manager_link_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The client customer linked to this customer.
    #[prost(string, optional, tag = "7")]
    pub client_customer: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. This is uniquely identifies a customer client link. Read only.
    #[prost(int64, optional, tag = "8")]
    pub manager_link_id: ::core::option::Option<i64>,
    /// This is the status of the link between client and manager.
    #[prost(
        enumeration = "super::enums::manager_link_status_enum::ManagerLinkStatus",
        tag = "5"
    )]
    pub status: i32,
    /// The visibility of the link. Users can choose whether or not to see hidden
    /// links in the Google Ads UI.
    /// Default value is false
    #[prost(bool, optional, tag = "9")]
    pub hidden: ::core::option::Option<bool>,
}
/// Biddability control for conversion actions with a matching category and
/// origin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerConversionGoal {
    /// Immutable. The resource name of the customer conversion goal.
    /// Customer conversion goal resource names have the form:
    ///
    /// `customers/{customer_id}/customerConversionGoals/{category}~{origin}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The conversion category of this customer conversion goal. Only
    /// conversion actions that have this category will be included in this goal.
    #[prost(
        enumeration = "super::enums::conversion_action_category_enum::ConversionActionCategory",
        tag = "2"
    )]
    pub category: i32,
    /// The conversion origin of this customer conversion goal. Only
    /// conversion actions that have this conversion origin will be included in
    /// this goal.
    #[prost(
        enumeration = "super::enums::conversion_origin_enum::ConversionOrigin",
        tag = "3"
    )]
    pub origin: i32,
    /// The biddability of the customer conversion goal.
    #[prost(bool, tag = "4")]
    pub biddable: bool,
}
/// A customizer value for the associated CustomizerAttribute at the Customer
/// level.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerCustomizer {
    /// Immutable. The resource name of the customer customizer.
    /// Customer customizer resource names have the form:
    ///
    /// `customers/{customer_id}/customerCustomizers/{customizer_attribute_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Required. Immutable. The customizer attribute which is linked to the customer.
    #[prost(string, tag = "2")]
    pub customizer_attribute: ::prost::alloc::string::String,
    /// Output only. The status of the customer customizer attribute.
    #[prost(
        enumeration = "super::enums::customizer_value_status_enum::CustomizerValueStatus",
        tag = "3"
    )]
    pub status: i32,
    /// Required. The value to associate with the customizer attribute at this level. The
    /// value must be of the type specified for the CustomizerAttribute.
    #[prost(message, optional, tag = "4")]
    pub value: ::core::option::Option<super::common::CustomizerValue>,
}
// Proto file describing the CustomerExtensionSetting resource.

/// A customer extension setting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerExtensionSetting {
    /// Immutable. The resource name of the customer extension setting.
    /// CustomerExtensionSetting resource names have the form:
    ///
    /// `customers/{customer_id}/customerExtensionSettings/{extension_type}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The extension type of the customer extension setting.
    #[prost(
        enumeration = "super::enums::extension_type_enum::ExtensionType",
        tag = "2"
    )]
    pub extension_type: i32,
    /// The resource names of the extension feed items to serve under the customer.
    /// ExtensionFeedItem resource names have the form:
    ///
    /// `customers/{customer_id}/extensionFeedItems/{feed_item_id}`
    #[prost(string, repeated, tag = "5")]
    pub extension_feed_items: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The device for which the extensions will serve. Optional.
    #[prost(
        enumeration = "super::enums::extension_setting_device_enum::ExtensionSettingDevice",
        tag = "4"
    )]
    pub device: i32,
}
// Proto file describing the CustomerFeed resource.

/// A customer feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerFeed {
    /// Immutable. The resource name of the customer feed.
    /// Customer feed resource names have the form:
    ///
    /// `customers/{customer_id}/customerFeeds/{feed_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The feed being linked to the customer.
    #[prost(string, optional, tag = "6")]
    pub feed: ::core::option::Option<::prost::alloc::string::String>,
    /// Indicates which placeholder types the feed may populate under the connected
    /// customer. Required.
    #[prost(
        enumeration = "super::enums::placeholder_type_enum::PlaceholderType",
        repeated,
        tag = "3"
    )]
    pub placeholder_types: ::prost::alloc::vec::Vec<i32>,
    /// Matching function associated with the CustomerFeed.
    /// The matching function is used to filter the set of feed items selected.
    /// Required.
    #[prost(message, optional, tag = "4")]
    pub matching_function: ::core::option::Option<super::common::MatchingFunction>,
    /// Output only. Status of the customer feed.
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::feed_link_status_enum::FeedLinkStatus",
        tag = "5"
    )]
    pub status: i32,
}
// Proto file describing the customer label resource.

/// Represents a relationship between a customer and a label. This customer may
/// not have access to all the labels attached to it. Additional CustomerLabels
/// may be returned by increasing permissions with login-customer-id.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerLabel {
    /// Immutable. Name of the resource.
    /// Customer label resource names have the form:
    /// `customers/{customer_id}/customerLabels/{label_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The resource name of the customer to which the label is attached.
    /// Read only.
    #[prost(string, optional, tag = "4")]
    pub customer: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The resource name of the label assigned to the customer.
    ///
    /// Note: the Customer ID portion of the label resource name is not
    /// validated when creating a new CustomerLabel.
    #[prost(string, optional, tag = "5")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the CustomerManagerLink resource.

/// Represents customer-manager link relationship.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerManagerLink {
    /// Immutable. Name of the resource.
    /// CustomerManagerLink resource names have the form:
    /// `customers/{customer_id}/customerManagerLinks/{manager_customer_id}~{manager_link_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The manager customer linked to the customer.
    #[prost(string, optional, tag = "6")]
    pub manager_customer: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. ID of the customer-manager link. This field is read only.
    #[prost(int64, optional, tag = "7")]
    pub manager_link_id: ::core::option::Option<i64>,
    /// Status of the link between the customer and the manager.
    #[prost(
        enumeration = "super::enums::manager_link_status_enum::ManagerLinkStatus",
        tag = "5"
    )]
    pub status: i32,
}
// Proto file describing the Customer Negative Criterion resource.

/// A negative criterion for exclusions at the customer level.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerNegativeCriterion {
    /// Immutable. The resource name of the customer negative criterion.
    /// Customer negative criterion resource names have the form:
    ///
    /// `customers/{customer_id}/customerNegativeCriteria/{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the criterion.
    #[prost(int64, optional, tag = "10")]
    pub id: ::core::option::Option<i64>,
    /// Output only. The type of the criterion.
    #[prost(
        enumeration = "super::enums::criterion_type_enum::CriterionType",
        tag = "3"
    )]
    pub r#type: i32,
    /// The customer negative criterion.
    ///
    /// Exactly one must be set.
    #[prost(
        oneof = "customer_negative_criterion::Criterion",
        tags = "4, 5, 6, 7, 8, 9"
    )]
    pub criterion: ::core::option::Option<customer_negative_criterion::Criterion>,
}
/// Nested message and enum types in `CustomerNegativeCriterion`.
pub mod customer_negative_criterion {
    /// The customer negative criterion.
    ///
    /// Exactly one must be set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Criterion {
        /// Immutable. ContentLabel.
        #[prost(message, tag = "4")]
        ContentLabel(super::super::common::ContentLabelInfo),
        /// Immutable. MobileApplication.
        #[prost(message, tag = "5")]
        MobileApplication(super::super::common::MobileApplicationInfo),
        /// Immutable. MobileAppCategory.
        #[prost(message, tag = "6")]
        MobileAppCategory(super::super::common::MobileAppCategoryInfo),
        /// Immutable. Placement.
        #[prost(message, tag = "7")]
        Placement(super::super::common::PlacementInfo),
        /// Immutable. YouTube Video.
        #[prost(message, tag = "8")]
        YoutubeVideo(super::super::common::YouTubeVideoInfo),
        /// Immutable. YouTube Channel.
        #[prost(message, tag = "9")]
        YoutubeChannel(super::super::common::YouTubeChannelInfo),
    }
}
// Proto file describing the CustomerUserAccess resource.

/// Represents the permission of a single user onto a single customer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerUserAccess {
    /// Immutable. Name of the resource.
    /// Resource names have the form:
    /// `customers/{customer_id}/customerUserAccesses/{user_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. User id of the user with the customer access.
    /// Read only field
    #[prost(int64, tag = "2")]
    pub user_id: i64,
    /// Output only. Email address of the user.
    /// Read only field
    #[prost(string, optional, tag = "3")]
    pub email_address: ::core::option::Option<::prost::alloc::string::String>,
    /// Access role of the user.
    #[prost(enumeration = "super::enums::access_role_enum::AccessRole", tag = "4")]
    pub access_role: i32,
    /// Output only. The customer user access creation time.
    /// Read only field
    /// The format is "YYYY-MM-DD HH:MM:SS".
    /// Examples: "2018-03-05 09:15:00" or "2018-02-01 14:34:30"
    #[prost(string, optional, tag = "6")]
    pub access_creation_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The email address of the inviter user.
    /// Read only field
    #[prost(string, optional, tag = "7")]
    pub inviter_user_email_address: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the CustomerUserAccessInvitation resource.

/// Represent an invitation to a new user on this customer account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerUserAccessInvitation {
    /// Immutable. Name of the resource.
    /// Resource names have the form:
    /// `customers/{customer_id}/customerUserAccessInvitations/{invitation_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the invitation.
    /// This field is read-only.
    #[prost(int64, tag = "2")]
    pub invitation_id: i64,
    /// Immutable. Access role of the user.
    #[prost(enumeration = "super::enums::access_role_enum::AccessRole", tag = "3")]
    pub access_role: i32,
    /// Immutable. Email address the invitation was sent to.
    /// This can differ from the email address of the account
    /// that accepts the invite.
    #[prost(string, tag = "4")]
    pub email_address: ::prost::alloc::string::String,
    /// Output only. Time invitation was created.
    /// This field is read-only.
    /// The format is "YYYY-MM-DD HH:MM:SS".
    /// Examples: "2018-03-05 09:15:00" or "2018-02-01 14:34:30"
    #[prost(string, tag = "5")]
    pub creation_date_time: ::prost::alloc::string::String,
    /// Output only. Invitation status of the user.
    #[prost(
        enumeration = "super::enums::access_invitation_status_enum::AccessInvitationStatus",
        tag = "6"
    )]
    pub invitation_status: i32,
}
/// A customizer attribute.
/// Use CustomerCustomizer, CampaignCustomizer, AdGroupCustomizer, or
/// AdGroupCriterionCustomizer to associate a customizer attribute and
/// set its value at the customer, campaign, ad group, or ad group criterion
/// level, respectively.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomizerAttribute {
    /// Immutable. The resource name of the customizer attribute.
    /// Customizer Attribute resource names have the form:
    ///
    /// `customers/{customer_id}/customizerAttributes/{customizer_attribute_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the customizer attribute.
    #[prost(int64, tag = "2")]
    pub id: i64,
    /// Required. Immutable. Name of the customizer attribute. Required. It must have a minimum length
    /// of 1 and maximum length of 40. Name of an enabled customizer attribute must
    /// be unique (case insensitive).
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The type of the customizer attribute.
    #[prost(
        enumeration = "super::enums::customizer_attribute_type_enum::CustomizerAttributeType",
        tag = "4"
    )]
    pub r#type: i32,
    /// Output only. The status of the customizer attribute.
    #[prost(
        enumeration = "super::enums::customizer_attribute_status_enum::CustomizerAttributeStatus",
        tag = "5"
    )]
    pub status: i32,
}
// Proto file describing the detail placement view resource.

/// A view with metrics aggregated by ad group and URL or YouTube video.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetailPlacementView {
    /// Output only. The resource name of the detail placement view.
    /// Detail placement view resource names have the form:
    ///
    /// `customers/{customer_id}/detailPlacementViews/{ad_group_id}~{base64_placement}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The automatic placement string at detail level, e. g. website URL, mobile
    /// application ID, or a YouTube video ID.
    #[prost(string, optional, tag = "7")]
    pub placement: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The display name is URL name for websites, YouTube video name for YouTube
    /// videos, and translated mobile app name for mobile apps.
    #[prost(string, optional, tag = "8")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. URL of the group placement, e.g. domain, link to the mobile application in
    /// app store, or a YouTube channel URL.
    #[prost(string, optional, tag = "9")]
    pub group_placement_target_url: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. URL of the placement, e.g. website, link to the mobile application in app
    /// store, or a YouTube video URL.
    #[prost(string, optional, tag = "10")]
    pub target_url: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Type of the placement, e.g. Website, YouTube Video, and Mobile Application.
    #[prost(
        enumeration = "super::enums::placement_type_enum::PlacementType",
        tag = "6"
    )]
    pub placement_type: i32,
}
// Proto file describing the Detailed Demographic resource.

/// A detailed demographic: a particular interest-based vertical to be targeted
/// to reach users based on long-term life facts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetailedDemographic {
    /// Output only. The resource name of the detailed demographic.
    /// Detailed demographic resource names have the form:
    ///
    /// `customers/{customer_id}/detailedDemographics/{detailed_demographic_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the detailed demographic.
    #[prost(int64, tag = "2")]
    pub id: i64,
    /// Output only. The name of the detailed demographic. E.g."Highest Level of Educational
    /// Attainment"
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The parent of the detailed_demographic.
    #[prost(string, tag = "4")]
    pub parent: ::prost::alloc::string::String,
    /// Output only. True if the detailed demographic is launched to all channels and locales.
    #[prost(bool, tag = "5")]
    pub launched_to_all: bool,
    /// Output only. Availability information of the detailed demographic.
    #[prost(message, repeated, tag = "6")]
    pub availabilities: ::prost::alloc::vec::Vec<super::common::CriterionCategoryAvailability>,
}
// Proto file describing the display keyword view resource.

/// A display keyword view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayKeywordView {
    /// Output only. The resource name of the display keyword view.
    /// Display Keyword view resource names have the form:
    ///
    /// `customers/{customer_id}/displayKeywordViews/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
// Proto file describing the DistanceView resource.

/// A distance view with metrics aggregated by the user's distance from an
/// advertiser's location extensions. Each DistanceBucket includes all
/// impressions that fall within its distance and a single impression will
/// contribute to the metrics for all DistanceBuckets that include the user's
/// distance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistanceView {
    /// Output only. The resource name of the distance view.
    /// Distance view resource names have the form:
    ///
    /// `customers/{customer_id}/distanceViews/1~{distance_bucket}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. Grouping of user distance from location extensions.
    #[prost(
        enumeration = "super::enums::distance_bucket_enum::DistanceBucket",
        tag = "2"
    )]
    pub distance_bucket: i32,
    /// Output only. True if the DistanceBucket is using the metric system, false otherwise.
    #[prost(bool, optional, tag = "4")]
    pub metric_system: ::core::option::Option<bool>,
}
// Proto file describing the Domain Category resource.

/// A category generated automatically by crawling a domain. If a campaign uses
/// the DynamicSearchAdsSetting, then domain categories will be generated for
/// the domain. The categories can be targeted using WebpageConditionInfo.
/// See: <https://support.google.com/google-ads/answer/2471185>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DomainCategory {
    /// Output only. The resource name of the domain category.
    /// Domain category resource names have the form:
    ///
    /// `customers/{customer_id}/domainCategories/{campaign_id}~{category_base64}~{language_code}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The campaign this category is recommended for.
    #[prost(string, optional, tag = "10")]
    pub campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Recommended category for the website domain. e.g. if you have a website
    /// about electronics, the categories could be "cameras", "televisions", etc.
    #[prost(string, optional, tag = "11")]
    pub category: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The language code specifying the language of the website. e.g. "en" for
    /// English. The language can be specified in the DynamicSearchAdsSetting
    /// required for dynamic search ads. This is the language of the pages from
    /// your website that you want Google Ads to find, create ads for,
    /// and match searches with.
    #[prost(string, optional, tag = "12")]
    pub language_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The domain for the website. The domain can be specified in the
    /// DynamicSearchAdsSetting required for dynamic search ads.
    #[prost(string, optional, tag = "13")]
    pub domain: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Fraction of pages on your site that this category matches.
    #[prost(double, optional, tag = "14")]
    pub coverage_fraction: ::core::option::Option<f64>,
    /// Output only. The position of this category in the set of categories. Lower numbers
    /// indicate a better match for the domain. null indicates not recommended.
    #[prost(int64, optional, tag = "15")]
    pub category_rank: ::core::option::Option<i64>,
    /// Output only. Indicates whether this category has sub-categories.
    #[prost(bool, optional, tag = "16")]
    pub has_children: ::core::option::Option<bool>,
    /// Output only. The recommended cost per click for the category.
    #[prost(int64, optional, tag = "17")]
    pub recommended_cpc_bid_micros: ::core::option::Option<i64>,
}
// Proto file describing the Dynamic Search Ads Search Term View resource.

/// A dynamic search ads search term view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicSearchAdsSearchTermView {
    /// Output only. The resource name of the dynamic search ads search term view.
    /// Dynamic search ads search term view resource names have the form:
    ///
    /// `customers/{customer_id}/dynamicSearchAdsSearchTermViews/{ad_group_id}~{search_term_fingerprint}~{headline_fingerprint}~{landing_page_fingerprint}~{page_url_fingerprint}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. Search term
    ///
    /// This field is read-only.
    #[prost(string, optional, tag = "9")]
    pub search_term: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The dynamically generated headline of the Dynamic Search Ad.
    ///
    /// This field is read-only.
    #[prost(string, optional, tag = "10")]
    pub headline: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The dynamically selected landing page URL of the impression.
    ///
    /// This field is read-only.
    #[prost(string, optional, tag = "11")]
    pub landing_page: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The URL of page feed item served for the impression.
    ///
    /// This field is read-only.
    #[prost(string, optional, tag = "12")]
    pub page_url: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. True if query matches a negative keyword.
    ///
    /// This field is read-only.
    #[prost(bool, optional, tag = "13")]
    pub has_negative_keyword: ::core::option::Option<bool>,
    /// Output only. True if query is added to targeted keywords.
    ///
    /// This field is read-only.
    #[prost(bool, optional, tag = "14")]
    pub has_matching_keyword: ::core::option::Option<bool>,
    /// Output only. True if query matches a negative url.
    ///
    /// This field is read-only.
    #[prost(bool, optional, tag = "15")]
    pub has_negative_url: ::core::option::Option<bool>,
}
// Proto file describing the expanded landing page view resource.

/// A landing page view with metrics aggregated at the expanded final URL
/// level.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandedLandingPageView {
    /// Output only. The resource name of the expanded landing page view.
    /// Expanded landing page view resource names have the form:
    ///
    /// `customers/{customer_id}/expandedLandingPageViews/{expanded_final_url_fingerprint}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The final URL that clicks are directed to.
    #[prost(string, optional, tag = "3")]
    pub expanded_final_url: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the Experiment resource.

/// A Google ads experiment for users to experiment changes on multiple
/// campaigns, compare the performance, and apply the effective changes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Experiment {
    /// Immutable. The resource name of the experiment.
    /// Experiment resource names have the form:
    ///
    /// `customers/{customer_id}/experiments/{experiment_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the experiment. Read only.
    #[prost(int64, optional, tag = "9")]
    pub experiment_id: ::core::option::Option<i64>,
    /// Required. The name of the experiment. It must have a minimum length of 1 and
    /// maximum length of 1024. It must be unique under a customer.
    #[prost(string, tag = "10")]
    pub name: ::prost::alloc::string::String,
    /// The description of the experiment. It must have a minimum length of 1 and
    /// maximum length of 2048.
    #[prost(string, tag = "11")]
    pub description: ::prost::alloc::string::String,
    /// For system managed experiments, the advertiser must provide a suffix during
    /// construction, in the setup stage before moving to initiated. The suffix
    /// will be appended to the in-design and experiment campaign names so that the
    /// name is base campaign name + suffix.
    #[prost(string, tag = "12")]
    pub suffix: ::prost::alloc::string::String,
    /// The product/feature that uses this experiment.
    #[prost(
        enumeration = "super::enums::experiment_type_enum::ExperimentType",
        tag = "13"
    )]
    pub r#type: i32,
    /// The Advertiser-desired status of this experiment.
    #[prost(
        enumeration = "super::enums::experiment_status_enum::ExperimentStatus",
        tag = "14"
    )]
    pub status: i32,
    /// Date when the experiment starts. By default, the experiment starts
    /// now or on the campaign's start date, whichever is later. If this field is
    /// set, then the experiment starts at the beginning of the specified date in
    /// the customer's time zone.
    ///
    /// Format: YYYY-MM-DD
    /// Example: 2019-03-14
    #[prost(string, optional, tag = "15")]
    pub start_date: ::core::option::Option<::prost::alloc::string::String>,
    /// Date when the experiment ends. By default, the experiment ends on
    /// the campaign's end date. If this field is set, then the experiment ends at
    /// the end of the specified date in the customer's time zone.
    ///
    /// Format: YYYY-MM-DD
    /// Example: 2019-04-18
    #[prost(string, optional, tag = "16")]
    pub end_date: ::core::option::Option<::prost::alloc::string::String>,
    /// The goals of this experiment.
    #[prost(message, repeated, tag = "17")]
    pub goals: ::prost::alloc::vec::Vec<super::common::MetricGoal>,
    /// Output only. The resource name of the long-running operation that can be used to poll
    /// for completion of experiment schedule or promote. The most recent long
    /// running operation is returned.
    #[prost(string, optional, tag = "18")]
    pub long_running_operation: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The status of the experiment promotion process.
    #[prost(
        enumeration = "super::enums::async_action_status_enum::AsyncActionStatus",
        tag = "19"
    )]
    pub promote_status: i32,
}
// Proto file describing the Experiment arm resource.

/// A Google ads experiment for users to experiment changes on multiple
/// campaigns, compare the performance, and apply the effective changes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExperimentArm {
    /// Immutable. The resource name of the experiment arm.
    /// Experiment arm resource names have the form:
    ///
    /// `customers/{customer_id}/experimentArms/{TrialArm.trial_id}~{TrialArm.trial_arm_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The experiment to which the ExperimentArm belongs.
    #[prost(string, tag = "2")]
    pub trial: ::prost::alloc::string::String,
    /// Required. The name of the experiment arm. It must have a minimum length of 1 and
    /// maximum length of 1024. It must be unique under an experiment.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Whether this arm is a control arm. A control arm is the arm against
    /// which the other arms are compared.
    #[prost(bool, tag = "4")]
    pub control: bool,
    /// Traffic split of the trial arm. The value should be between 1 and 100
    /// and must total 100 between the two trial arms.
    #[prost(int64, tag = "5")]
    pub traffic_split: i64,
    /// List of campaigns in the trial arm. The max length is one.
    #[prost(string, repeated, tag = "6")]
    pub campaigns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The in design campaigns in the treatment experiment arm.
    #[prost(string, repeated, tag = "7")]
    pub in_design_campaigns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
// Proto file describing the ExtensionFeedItem resource.

/// An extension feed item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionFeedItem {
    /// Immutable. The resource name of the extension feed item.
    /// Extension feed item resource names have the form:
    ///
    /// `customers/{customer_id}/extensionFeedItems/{feed_item_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of this feed item. Read-only.
    #[prost(int64, optional, tag = "25")]
    pub id: ::core::option::Option<i64>,
    /// Output only. The extension type of the extension feed item.
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::extension_type_enum::ExtensionType",
        tag = "13"
    )]
    pub extension_type: i32,
    /// Start time in which this feed item is effective and can begin serving. The
    /// time is in the customer's time zone.
    /// The format is "YYYY-MM-DD HH:MM:SS".
    /// Examples: "2018-03-05 09:15:00" or "2018-02-01 14:34:30"
    #[prost(string, optional, tag = "26")]
    pub start_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// End time in which this feed item is no longer effective and will stop
    /// serving. The time is in the customer's time zone.
    /// The format is "YYYY-MM-DD HH:MM:SS".
    /// Examples: "2018-03-05 09:15:00" or "2018-02-01 14:34:30"
    #[prost(string, optional, tag = "27")]
    pub end_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// List of non-overlapping schedules specifying all time intervals
    /// for which the feed item may serve. There can be a maximum of 6 schedules
    /// per day.
    #[prost(message, repeated, tag = "16")]
    pub ad_schedules: ::prost::alloc::vec::Vec<super::common::AdScheduleInfo>,
    /// The targeted device.
    #[prost(
        enumeration = "super::enums::feed_item_target_device_enum::FeedItemTargetDevice",
        tag = "17"
    )]
    pub device: i32,
    /// The targeted geo target constant.
    #[prost(string, optional, tag = "30")]
    pub targeted_geo_target_constant: ::core::option::Option<::prost::alloc::string::String>,
    /// The targeted keyword.
    #[prost(message, optional, tag = "22")]
    pub targeted_keyword: ::core::option::Option<super::common::KeywordInfo>,
    /// Output only. Status of the feed item.
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::feed_item_status_enum::FeedItemStatus",
        tag = "4"
    )]
    pub status: i32,
    /// Extension type.
    #[prost(
        oneof = "extension_feed_item::Extension",
        tags = "2, 3, 7, 8, 9, 10, 11, 12, 14, 15, 23, 31"
    )]
    pub extension: ::core::option::Option<extension_feed_item::Extension>,
    /// Targeting at either the campaign or ad group level. Feed items that target
    /// a campaign or ad group will only serve with that resource.
    #[prost(
        oneof = "extension_feed_item::ServingResourceTargeting",
        tags = "28, 29"
    )]
    pub serving_resource_targeting:
        ::core::option::Option<extension_feed_item::ServingResourceTargeting>,
}
/// Nested message and enum types in `ExtensionFeedItem`.
pub mod extension_feed_item {
    /// Extension type.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Extension {
        /// Sitelink extension.
        #[prost(message, tag = "2")]
        SitelinkFeedItem(super::super::common::SitelinkFeedItem),
        /// Structured snippet extension.
        #[prost(message, tag = "3")]
        StructuredSnippetFeedItem(super::super::common::StructuredSnippetFeedItem),
        /// App extension.
        #[prost(message, tag = "7")]
        AppFeedItem(super::super::common::AppFeedItem),
        /// Call extension.
        #[prost(message, tag = "8")]
        CallFeedItem(super::super::common::CallFeedItem),
        /// Callout extension.
        #[prost(message, tag = "9")]
        CalloutFeedItem(super::super::common::CalloutFeedItem),
        /// Text message extension.
        #[prost(message, tag = "10")]
        TextMessageFeedItem(super::super::common::TextMessageFeedItem),
        /// Price extension.
        #[prost(message, tag = "11")]
        PriceFeedItem(super::super::common::PriceFeedItem),
        /// Promotion extension.
        #[prost(message, tag = "12")]
        PromotionFeedItem(super::super::common::PromotionFeedItem),
        /// Output only. Location extension. Locations are synced from a Business Profile into a
        /// feed. This field is read-only.
        #[prost(message, tag = "14")]
        LocationFeedItem(super::super::common::LocationFeedItem),
        /// Output only. Affiliate location extension. Feed locations are populated by Google Ads
        /// based on a chain ID.
        /// This field is read-only.
        #[prost(message, tag = "15")]
        AffiliateLocationFeedItem(super::super::common::AffiliateLocationFeedItem),
        /// Hotel Callout extension.
        #[prost(message, tag = "23")]
        HotelCalloutFeedItem(super::super::common::HotelCalloutFeedItem),
        /// Immutable. Advertiser provided image extension.
        #[prost(message, tag = "31")]
        ImageFeedItem(super::super::common::ImageFeedItem),
    }
    /// Targeting at either the campaign or ad group level. Feed items that target
    /// a campaign or ad group will only serve with that resource.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ServingResourceTargeting {
        /// The targeted campaign.
        #[prost(string, tag = "28")]
        TargetedCampaign(::prost::alloc::string::String),
        /// The targeted ad group.
        #[prost(string, tag = "29")]
        TargetedAdGroup(::prost::alloc::string::String),
    }
}
/// Represents a set of feed items. The set can be used and shared among certain
/// feed item features. For instance, the set can be referenced within the
/// matching functions of CustomerFeed, CampaignFeed, and AdGroupFeed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemSet {
    /// Immutable. The resource name of the feed item set.
    /// Feed item set resource names have the form:
    /// `customers/{customer_id}/feedItemSets/{feed_id}~{feed_item_set_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The resource name of the feed containing the feed items in the set.
    /// Immutable. Required.
    #[prost(string, tag = "2")]
    pub feed: ::prost::alloc::string::String,
    /// Output only. ID of the set.
    #[prost(int64, tag = "3")]
    pub feed_item_set_id: i64,
    /// Name of the set. Must be unique within the account.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Status of the feed item set.
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::feed_item_set_status_enum::FeedItemSetStatus",
        tag = "8"
    )]
    pub status: i32,
    /// Represents a filter on locations in a feed item set.
    /// Only applicable if the parent Feed of the FeedItemSet is a LOCATION feed.
    #[prost(oneof = "feed_item_set::DynamicSetFilter", tags = "5, 6")]
    pub dynamic_set_filter: ::core::option::Option<feed_item_set::DynamicSetFilter>,
}
/// Nested message and enum types in `FeedItemSet`.
pub mod feed_item_set {
    /// Represents a filter on locations in a feed item set.
    /// Only applicable if the parent Feed of the FeedItemSet is a LOCATION feed.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DynamicSetFilter {
        /// Filter for dynamic location set.
        /// It is only used for sets of locations.
        #[prost(message, tag = "5")]
        DynamicLocationSetFilter(super::super::common::DynamicLocationSetFilter),
        /// Filter for dynamic affiliate location set.
        /// This field doesn't apply generally to feed item sets. It is only used for
        /// sets of affiliate locations.
        #[prost(message, tag = "6")]
        DynamicAffiliateLocationSetFilter(super::super::common::DynamicAffiliateLocationSetFilter),
    }
}
// Proto file describing the FeedItemSetLink resource.

/// Represents a link between a FeedItem and a FeedItemSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemSetLink {
    /// Immutable. The resource name of the feed item set link.
    /// Feed item set link resource names have the form:
    /// `customers/{customer_id}/feedItemSetLinks/{feed_id}~{feed_item_set_id}~{feed_item_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The linked FeedItem.
    #[prost(string, tag = "2")]
    pub feed_item: ::prost::alloc::string::String,
    /// Immutable. The linked FeedItemSet.
    #[prost(string, tag = "3")]
    pub feed_item_set: ::prost::alloc::string::String,
}
// Proto file describing the FeedItemTarget resource.

/// A feed item target.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemTarget {
    /// Immutable. The resource name of the feed item target.
    /// Feed item target resource names have the form:
    /// `customers/{customer_id}/feedItemTargets/{feed_id}~{feed_item_id}~{feed_item_target_type}~{feed_item_target_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The feed item to which this feed item target belongs.
    #[prost(string, optional, tag = "12")]
    pub feed_item: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The target type of this feed item target. This field is read-only.
    #[prost(
        enumeration = "super::enums::feed_item_target_type_enum::FeedItemTargetType",
        tag = "3"
    )]
    pub feed_item_target_type: i32,
    /// Output only. The ID of the targeted resource. This field is read-only.
    #[prost(int64, optional, tag = "13")]
    pub feed_item_target_id: ::core::option::Option<i64>,
    /// Output only. Status of the feed item target.
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::feed_item_target_status_enum::FeedItemTargetStatus",
        tag = "11"
    )]
    pub status: i32,
    /// The targeted resource.
    #[prost(oneof = "feed_item_target::Target", tags = "14, 15, 7, 16, 9, 10")]
    pub target: ::core::option::Option<feed_item_target::Target>,
}
/// Nested message and enum types in `FeedItemTarget`.
pub mod feed_item_target {
    /// The targeted resource.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// Immutable. The targeted campaign.
        #[prost(string, tag = "14")]
        Campaign(::prost::alloc::string::String),
        /// Immutable. The targeted ad group.
        #[prost(string, tag = "15")]
        AdGroup(::prost::alloc::string::String),
        /// Immutable. The targeted keyword.
        #[prost(message, tag = "7")]
        Keyword(super::super::common::KeywordInfo),
        /// Immutable. The targeted geo target constant resource name.
        #[prost(string, tag = "16")]
        GeoTargetConstant(::prost::alloc::string::String),
        /// Immutable. The targeted device.
        #[prost(
            enumeration = "super::super::enums::feed_item_target_device_enum::FeedItemTargetDevice",
            tag = "9"
        )]
        Device(i32),
        /// Immutable. The targeted schedule.
        #[prost(message, tag = "10")]
        AdSchedule(super::super::common::AdScheduleInfo),
    }
}
// Proto file describing the FeedMapping resource.

/// A feed mapping.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMapping {
    /// Immutable. The resource name of the feed mapping.
    /// Feed mapping resource names have the form:
    ///
    /// `customers/{customer_id}/feedMappings/{feed_id}~{feed_mapping_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The feed of this feed mapping.
    #[prost(string, optional, tag = "7")]
    pub feed: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. Feed attributes to field mappings. These mappings are a one-to-many
    /// relationship meaning that 1 feed attribute can be used to populate
    /// multiple placeholder fields, but 1 placeholder field can only draw
    /// data from 1 feed attribute. Ad Customizer is an exception, 1 placeholder
    /// field can be mapped to multiple feed attributes. Required.
    #[prost(message, repeated, tag = "5")]
    pub attribute_field_mappings: ::prost::alloc::vec::Vec<AttributeFieldMapping>,
    /// Output only. Status of the feed mapping.
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::feed_mapping_status_enum::FeedMappingStatus",
        tag = "6"
    )]
    pub status: i32,
    /// Feed mapping target. Can be either a placeholder or a criterion. For a
    /// given feed, the active FeedMappings must have unique targets. Required.
    #[prost(oneof = "feed_mapping::Target", tags = "3, 4")]
    pub target: ::core::option::Option<feed_mapping::Target>,
}
/// Nested message and enum types in `FeedMapping`.
pub mod feed_mapping {
    /// Feed mapping target. Can be either a placeholder or a criterion. For a
    /// given feed, the active FeedMappings must have unique targets. Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// Immutable. The placeholder type of this mapping (i.e., if the mapping maps feed
        /// attributes to placeholder fields).
        #[prost(
            enumeration = "super::super::enums::placeholder_type_enum::PlaceholderType",
            tag = "3"
        )]
        PlaceholderType(i32),
        /// Immutable. The criterion type of this mapping (i.e., if the mapping maps feed
        /// attributes to criterion fields).
        #[prost(
            enumeration = "super::super::enums::feed_mapping_criterion_type_enum::FeedMappingCriterionType",
            tag = "4"
        )]
        CriterionType(i32),
    }
}
/// Maps from feed attribute id to a placeholder or criterion field id.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttributeFieldMapping {
    /// Immutable. Feed attribute from which to map.
    #[prost(int64, optional, tag = "24")]
    pub feed_attribute_id: ::core::option::Option<i64>,
    /// Output only. The placeholder field ID. If a placeholder field enum is not published in
    /// the current API version, then this field will be populated and the field
    /// oneof will be empty.
    /// This field is read-only.
    #[prost(int64, optional, tag = "25")]
    pub field_id: ::core::option::Option<i64>,
    /// Placeholder or criterion field to be populated using data from
    /// the above feed attribute. Required.
    #[prost(
        oneof = "attribute_field_mapping::Field",
        tags = "3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 26"
    )]
    pub field: ::core::option::Option<attribute_field_mapping::Field>,
}
/// Nested message and enum types in `AttributeFieldMapping`.
pub mod attribute_field_mapping {
    /// Placeholder or criterion field to be populated using data from
    /// the above feed attribute. Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Field {
        /// Immutable. Sitelink Placeholder Fields.
        #[prost(
            enumeration = "super::super::enums::sitelink_placeholder_field_enum::SitelinkPlaceholderField",
            tag = "3"
        )]
        SitelinkField(i32),
        /// Immutable. Call Placeholder Fields.
        #[prost(
            enumeration = "super::super::enums::call_placeholder_field_enum::CallPlaceholderField",
            tag = "4"
        )]
        CallField(i32),
        /// Immutable. App Placeholder Fields.
        #[prost(
            enumeration = "super::super::enums::app_placeholder_field_enum::AppPlaceholderField",
            tag = "5"
        )]
        AppField(i32),
        /// Output only. Location Placeholder Fields. This field is read-only.
        #[prost(
            enumeration = "super::super::enums::location_placeholder_field_enum::LocationPlaceholderField",
            tag = "6"
        )]
        LocationField(i32),
        /// Output only. Affiliate Location Placeholder Fields. This field is read-only.
        #[prost(
            enumeration = "super::super::enums::affiliate_location_placeholder_field_enum::AffiliateLocationPlaceholderField",
            tag = "7"
        )]
        AffiliateLocationField(i32),
        /// Immutable. Callout Placeholder Fields.
        #[prost(
            enumeration = "super::super::enums::callout_placeholder_field_enum::CalloutPlaceholderField",
            tag = "8"
        )]
        CalloutField(i32),
        /// Immutable. Structured Snippet Placeholder Fields.
        #[prost(
            enumeration = "super::super::enums::structured_snippet_placeholder_field_enum::StructuredSnippetPlaceholderField",
            tag = "9"
        )]
        StructuredSnippetField(i32),
        /// Immutable. Message Placeholder Fields.
        #[prost(
            enumeration = "super::super::enums::message_placeholder_field_enum::MessagePlaceholderField",
            tag = "10"
        )]
        MessageField(i32),
        /// Immutable. Price Placeholder Fields.
        #[prost(
            enumeration = "super::super::enums::price_placeholder_field_enum::PricePlaceholderField",
            tag = "11"
        )]
        PriceField(i32),
        /// Immutable. Promotion Placeholder Fields.
        #[prost(
            enumeration = "super::super::enums::promotion_placeholder_field_enum::PromotionPlaceholderField",
            tag = "12"
        )]
        PromotionField(i32),
        /// Immutable. Ad Customizer Placeholder Fields
        #[prost(
            enumeration = "super::super::enums::ad_customizer_placeholder_field_enum::AdCustomizerPlaceholderField",
            tag = "13"
        )]
        AdCustomizerField(i32),
        /// Immutable. Dynamic Search Ad Page Feed Fields.
        #[prost(
            enumeration = "super::super::enums::dsa_page_feed_criterion_field_enum::DsaPageFeedCriterionField",
            tag = "14"
        )]
        DsaPageFeedField(i32),
        /// Immutable. Location Target Fields.
        #[prost(
            enumeration = "super::super::enums::location_extension_targeting_criterion_field_enum::LocationExtensionTargetingCriterionField",
            tag = "15"
        )]
        LocationExtensionTargetingField(i32),
        /// Immutable. Education Placeholder Fields
        #[prost(
            enumeration = "super::super::enums::education_placeholder_field_enum::EducationPlaceholderField",
            tag = "16"
        )]
        EducationField(i32),
        /// Immutable. Flight Placeholder Fields
        #[prost(
            enumeration = "super::super::enums::flight_placeholder_field_enum::FlightPlaceholderField",
            tag = "17"
        )]
        FlightField(i32),
        /// Immutable. Custom Placeholder Fields
        #[prost(
            enumeration = "super::super::enums::custom_placeholder_field_enum::CustomPlaceholderField",
            tag = "18"
        )]
        CustomField(i32),
        /// Immutable. Hotel Placeholder Fields
        #[prost(
            enumeration = "super::super::enums::hotel_placeholder_field_enum::HotelPlaceholderField",
            tag = "19"
        )]
        HotelField(i32),
        /// Immutable. Real Estate Placeholder Fields
        #[prost(
            enumeration = "super::super::enums::real_estate_placeholder_field_enum::RealEstatePlaceholderField",
            tag = "20"
        )]
        RealEstateField(i32),
        /// Immutable. Travel Placeholder Fields
        #[prost(
            enumeration = "super::super::enums::travel_placeholder_field_enum::TravelPlaceholderField",
            tag = "21"
        )]
        TravelField(i32),
        /// Immutable. Local Placeholder Fields
        #[prost(
            enumeration = "super::super::enums::local_placeholder_field_enum::LocalPlaceholderField",
            tag = "22"
        )]
        LocalField(i32),
        /// Immutable. Job Placeholder Fields
        #[prost(
            enumeration = "super::super::enums::job_placeholder_field_enum::JobPlaceholderField",
            tag = "23"
        )]
        JobField(i32),
        /// Immutable. Image Placeholder Fields
        #[prost(
            enumeration = "super::super::enums::image_placeholder_field_enum::ImagePlaceholderField",
            tag = "26"
        )]
        ImageField(i32),
    }
}
// Proto file describing the FeedPlaceholderView resource.

/// A feed placeholder view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedPlaceholderView {
    /// Output only. The resource name of the feed placeholder view.
    /// Feed placeholder view resource names have the form:
    ///
    /// `customers/{customer_id}/feedPlaceholderViews/{placeholder_type}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The placeholder type of the feed placeholder view.
    #[prost(
        enumeration = "super::enums::placeholder_type_enum::PlaceholderType",
        tag = "2"
    )]
    pub placeholder_type: i32,
}
// Proto file describing the gender view resource.

/// A gender view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenderView {
    /// Output only. The resource name of the gender view.
    /// Gender view resource names have the form:
    ///
    /// `customers/{customer_id}/genderViews/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
// Proto file describing the geo target constant resource.

/// A geo target constant.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoTargetConstant {
    /// Output only. The resource name of the geo target constant.
    /// Geo target constant resource names have the form:
    ///
    /// `geoTargetConstants/{geo_target_constant_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the geo target constant.
    #[prost(int64, optional, tag = "10")]
    pub id: ::core::option::Option<i64>,
    /// Output only. Geo target constant English name.
    #[prost(string, optional, tag = "11")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The ISO-3166-1 alpha-2 country code that is associated with the target.
    #[prost(string, optional, tag = "12")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Geo target constant target type.
    #[prost(string, optional, tag = "13")]
    pub target_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Geo target constant status.
    #[prost(
        enumeration = "super::enums::geo_target_constant_status_enum::GeoTargetConstantStatus",
        tag = "7"
    )]
    pub status: i32,
    /// Output only. The fully qualified English name, consisting of the target's name and that
    /// of its parent and country.
    #[prost(string, optional, tag = "14")]
    pub canonical_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The resource name of the parent geo target constant.
    /// Geo target constant resource names have the form:
    ///
    /// `geoTargetConstants/{parent_geo_target_constant_id}`
    #[prost(string, optional, tag = "9")]
    pub parent_geo_target: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the geographic view resource.

/// A geographic view.
///
/// Geographic View includes all metrics aggregated at the country level,
/// one row per country. It reports metrics at either actual physical location of
/// the user or an area of interest. If other segment fields are used, you may
/// get more than one row per country.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeographicView {
    /// Output only. The resource name of the geographic view.
    /// Geographic view resource names have the form:
    ///
    /// `customers/{customer_id}/geographicViews/{country_criterion_id}~{location_type}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. Type of the geo targeting of the campaign.
    #[prost(
        enumeration = "super::enums::geo_targeting_type_enum::GeoTargetingType",
        tag = "3"
    )]
    pub location_type: i32,
    /// Output only. Criterion Id for the country.
    #[prost(int64, optional, tag = "5")]
    pub country_criterion_id: ::core::option::Option<i64>,
}
// Proto file describing the Google Ads Field resource.

/// A field or resource (artifact) used by GoogleAdsService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleAdsField {
    /// Output only. The resource name of the artifact.
    /// Artifact resource names have the form:
    ///
    /// `googleAdsFields/{name}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The name of the artifact.
    #[prost(string, optional, tag = "21")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The category of the artifact.
    #[prost(
        enumeration = "super::enums::google_ads_field_category_enum::GoogleAdsFieldCategory",
        tag = "3"
    )]
    pub category: i32,
    /// Output only. Whether the artifact can be used in a SELECT clause in search
    /// queries.
    #[prost(bool, optional, tag = "22")]
    pub selectable: ::core::option::Option<bool>,
    /// Output only. Whether the artifact can be used in a WHERE clause in search
    /// queries.
    #[prost(bool, optional, tag = "23")]
    pub filterable: ::core::option::Option<bool>,
    /// Output only. Whether the artifact can be used in a ORDER BY clause in search
    /// queries.
    #[prost(bool, optional, tag = "24")]
    pub sortable: ::core::option::Option<bool>,
    /// Output only. The names of all resources, segments, and metrics that are selectable with
    /// the described artifact.
    #[prost(string, repeated, tag = "25")]
    pub selectable_with: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The names of all resources that are selectable with the described
    /// artifact. Fields from these resources do not segment metrics when included
    /// in search queries.
    ///
    /// This field is only set for artifacts whose category is RESOURCE.
    #[prost(string, repeated, tag = "26")]
    pub attribute_resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. This field lists the names of all metrics that are selectable with the
    /// described artifact when it is used in the FROM clause.
    /// It is only set for artifacts whose category is RESOURCE.
    #[prost(string, repeated, tag = "27")]
    pub metrics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. This field lists the names of all artifacts, whether a segment or another
    /// resource, that segment metrics when included in search queries and when the
    /// described artifact is used in the FROM clause. It is only set for artifacts
    /// whose category is RESOURCE.
    #[prost(string, repeated, tag = "28")]
    pub segments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. Values the artifact can assume if it is a field of type ENUM.
    ///
    /// This field is only set for artifacts of category SEGMENT or ATTRIBUTE.
    #[prost(string, repeated, tag = "29")]
    pub enum_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. This field determines the operators that can be used with the artifact
    /// in WHERE clauses.
    #[prost(
        enumeration = "super::enums::google_ads_field_data_type_enum::GoogleAdsFieldDataType",
        tag = "12"
    )]
    pub data_type: i32,
    /// Output only. The URL of proto describing the artifact's data type.
    #[prost(string, optional, tag = "30")]
    pub type_url: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Whether the field artifact is repeated.
    #[prost(bool, optional, tag = "31")]
    pub is_repeated: ::core::option::Option<bool>,
}
// Proto file describing the group placement view resource.

/// A group placement view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupPlacementView {
    /// Output only. The resource name of the group placement view.
    /// Group placement view resource names have the form:
    ///
    /// `customers/{customer_id}/groupPlacementViews/{ad_group_id}~{base64_placement}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The automatic placement string at group level, e. g. web domain, mobile
    /// app ID, or a YouTube channel ID.
    #[prost(string, optional, tag = "6")]
    pub placement: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Domain name for websites and YouTube channel name for YouTube channels.
    #[prost(string, optional, tag = "7")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. URL of the group placement, e.g. domain, link to the mobile application in
    /// app store, or a YouTube channel URL.
    #[prost(string, optional, tag = "8")]
    pub target_url: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Type of the placement, e.g. Website, YouTube Channel, Mobile Application.
    #[prost(
        enumeration = "super::enums::placement_type_enum::PlacementType",
        tag = "5"
    )]
    pub placement_type: i32,
}
// Proto file describing the hotel group view resource.

/// A hotel group view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelGroupView {
    /// Output only. The resource name of the hotel group view.
    /// Hotel Group view resource names have the form:
    ///
    /// `customers/{customer_id}/hotelGroupViews/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
// Proto file describing the hotel performance view resource.

/// A hotel performance view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelPerformanceView {
    /// Output only. The resource name of the hotel performance view.
    /// Hotel performance view resource names have the form:
    ///
    /// `customers/{customer_id}/hotelPerformanceView`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
// Proto file describing the hotel reconciliation resource.

/// A hotel reconciliation. It contains conversion information from Hotel
/// bookings to reconcile with advertiser records. These rows may be updated
/// or canceled before billing via Bulk Uploads.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelReconciliation {
    /// Immutable. The resource name of the hotel reconciliation.
    /// Hotel reconciliation resource names have the form:
    ///
    /// `customers/{customer_id}/hotelReconciliations/{commission_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Required. Output only. The commission ID is Google's ID for this booking. Every booking event is
    /// assigned a Commission ID to help you match it to a guest stay.
    #[prost(string, tag = "2")]
    pub commission_id: ::prost::alloc::string::String,
    /// Output only. The order ID is the identifier for this booking as provided in the
    /// 'transaction_id' parameter of the conversion tracking tag.
    #[prost(string, tag = "3")]
    pub order_id: ::prost::alloc::string::String,
    /// Output only. The resource name for the Campaign associated with the conversion.
    #[prost(string, tag = "11")]
    pub campaign: ::prost::alloc::string::String,
    /// Output only. Identifier for the Hotel Center account which provides the rates for the
    /// Hotel campaign.
    #[prost(int64, tag = "4")]
    pub hotel_center_id: i64,
    /// Output only. Unique identifier for the booked property, as provided in the Hotel Center
    /// feed. The hotel ID comes from the 'ID' parameter of the conversion tracking
    /// tag.
    #[prost(string, tag = "5")]
    pub hotel_id: ::prost::alloc::string::String,
    /// Output only. Check-in date recorded when the booking is made. If the check-in date is
    /// modified at reconciliation, the revised date will then take the place of
    /// the original date in this column. Format is YYYY-MM-DD.
    #[prost(string, tag = "6")]
    pub check_in_date: ::prost::alloc::string::String,
    /// Output only. Check-out date recorded when the booking is made. If the check-in date is
    /// modified at reconciliation, the revised date will then take the place of
    /// the original date in this column. Format is YYYY-MM-DD.
    #[prost(string, tag = "7")]
    pub check_out_date: ::prost::alloc::string::String,
    /// Required. Output only. Reconciled value is the final value of a booking as paid by the guest. If
    /// original booking value changes for any reason, such as itinerary changes or
    /// room upsells, the reconciled value should be the full final amount
    /// collected. If a booking is canceled, the reconciled value should include
    /// the value of any cancellation fees or non-refundable nights charged. Value
    /// is in millionths of the base unit currency. For example, $12.35 would be
    /// represented as 12350000. Currency unit is in the default customer currency.
    #[prost(int64, tag = "8")]
    pub reconciled_value_micros: i64,
    /// Output only. Whether a given booking has been billed. Once billed, a booking can't be
    /// modified.
    #[prost(bool, tag = "9")]
    pub billed: bool,
    /// Required. Output only. Current status of a booking with regards to reconciliation and billing.
    /// Bookings should be reconciled within 45 days after the check-out date.
    /// Any booking not reconciled within 45 days will be billed at its original
    /// value.
    #[prost(
        enumeration = "super::enums::hotel_reconciliation_status_enum::HotelReconciliationStatus",
        tag = "10"
    )]
    pub status: i32,
}
// Proto file describing the income range view resource.

/// An income range view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncomeRangeView {
    /// Output only. The resource name of the income range view.
    /// Income range view resource names have the form:
    ///
    /// `customers/{customer_id}/incomeRangeViews/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
// Proto file describing the Invoice resource.

/// An invoice. All invoice information is snapshotted to match the PDF invoice.
/// For invoices older than the launch of InvoiceService, the snapshotted
/// information may not match the PDF invoice.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Invoice {
    /// Output only. The resource name of the invoice. Multiple customers can share a given
    /// invoice, so multiple resource names may point to the same invoice.
    /// Invoice resource names have the form:
    ///
    /// `customers/{customer_id}/invoices/{invoice_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the invoice. It appears on the invoice PDF as "Invoice number".
    #[prost(string, optional, tag = "25")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The type of invoice.
    #[prost(
        enumeration = "super::enums::invoice_type_enum::InvoiceType",
        tag = "3"
    )]
    pub r#type: i32,
    /// Output only. The resource name of this invoice's billing setup.
    ///
    /// `customers/{customer_id}/billingSetups/{billing_setup_id}`
    #[prost(string, optional, tag = "26")]
    pub billing_setup: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. A 16 digit ID used to identify the payments account associated with the
    /// billing setup, e.g. "1234-5678-9012-3456". It appears on the invoice PDF as
    /// "Billing Account Number".
    #[prost(string, optional, tag = "27")]
    pub payments_account_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. A 12 digit ID used to identify the payments profile associated with the
    /// billing setup, e.g. "1234-5678-9012". It appears on the invoice PDF as
    /// "Billing ID".
    #[prost(string, optional, tag = "28")]
    pub payments_profile_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The issue date in yyyy-mm-dd format. It appears on the invoice PDF as
    /// either "Issue date" or "Invoice date".
    #[prost(string, optional, tag = "29")]
    pub issue_date: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The due date in yyyy-mm-dd format.
    #[prost(string, optional, tag = "30")]
    pub due_date: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The service period date range of this invoice. The end date is inclusive.
    #[prost(message, optional, tag = "9")]
    pub service_date_range: ::core::option::Option<super::common::DateRange>,
    /// Output only. The currency code. All costs are returned in this currency. A subset of the
    /// currency codes derived from the ISO 4217 standard is supported.
    #[prost(string, optional, tag = "31")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The pretax subtotal amount of invoice level adjustments, in micros.
    #[prost(int64, tag = "19")]
    pub adjustments_subtotal_amount_micros: i64,
    /// Output only. The sum of taxes on the invoice level adjustments, in micros.
    #[prost(int64, tag = "20")]
    pub adjustments_tax_amount_micros: i64,
    /// Output only. The total amount of invoice level adjustments, in micros.
    #[prost(int64, tag = "21")]
    pub adjustments_total_amount_micros: i64,
    /// Output only. The pretax subtotal amount of invoice level regulatory costs, in micros.
    #[prost(int64, tag = "22")]
    pub regulatory_costs_subtotal_amount_micros: i64,
    /// Output only. The sum of taxes on the invoice level regulatory costs, in micros.
    #[prost(int64, tag = "23")]
    pub regulatory_costs_tax_amount_micros: i64,
    /// Output only. The total amount of invoice level regulatory costs, in micros.
    #[prost(int64, tag = "24")]
    pub regulatory_costs_total_amount_micros: i64,
    /// Output only. The pretax subtotal amount, in micros. This equals the
    /// sum of the AccountBudgetSummary subtotal amounts,
    /// Invoice.adjustments_subtotal_amount_micros, and
    /// Invoice.regulatory_costs_subtotal_amount_micros.
    /// Starting with v6, the Invoice.regulatory_costs_subtotal_amount_micros is no
    /// longer included.
    #[prost(int64, optional, tag = "33")]
    pub subtotal_amount_micros: ::core::option::Option<i64>,
    /// Output only. The sum of all taxes on the invoice, in micros. This equals the sum of the
    /// AccountBudgetSummary tax amounts, plus taxes not associated with a specific
    /// account budget.
    #[prost(int64, optional, tag = "34")]
    pub tax_amount_micros: ::core::option::Option<i64>,
    /// Output only. The total amount, in micros. This equals the sum of
    /// Invoice.subtotal_amount_micros and Invoice.tax_amount_micros.
    /// Starting with v6, Invoice.regulatory_costs_subtotal_amount_micros is
    /// also added as it is no longer already included in
    /// Invoice.tax_amount_micros.
    #[prost(int64, optional, tag = "35")]
    pub total_amount_micros: ::core::option::Option<i64>,
    /// Output only. The resource name of the original invoice corrected, wrote off, or canceled
    /// by this invoice, if applicable. If `corrected_invoice` is set,
    /// `replaced_invoices` will not be set.
    /// Invoice resource names have the form:
    ///
    /// `customers/{customer_id}/invoices/{invoice_id}`
    #[prost(string, optional, tag = "36")]
    pub corrected_invoice: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The resource name of the original invoice(s) being rebilled or replaced by
    /// this invoice, if applicable. There might be multiple replaced invoices due
    /// to invoice consolidation. The replaced invoices may not belong to the same
    /// payments account. If `replaced_invoices` is set, `corrected_invoice` will
    /// not be set.
    /// Invoice resource names have the form:
    ///
    /// `customers/{customer_id}/invoices/{invoice_id}`
    #[prost(string, repeated, tag = "37")]
    pub replaced_invoices: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The URL to a PDF copy of the invoice. Users need to pass in their OAuth
    /// token to request the PDF with this URL.
    #[prost(string, optional, tag = "38")]
    pub pdf_url: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The list of summarized account budget information associated with this
    /// invoice.
    #[prost(message, repeated, tag = "18")]
    pub account_budget_summaries: ::prost::alloc::vec::Vec<invoice::AccountBudgetSummary>,
}
/// Nested message and enum types in `Invoice`.
pub mod invoice {
    /// Represents a summarized account budget billable cost.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccountBudgetSummary {
        /// Output only. The resource name of the customer associated with this account budget.
        /// This contains the customer ID, which appears on the invoice PDF as
        /// "Account ID".
        /// Customer resource names have the form:
        ///
        /// `customers/{customer_id}`
        #[prost(string, optional, tag = "10")]
        pub customer: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. The descriptive name of the account budget's customer. It appears on the
        /// invoice PDF as "Account".
        #[prost(string, optional, tag = "11")]
        pub customer_descriptive_name: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. The resource name of the account budget associated with this summarized
        /// billable cost.
        /// AccountBudget resource names have the form:
        ///
        /// `customers/{customer_id}/accountBudgets/{account_budget_id}`
        #[prost(string, optional, tag = "12")]
        pub account_budget: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. The name of the account budget. It appears on the invoice PDF as "Account
        /// budget".
        #[prost(string, optional, tag = "13")]
        pub account_budget_name: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. The purchase order number of the account budget. It appears on the
        /// invoice PDF as "Purchase order".
        #[prost(string, optional, tag = "14")]
        pub purchase_order_number: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. The pretax subtotal amount attributable to this budget during the service
        /// period, in micros.
        #[prost(int64, optional, tag = "15")]
        pub subtotal_amount_micros: ::core::option::Option<i64>,
        /// Output only. The tax amount attributable to this budget during the service period, in
        /// micros.
        #[prost(int64, optional, tag = "16")]
        pub tax_amount_micros: ::core::option::Option<i64>,
        /// Output only. The total amount attributable to this budget during the service period,
        /// in micros. This equals the sum of the account budget subtotal amount and
        /// the account budget tax amount.
        #[prost(int64, optional, tag = "17")]
        pub total_amount_micros: ::core::option::Option<i64>,
        /// Output only. The billable activity date range of the account budget, within the
        /// service date range of this invoice. The end date is inclusive. This can
        /// be different from the account budget's start and end time.
        #[prost(message, optional, tag = "9")]
        pub billable_activity_date_range: ::core::option::Option<super::super::common::DateRange>,
    }
}
// Proto file describing the keyword plan resource.

/// A Keyword Planner plan.
/// Max number of saved keyword plans: 10000.
/// It's possible to remove plans if limit is reached.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlan {
    /// Immutable. The resource name of the Keyword Planner plan.
    /// KeywordPlan resource names have the form:
    ///
    /// `customers/{customer_id}/keywordPlans/{kp_plan_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the keyword plan.
    #[prost(int64, optional, tag = "5")]
    pub id: ::core::option::Option<i64>,
    /// The name of the keyword plan.
    ///
    /// This field is required and should not be empty when creating new keyword
    /// plans.
    #[prost(string, optional, tag = "6")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The date period used for forecasting the plan.
    #[prost(message, optional, tag = "4")]
    pub forecast_period: ::core::option::Option<KeywordPlanForecastPeriod>,
}
/// The forecasting period associated with the keyword plan.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanForecastPeriod {
    /// Required. The date used for forecasting the Plan.
    #[prost(oneof = "keyword_plan_forecast_period::Interval", tags = "1, 2")]
    pub interval: ::core::option::Option<keyword_plan_forecast_period::Interval>,
}
/// Nested message and enum types in `KeywordPlanForecastPeriod`.
pub mod keyword_plan_forecast_period {
    /// Required. The date used for forecasting the Plan.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Interval {
        /// A future date range relative to the current date used for forecasting.
        #[prost(
            enumeration = "super::super::enums::keyword_plan_forecast_interval_enum::KeywordPlanForecastInterval",
            tag = "1"
        )]
        DateInterval(i32),
        /// The custom date range used for forecasting.
        /// The start and end dates must be in the future. Otherwise, an error will
        /// be returned when the forecasting action is performed.
        /// The start and end dates are inclusive.
        #[prost(message, tag = "2")]
        DateRange(super::super::common::DateRange),
    }
}
// Proto file describing the keyword plan ad group resource.

/// A Keyword Planner ad group.
/// Max number of keyword plan ad groups per plan: 200.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanAdGroup {
    /// Immutable. The resource name of the Keyword Planner ad group.
    /// KeywordPlanAdGroup resource names have the form:
    ///
    /// `customers/{customer_id}/keywordPlanAdGroups/{kp_ad_group_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The keyword plan campaign to which this ad group belongs.
    #[prost(string, optional, tag = "6")]
    pub keyword_plan_campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The ID of the keyword plan ad group.
    #[prost(int64, optional, tag = "7")]
    pub id: ::core::option::Option<i64>,
    /// The name of the keyword plan ad group.
    ///
    /// This field is required and should not be empty when creating keyword plan
    /// ad group.
    #[prost(string, optional, tag = "8")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// A default ad group max cpc bid in micros in account currency for all
    /// biddable keywords under the keyword plan ad group.
    /// If not set, will inherit from parent campaign.
    #[prost(int64, optional, tag = "9")]
    pub cpc_bid_micros: ::core::option::Option<i64>,
}
// Proto file describing the keyword plan ad group keyword resource.

/// A Keyword Plan ad group keyword.
/// Max number of keyword plan keywords per plan: 10000.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanAdGroupKeyword {
    /// Immutable. The resource name of the Keyword Plan ad group keyword.
    /// KeywordPlanAdGroupKeyword resource names have the form:
    ///
    /// `customers/{customer_id}/keywordPlanAdGroupKeywords/{kp_ad_group_keyword_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The Keyword Plan ad group to which this keyword belongs.
    #[prost(string, optional, tag = "8")]
    pub keyword_plan_ad_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The ID of the Keyword Plan keyword.
    #[prost(int64, optional, tag = "9")]
    pub id: ::core::option::Option<i64>,
    /// The keyword text.
    #[prost(string, optional, tag = "10")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    /// The keyword match type.
    #[prost(
        enumeration = "super::enums::keyword_match_type_enum::KeywordMatchType",
        tag = "5"
    )]
    pub match_type: i32,
    /// A keyword level max cpc bid in micros (e.g. $1 = 1mm). The currency is the
    /// same as the account currency code. This will override any CPC bid set at
    /// the keyword plan ad group level.
    /// Not applicable for negative keywords. (negative = true)
    /// This field is Optional.
    #[prost(int64, optional, tag = "11")]
    pub cpc_bid_micros: ::core::option::Option<i64>,
    /// Immutable. If true, the keyword is negative.
    #[prost(bool, optional, tag = "12")]
    pub negative: ::core::option::Option<bool>,
}
// Proto file describing the keyword plan campaign resource.

/// A Keyword Plan campaign.
/// Max number of keyword plan campaigns per plan allowed: 1.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanCampaign {
    /// Immutable. The resource name of the Keyword Plan campaign.
    /// KeywordPlanCampaign resource names have the form:
    ///
    /// `customers/{customer_id}/keywordPlanCampaigns/{kp_campaign_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The keyword plan this campaign belongs to.
    #[prost(string, optional, tag = "9")]
    pub keyword_plan: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The ID of the Keyword Plan campaign.
    #[prost(int64, optional, tag = "10")]
    pub id: ::core::option::Option<i64>,
    /// The name of the Keyword Plan campaign.
    ///
    /// This field is required and should not be empty when creating Keyword Plan
    /// campaigns.
    #[prost(string, optional, tag = "11")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The languages targeted for the Keyword Plan campaign.
    /// Max allowed: 1.
    #[prost(string, repeated, tag = "12")]
    pub language_constants: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Targeting network.
    ///
    /// This field is required and should not be empty when creating Keyword Plan
    /// campaigns.
    #[prost(
        enumeration = "super::enums::keyword_plan_network_enum::KeywordPlanNetwork",
        tag = "6"
    )]
    pub keyword_plan_network: i32,
    /// A default max cpc bid in micros, and in the account currency, for all ad
    /// groups under the campaign.
    ///
    /// This field is required and should not be empty when creating Keyword Plan
    /// campaigns.
    #[prost(int64, optional, tag = "13")]
    pub cpc_bid_micros: ::core::option::Option<i64>,
    /// The geo targets.
    /// Max number allowed: 20.
    #[prost(message, repeated, tag = "8")]
    pub geo_targets: ::prost::alloc::vec::Vec<KeywordPlanGeoTarget>,
}
/// A geo target.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanGeoTarget {
    /// Required. The resource name of the geo target.
    #[prost(string, optional, tag = "2")]
    pub geo_target_constant: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the keyword plan negative keyword resource.

/// A Keyword Plan Campaign keyword.
/// Only negative keywords are supported for Campaign Keyword.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanCampaignKeyword {
    /// Immutable. The resource name of the Keyword Plan Campaign keyword.
    /// KeywordPlanCampaignKeyword resource names have the form:
    ///
    /// `customers/{customer_id}/keywordPlanCampaignKeywords/{kp_campaign_keyword_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The Keyword Plan campaign to which this negative keyword belongs.
    #[prost(string, optional, tag = "8")]
    pub keyword_plan_campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The ID of the Keyword Plan negative keyword.
    #[prost(int64, optional, tag = "9")]
    pub id: ::core::option::Option<i64>,
    /// The keyword text.
    #[prost(string, optional, tag = "10")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    /// The keyword match type.
    #[prost(
        enumeration = "super::enums::keyword_match_type_enum::KeywordMatchType",
        tag = "5"
    )]
    pub match_type: i32,
    /// Immutable. If true, the keyword is negative.
    /// Must be set to true. Only negative campaign keywords are supported.
    #[prost(bool, optional, tag = "11")]
    pub negative: ::core::option::Option<bool>,
}
// Proto file describing the Smart Campaign keyword theme constant resource.

/// A Smart Campaign keyword theme constant.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordThemeConstant {
    /// Output only. The resource name of the keyword theme constant.
    /// Keyword theme constant resource names have the form:
    ///
    /// `keywordThemeConstants/{keyword_theme_id}~{sub_keyword_theme_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ISO-3166 Alpha-2 country code of the constant, eg. "US".
    /// To display and query matching purpose, the keyword theme needs to be
    /// localized.
    #[prost(string, optional, tag = "2")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The ISO-639-1 language code with 2 letters of the constant, eg. "en".
    /// To display and query matching purpose, the keyword theme needs to be
    /// localized.
    #[prost(string, optional, tag = "3")]
    pub language_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The display name of the keyword theme or sub keyword theme.
    #[prost(string, optional, tag = "4")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the keyword view resource.

/// A keyword view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordView {
    /// Output only. The resource name of the keyword view.
    /// Keyword view resource names have the form:
    ///
    /// `customers/{customer_id}/keywordViews/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// A label.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Label {
    /// Immutable. Name of the resource.
    /// Label resource names have the form:
    /// `customers/{customer_id}/labels/{label_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. Id of the label. Read only.
    #[prost(int64, optional, tag = "6")]
    pub id: ::core::option::Option<i64>,
    /// The name of the label.
    ///
    /// This field is required and should not be empty when creating a new label.
    ///
    /// The length of this string should be between 1 and 80, inclusive.
    #[prost(string, optional, tag = "7")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Status of the label. Read only.
    #[prost(
        enumeration = "super::enums::label_status_enum::LabelStatus",
        tag = "4"
    )]
    pub status: i32,
    /// A type of label displaying text on a colored background.
    #[prost(message, optional, tag = "5")]
    pub text_label: ::core::option::Option<super::common::TextLabel>,
}
// Proto file describing the landing page view resource.

/// A landing page view with metrics aggregated at the unexpanded final URL
/// level.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LandingPageView {
    /// Output only. The resource name of the landing page view.
    /// Landing page view resource names have the form:
    ///
    /// `customers/{customer_id}/landingPageViews/{unexpanded_final_url_fingerprint}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The advertiser-specified final URL.
    #[prost(string, optional, tag = "3")]
    pub unexpanded_final_url: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the language constant resource.

/// A language.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LanguageConstant {
    /// Output only. The resource name of the language constant.
    /// Language constant resource names have the form:
    ///
    /// `languageConstants/{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the language constant.
    #[prost(int64, optional, tag = "6")]
    pub id: ::core::option::Option<i64>,
    /// Output only. The language code, e.g. "en_US", "en_AU", "es", "fr", etc.
    #[prost(string, optional, tag = "7")]
    pub code: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The full name of the language in English, e.g., "English (US)", "Spanish",
    /// etc.
    #[prost(string, optional, tag = "8")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Whether the language is targetable.
    #[prost(bool, optional, tag = "9")]
    pub targetable: ::core::option::Option<bool>,
}
// Proto file describing the lead form submission data resource.

/// Data from lead form submissions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeadFormSubmissionData {
    /// Output only. The resource name of the lead form submission data.
    /// Lead form submission data resource names have the form:
    ///
    /// `customers/{customer_id}/leadFormSubmissionData/{lead_form_submission_data_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. ID of this lead form submission.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// Output only. Asset associated with the submitted lead form.
    #[prost(string, tag = "3")]
    pub asset: ::prost::alloc::string::String,
    /// Output only. Campaign associated with the submitted lead form.
    #[prost(string, tag = "4")]
    pub campaign: ::prost::alloc::string::String,
    /// Output only. Submission data associated with a lead form.
    #[prost(message, repeated, tag = "5")]
    pub lead_form_submission_fields: ::prost::alloc::vec::Vec<LeadFormSubmissionField>,
    /// Output only. AdGroup associated with the submitted lead form.
    #[prost(string, tag = "6")]
    pub ad_group: ::prost::alloc::string::String,
    /// Output only. AdGroupAd associated with the submitted lead form.
    #[prost(string, tag = "7")]
    pub ad_group_ad: ::prost::alloc::string::String,
    /// Output only. Google Click Id associated with the submissed lead form.
    #[prost(string, tag = "8")]
    pub gclid: ::prost::alloc::string::String,
    /// Output only. The date and time at which the lead form was submitted. The format is
    /// "yyyy-mm-dd hh:mm:ss+|-hh:mm", e.g. "2019-01-01 12:32:45-08:00".
    #[prost(string, tag = "9")]
    pub submission_date_time: ::prost::alloc::string::String,
}
/// Fields in the submitted lead form.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeadFormSubmissionField {
    /// Output only. Field type for lead form fields.
    #[prost(
        enumeration = "super::enums::lead_form_field_user_input_type_enum::LeadFormFieldUserInputType",
        tag = "1"
    )]
    pub field_type: i32,
    /// Output only. Field value for lead form fields.
    #[prost(string, tag = "2")]
    pub field_value: ::prost::alloc::string::String,
}
// Proto file describing the Life Event resource.

/// A life event: a particular interest-based vertical to be targeted to reach
/// users when they are in the midst of important life milestones.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LifeEvent {
    /// Output only. The resource name of the life event.
    /// Life event resource names have the form:
    ///
    /// `customers/{customer_id}/lifeEvents/{life_event_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the life event.
    #[prost(int64, tag = "2")]
    pub id: i64,
    /// Output only. The name of the life event. E.g.,"Recently Moved"
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The parent of the life_event.
    #[prost(string, tag = "4")]
    pub parent: ::prost::alloc::string::String,
    /// Output only. True if the life event is launched to all channels and locales.
    #[prost(bool, tag = "5")]
    pub launched_to_all: bool,
    /// Output only. Availability information of the life event.
    #[prost(message, repeated, tag = "6")]
    pub availabilities: ::prost::alloc::vec::Vec<super::common::CriterionCategoryAvailability>,
}
// Proto file describing the location view resource.

/// A location view summarizes the performance of campaigns by
/// Location criteria.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationView {
    /// Output only. The resource name of the location view.
    /// Location view resource names have the form:
    ///
    /// `customers/{customer_id}/locationViews/{campaign_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
// Proto file describing the Managed Placement view resource.

/// A managed placement view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagedPlacementView {
    /// Output only. The resource name of the Managed Placement view.
    /// Managed placement view resource names have the form:
    ///
    /// `customers/{customer_id}/managedPlacementViews/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
// Proto file describing the media file resource.

/// A media file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaFile {
    /// Immutable. The resource name of the media file.
    /// Media file resource names have the form:
    ///
    /// `customers/{customer_id}/mediaFiles/{media_file_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the media file.
    #[prost(int64, optional, tag = "12")]
    pub id: ::core::option::Option<i64>,
    /// Immutable. Type of the media file.
    #[prost(enumeration = "super::enums::media_type_enum::MediaType", tag = "5")]
    pub r#type: i32,
    /// Output only. The mime type of the media file.
    #[prost(enumeration = "super::enums::mime_type_enum::MimeType", tag = "6")]
    pub mime_type: i32,
    /// Immutable. The URL of where the original media file was downloaded from (or a file
    /// name). Only used for media of type AUDIO and IMAGE.
    #[prost(string, optional, tag = "13")]
    pub source_url: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The name of the media file. The name can be used by clients to help
    /// identify previously uploaded media.
    #[prost(string, optional, tag = "14")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The size of the media file in bytes.
    #[prost(int64, optional, tag = "15")]
    pub file_size: ::core::option::Option<i64>,
    /// The specific type of the media file.
    #[prost(oneof = "media_file::Mediatype", tags = "3, 4, 10, 11")]
    pub mediatype: ::core::option::Option<media_file::Mediatype>,
}
/// Nested message and enum types in `MediaFile`.
pub mod media_file {
    /// The specific type of the media file.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Mediatype {
        /// Immutable. Encapsulates an Image.
        #[prost(message, tag = "3")]
        Image(super::MediaImage),
        /// Immutable. A ZIP archive media the content of which contains HTML5 assets.
        #[prost(message, tag = "4")]
        MediaBundle(super::MediaBundle),
        /// Output only. Encapsulates an Audio.
        #[prost(message, tag = "10")]
        Audio(super::MediaAudio),
        /// Immutable. Encapsulates a Video.
        #[prost(message, tag = "11")]
        Video(super::MediaVideo),
    }
}
/// Encapsulates an Image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaImage {
    /// Immutable. Raw image data.
    #[prost(bytes = "vec", optional, tag = "4")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// Output only. The url to the full size version of the image.
    #[prost(string, optional, tag = "2")]
    pub full_size_image_url: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The url to the preview size version of the image.
    #[prost(string, optional, tag = "3")]
    pub preview_size_image_url: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents a ZIP archive media the content of which contains HTML5 assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaBundle {
    /// Immutable. Raw zipped data.
    #[prost(bytes = "vec", optional, tag = "3")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// Output only. The url to access the uploaded zipped data.
    /// E.g. <https://tpc.googlesyndication.com/simgad/123>
    /// This field is read-only.
    #[prost(string, optional, tag = "2")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
}
/// Encapsulates an Audio.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaAudio {
    /// Output only. The duration of the Audio in milliseconds.
    #[prost(int64, optional, tag = "2")]
    pub ad_duration_millis: ::core::option::Option<i64>,
}
/// Encapsulates a Video.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaVideo {
    /// Output only. The duration of the Video in milliseconds.
    #[prost(int64, optional, tag = "5")]
    pub ad_duration_millis: ::core::option::Option<i64>,
    /// Immutable. The YouTube video ID (as seen in YouTube URLs). Adding prefix
    /// "<https://www.youtube.com/watch?v="> to this ID will get the YouTube
    /// streaming URL for this video.
    #[prost(string, optional, tag = "6")]
    pub youtube_video_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The Advertising Digital Identification code for this video, as defined by
    /// the American Association of Advertising Agencies, used mainly for
    /// television commercials.
    #[prost(string, optional, tag = "7")]
    pub advertising_id_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The Industry Standard Commercial Identifier code for this video, used
    /// mainly for television commercials.
    #[prost(string, optional, tag = "8")]
    pub isci_code: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the Merchant Center link resource.

/// A data sharing connection, proposed or in use,
/// between a Google Ads Customer and a Merchant Center account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerchantCenterLink {
    /// Immutable. The resource name of the merchant center link.
    /// Merchant center link resource names have the form:
    ///
    /// `customers/{customer_id}/merchantCenterLinks/{merchant_center_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the Merchant Center account.
    /// This field is readonly.
    #[prost(int64, optional, tag = "6")]
    pub id: ::core::option::Option<i64>,
    /// Output only. The name of the Merchant Center account.
    /// This field is readonly.
    #[prost(string, optional, tag = "7")]
    pub merchant_center_account_name: ::core::option::Option<::prost::alloc::string::String>,
    /// The status of the link.
    #[prost(
        enumeration = "super::enums::merchant_center_link_status_enum::MerchantCenterLinkStatus",
        tag = "5"
    )]
    pub status: i32,
}
// Proto file describing the Mobile App Category Constant resource.

/// A mobile application category constant.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MobileAppCategoryConstant {
    /// Output only. The resource name of the mobile app category constant.
    /// Mobile app category constant resource names have the form:
    ///
    /// `mobileAppCategoryConstants/{mobile_app_category_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the mobile app category constant.
    #[prost(int32, optional, tag = "4")]
    pub id: ::core::option::Option<i32>,
    /// Output only. Mobile app category name.
    #[prost(string, optional, tag = "5")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the mobile device constant resource.

/// A mobile device constant.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MobileDeviceConstant {
    /// Output only. The resource name of the mobile device constant.
    /// Mobile device constant resource names have the form:
    ///
    /// `mobileDeviceConstants/{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the mobile device constant.
    #[prost(int64, optional, tag = "7")]
    pub id: ::core::option::Option<i64>,
    /// Output only. The name of the mobile device.
    #[prost(string, optional, tag = "8")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The manufacturer of the mobile device.
    #[prost(string, optional, tag = "9")]
    pub manufacturer_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The operating system of the mobile device.
    #[prost(string, optional, tag = "10")]
    pub operating_system_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The type of mobile device.
    #[prost(
        enumeration = "super::enums::mobile_device_type_enum::MobileDeviceType",
        tag = "6"
    )]
    pub r#type: i32,
}
// Proto file describing the offline user data job resource.

/// A job containing offline user data of store visitors, or user list members
/// that will be processed asynchronously. The uploaded data isn't readable and
/// the processing results of the job can only be read using
/// GoogleAdsService.Search/SearchStream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfflineUserDataJob {
    /// Immutable. The resource name of the offline user data job.
    /// Offline user data job resource names have the form:
    ///
    /// `customers/{customer_id}/offlineUserDataJobs/{offline_user_data_job_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. ID of this offline user data job.
    #[prost(int64, optional, tag = "9")]
    pub id: ::core::option::Option<i64>,
    /// Immutable. User specified job ID.
    #[prost(int64, optional, tag = "10")]
    pub external_id: ::core::option::Option<i64>,
    /// Immutable. Type of the job.
    #[prost(
        enumeration = "super::enums::offline_user_data_job_type_enum::OfflineUserDataJobType",
        tag = "4"
    )]
    pub r#type: i32,
    /// Output only. Status of the job.
    #[prost(
        enumeration = "super::enums::offline_user_data_job_status_enum::OfflineUserDataJobStatus",
        tag = "5"
    )]
    pub status: i32,
    /// Output only. Reason for the processing failure, if status is FAILED.
    #[prost(
        enumeration = "super::enums::offline_user_data_job_failure_reason_enum::OfflineUserDataJobFailureReason",
        tag = "6"
    )]
    pub failure_reason: i32,
    /// Output only. Metadata of offline user data job depicting match rate range.
    #[prost(message, optional, tag = "11")]
    pub operation_metadata: ::core::option::Option<OfflineUserDataJobMetadata>,
    /// Metadata of the job.
    #[prost(oneof = "offline_user_data_job::Metadata", tags = "7, 8")]
    pub metadata: ::core::option::Option<offline_user_data_job::Metadata>,
}
/// Nested message and enum types in `OfflineUserDataJob`.
pub mod offline_user_data_job {
    /// Metadata of the job.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        /// Immutable. Metadata for data updates to a CRM-based user list.
        #[prost(message, tag = "7")]
        CustomerMatchUserListMetadata(super::super::common::CustomerMatchUserListMetadata),
        /// Immutable. Metadata for store sales data update.
        #[prost(message, tag = "8")]
        StoreSalesMetadata(super::super::common::StoreSalesMetadata),
    }
}
/// Metadata of offline user data job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfflineUserDataJobMetadata {
    /// Output only. Match rate of the Customer Match user list upload. Describes the estimated
    /// match rate when the status of the job is "RUNNING" and final match rate
    /// when the final match rate is available after the status of the job is
    /// "SUCCESS/FAILED".
    #[prost(
        enumeration = "super::enums::offline_user_data_job_match_rate_range_enum::OfflineUserDataJobMatchRateRange",
        tag = "1"
    )]
    pub match_rate_range: i32,
}
// Proto file describing the operating system version constant resource.

/// A mobile operating system version or a range of versions, depending on
/// `operator_type`. List of available mobile platforms at
/// <https://developers.google.com/google-ads/api/reference/data/codes-formats#mobile-platforms>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperatingSystemVersionConstant {
    /// Output only. The resource name of the operating system version constant.
    /// Operating system version constant resource names have the form:
    ///
    /// `operatingSystemVersionConstants/{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the operating system version.
    #[prost(int64, optional, tag = "7")]
    pub id: ::core::option::Option<i64>,
    /// Output only. Name of the operating system.
    #[prost(string, optional, tag = "8")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The OS Major Version number.
    #[prost(int32, optional, tag = "9")]
    pub os_major_version: ::core::option::Option<i32>,
    /// Output only. The OS Minor Version number.
    #[prost(int32, optional, tag = "10")]
    pub os_minor_version: ::core::option::Option<i32>,
    /// Output only. Determines whether this constant represents a single version or a range of
    /// versions.
    #[prost(
        enumeration = "super::enums::operating_system_version_operator_type_enum::OperatingSystemVersionOperatorType",
        tag = "6"
    )]
    pub operator_type: i32,
}
// Proto file describing the PaidOrganicSearchTermView resource.

/// A paid organic search term view providing a view of search stats across
/// ads and organic listings aggregated by search term at the ad group level.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaidOrganicSearchTermView {
    /// Output only. The resource name of the search term view.
    /// Search term view resource names have the form:
    ///
    /// `customers/{customer_id}/paidOrganicSearchTermViews/{campaign_id}~
    /// {ad_group_id}~{URL-base64 search term}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The search term.
    #[prost(string, optional, tag = "3")]
    pub search_term: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the parental status view resource.

/// A parental status view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParentalStatusView {
    /// Output only. The resource name of the parental status view.
    /// Parental Status view resource names have the form:
    ///
    /// `customers/{customer_id}/parentalStatusViews/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
// Proto file describing the PaymentsAccount resource.

/// A payments account, which can be used to set up billing for an Ads customer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentsAccount {
    /// Output only. The resource name of the payments account.
    /// PaymentsAccount resource names have the form:
    ///
    /// `customers/{customer_id}/paymentsAccounts/{payments_account_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. A 16 digit ID used to identify a payments account.
    #[prost(string, optional, tag = "8")]
    pub payments_account_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The name of the payments account.
    #[prost(string, optional, tag = "9")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The currency code of the payments account.
    /// A subset of the currency codes derived from the ISO 4217 standard is
    /// supported.
    #[prost(string, optional, tag = "10")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. A 12 digit ID used to identify the payments profile associated with the
    /// payments account.
    #[prost(string, optional, tag = "11")]
    pub payments_profile_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. A secondary payments profile ID present in uncommon situations, e.g.
    /// when a sequential liability agreement has been arranged.
    #[prost(string, optional, tag = "12")]
    pub secondary_payments_profile_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Paying manager of this payment account.
    #[prost(string, optional, tag = "13")]
    pub paying_manager_customer: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the ProductBiddingCategoryConstant resource.

/// A Product Bidding Category.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductBiddingCategoryConstant {
    /// Output only. The resource name of the product bidding category.
    /// Product bidding category resource names have the form:
    ///
    /// `productBiddingCategoryConstants/{country_code}~{level}~{id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. ID of the product bidding category.
    ///
    /// This ID is equivalent to the google_product_category ID as described in
    /// this article: <https://support.google.com/merchants/answer/6324436.>
    #[prost(int64, optional, tag = "10")]
    pub id: ::core::option::Option<i64>,
    /// Output only. Two-letter upper-case country code of the product bidding category.
    #[prost(string, optional, tag = "11")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Resource name of the parent product bidding category.
    #[prost(string, optional, tag = "12")]
    pub product_bidding_category_constant_parent:
        ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Level of the product bidding category.
    #[prost(
        enumeration = "super::enums::product_bidding_category_level_enum::ProductBiddingCategoryLevel",
        tag = "5"
    )]
    pub level: i32,
    /// Output only. Status of the product bidding category.
    #[prost(
        enumeration = "super::enums::product_bidding_category_status_enum::ProductBiddingCategoryStatus",
        tag = "6"
    )]
    pub status: i32,
    /// Output only. Language code of the product bidding category.
    #[prost(string, optional, tag = "13")]
    pub language_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Display value of the product bidding category localized according to
    /// language_code.
    #[prost(string, optional, tag = "14")]
    pub localized_name: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the ProductGroup View resource.

/// A product group view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductGroupView {
    /// Output only. The resource name of the product group view.
    /// Product group view resource names have the form:
    ///
    /// `customers/{customer_id}/productGroupViews/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
// Proto file describing the Recommendation resource.

/// A recommendation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Recommendation {
    /// Immutable. The resource name of the recommendation.
    ///
    /// `customers/{customer_id}/recommendations/{recommendation_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The type of recommendation.
    #[prost(
        enumeration = "super::enums::recommendation_type_enum::RecommendationType",
        tag = "2"
    )]
    pub r#type: i32,
    /// Output only. The impact on account performance as a result of applying the
    /// recommendation.
    #[prost(message, optional, tag = "3")]
    pub impact: ::core::option::Option<recommendation::RecommendationImpact>,
    /// Output only. The budget targeted by this recommendation. This will be set only when
    /// the recommendation affects a single campaign budget.
    ///
    /// This field will be set for the following recommendation types:
    /// CAMPAIGN_BUDGET, FORECASTING_CAMPAIGN_BUDGET, MARGINAL_ROI_CAMPAIGN_BUDGET,
    /// MOVE_UNUSED_BUDGET
    #[prost(string, optional, tag = "24")]
    pub campaign_budget: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The campaign targeted by this recommendation. This will be set only when
    /// the recommendation affects a single campaign.
    ///
    /// This field will be set for the following recommendation types:
    /// CALL_EXTENSION, CALLOUT_EXTENSION, ENHANCED_CPC_OPT_IN,
    /// USE_BROAD_MATCH_KEYWORD, KEYWORD, KEYWORD_MATCH_TYPE,
    /// MAXIMIZE_CLICKS_OPT_IN, MAXIMIZE_CONVERSIONS_OPT_IN, OPTIMIZE_AD_ROTATION,
    /// RESPONSIVE_SEARCH_AD, RESPONSIVE_SEARCH_AD_ASSET, SEARCH_PARTNERS_OPT_IN,
    /// SITELINK_EXTENSION, TARGET_CPA_OPT_IN, TARGET_ROAS_OPT_IN, TEXT_AD
    #[prost(string, optional, tag = "25")]
    pub campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The ad group targeted by this recommendation. This will be set only when
    /// the recommendation affects a single ad group.
    ///
    /// This field will be set for the following recommendation types:
    /// KEYWORD, OPTIMIZE_AD_ROTATION, RESPONSIVE_SEARCH_AD,
    /// RESPONSIVE_SEARCH_AD_ASSET, TEXT_AD
    #[prost(string, optional, tag = "26")]
    pub ad_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Whether the recommendation is dismissed or not.
    #[prost(bool, optional, tag = "27")]
    pub dismissed: ::core::option::Option<bool>,
    /// The details of recommendation.
    #[prost(
        oneof = "recommendation::Recommendation",
        tags = "4, 22, 8, 9, 10, 11, 12, 14, 15, 16, 17, 18, 19, 20, 21, 23, 28, 29, 30, 31"
    )]
    pub recommendation: ::core::option::Option<recommendation::Recommendation>,
}
/// Nested message and enum types in `Recommendation`.
pub mod recommendation {
    /// The impact of making the change as described in the recommendation.
    /// Some types of recommendations may not have impact information.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RecommendationImpact {
        /// Output only. Base metrics at the time the recommendation was generated.
        #[prost(message, optional, tag = "1")]
        pub base_metrics: ::core::option::Option<RecommendationMetrics>,
        /// Output only. Estimated metrics if the recommendation is applied.
        #[prost(message, optional, tag = "2")]
        pub potential_metrics: ::core::option::Option<RecommendationMetrics>,
    }
    /// Weekly account performance metrics. For some recommendation types, these
    /// are averaged over the past 90-day period and hence can be fractional.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RecommendationMetrics {
        /// Output only. Number of ad impressions.
        #[prost(double, optional, tag = "6")]
        pub impressions: ::core::option::Option<f64>,
        /// Output only. Number of ad clicks.
        #[prost(double, optional, tag = "7")]
        pub clicks: ::core::option::Option<f64>,
        /// Output only. Cost (in micros) for advertising, in the local currency for the account.
        #[prost(int64, optional, tag = "8")]
        pub cost_micros: ::core::option::Option<i64>,
        /// Output only. Number of conversions.
        #[prost(double, optional, tag = "9")]
        pub conversions: ::core::option::Option<f64>,
        /// Output only. Number of video views for a video ad campaign.
        #[prost(double, optional, tag = "10")]
        pub video_views: ::core::option::Option<f64>,
    }
    /// The budget recommendation for budget constrained campaigns.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CampaignBudgetRecommendation {
        /// Output only. The current budget amount in micros.
        #[prost(int64, optional, tag = "7")]
        pub current_budget_amount_micros: ::core::option::Option<i64>,
        /// Output only. The recommended budget amount in micros.
        #[prost(int64, optional, tag = "8")]
        pub recommended_budget_amount_micros: ::core::option::Option<i64>,
        /// Output only. The budget amounts and associated impact estimates for some values of
        /// possible budget amounts.
        #[prost(message, repeated, tag = "3")]
        pub budget_options: ::prost::alloc::vec::Vec<
            campaign_budget_recommendation::CampaignBudgetRecommendationOption,
        >,
    }
    /// Nested message and enum types in `CampaignBudgetRecommendation`.
    pub mod campaign_budget_recommendation {
        /// The impact estimates for a given budget amount.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CampaignBudgetRecommendationOption {
            /// Output only. The budget amount for this option.
            #[prost(int64, optional, tag = "3")]
            pub budget_amount_micros: ::core::option::Option<i64>,
            /// Output only. The impact estimate if budget is changed to amount specified in this
            /// option.
            #[prost(message, optional, tag = "2")]
            pub impact: ::core::option::Option<super::RecommendationImpact>,
        }
    }
    /// The keyword recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeywordRecommendation {
        /// Output only. The recommended keyword.
        #[prost(message, optional, tag = "1")]
        pub keyword: ::core::option::Option<super::super::common::KeywordInfo>,
        /// Output only. The recommended CPC (cost-per-click) bid.
        #[prost(int64, optional, tag = "3")]
        pub recommended_cpc_bid_micros: ::core::option::Option<i64>,
    }
    /// The text ad recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextAdRecommendation {
        /// Output only. Recommended ad.
        #[prost(message, optional, tag = "1")]
        pub ad: ::core::option::Option<super::Ad>,
        /// Output only. Creation date of the recommended ad.
        /// YYYY-MM-DD format, e.g., 2018-04-17.
        #[prost(string, optional, tag = "4")]
        pub creation_date: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. Date, if present, is the earliest when the recommendation will be auto
        /// applied.
        /// YYYY-MM-DD format, e.g., 2018-04-17.
        #[prost(string, optional, tag = "5")]
        pub auto_apply_date: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// The Target CPA opt-in recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TargetCpaOptInRecommendation {
        /// Output only. The available goals and corresponding options for Target CPA strategy.
        #[prost(message, repeated, tag = "1")]
        pub options: ::prost::alloc::vec::Vec<
            target_cpa_opt_in_recommendation::TargetCpaOptInRecommendationOption,
        >,
        /// Output only. The recommended average CPA target. See required budget amount and impact
        /// of using this recommendation in options list.
        #[prost(int64, optional, tag = "3")]
        pub recommended_target_cpa_micros: ::core::option::Option<i64>,
    }
    /// Nested message and enum types in `TargetCpaOptInRecommendation`.
    pub mod target_cpa_opt_in_recommendation {
        /// The Target CPA opt-in option with impact estimate.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TargetCpaOptInRecommendationOption {
            /// Output only. The goal achieved by this option.
            #[prost(
                enumeration = "super::super::super::enums::target_cpa_opt_in_recommendation_goal_enum::TargetCpaOptInRecommendationGoal",
                tag = "1"
            )]
            pub goal: i32,
            /// Output only. Average CPA target.
            #[prost(int64, optional, tag = "5")]
            pub target_cpa_micros: ::core::option::Option<i64>,
            /// Output only. The minimum campaign budget, in local currency for the account,
            /// required to achieve the target CPA.
            /// Amount is specified in micros, where one million is equivalent to one
            /// currency unit.
            #[prost(int64, optional, tag = "6")]
            pub required_campaign_budget_amount_micros: ::core::option::Option<i64>,
            /// Output only. The impact estimate if this option is selected.
            #[prost(message, optional, tag = "4")]
            pub impact: ::core::option::Option<super::RecommendationImpact>,
        }
    }
    /// The Maximize Conversions Opt-In recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MaximizeConversionsOptInRecommendation {
        /// Output only. The recommended new budget amount.
        #[prost(int64, optional, tag = "2")]
        pub recommended_budget_amount_micros: ::core::option::Option<i64>,
    }
    /// The Enhanced Cost-Per-Click Opt-In recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnhancedCpcOptInRecommendation {}
    /// The Search Partners Opt-In recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SearchPartnersOptInRecommendation {}
    /// The Maximize Clicks opt-in recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MaximizeClicksOptInRecommendation {
        /// Output only. The recommended new budget amount.
        /// Only set if the current budget is too high.
        #[prost(int64, optional, tag = "2")]
        pub recommended_budget_amount_micros: ::core::option::Option<i64>,
    }
    /// The Optimize Ad Rotation recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OptimizeAdRotationRecommendation {}
    /// The Callout extension recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CalloutExtensionRecommendation {
        /// Output only. Callout extensions recommended to be added.
        #[prost(message, repeated, tag = "1")]
        pub recommended_extensions: ::prost::alloc::vec::Vec<super::super::common::CalloutFeedItem>,
    }
    /// The Sitelink extension recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SitelinkExtensionRecommendation {
        /// Output only. Sitelink extensions recommended to be added.
        #[prost(message, repeated, tag = "1")]
        pub recommended_extensions:
            ::prost::alloc::vec::Vec<super::super::common::SitelinkFeedItem>,
    }
    /// The Call extension recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CallExtensionRecommendation {
        /// Output only. Call extensions recommended to be added.
        #[prost(message, repeated, tag = "1")]
        pub recommended_extensions: ::prost::alloc::vec::Vec<super::super::common::CallFeedItem>,
    }
    /// The keyword match type recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeywordMatchTypeRecommendation {
        /// Output only. The existing keyword where the match type should be more broad.
        #[prost(message, optional, tag = "1")]
        pub keyword: ::core::option::Option<super::super::common::KeywordInfo>,
        /// Output only. The recommended new match type.
        #[prost(
            enumeration = "super::super::enums::keyword_match_type_enum::KeywordMatchType",
            tag = "2"
        )]
        pub recommended_match_type: i32,
    }
    /// The move unused budget recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MoveUnusedBudgetRecommendation {
        /// Output only. The excess budget's resource_name.
        #[prost(string, optional, tag = "3")]
        pub excess_campaign_budget: ::core::option::Option<::prost::alloc::string::String>,
        /// Output only. The recommendation for the constrained budget to increase.
        #[prost(message, optional, tag = "2")]
        pub budget_recommendation: ::core::option::Option<CampaignBudgetRecommendation>,
    }
    /// The Target ROAS opt-in recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TargetRoasOptInRecommendation {
        /// Output only. The recommended target ROAS (revenue per unit of spend).
        /// The value is between 0.01 and 1000.0, inclusive.
        #[prost(double, optional, tag = "1")]
        pub recommended_target_roas: ::core::option::Option<f64>,
        /// Output only. The minimum campaign budget, in local currency for the account,
        /// required to achieve the target ROAS.
        /// Amount is specified in micros, where one million is equivalent to one
        /// currency unit.
        #[prost(int64, optional, tag = "2")]
        pub required_campaign_budget_amount_micros: ::core::option::Option<i64>,
    }
    /// The add responsive search ad asset recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResponsiveSearchAdAssetRecommendation {
        /// Output only. The current ad to be updated.
        #[prost(message, optional, tag = "1")]
        pub current_ad: ::core::option::Option<super::Ad>,
        /// Output only. The recommended assets. This is populated only with the new headlines
        /// and/or descriptions, and is otherwise empty.
        #[prost(message, optional, tag = "2")]
        pub recommended_assets: ::core::option::Option<super::Ad>,
    }
    /// The add responsive search ad recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResponsiveSearchAdRecommendation {
        /// Output only. Recommended ad.
        #[prost(message, optional, tag = "1")]
        pub ad: ::core::option::Option<super::Ad>,
    }
    /// The use broad match keyword recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UseBroadMatchKeywordRecommendation {
        /// Output only. Sample of keywords to be expanded to Broad Match.
        #[prost(message, repeated, tag = "1")]
        pub keyword: ::prost::alloc::vec::Vec<super::super::common::KeywordInfo>,
        /// Output only. Total number of keywords to be expanded to Broad Match in the campaign.
        #[prost(int64, tag = "2")]
        pub suggested_keywords_count: i64,
        /// Output only. Total number of keywords in the campaign.
        #[prost(int64, tag = "3")]
        pub campaign_keywords_count: i64,
        /// Output only. Whether the associated campaign uses a shared budget.
        #[prost(bool, tag = "4")]
        pub campaign_uses_shared_budget: bool,
        /// Output only. The budget recommended to avoid becoming budget constrained after
        /// applying the recommendation.
        #[prost(int64, tag = "5")]
        pub required_campaign_budget_amount_micros: i64,
    }
    /// The details of recommendation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Recommendation {
        /// Output only. The campaign budget recommendation.
        #[prost(message, tag = "4")]
        CampaignBudgetRecommendation(CampaignBudgetRecommendation),
        /// Output only. The forecasting campaign budget recommendation.
        #[prost(message, tag = "22")]
        ForecastingCampaignBudgetRecommendation(CampaignBudgetRecommendation),
        /// Output only. The keyword recommendation.
        #[prost(message, tag = "8")]
        KeywordRecommendation(KeywordRecommendation),
        /// Output only. Add expanded text ad recommendation.
        #[prost(message, tag = "9")]
        TextAdRecommendation(TextAdRecommendation),
        /// Output only. The TargetCPA opt-in recommendation.
        #[prost(message, tag = "10")]
        TargetCpaOptInRecommendation(TargetCpaOptInRecommendation),
        /// Output only. The MaximizeConversions Opt-In recommendation.
        #[prost(message, tag = "11")]
        MaximizeConversionsOptInRecommendation(MaximizeConversionsOptInRecommendation),
        /// Output only. The Enhanced Cost-Per-Click Opt-In recommendation.
        #[prost(message, tag = "12")]
        EnhancedCpcOptInRecommendation(EnhancedCpcOptInRecommendation),
        /// Output only. The Search Partners Opt-In recommendation.
        #[prost(message, tag = "14")]
        SearchPartnersOptInRecommendation(SearchPartnersOptInRecommendation),
        /// Output only. The MaximizeClicks Opt-In recommendation.
        #[prost(message, tag = "15")]
        MaximizeClicksOptInRecommendation(MaximizeClicksOptInRecommendation),
        /// Output only. The Optimize Ad Rotation recommendation.
        #[prost(message, tag = "16")]
        OptimizeAdRotationRecommendation(OptimizeAdRotationRecommendation),
        /// Output only. The Callout extension recommendation.
        #[prost(message, tag = "17")]
        CalloutExtensionRecommendation(CalloutExtensionRecommendation),
        /// Output only. The Sitelink extension recommendation.
        #[prost(message, tag = "18")]
        SitelinkExtensionRecommendation(SitelinkExtensionRecommendation),
        /// Output only. The Call extension recommendation.
        #[prost(message, tag = "19")]
        CallExtensionRecommendation(CallExtensionRecommendation),
        /// Output only. The keyword match type recommendation.
        #[prost(message, tag = "20")]
        KeywordMatchTypeRecommendation(KeywordMatchTypeRecommendation),
        /// Output only. The move unused budget recommendation.
        #[prost(message, tag = "21")]
        MoveUnusedBudgetRecommendation(MoveUnusedBudgetRecommendation),
        /// Output only. The Target ROAS opt-in recommendation.
        #[prost(message, tag = "23")]
        TargetRoasOptInRecommendation(TargetRoasOptInRecommendation),
        /// Output only. The add responsive search ad recommendation.
        #[prost(message, tag = "28")]
        ResponsiveSearchAdRecommendation(ResponsiveSearchAdRecommendation),
        /// Output only. The marginal ROI campaign budget recommendation.
        #[prost(message, tag = "29")]
        MarginalRoiCampaignBudgetRecommendation(CampaignBudgetRecommendation),
        /// Output only. The use broad match keyword recommendation.
        #[prost(message, tag = "30")]
        UseBroadMatchKeywordRecommendation(UseBroadMatchKeywordRecommendation),
        /// Output only. The add responsive search ad asset recommendation.
        #[prost(message, tag = "31")]
        ResponsiveSearchAdAssetRecommendation(ResponsiveSearchAdAssetRecommendation),
    }
}
// Proto file describing the Remarketing Action resource.

/// A remarketing action. A snippet of JavaScript code that will collect the
/// product id and the type of page people visited (product page, shopping cart
/// page, purchase page, general site visit) on an advertiser's website.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemarketingAction {
    /// Immutable. The resource name of the remarketing action.
    /// Remarketing action resource names have the form:
    ///
    /// `customers/{customer_id}/remarketingActions/{remarketing_action_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. Id of the remarketing action.
    #[prost(int64, optional, tag = "5")]
    pub id: ::core::option::Option<i64>,
    /// The name of the remarketing action.
    ///
    /// This field is required and should not be empty when creating new
    /// remarketing actions.
    #[prost(string, optional, tag = "6")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The snippets used for tracking remarketing actions.
    #[prost(message, repeated, tag = "4")]
    pub tag_snippets: ::prost::alloc::vec::Vec<super::common::TagSnippet>,
}
// Proto file describing the SearchTermView resource.

/// A search term view with metrics aggregated by search term at the ad group
/// level.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTermView {
    /// Output only. The resource name of the search term view.
    /// Search term view resource names have the form:
    ///
    /// `customers/{customer_id}/searchTermViews/{campaign_id}~{ad_group_id}~{URL-base64_search_term}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The search term.
    #[prost(string, optional, tag = "5")]
    pub search_term: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The ad group the search term served in.
    #[prost(string, optional, tag = "6")]
    pub ad_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Indicates whether the search term is currently one of your
    /// targeted or excluded keywords.
    #[prost(
        enumeration = "super::enums::search_term_targeting_status_enum::SearchTermTargetingStatus",
        tag = "4"
    )]
    pub status: i32,
}
// Proto file describing the SharedCriterion resource.

/// A criterion belonging to a shared set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedCriterion {
    /// Immutable. The resource name of the shared criterion.
    /// Shared set resource names have the form:
    ///
    /// `customers/{customer_id}/sharedCriteria/{shared_set_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The shared set to which the shared criterion belongs.
    #[prost(string, optional, tag = "10")]
    pub shared_set: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The ID of the criterion.
    ///
    /// This field is ignored for mutates.
    #[prost(int64, optional, tag = "11")]
    pub criterion_id: ::core::option::Option<i64>,
    /// Output only. The type of the criterion.
    #[prost(
        enumeration = "super::enums::criterion_type_enum::CriterionType",
        tag = "4"
    )]
    pub r#type: i32,
    /// The criterion.
    ///
    /// Exactly one must be set.
    #[prost(oneof = "shared_criterion::Criterion", tags = "3, 5, 6, 7, 8, 9")]
    pub criterion: ::core::option::Option<shared_criterion::Criterion>,
}
/// Nested message and enum types in `SharedCriterion`.
pub mod shared_criterion {
    /// The criterion.
    ///
    /// Exactly one must be set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Criterion {
        /// Immutable. Keyword.
        #[prost(message, tag = "3")]
        Keyword(super::super::common::KeywordInfo),
        /// Immutable. YouTube Video.
        #[prost(message, tag = "5")]
        YoutubeVideo(super::super::common::YouTubeVideoInfo),
        /// Immutable. YouTube Channel.
        #[prost(message, tag = "6")]
        YoutubeChannel(super::super::common::YouTubeChannelInfo),
        /// Immutable. Placement.
        #[prost(message, tag = "7")]
        Placement(super::super::common::PlacementInfo),
        /// Immutable. Mobile App Category.
        #[prost(message, tag = "8")]
        MobileAppCategory(super::super::common::MobileAppCategoryInfo),
        /// Immutable. Mobile application.
        #[prost(message, tag = "9")]
        MobileApplication(super::super::common::MobileApplicationInfo),
    }
}
// Proto file describing the SharedSet resource.

/// SharedSets are used for sharing criterion exclusions across multiple
/// campaigns.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedSet {
    /// Immutable. The resource name of the shared set.
    /// Shared set resource names have the form:
    ///
    /// `customers/{customer_id}/sharedSets/{shared_set_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of this shared set. Read only.
    #[prost(int64, optional, tag = "8")]
    pub id: ::core::option::Option<i64>,
    /// Immutable. The type of this shared set: each shared set holds only a single kind
    /// of resource. Required. Immutable.
    #[prost(
        enumeration = "super::enums::shared_set_type_enum::SharedSetType",
        tag = "3"
    )]
    pub r#type: i32,
    /// The name of this shared set. Required.
    /// Shared Sets must have names that are unique among active shared sets of
    /// the same type.
    /// The length of this string should be between 1 and 255 UTF-8 bytes,
    /// inclusive.
    #[prost(string, optional, tag = "9")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The status of this shared set. Read only.
    #[prost(
        enumeration = "super::enums::shared_set_status_enum::SharedSetStatus",
        tag = "5"
    )]
    pub status: i32,
    /// Output only. The number of shared criteria within this shared set. Read only.
    #[prost(int64, optional, tag = "10")]
    pub member_count: ::core::option::Option<i64>,
    /// Output only. The number of campaigns associated with this shared set. Read only.
    #[prost(int64, optional, tag = "11")]
    pub reference_count: ::core::option::Option<i64>,
}
// Proto file describing the ShoppingPerformanceView resource.

/// Shopping performance view.
/// Provides Shopping campaign statistics aggregated at several product dimension
/// levels. Product dimension values from Merchant Center such as brand,
/// category, custom attributes, product condition and product type will reflect
/// the state of each dimension as of the date and time when the corresponding
/// event was recorded.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShoppingPerformanceView {
    /// Output only. The resource name of the Shopping performance view.
    /// Shopping performance view resource names have the form:
    /// `customers/{customer_id}/shoppingPerformanceView`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
// Proto file describing the SmartCampaignSearchTermView resource.

/// A Smart campaign search term view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmartCampaignSearchTermView {
    /// Output only. The resource name of the Smart campaign search term view.
    /// Smart campaign search term view resource names have the form:
    ///
    /// `customers/{customer_id}/smartCampaignSearchTermViews/{campaign_id}~{URL-base64_search_term}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The search term.
    #[prost(string, tag = "2")]
    pub search_term: ::prost::alloc::string::String,
    /// Output only. The Smart campaign the search term served in.
    #[prost(string, tag = "3")]
    pub campaign: ::prost::alloc::string::String,
}
// Proto file describing the Smart campaign setting resource.

/// Settings for configuring Smart campaigns.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmartCampaignSetting {
    /// Immutable. The resource name of the Smart campaign setting.
    /// Smart campaign setting resource names have the form:
    ///
    /// `customers/{customer_id}/smartCampaignSettings/{campaign_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The campaign to which these settings apply.
    #[prost(string, tag = "2")]
    pub campaign: ::prost::alloc::string::String,
    /// Phone number and country code.
    #[prost(message, optional, tag = "3")]
    pub phone_number: ::core::option::Option<smart_campaign_setting::PhoneNumber>,
    /// Landing page url given by user for this Campaign.
    #[prost(string, tag = "4")]
    pub final_url: ::prost::alloc::string::String,
    /// The ISO-639-1 language code to advertise in.
    #[prost(string, tag = "7")]
    pub advertising_language_code: ::prost::alloc::string::String,
    /// The business setting of this campaign.
    #[prost(oneof = "smart_campaign_setting::BusinessSetting", tags = "5, 6")]
    pub business_setting: ::core::option::Option<smart_campaign_setting::BusinessSetting>,
}
/// Nested message and enum types in `SmartCampaignSetting`.
pub mod smart_campaign_setting {
    /// Phone number and country code in smart campaign settings.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PhoneNumber {
        /// Phone number of the smart campaign.
        #[prost(string, optional, tag = "1")]
        pub phone_number: ::core::option::Option<::prost::alloc::string::String>,
        /// Upper-case, two-letter country code as defined by ISO-3166.
        #[prost(string, optional, tag = "2")]
        pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// The business setting of this campaign.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BusinessSetting {
        /// The name of the business.
        #[prost(string, tag = "5")]
        BusinessName(::prost::alloc::string::String),
        /// The ID of the Business Profile location.
        /// The location ID can be fetched by Business Profile API with its form:
        /// accounts/{accountId}/locations/{locationId}. The last {locationId}
        /// component from the Business Profile API represents the
        /// business_location_id. See the [Business Profile API]
        /// (<https://developers.google.com/my-business/reference/rest/v4/accounts.locations>)
        #[prost(int64, tag = "6")]
        BusinessLocationId(i64),
    }
}
/// A data sharing connection, allowing the import of third party app analytics
/// into a Google Ads Customer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThirdPartyAppAnalyticsLink {
    /// Immutable. The resource name of the third party app analytics link.
    /// Third party app analytics link resource names have the form:
    ///
    /// `customers/{customer_id}/thirdPartyAppAnalyticsLinks/{account_link_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The shareable link ID that should be provided to the third party when
    /// setting up app analytics. This is able to be regenerated using regenerate
    /// method in the ThirdPartyAppAnalyticsLinkService.
    #[prost(string, optional, tag = "3")]
    pub shareable_link_id: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the Topic Constant resource.

/// Use topics to target or exclude placements in the Google Display Network
/// based on the category into which the placement falls (for example,
/// "Pets & Animals/Pets/Dogs").
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicConstant {
    /// Output only. The resource name of the topic constant.
    /// topic constant resource names have the form:
    ///
    /// `topicConstants/{topic_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the topic.
    #[prost(int64, optional, tag = "5")]
    pub id: ::core::option::Option<i64>,
    /// Output only. Resource name of parent of the topic constant.
    #[prost(string, optional, tag = "6")]
    pub topic_constant_parent: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The category to target or exclude. Each subsequent element in the array
    /// describes a more specific sub-category. For example,
    /// {"Pets & Animals", "Pets", "Dogs"} represents the
    /// "Pets & Animals/Pets/Dogs" category. List of available topic categories at
    /// <https://developers.google.com/adwords/api/docs/appendix/verticals>
    #[prost(string, repeated, tag = "7")]
    pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
// Proto file describing the topic view resource.

/// A topic view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicView {
    /// Output only. The resource name of the topic view.
    /// Topic view resource names have the form:
    ///
    /// `customers/{customer_id}/topicViews/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
// Proto file describing the User Interest resource.

/// A user interest: a particular interest-based vertical to be targeted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInterest {
    /// Output only. The resource name of the user interest.
    /// User interest resource names have the form:
    ///
    /// `customers/{customer_id}/userInterests/{user_interest_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. Taxonomy type of the user interest.
    #[prost(
        enumeration = "super::enums::user_interest_taxonomy_type_enum::UserInterestTaxonomyType",
        tag = "2"
    )]
    pub taxonomy_type: i32,
    /// Output only. The ID of the user interest.
    #[prost(int64, optional, tag = "8")]
    pub user_interest_id: ::core::option::Option<i64>,
    /// Output only. The name of the user interest.
    #[prost(string, optional, tag = "9")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The parent of the user interest.
    #[prost(string, optional, tag = "10")]
    pub user_interest_parent: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. True if the user interest is launched to all channels and locales.
    #[prost(bool, optional, tag = "11")]
    pub launched_to_all: ::core::option::Option<bool>,
    /// Output only. Availability information of the user interest.
    #[prost(message, repeated, tag = "7")]
    pub availabilities: ::prost::alloc::vec::Vec<super::common::CriterionCategoryAvailability>,
}
// Proto file describing the User List resource.

/// A user list. This is a list of users a customer may target.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserList {
    /// Immutable. The resource name of the user list.
    /// User list resource names have the form:
    ///
    /// `customers/{customer_id}/userLists/{user_list_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. Id of the user list.
    #[prost(int64, optional, tag = "25")]
    pub id: ::core::option::Option<i64>,
    /// Output only. A flag that indicates if a user may edit a list. Depends on the list
    /// ownership and list type. For example, external remarketing user lists are
    /// not editable.
    ///
    /// This field is read-only.
    #[prost(bool, optional, tag = "26")]
    pub read_only: ::core::option::Option<bool>,
    /// Name of this user list. Depending on its access_reason, the user list name
    /// may not be unique (e.g. if access_reason=SHARED)
    #[prost(string, optional, tag = "27")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Description of this user list.
    #[prost(string, optional, tag = "28")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Membership status of this user list. Indicates whether a user list is open
    /// or active. Only open user lists can accumulate more users and can be
    /// targeted to.
    #[prost(
        enumeration = "super::enums::user_list_membership_status_enum::UserListMembershipStatus",
        tag = "6"
    )]
    pub membership_status: i32,
    /// An ID from external system. It is used by user list sellers to correlate
    /// IDs on their systems.
    #[prost(string, optional, tag = "29")]
    pub integration_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Number of days a user's cookie stays on your list since its most recent
    /// addition to the list. This field must be between 0 and 540 inclusive.
    /// However, for CRM based userlists, this field can be set to 10000 which
    /// means no expiration.
    ///
    /// It'll be ignored for logical_user_list.
    #[prost(int64, optional, tag = "30")]
    pub membership_life_span: ::core::option::Option<i64>,
    /// Output only. Estimated number of users in this user list, on the Google Display Network.
    /// This value is null if the number of users has not yet been determined.
    ///
    /// This field is read-only.
    #[prost(int64, optional, tag = "31")]
    pub size_for_display: ::core::option::Option<i64>,
    /// Output only. Size range in terms of number of users of the UserList, on the Google
    /// Display Network.
    ///
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::user_list_size_range_enum::UserListSizeRange",
        tag = "10"
    )]
    pub size_range_for_display: i32,
    /// Output only. Estimated number of users in this user list in the google.com domain.
    /// These are the users available for targeting in Search campaigns.
    /// This value is null if the number of users has not yet been determined.
    ///
    /// This field is read-only.
    #[prost(int64, optional, tag = "32")]
    pub size_for_search: ::core::option::Option<i64>,
    /// Output only. Size range in terms of number of users of the UserList, for Search ads.
    ///
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::user_list_size_range_enum::UserListSizeRange",
        tag = "12"
    )]
    pub size_range_for_search: i32,
    /// Output only. Type of this list.
    ///
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::user_list_type_enum::UserListType",
        tag = "13"
    )]
    pub r#type: i32,
    /// Indicating the reason why this user list membership status is closed. It is
    /// only populated on lists that were automatically closed due to inactivity,
    /// and will be cleared once the list membership status becomes open.
    #[prost(
        enumeration = "super::enums::user_list_closing_reason_enum::UserListClosingReason",
        tag = "14"
    )]
    pub closing_reason: i32,
    /// Output only. Indicates the reason this account has been granted access to the list.
    /// The reason can be SHARED, OWNED, LICENSED or SUBSCRIBED.
    ///
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::access_reason_enum::AccessReason",
        tag = "15"
    )]
    pub access_reason: i32,
    /// Indicates if this share is still enabled. When a UserList is shared with
    /// the user this field is set to ENABLED. Later the userList owner can decide
    /// to revoke the share and make it DISABLED.
    /// The default value of this field is set to ENABLED.
    #[prost(
        enumeration = "super::enums::user_list_access_status_enum::UserListAccessStatus",
        tag = "16"
    )]
    pub account_user_list_status: i32,
    /// Indicates if this user list is eligible for Google Search Network.
    #[prost(bool, optional, tag = "33")]
    pub eligible_for_search: ::core::option::Option<bool>,
    /// Output only. Indicates this user list is eligible for Google Display Network.
    ///
    /// This field is read-only.
    #[prost(bool, optional, tag = "34")]
    pub eligible_for_display: ::core::option::Option<bool>,
    /// Output only. Indicates match rate for Customer Match lists. The range of this field is
    /// \[0-100\]. This will be null for other list types or when it's not possible
    /// to calculate the match rate.
    ///
    /// This field is read-only.
    #[prost(int32, optional, tag = "24")]
    pub match_rate_percentage: ::core::option::Option<i32>,
    /// The user list.
    ///
    /// Exactly one must be set.
    #[prost(oneof = "user_list::UserList", tags = "19, 20, 21, 22, 23")]
    pub user_list: ::core::option::Option<user_list::UserList>,
}
/// Nested message and enum types in `UserList`.
pub mod user_list {
    /// The user list.
    ///
    /// Exactly one must be set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum UserList {
        /// User list of CRM users provided by the advertiser.
        #[prost(message, tag = "19")]
        CrmBasedUserList(super::super::common::CrmBasedUserListInfo),
        /// Output only. User list which are similar to users from another UserList.
        /// These lists are readonly and automatically created by google.
        #[prost(message, tag = "20")]
        SimilarUserList(super::super::common::SimilarUserListInfo),
        /// User list generated by a rule.
        #[prost(message, tag = "21")]
        RuleBasedUserList(super::super::common::RuleBasedUserListInfo),
        /// User list that is a custom combination of user lists and user interests.
        #[prost(message, tag = "22")]
        LogicalUserList(super::super::common::LogicalUserListInfo),
        /// User list targeting as a collection of conversion or remarketing actions.
        #[prost(message, tag = "23")]
        BasicUserList(super::super::common::BasicUserListInfo),
    }
}
// Proto file describing the user location view resource.

/// A user location view.
///
/// User Location View includes all metrics aggregated at the country level,
/// one row per country. It reports metrics at the actual physical location of
/// the user by targeted or not targeted location. If other segment fields are
/// used, you may get more than one row per country.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserLocationView {
    /// Output only. The resource name of the user location view.
    /// UserLocation view resource names have the form:
    ///
    /// `customers/{customer_id}/userLocationViews/{country_criterion_id}~{targeting_location}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. Criterion Id for the country.
    #[prost(int64, optional, tag = "4")]
    pub country_criterion_id: ::core::option::Option<i64>,
    /// Output only. Indicates whether location was targeted or not.
    #[prost(bool, optional, tag = "5")]
    pub targeting_location: ::core::option::Option<bool>,
}
// Proto file describing the video resource.

/// A video.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Video {
    /// Output only. The resource name of the video.
    /// Video resource names have the form:
    ///
    /// `customers/{customer_id}/videos/{video_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the video.
    #[prost(string, optional, tag = "6")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The owner channel id of the video.
    #[prost(string, optional, tag = "7")]
    pub channel_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The duration of the video in milliseconds.
    #[prost(int64, optional, tag = "8")]
    pub duration_millis: ::core::option::Option<i64>,
    /// Output only. The title of the video.
    #[prost(string, optional, tag = "9")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing the webpage view resource.

/// A webpage view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebpageView {
    /// Output only. The resource name of the webpage view.
    /// Webpage view resource names have the form:
    ///
    /// `customers/{customer_id}/webpageViews/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
