/// A Google Cloud Platform account that identifies support eligibility for a
/// Cloud resource. Currently the Cloud resource can only be an Organization
/// but this might change in future.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SupportAccount {
    /// The resource name for a support account in format
    /// `supportAccounts/{account_id}`.
    /// Output only.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Identifier for this entity that gets persisted in storage system. The
    /// resource name is populated using this field in format
    /// `supportAccounts/{account_id}`.
    #[prost(string, tag = "2")]
    pub account_id: ::prost::alloc::string::String,
    /// The Cloud resource with which this support account is associated.
    #[prost(string, tag = "3")]
    pub cloud_resource: ::prost::alloc::string::String,
    /// A user friendly display name assigned to this support account.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// Indicates the current state of an account.
    #[prost(enumeration = "support_account::State", tag = "5")]
    pub state: i32,
    /// Time when this account was created.
    /// Output only.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The resource name of a billing account associated with this support
    /// account. For example, `billingAccounts/ABCDEF-012345-567890`.
    #[prost(string, tag = "7")]
    pub billing_account_name: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub unify_account_id: ::prost::alloc::string::String,
    /// The PricingModel applicable to this support account.
    #[prost(enumeration = "support_account::PricingModel", tag = "9")]
    pub pricing_model: i32,
}
/// Nested message and enum types in `SupportAccount`.
pub mod support_account {
    /// The current state of this SupportAccount.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Account is in an unknown state.
        Unspecified = 0,
        /// Account is in an active state.
        Active = 1,
        /// Account has been created but is being provisioned in support systems.
        Pending = 2,
        /// Account deletion has been requested by the user.
        PendingDeletion = 3,
    }
    /// Pricing model applicable to this support account.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PricingModel {
        /// This account is subscribed to an unknown pricing model.
        Unknown = 0,
        /// Package based pricing (Platinum, Gold, Silver, Bronze).
        Packages = 1,
        /// Support charges are calculated based on user seats a.k.a,
        /// "Pick Your Team" model.
        UserRoles = 2,
    }
}
/// A support case created by the user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Case {
    /// The resource name for the Case in format
    /// `supportAccounts/{account_id}/cases/{case_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The short summary of the issue reported in this case.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The board description of issue provided with initial summary.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The product component for which this Case is reported.
    #[prost(string, tag = "4")]
    pub component: ::prost::alloc::string::String,
    /// The product subcomponent for which this Case is reported.
    #[prost(string, tag = "5")]
    pub subcomponent: ::prost::alloc::string::String,
    /// Timezone the client sending this request is in.
    /// It should be in a format IANA recognizes: <https://www.iana.org/time-zone>
    /// There is no additional validation done by the API.
    #[prost(string, tag = "6")]
    pub client_timezone: ::prost::alloc::string::String,
    /// The email addresses that can be copied to receive updates on this case.
    /// Users can specify a maximum of 10 email addresses.
    #[prost(string, repeated, tag = "7")]
    pub cc_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Google Cloud Platform project ID for which this case is created.
    #[prost(string, tag = "8")]
    pub project_id: ::prost::alloc::string::String,
    /// List of customer issues associated with this case.
    #[prost(message, repeated, tag = "10")]
    pub issues: ::prost::alloc::vec::Vec<CustomerIssue>,
    /// The current priority of this case.
    #[prost(enumeration = "case::Priority", tag = "11")]
    pub priority: i32,
    /// The current state of this case.
    #[prost(enumeration = "case::State", tag = "12")]
    pub state: i32,
    /// Time when this case was created.
    /// Output only.
    #[prost(message, optional, tag = "13")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time when this case was last updated.
    /// Output only.
    #[prost(message, optional, tag = "14")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Email address of user who created this case.
    /// Output only. It is inferred from credentials supplied during case creation.
    #[prost(string, tag = "15")]
    pub creator_email: ::prost::alloc::string::String,
    /// The issue category applicable to this case.
    #[prost(string, tag = "16")]
    pub category: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Case`.
pub mod case {
    /// The case priority with P0 being the most urgent and P4 the least.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Priority {
        /// Priority is undefined or has not been set yet.
        Unspecified = 0,
        /// Extreme impact on a production service - Service is hard down.
        P0 = 1,
        /// Critical impact on a production service - Service is currently unusable.
        P1 = 2,
        /// Severe impact on a production service - Service is usable but greatly
        /// impaired.
        P2 = 3,
        /// Medium impact on a production service - Service is available, but
        /// moderately impaired.
        P3 = 4,
        /// General questions or minor issues - Production service is fully
        /// available.
        P4 = 5,
    }
    /// The state of a case.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Case is in an unknown state.
        Unspecified = 0,
        /// Case has been created but no one is assigned to work on it yet.
        New = 1,
        /// Case has been assigned to a support agent.
        Assigned = 2,
        /// A support agent is currently investigating the case.
        InProgressGoogleSupport = 3,
        /// Case has been forwarded to product team for further investigation.
        InProgressGoogleEng = 4,
        /// Case is under investigation and relates to a known issue.
        InProgressKnownIssue = 5,
        /// Case is waiting for a response from the customer.
        WaitingForCustomerResponse = 6,
        /// A solution has been offered for the case but it isn't closed yet.
        SolutionOffered = 7,
        /// Cases has been fully resolved and is in a closed state.
        Closed = 8,
    }
}
/// Reference to a Google internal ticket used for investigating a support case.
/// Not every support case will have an internal ticket associated with it.
/// A support case can have multiple tickets linked to it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerIssue {
    /// Unique identifier for the internal issue.
    /// Output only.
    #[prost(string, tag = "1")]
    pub issue_id: ::prost::alloc::string::String,
    /// Represents current status of the internal ticket.
    /// Output only.
    #[prost(enumeration = "customer_issue::IssueState", tag = "2")]
    pub state: i32,
    /// Time when the internal issue was created.
    /// Output only.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time when the internal issue was marked as resolved.
    /// Output only.
    #[prost(message, optional, tag = "4")]
    pub resolve_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time when the internal issue was last updated.
    /// Output only.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `CustomerIssue`.
pub mod customer_issue {
    /// The status of a customer issue.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum IssueState {
        /// Issue in an unknown state.
        Unspecified = 0,
        /// Issue is currently open but the work on it has not been started.
        Open = 1,
        /// Issue is currently being worked on.
        InProgress = 2,
        /// Issue is fixed.
        Fixed = 3,
        /// Issue has been marked as invalid.
        WontFix = 4,
        /// Issue verified and in production.
        Verified = 5,
    }
}
/// A message that contains mapping of a user and their role under a support
/// account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SupportRole {
    /// Email address of user being added through this Role.
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    /// The type of role assigned to user.
    #[prost(enumeration = "support_role::Role", tag = "2")]
    pub role: i32,
}
/// Nested message and enum types in `SupportRole`.
pub mod support_role {
    /// A role which determines the support resources and features a user might
    /// get access to.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Role {
        /// An unknown role.
        Unspecified = 0,
        /// The basic support role.
        Basic = 1,
        /// The developer role.
        Developer = 2,
        /// The operation role.
        Operation = 3,
        /// The site reliability role.
        SiteReliability = 4,
    }
}
/// The comment text associated with a `Case`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Comment {
    /// Text containing a maximum of 3000 characters.
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// Time when this update was created.
    /// Output only.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The email address/name of user who created this comment.
    /// Output only.
    #[prost(string, tag = "3")]
    pub author: ::prost::alloc::string::String,
    /// The resource name for this comment in format
    /// `supportAccounts/{account_id}/cases/{case_id}/{comment_id}`.
    /// Output only.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the product component taxonomy that is to be used while creating
/// or updating a `Case`. A client should obtain the list of issue categories,
/// component/subcomponent from this object and specify it in `Case.category`,
/// `Case.component` and `Case.subcomponent` fields respectively.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssueTaxonomy {
    /// Map of available categories.
    #[prost(map = "string, message", tag = "1")]
    pub categories:
        ::std::collections::HashMap<::prost::alloc::string::String, issue_taxonomy::Category>,
}
/// Nested message and enum types in `IssueTaxonomy`.
pub mod issue_taxonomy {
    /// The representation of a product component. It is composed of a canonical
    /// name for the product (e.g., Google App Engine), languages in which a
    /// support ticket can be created under this component, a template that
    /// provides hints on important details to be filled out before submitting a
    /// case. It also contains an embedded list of product subcomponents that have
    /// similar attributes as top-level components.
    /// (e.g., Google App Engine > Memcache).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Component {
        /// User friendly name of this component.
        #[prost(string, tag = "1")]
        pub display_name: ::prost::alloc::string::String,
        /// List of languages in which a support case can be created under this
        /// component. Represented by language codes in ISO_639-1 standard.
        #[prost(string, repeated, tag = "2")]
        pub languages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Template to be used while filling the description of a support case.
        #[prost(string, tag = "3")]
        pub template: ::prost::alloc::string::String,
        /// List of subcomponents under this component.
        #[prost(message, repeated, tag = "4")]
        pub subcomponents: ::prost::alloc::vec::Vec<Component>,
    }
    /// Represents the category of issue (Technical or Non-Technical)
    /// reported through a support case.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Category {
        /// User friendly name of this category.
        #[prost(string, tag = "1")]
        pub display_name: ::prost::alloc::string::String,
        /// Map of product components under this category.
        #[prost(map = "string, message", tag = "2")]
        pub components: ::std::collections::HashMap<::prost::alloc::string::String, Component>,
    }
}
