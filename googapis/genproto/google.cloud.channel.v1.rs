/// Required Edu Attributes
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EduData {
    /// Designated institute type of customer.
    #[prost(enumeration = "edu_data::InstituteType", tag = "1")]
    pub institute_type: i32,
    /// Size of the institute.
    #[prost(enumeration = "edu_data::InstituteSize", tag = "2")]
    pub institute_size: i32,
    /// Web address for the edu customer's institution.
    #[prost(string, tag = "3")]
    pub website: ::prost::alloc::string::String,
}
/// Nested message and enum types in `EduData`.
pub mod edu_data {
    /// Enum to specify the institute type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum InstituteType {
        /// Default value.  This state doesn't show unless an error occurs.
        Unspecified = 0,
        /// Elementary/Secondary Schools & Districts
        K12 = 1,
        /// Higher Education Universities & Colleges
        University = 2,
    }
    /// Number of students and staff the institute has.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum InstituteSize {
        /// Default value. This state doesn't show unless an error occurs.
        Unspecified = 0,
        /// 1 - 100
        Size1100 = 1,
        /// 101 - 500
        Size101500 = 2,
        /// 501 - 1,000
        Size5011000 = 3,
        /// 1,001 - 2,000
        Size10012000 = 4,
        /// 2,001 - 5,000
        Size20015000 = 5,
        /// 5,001 - 10,000
        Size500110000 = 6,
        /// 10,001 +
        Size10001OrMore = 7,
    }
}
/// Cloud Identity information for the Cloud Channel Customer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudIdentityInfo {
    /// CustomerType indicates verification type needed for using services.
    #[prost(enumeration = "cloud_identity_info::CustomerType", tag = "1")]
    pub customer_type: i32,
    /// Output only. The primary domain name.
    #[prost(string, tag = "9")]
    pub primary_domain: ::prost::alloc::string::String,
    /// Whether the domain is verified.
    #[prost(bool, tag = "4")]
    pub is_domain_verified: bool,
    /// The alternate email.
    #[prost(string, tag = "6")]
    pub alternate_email: ::prost::alloc::string::String,
    /// Phone number associated with the Cloud Identity.
    #[prost(string, tag = "7")]
    pub phone_number: ::prost::alloc::string::String,
    /// Language code.
    #[prost(string, tag = "8")]
    pub language_code: ::prost::alloc::string::String,
    /// Output only. URI of Customer's Admin console dashboard.
    #[prost(string, tag = "10")]
    pub admin_console_uri: ::prost::alloc::string::String,
    /// Edu information about the customer.
    #[prost(message, optional, tag = "22")]
    pub edu_data: ::core::option::Option<EduData>,
}
/// Nested message and enum types in `CloudIdentityInfo`.
pub mod cloud_identity_info {
    /// CustomerType of the customer
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomerType {
        /// Default value. This state doesn't show unless an error occurs.
        Unspecified = 0,
        /// Domain-owning customer which needs domain verification to use services.
        Domain = 1,
        /// Team customer which needs email verification to use services.
        Team = 2,
    }
}
/// Data type and value of a parameter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// The kind of value.
    #[prost(oneof = "value::Kind", tags = "1, 2, 3, 4")]
    pub kind: ::core::option::Option<value::Kind>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// The kind of value.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// Represents an int64 value.
        #[prost(int64, tag = "1")]
        Int64Value(i64),
        /// Represents a string value.
        #[prost(string, tag = "2")]
        StringValue(::prost::alloc::string::String),
        /// Represents a double value.
        #[prost(double, tag = "3")]
        DoubleValue(f64),
        /// Represents an 'Any' proto value.
        #[prost(message, tag = "4")]
        ProtoValue(::prost_types::Any),
    }
}
/// Information needed to create an Admin User for Google Workspace.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminUser {
    /// Primary email of the admin user.
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    /// Given name of the admin user.
    #[prost(string, tag = "2")]
    pub given_name: ::prost::alloc::string::String,
    /// Family name of the admin user.
    #[prost(string, tag = "3")]
    pub family_name: ::prost::alloc::string::String,
}
/// Entity representing a link between distributors and their indirect
/// resellers in an n-tier resale channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPartnerLink {
    /// Output only. Resource name for the channel partner link, in the format
    /// accounts/{account_id}/channelPartnerLinks/{id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Cloud Identity ID of the linked reseller.
    #[prost(string, tag = "2")]
    pub reseller_cloud_identity_id: ::prost::alloc::string::String,
    /// Required. State of the channel partner link.
    #[prost(enumeration = "ChannelPartnerLinkState", tag = "3")]
    pub link_state: i32,
    /// Output only. URI of the web page where partner accepts the link invitation.
    #[prost(string, tag = "4")]
    pub invite_link_uri: ::prost::alloc::string::String,
    /// Output only. Timestamp of when the channel partner link is created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp of when the channel partner link is updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Public identifier that a customer must use to generate a transfer token
    /// to move to this distributor-reseller combination.
    #[prost(string, tag = "7")]
    pub public_id: ::prost::alloc::string::String,
    /// Output only. Cloud Identity info of the channel partner (IR).
    #[prost(message, optional, tag = "8")]
    pub channel_partner_cloud_identity_info: ::core::option::Option<CloudIdentityInfo>,
}
/// The level of granularity the [ChannelPartnerLink][google.cloud.channel.v1.ChannelPartnerLink] will display.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChannelPartnerLinkView {
    /// The default / unset value.
    /// The API will default to the BASIC view.
    Unspecified = 0,
    /// Includes all fields except the
    /// [ChannelPartnerLink.channel_partner_cloud_identity_info][google.cloud.channel.v1.ChannelPartnerLink.channel_partner_cloud_identity_info].
    Basic = 1,
    /// Includes all fields.
    Full = 2,
}
/// ChannelPartnerLinkState represents state of a channel partner link.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChannelPartnerLinkState {
    /// The state is not specified.
    Unspecified = 0,
    /// An invitation has been sent to the reseller to create a channel partner
    /// link.
    Invited = 1,
    /// Status when the reseller is active.
    Active = 2,
    /// Status when the reseller has been revoked by the distributor.
    Revoked = 3,
    /// Status when the reseller is suspended by Google or distributor.
    Suspended = 4,
}
/// Entity representing a customer of a reseller or distributor.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Customer {
    /// Output only. Resource name of the customer.
    /// Format: accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Name of the organization that the customer entity represents.
    #[prost(string, tag = "2")]
    pub org_display_name: ::prost::alloc::string::String,
    /// Required. Address of the organization of the customer entity.
    /// Region and zip codes are required to enforce US laws and embargoes.
    /// Language code is discarded. Use the Customer-level language code to set the
    /// customer's language.
    #[prost(message, optional, tag = "3")]
    pub org_postal_address: ::core::option::Option<super::super::super::r#type::PostalAddress>,
    /// Primary contact info.
    #[prost(message, optional, tag = "4")]
    pub primary_contact_info: ::core::option::Option<ContactInfo>,
    /// Secondary contact email.
    /// Alternate email and primary contact email are required to have different
    /// domains if primary contact email is present.
    /// When creating admin.google.com accounts, users get notified credentials at
    /// this email. This email address is also used as a recovery email.
    #[prost(string, tag = "5")]
    pub alternate_email: ::prost::alloc::string::String,
    /// Required. Primary domain used by the customer.
    /// Domain of primary contact email is required to be same as the provided
    /// domain.
    #[prost(string, tag = "6")]
    pub domain: ::prost::alloc::string::String,
    /// Output only. The time at which the customer is created.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which the customer is updated.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Customer's cloud_identity_id.
    /// Populated only if a Cloud Identity resource exists for this customer.
    #[prost(string, tag = "9")]
    pub cloud_identity_id: ::prost::alloc::string::String,
    /// Optional. The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// https://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[prost(string, tag = "10")]
    pub language_code: ::prost::alloc::string::String,
    /// Output only. Cloud Identity information for the customer.
    /// Populated only if a Cloud Identity account exists for this customer.
    #[prost(message, optional, tag = "12")]
    pub cloud_identity_info: ::core::option::Option<CloudIdentityInfo>,
    /// Cloud Identity ID of the customer's channel partner.
    /// Populated only if a channel partner exists for this customer.
    #[prost(string, tag = "13")]
    pub channel_partner_id: ::prost::alloc::string::String,
}
/// Contact information for a customer account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContactInfo {
    /// First name of the contact in the customer account.
    #[prost(string, tag = "1")]
    pub first_name: ::prost::alloc::string::String,
    /// Last name of the contact in the customer account.
    #[prost(string, tag = "2")]
    pub last_name: ::prost::alloc::string::String,
    /// Output only. Display name of the contact in the customer account.
    /// Populated by combining customer first name and last name.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// Email of the contact in the customer account.
    /// Email is required for entitlements that need creation of admin.google.com
    /// accounts. The email will be the username used in credentials to access the
    /// admin.google.com account.
    #[prost(string, tag = "5")]
    pub email: ::prost::alloc::string::String,
    /// Optional. Job title of the contact in the customer account.
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
    /// Phone number of the contact in the customer account.
    #[prost(string, tag = "7")]
    pub phone: ::prost::alloc::string::String,
}
/// A Product is the entity a customer uses when placing an order. For example,
/// Google Workspace, Google Voice, etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Product {
    /// Resource Name of the Product.
    /// Format: products/{product_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Marketing information for the product.
    #[prost(message, optional, tag = "2")]
    pub marketing_info: ::core::option::Option<MarketingInfo>,
}
/// Represents a product's purchasable Stock Keeping Unit (SKU).
/// SKUs represent the different variations of the product. For example, Google
/// Workspace Business Standard and Google Workspace Business Plus are Google
/// Workspace product SKUs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sku {
    /// Resource Name of the SKU.
    /// Format: products/{product_id}/skus/{sku_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Marketing information for the SKU.
    #[prost(message, optional, tag = "2")]
    pub marketing_info: ::core::option::Option<MarketingInfo>,
    /// Product the SKU is associated with.
    #[prost(message, optional, tag = "3")]
    pub product: ::core::option::Option<Product>,
}
/// Represents the marketing information for a Product, SKU or Offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketingInfo {
    /// Human readable name.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// Human readable description. Description can contain HTML.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Default logo.
    #[prost(message, optional, tag = "3")]
    pub default_logo: ::core::option::Option<Media>,
}
/// Represents media information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Media {
    /// Title of the media.
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// URL of the media.
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
    /// Type of the media.
    #[prost(enumeration = "MediaType", tag = "3")]
    pub r#type: i32,
}
/// Type of media used.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MediaType {
    /// Not used.
    Unspecified = 0,
    /// Type of image.
    Image = 1,
}
/// Represents an offer made to resellers for purchase.
/// An offer is associated with a [Sku][google.cloud.channel.v1.Sku], has a plan for payment, a price, and
/// defines the constraints for buying.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Offer {
    /// Resource Name of the Offer.
    /// Format: accounts/{account_id}/offers/{offer_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Marketing information for the Offer.
    #[prost(message, optional, tag = "2")]
    pub marketing_info: ::core::option::Option<MarketingInfo>,
    /// SKU the offer is associated with.
    #[prost(message, optional, tag = "3")]
    pub sku: ::core::option::Option<Sku>,
    /// Describes the payment plan for the Offer.
    #[prost(message, optional, tag = "4")]
    pub plan: ::core::option::Option<Plan>,
    /// Constraints on transacting the Offer.
    #[prost(message, optional, tag = "5")]
    pub constraints: ::core::option::Option<Constraints>,
    /// Price for each monetizable resource type.
    #[prost(message, repeated, tag = "6")]
    pub price_by_resources: ::prost::alloc::vec::Vec<PriceByResource>,
    /// Start of the Offer validity time.
    #[prost(message, optional, tag = "7")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. End of the Offer validity time.
    #[prost(message, optional, tag = "8")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Parameters required to use current Offer to purchase.
    #[prost(message, repeated, tag = "9")]
    pub parameter_definitions: ::prost::alloc::vec::Vec<ParameterDefinition>,
}
/// Parameter's definition. Specifies what parameter is required to use the
/// current Offer to purchase.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParameterDefinition {
    /// Name of the parameter.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Data type of the parameter. Minimal value, Maximum value and allowed values
    /// will use specified data type here.
    #[prost(enumeration = "parameter_definition::ParameterType", tag = "2")]
    pub parameter_type: i32,
    /// Minimal value of the parameter, if applicable. Inclusive. For example,
    /// minimal commitment when purchasing Anthos is 0.01.
    /// Applicable to INT64 and DOUBLE parameter types.
    #[prost(message, optional, tag = "3")]
    pub min_value: ::core::option::Option<Value>,
    /// Maximum value of the parameter, if applicable. Inclusive. For example,
    /// maximum seats when purchasing Google Workspace Business Standard.
    /// Applicable to INT64 and DOUBLE parameter types.
    #[prost(message, optional, tag = "4")]
    pub max_value: ::core::option::Option<Value>,
    /// If not empty, parameter values must be drawn from this list.
    /// For example, [us-west1, us-west2, ...]
    /// Applicable to STRING parameter type.
    #[prost(message, repeated, tag = "5")]
    pub allowed_values: ::prost::alloc::vec::Vec<Value>,
    /// If set to true, parameter is optional to purchase this Offer.
    #[prost(bool, tag = "6")]
    pub optional: bool,
}
/// Nested message and enum types in `ParameterDefinition`.
pub mod parameter_definition {
    /// Data type of the parameter.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ParameterType {
        /// Not used.
        Unspecified = 0,
        /// Int64 type.
        Int64 = 1,
        /// String type.
        String = 2,
        /// Double type.
        Double = 3,
    }
}
/// Represents the constraints for buying the Offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Constraints {
    /// Represents constraints required to purchase the Offer for a customer.
    #[prost(message, optional, tag = "1")]
    pub customer_constraints: ::core::option::Option<CustomerConstraints>,
}
/// Represents constraints required to purchase the Offer for a customer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerConstraints {
    /// Allowed geographical regions of the customer.
    #[prost(string, repeated, tag = "1")]
    pub allowed_regions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Allowed Customer Type.
    #[prost(enumeration = "cloud_identity_info::CustomerType", repeated, tag = "2")]
    pub allowed_customer_types: ::prost::alloc::vec::Vec<i32>,
    /// Allowed Promotional Order Type. Present for Promotional offers.
    #[prost(enumeration = "PromotionalOrderType", repeated, tag = "3")]
    pub promotional_order_types: ::prost::alloc::vec::Vec<i32>,
}
/// The payment plan for the Offer. Describes how to make a payment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Plan {
    /// Describes how a reseller will be billed.
    #[prost(enumeration = "PaymentPlan", tag = "1")]
    pub payment_plan: i32,
    /// Specifies when the payment needs to happen.
    #[prost(enumeration = "PaymentType", tag = "2")]
    pub payment_type: i32,
    /// Describes how frequently the reseller will be billed, such as
    /// once per month.
    #[prost(message, optional, tag = "3")]
    pub payment_cycle: ::core::option::Option<Period>,
    /// Present for Offers with a trial period.
    /// For trial-only Offers, a paid service needs to start before the trial
    /// period ends for continued service.
    /// For Regular Offers with a trial period, the regular pricing goes into
    /// effect when trial period ends, or if paid service is started before the end
    /// of the trial period.
    #[prost(message, optional, tag = "4")]
    pub trial_period: ::core::option::Option<Period>,
}
/// Represents price by resource type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceByResource {
    /// Resource Type. Example: SEAT
    #[prost(enumeration = "ResourceType", tag = "1")]
    pub resource_type: i32,
    /// Price of the Offer. Present if there are no price phases.
    #[prost(message, optional, tag = "2")]
    pub price: ::core::option::Option<Price>,
    /// Specifies the price by time range.
    #[prost(message, repeated, tag = "3")]
    pub price_phases: ::prost::alloc::vec::Vec<PricePhase>,
}
/// Represents the price of the Offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Price {
    /// Base price.
    #[prost(message, optional, tag = "1")]
    pub base_price: ::core::option::Option<super::super::super::r#type::Money>,
    /// Discount percentage, represented as decimal.
    /// For example, a 20% discount will be represent as 0.2.
    #[prost(double, tag = "2")]
    pub discount: f64,
    /// Effective Price after applying the discounts.
    #[prost(message, optional, tag = "3")]
    pub effective_price: ::core::option::Option<super::super::super::r#type::Money>,
    /// Link to external price list, such as link to Google Voice rate card.
    #[prost(string, tag = "4")]
    pub external_price_uri: ::prost::alloc::string::String,
}
/// Specifies the price by the duration of months.
/// For example, a 20% discount for the first six months, then a 10% discount
/// starting on the seventh month.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PricePhase {
    /// Defines the phase period type.
    #[prost(enumeration = "PeriodType", tag = "1")]
    pub period_type: i32,
    /// Defines first period for the phase.
    #[prost(int32, tag = "2")]
    pub first_period: i32,
    /// Defines first period for the phase.
    #[prost(int32, tag = "3")]
    pub last_period: i32,
    /// Price of the phase. Present if there are no price tiers.
    #[prost(message, optional, tag = "4")]
    pub price: ::core::option::Option<Price>,
    /// Price by the resource tiers.
    #[prost(message, repeated, tag = "5")]
    pub price_tiers: ::prost::alloc::vec::Vec<PriceTier>,
}
/// Defines price at resource tier level.
/// For example, an offer with following definition :
///
/// * Tier 1: Provide 25% discount for all seats between 1 and 25.
/// * Tier 2: Provide 10% discount for all seats between 26 and 100.
/// * Tier 3: Provide flat 15% discount for all seats above 100.
///
/// Each of these tiers is represented as a PriceTier.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceTier {
    /// First resource for which the tier price applies.
    #[prost(int32, tag = "1")]
    pub first_resource: i32,
    /// Last resource for which the tier price applies.
    #[prost(int32, tag = "2")]
    pub last_resource: i32,
    /// Price of the tier.
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<Price>,
}
/// Represents period in days/months/years.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Period {
    /// Total duration of Period Type defined.
    #[prost(int32, tag = "1")]
    pub duration: i32,
    /// Period Type.
    #[prost(enumeration = "PeriodType", tag = "2")]
    pub period_type: i32,
}
/// Constraints type for Promotional offers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PromotionalOrderType {
    /// Not used.
    PromotionalTypeUnspecified = 0,
    /// Order used for new customers, trial conversions and upgrades.
    NewUpgrade = 1,
    /// All orders for transferring an existing customer.
    Transfer = 2,
    /// Orders for modifying an existing customer's promotion on the same SKU.
    PromotionSwitch = 3,
}
/// Describes how the reseller will be billed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PaymentPlan {
    /// Not used.
    Unspecified = 0,
    /// Commitment.
    Commitment = 1,
    /// No commitment.
    Flexible = 2,
    /// Free.
    Free = 3,
    /// Trial.
    Trial = 4,
    /// Price and ordering not available through API.
    Offline = 5,
}
/// Specifies when the payment needs to happen.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PaymentType {
    /// Not used.
    Unspecified = 0,
    /// Prepay. Amount has to be paid before service is rendered.
    Prepay = 1,
    /// Postpay. Reseller is charged at the end of the Payment cycle.
    Postpay = 2,
}
/// Represents the type for a monetizable resource(any entity on which billing
/// happens). For example, this could be MINUTES for Google Voice and GB for
/// Google Drive. One SKU can map to multiple monetizable resources.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResourceType {
    /// Not used.
    Unspecified = 0,
    /// Seat.
    Seat = 1,
    /// Monthly active user.
    Mau = 2,
    /// GB (used for storage SKUs).
    Gb = 3,
    /// Active licensed users(for Voice SKUs).
    LicensedUser = 4,
    /// Voice usage.
    Minutes = 5,
    /// For IaaS SKUs like Google Cloud Platform, monetization is based on usage
    /// accrued on your billing account irrespective of the type of monetizable
    /// resource. This enum represents an aggregated resource/container for all
    /// usage SKUs on a billing account. Currently, only applicable to Google Cloud
    /// Platform.
    IaasUsage = 6,
    /// For Google Cloud Platform subscriptions like Anthos or SAP.
    Subscription = 7,
}
/// Period Type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PeriodType {
    /// Not used.
    Unspecified = 0,
    /// Day.
    Day = 1,
    /// Month.
    Month = 2,
    /// Year.
    Year = 3,
}
/// An entitlement is a representation of a customer's ability to use a service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entitlement {
    /// Output only. Resource name of an entitlement in the form:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time at which the entitlement is created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which the entitlement is updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The offer resource name for which the entitlement is to be
    /// created. Takes the form: accounts/{account_id}/offers/{offer_id}.
    #[prost(string, tag = "8")]
    pub offer: ::prost::alloc::string::String,
    /// Commitment settings for a commitment-based Offer.
    /// Required for commitment based offers.
    #[prost(message, optional, tag = "12")]
    pub commitment_settings: ::core::option::Option<CommitmentSettings>,
    /// Output only. Current provisioning state of the entitlement.
    #[prost(enumeration = "entitlement::ProvisioningState", tag = "13")]
    pub provisioning_state: i32,
    /// Output only. Service provisioning details for the entitlement.
    #[prost(message, optional, tag = "16")]
    pub provisioned_service: ::core::option::Option<ProvisionedService>,
    /// Output only. Enumerable of all current suspension reasons for an entitlement.
    #[prost(
        enumeration = "entitlement::SuspensionReason",
        repeated,
        packed = "false",
        tag = "18"
    )]
    pub suspension_reasons: ::prost::alloc::vec::Vec<i32>,
    /// Optional. This purchase order (PO) information is for resellers to use for their
    /// company tracking usage. If a purchaseOrderId value is given, it appears in
    /// the API responses and shows up in the invoice. The property accepts up to
    /// 80 plain text characters.
    #[prost(string, tag = "19")]
    pub purchase_order_id: ::prost::alloc::string::String,
    /// Output only. Settings for trial offers.
    #[prost(message, optional, tag = "21")]
    pub trial_settings: ::core::option::Option<TrialSettings>,
    /// Association information to other entitlements.
    #[prost(message, optional, tag = "23")]
    pub association_info: ::core::option::Option<AssociationInfo>,
    /// Extended entitlement parameters. When creating an entitlement, valid
    /// parameters' names and values are defined in the offer's parameter
    /// definitions.
    #[prost(message, repeated, tag = "26")]
    pub parameters: ::prost::alloc::vec::Vec<Parameter>,
}
/// Nested message and enum types in `Entitlement`.
pub mod entitlement {
    /// Indicates the current provisioning state of the entitlement.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ProvisioningState {
        /// Default value. This state doesn't show unless an error occurs.
        Unspecified = 0,
        /// The entitlement is currently active.
        Active = 1,
        /// The entitlement is currently suspended.
        Suspended = 5,
    }
    /// Suspension reason for an entitlement if [provisioning_state][google.cloud.channel.v1.Entitlement.provisioning_state] = SUSPENDED.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SuspensionReason {
        /// Default value. This state doesn't show unless an error occurs.
        Unspecified = 0,
        /// Entitlement was manually suspended by the Reseller.
        ResellerInitiated = 1,
        /// Trial ended.
        TrialEnded = 2,
        /// Entitlement renewal was canceled.
        RenewalWithTypeCancel = 3,
        /// Entitlement was automatically suspended on creation for pending ToS
        /// acceptance on customer.
        PendingTosAcceptance = 4,
        /// Other reasons (internal reasons, abuse, etc.).
        Other = 100,
    }
}
/// Definition for extended entitlement parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Parameter {
    /// Name of the parameter.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Value of the parameter.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Value>,
    /// Output only. Specifies whether this parameter is allowed to be changed. For example, for
    /// a Google Workspace Business Starter entitlement in commitment plan,
    /// num_units is editable when entitlement is active.
    #[prost(bool, tag = "3")]
    pub editable: bool,
}
/// Association links that an entitlement has to other entitlements.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociationInfo {
    /// The name of the base entitlement, for which this entitlement is an add-on.
    #[prost(string, tag = "1")]
    pub base_entitlement: ::prost::alloc::string::String,
}
/// Service provisioned for an entitlement.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisionedService {
    /// Output only. Provisioning ID of the entitlement. For Google Workspace, this would be the
    /// underlying Subscription ID.
    #[prost(string, tag = "1")]
    pub provisioning_id: ::prost::alloc::string::String,
    /// Output only. The product pertaining to the provisioning resource as specified in the
    /// Offer.
    #[prost(string, tag = "2")]
    pub product_id: ::prost::alloc::string::String,
    /// Output only. The SKU pertaining to the provisioning resource as specified in the Offer.
    #[prost(string, tag = "3")]
    pub sku_id: ::prost::alloc::string::String,
}
/// Commitment settings for commitment-based offers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitmentSettings {
    /// Output only. Commitment start timestamp.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Commitment end timestamp.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Renewal settings applicable for a commitment-based Offer.
    #[prost(message, optional, tag = "4")]
    pub renewal_settings: ::core::option::Option<RenewalSettings>,
}
/// Renewal settings for renewable Offers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenewalSettings {
    /// If false, the plan will be completed at the end date.
    #[prost(bool, tag = "1")]
    pub enable_renewal: bool,
    /// If true and enable_renewal = true, the unit (for example seats or licenses)
    /// will be set to the number of active units at renewal time.
    #[prost(bool, tag = "2")]
    pub resize_unit_count: bool,
    /// Describes how a reseller will be billed.
    #[prost(enumeration = "PaymentPlan", tag = "5")]
    pub payment_plan: i32,
    /// Describes how frequently the reseller will be billed, such as
    /// once per month.
    #[prost(message, optional, tag = "6")]
    pub payment_cycle: ::core::option::Option<Period>,
}
/// Settings for trial offers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrialSettings {
    /// Determines if the entitlement is in a trial or not:
    ///
    /// * `true` - The entitlement is in trial.
    /// * `false` - The entitlement is not in trial.
    #[prost(bool, tag = "1")]
    pub trial: bool,
    /// Date when the trial ends. The value is in milliseconds
    /// using the UNIX Epoch format. See an example [Epoch
    /// converter](https://www.epochconverter.com).
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// TransferableSku represents information a reseller needs to view existing
/// provisioned services for a customer that they do not own.
/// Read-only.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferableSku {
    /// Whether a transferable SKU is commitment-based or not.
    #[prost(message, optional, tag = "6")]
    pub is_commitment: ::core::option::Option<bool>,
    /// Commitment end timestamp.
    #[prost(message, optional, tag = "7")]
    pub commitment_end_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Describes the transfer eligibility of a SKU.
    #[prost(message, optional, tag = "9")]
    pub transfer_eligibility: ::core::option::Option<TransferEligibility>,
    /// The SKU pertaining to the provisioning resource as specified in the Offer.
    #[prost(message, optional, tag = "11")]
    pub sku: ::core::option::Option<Sku>,
}
/// Specifies transfer eligibility of a SKU.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEligibility {
    /// Whether reseller is eligible to transfer the SKU.
    #[prost(bool, tag = "1")]
    pub is_eligible: bool,
    /// Localized description if reseller is not eligible to transfer the SKU.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Specified the reason for ineligibility.
    #[prost(enumeration = "transfer_eligibility::Reason", tag = "3")]
    pub ineligibility_reason: i32,
}
/// Nested message and enum types in `TransferEligibility`.
pub mod transfer_eligibility {
    /// Reason of ineligibility.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Reason {
        /// Reason is not available.
        Unspecified = 0,
        /// Reseller needs to accept TOS before transferring the SKU.
        PendingTosAcceptance = 1,
        /// Reseller not eligible to sell the SKU.
        SkuNotEligible = 2,
        /// SKU subscription is suspended
        SkuSuspended = 3,
    }
}
/// Provides contextual information about a [google.longrunning.Operation][google.longrunning.Operation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The RPC that initiated this Long Running Operation.
    #[prost(enumeration = "operation_metadata::OperationType", tag = "1")]
    pub operation_type: i32,
}
/// Nested message and enum types in `OperationMetadata`.
pub mod operation_metadata {
    /// RPCs that return a Long Running Operation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OperationType {
        /// Default value. This state doesn't show unless an error occurs.
        Unspecified = 0,
        /// Long Running Operation was triggered by CreateEntitlement.
        CreateEntitlement = 1,
        /// Long Running Operation was triggered by ChangeRenewalSettings.
        ChangeRenewalSettings = 3,
        /// Long Running Operation was triggered by StartPaidService.
        StartPaidService = 5,
        /// Long Running Operation was triggered by ActivateEntitlement.
        ActivateEntitlement = 7,
        /// Long Running Operation was triggered by SuspendEntitlement.
        SuspendEntitlement = 8,
        /// Long Running Operation was triggered by CancelEntitlement.
        CancelEntitlement = 9,
        /// Long Running Operation was triggered by TransferEntitlements.
        TransferEntitlements = 10,
        /// Long Running Operation was triggered by TransferEntitlementsToGoogle.
        TransferEntitlementsToGoogle = 11,
        /// Long Running Operation was triggered by ChangeOffer.
        ChangeOffer = 14,
        /// Long Running Operation was triggered by ChangeParameters.
        ChangeParameters = 15,
        /// Long Running Operation was triggered by ProvisionCloudIdentity.
        ProvisionCloudIdentity = 16,
    }
}
/// Request message for [CloudChannelService.CheckCloudIdentityAccountsExist][google.cloud.channel.v1.CloudChannelService.CheckCloudIdentityAccountsExist].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckCloudIdentityAccountsExistRequest {
    /// Required. The resource name of the reseller account.
    /// The parent takes the format: accounts/{account_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Domain for which the Cloud Identity account customer is fetched.
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
}
/// Entity representing a Cloud Identity account which may or may not be
/// associated with a Channel Services API partner.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudIdentityCustomerAccount {
    /// True if a Cloud Identity account exists for a specific domain.
    #[prost(bool, tag = "1")]
    pub existing: bool,
    /// True if the Cloud Identity account is associated with a customer
    /// belonging to the Channel Services partner making the API call.
    #[prost(bool, tag = "2")]
    pub owned: bool,
    /// Name of the customer that owns the Cloud Identity account. This field is
    /// populated ONLY if owned = true.
    /// The customer_name takes the format:
    /// accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "3")]
    pub customer_name: ::prost::alloc::string::String,
    /// Cloud Identity ID of the customer. This field is populated ONLY if
    /// existing = true.
    #[prost(string, tag = "4")]
    pub customer_cloud_identity_id: ::prost::alloc::string::String,
}
/// Response message for
/// [CloudChannelService.CheckCloudIdentityAccountsExist][google.cloud.channel.v1.CloudChannelService.CheckCloudIdentityAccountsExist].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckCloudIdentityAccountsExistResponse {
    /// The Cloud Identity accounts associated with the domain.
    #[prost(message, repeated, tag = "1")]
    pub cloud_identity_accounts: ::prost::alloc::vec::Vec<CloudIdentityCustomerAccount>,
}
/// Request message for [CloudChannelService.ListCustomers][google.cloud.channel.v1.CloudChannelService.ListCustomers]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomersRequest {
    /// Required. The resource name of the reseller account from which to list customers.
    /// The parent takes the format: accounts/{account_id}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of customers to return. The service may return fewer
    /// than this value. If unspecified, at most 10 customers will be returned. The
    /// maximum value is 50; values about 50 will be coerced to 50.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results, if other than the first one.
    /// Typically obtained via
    /// [ListCustomersResponse.next_page_token][google.cloud.channel.v1.ListCustomersResponse.next_page_token] of the previous
    /// [CloudChannelService.ListCustomers][google.cloud.channel.v1.CloudChannelService.ListCustomers] call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for [CloudChannelService.ListCustomers][google.cloud.channel.v1.CloudChannelService.ListCustomers].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomersResponse {
    /// The customers belonging to the reseller or distributor.
    #[prost(message, repeated, tag = "1")]
    pub customers: ::prost::alloc::vec::Vec<Customer>,
    /// A token to retrieve the next page of results.
    /// Pass to [ListCustomersRequest.page_token][google.cloud.channel.v1.ListCustomersRequest.page_token] to obtain that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [CloudChannelService.GetCustomer][google.cloud.channel.v1.CloudChannelService.GetCustomer].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomerRequest {
    /// Required. The resource name of the customer to retrieve.
    /// The name takes the format: accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [CloudChannelService.CreateCustomer][google.cloud.channel.v1.CloudChannelService.CreateCustomer]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCustomerRequest {
    /// Required. The resource name of reseller account in which to create the customer.
    /// The parent takes the format: accounts/{account_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The customer to create.
    #[prost(message, optional, tag = "2")]
    pub customer: ::core::option::Option<Customer>,
}
/// Request message for [CloudChannelService.UpdateCustomer][google.cloud.channel.v1.CloudChannelService.UpdateCustomer].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomerRequest {
    /// Required. New contents of the customer.
    #[prost(message, optional, tag = "2")]
    pub customer: ::core::option::Option<Customer>,
    /// The update mask that applies to the resource.
    /// Optional.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for [CloudChannelService.DeleteCustomer][google.cloud.channel.v1.CloudChannelService.DeleteCustomer].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCustomerRequest {
    /// Required. The resource name of the customer to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [CloudChannelService.ProvisionCloudIdentity][google.cloud.channel.v1.CloudChannelService.ProvisionCloudIdentity]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisionCloudIdentityRequest {
    /// Required. Resource name of the customer.
    /// Format: accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "1")]
    pub customer: ::prost::alloc::string::String,
    /// CloudIdentity-specific customer information.
    #[prost(message, optional, tag = "2")]
    pub cloud_identity_info: ::core::option::Option<CloudIdentityInfo>,
    /// Admin user information.
    #[prost(message, optional, tag = "3")]
    pub user: ::core::option::Option<AdminUser>,
    /// If set, validate the request and preview the review, but do not actually
    /// post it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Request message for [CloudChannelService.ListEntitlements][google.cloud.channel.v1.CloudChannelService.ListEntitlements]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntitlementsRequest {
    /// Required. The resource name of the reseller's customer account for which to list
    /// entitlements.
    /// The parent takes the format: accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server might return fewer results than requested.
    /// If unspecified, at most 50 entitlements will be returned.
    /// The maximum value is 100; values above 100 will be coerced to 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results, if other than the first one.
    /// Typically obtained via
    /// [ListEntitlementsResponse.next_page_token][google.cloud.channel.v1.ListEntitlementsResponse.next_page_token] of the previous
    /// [CloudChannelService.ListEntitlements][google.cloud.channel.v1.CloudChannelService.ListEntitlements] call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for [CloudChannelService.ListEntitlements][google.cloud.channel.v1.CloudChannelService.ListEntitlements].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntitlementsResponse {
    /// The entitlements belonging to the reseller's customer.
    #[prost(message, repeated, tag = "1")]
    pub entitlements: ::prost::alloc::vec::Vec<Entitlement>,
    /// A token to List next page of results.
    /// Pass to [ListEntitlementsRequest.page_token][google.cloud.channel.v1.ListEntitlementsRequest.page_token] to obtain that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [CloudChannelService.ListTransferableSkus][google.cloud.channel.v1.CloudChannelService.ListTransferableSkus]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferableSkusRequest {
    /// Required. The resource name of the reseller's account.
    /// The parent takes the format: accounts/{account_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server might return fewer results than requested.
    /// If unspecified, at most 100 SKUs will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    /// Optional.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results, if other than the first one.
    /// Typically obtained via
    /// [ListTransferableSkusResponse.next_page_token][google.cloud.channel.v1.ListTransferableSkusResponse.next_page_token] of the previous
    /// [CloudChannelService.ListTransferableSkus][google.cloud.channel.v1.CloudChannelService.ListTransferableSkus] call.
    /// Optional.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// This token is generated by the Super Admin of the resold customer to
    /// authorize a reseller to access their Cloud Identity and purchase
    /// entitlements on their behalf. This token can be omitted once the
    /// authorization is generated. See https://support.google.com/a/answer/7643790
    /// for more details.
    #[prost(string, tag = "5")]
    pub auth_token: ::prost::alloc::string::String,
    /// The BCP-47 language code, such as "en-US".  If specified, the
    /// response will be localized to the corresponding language code. Default is
    /// "en-US".
    /// Optional.
    #[prost(string, tag = "6")]
    pub language_code: ::prost::alloc::string::String,
    /// Specifies the identity of transferred customer.
    /// Either a cloud_identity_id of the customer OR the customer name is
    /// required to look up transferable SKUs.
    #[prost(
        oneof = "list_transferable_skus_request::TransferredCustomerIdentity",
        tags = "4, 7"
    )]
    pub transferred_customer_identity:
        ::core::option::Option<list_transferable_skus_request::TransferredCustomerIdentity>,
}
/// Nested message and enum types in `ListTransferableSkusRequest`.
pub mod list_transferable_skus_request {
    /// Specifies the identity of transferred customer.
    /// Either a cloud_identity_id of the customer OR the customer name is
    /// required to look up transferable SKUs.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TransferredCustomerIdentity {
        /// Customer's Cloud Identity ID
        #[prost(string, tag = "4")]
        CloudIdentityId(::prost::alloc::string::String),
        /// A reseller is required to create a customer and use the resource name of
        /// the created customer here.
        /// The customer_name takes the format:
        /// accounts/{account_id}/customers/{customer_id}
        #[prost(string, tag = "7")]
        CustomerName(::prost::alloc::string::String),
    }
}
/// Response message for [CloudChannelService.ListTransferableSkus][google.cloud.channel.v1.CloudChannelService.ListTransferableSkus].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferableSkusResponse {
    /// Information about existing SKUs for a customer that would need to be
    /// transferred.
    #[prost(message, repeated, tag = "1")]
    pub transferable_skus: ::prost::alloc::vec::Vec<TransferableSku>,
    /// A token to retrieve the next page of results.
    /// Pass to [ListTransferableSkusRequest.page_token][google.cloud.channel.v1.ListTransferableSkusRequest.page_token] to obtain
    /// that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [CloudChannelService.ListTransferableOffers][google.cloud.channel.v1.CloudChannelService.ListTransferableOffers]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferableOffersRequest {
    /// Required. The resource name of the reseller's account.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server might return fewer results than requested.
    /// If unspecified, at most 100 Offers will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results, if other than the first one.
    /// Typically obtained via
    /// [ListTransferableOffersResponse.next_page_token][google.cloud.channel.v1.ListTransferableOffersResponse.next_page_token] of the previous
    /// [CloudChannelService.ListTransferableOffers][google.cloud.channel.v1.CloudChannelService.ListTransferableOffers] call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Required. SKU for which the Offers are being looked up.
    #[prost(string, tag = "6")]
    pub sku: ::prost::alloc::string::String,
    /// The BCP-47 language code, such as "en-US".  If specified, the
    /// response will be localized to the corresponding language code. Default is
    /// "en-US".
    #[prost(string, tag = "7")]
    pub language_code: ::prost::alloc::string::String,
    /// Specifies the identity of transferred customer.
    /// Either a cloud_identity_id of the customer OR the customer name is
    /// required to look up transferrable Offers.
    #[prost(
        oneof = "list_transferable_offers_request::TransferredCustomerIdentity",
        tags = "4, 5"
    )]
    pub transferred_customer_identity:
        ::core::option::Option<list_transferable_offers_request::TransferredCustomerIdentity>,
}
/// Nested message and enum types in `ListTransferableOffersRequest`.
pub mod list_transferable_offers_request {
    /// Specifies the identity of transferred customer.
    /// Either a cloud_identity_id of the customer OR the customer name is
    /// required to look up transferrable Offers.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TransferredCustomerIdentity {
        /// Customer's Cloud Identity ID
        #[prost(string, tag = "4")]
        CloudIdentityId(::prost::alloc::string::String),
        /// A reseller should create a customer and use the resource name of
        /// the created customer here.
        #[prost(string, tag = "5")]
        CustomerName(::prost::alloc::string::String),
    }
}
/// Response message for [CloudChannelService.ListTransferableOffers][google.cloud.channel.v1.CloudChannelService.ListTransferableOffers].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferableOffersResponse {
    /// Information about Offers for a customer that can be used for
    /// transfer.
    #[prost(message, repeated, tag = "1")]
    pub transferable_offers: ::prost::alloc::vec::Vec<TransferableOffer>,
    /// A token to retrieve the next page of results.
    /// Pass to [ListTransferableOffersRequest.page_token][google.cloud.channel.v1.ListTransferableOffersRequest.page_token] to obtain
    /// that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// TransferableOffer represents an Offer that can be used in Transfer.
/// Read-only.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferableOffer {
    /// Offer with parameter constraints updated to allow the Transfer.
    #[prost(message, optional, tag = "1")]
    pub offer: ::core::option::Option<Offer>,
}
/// Request message for [CloudChannelService.GetEntitlement][google.cloud.channel.v1.CloudChannelService.GetEntitlement].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntitlementRequest {
    /// Required. The resource name of the entitlement to retrieve.
    /// The name takes the format:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [CloudChannelService.ListChannelPartnerLinks][google.cloud.channel.v1.CloudChannelService.ListChannelPartnerLinks]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelPartnerLinksRequest {
    /// Required. The resource name of the reseller account for listing channel partner
    /// links.
    /// The parent takes the format: accounts/{account_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server might return fewer results than requested.
    /// If unspecified, server will pick a default size (25).
    /// The maximum value is 200, values above 200 will be coerced to 200.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results, if other than the first one.
    /// Typically obtained via
    /// [ListChannelPartnerLinksResponse.next_page_token][google.cloud.channel.v1.ListChannelPartnerLinksResponse.next_page_token] of the previous
    /// [CloudChannelService.ListChannelPartnerLinks][google.cloud.channel.v1.CloudChannelService.ListChannelPartnerLinks] call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The level of granularity the ChannelPartnerLink will display.
    #[prost(enumeration = "ChannelPartnerLinkView", tag = "4")]
    pub view: i32,
}
/// Response message for [CloudChannelService.ListChannelPartnerLinks][google.cloud.channel.v1.CloudChannelService.ListChannelPartnerLinks].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelPartnerLinksResponse {
    /// The Channel partner links for a reseller.
    #[prost(message, repeated, tag = "1")]
    pub channel_partner_links: ::prost::alloc::vec::Vec<ChannelPartnerLink>,
    /// A token to retrieve the next page of results.
    /// Pass to [ListChannelPartnerLinksRequest.page_token][google.cloud.channel.v1.ListChannelPartnerLinksRequest.page_token] to obtain that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [CloudChannelService.GetChannelPartnerLink][google.cloud.channel.v1.CloudChannelService.GetChannelPartnerLink].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelPartnerLinkRequest {
    /// Required. The resource name of the channel partner link to retrieve.
    /// The name takes the format: accounts/{account_id}/channelPartnerLinks/{id}
    /// where {id} is the Cloud Identity ID of the partner.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The level of granularity the ChannelPartnerLink will display.
    #[prost(enumeration = "ChannelPartnerLinkView", tag = "2")]
    pub view: i32,
}
/// Request message for [CloudChannelService.CreateChannelPartnerLink][google.cloud.channel.v1.CloudChannelService.CreateChannelPartnerLink]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChannelPartnerLinkRequest {
    /// Required. The resource name of reseller's account for which to create a channel
    /// partner link.
    /// The parent takes the format: accounts/{account_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The channel partner link to create.
    /// Either channel_partner_link.reseller_cloud_identity_id or domain can be
    /// used to create a link.
    #[prost(message, optional, tag = "2")]
    pub channel_partner_link: ::core::option::Option<ChannelPartnerLink>,
    /// Optional. The invited partner's domain. Either domain or
    /// channel_partner_link.reseller_cloud_identity_id can be used to create a
    /// link.
    #[prost(string, tag = "3")]
    pub domain: ::prost::alloc::string::String,
}
/// Request message for [CloudChannelService.UpdateChannelPartnerLink][google.cloud.channel.v1.CloudChannelService.UpdateChannelPartnerLink]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateChannelPartnerLinkRequest {
    /// Required. The resource name of the channel partner link to cancel.
    /// The name takes the format: accounts/{account_id}/channelPartnerLinks/{id}
    /// where {id} is the Cloud Identity ID of the partner.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The channel partner link to update. Only field
    /// channel_partner_link.link_state is allowed to be updated.
    #[prost(message, optional, tag = "2")]
    pub channel_partner_link: ::core::option::Option<ChannelPartnerLink>,
    /// Required. The update mask that applies to the resource.
    /// The only allowable value for update mask is
    /// channel_partner_link.link_state.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for [CloudChannelService.CreateEntitlement][google.cloud.channel.v1.CloudChannelService.CreateEntitlement]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntitlementRequest {
    /// Required. The resource name of reseller's customer account in which to create the
    /// entitlement.
    /// The parent takes the format: accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The entitlement to create.
    #[prost(message, optional, tag = "2")]
    pub entitlement: ::core::option::Option<Entitlement>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID so
    /// that if you must retry your request, the server will know to ignore the
    /// request if it has already been completed.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same
    /// request ID, the server can check if the original operation with the same
    /// request ID was received, and if so, will ignore the second request.
    ///
    /// The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "5")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for [CloudChannelService.TransferEntitlements][google.cloud.channel.v1.CloudChannelService.TransferEntitlements].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEntitlementsRequest {
    /// Required. The resource name of reseller's customer account where the entitlements
    /// transfer to.
    /// The parent takes the format: accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The new entitlements to be created or transferred.
    #[prost(message, repeated, tag = "2")]
    pub entitlements: ::prost::alloc::vec::Vec<Entitlement>,
    /// This token is generated by the Super Admin of the resold customer to
    /// authorize a reseller to access their Cloud Identity and purchase
    /// entitlements on their behalf. This token can be omitted once the
    /// authorization is generated. See https://support.google.com/a/answer/7643790
    /// for more details.
    #[prost(string, tag = "4")]
    pub auth_token: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID so
    /// that if you must retry your request, the server will know to ignore the
    /// request if it has already been completed.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same
    /// request ID, the server can check if the original operation with the same
    /// request ID was received, and if so, will ignore the second request.
    ///
    /// The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "6")]
    pub request_id: ::prost::alloc::string::String,
}
/// Response message for [CloudChannelService.TransferEntitlements][google.cloud.channel.v1.CloudChannelService.TransferEntitlements].
/// This will be put into the response field of google.longrunning.Operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEntitlementsResponse {
    /// The entitlements that have been transferred.
    #[prost(message, repeated, tag = "1")]
    pub entitlements: ::prost::alloc::vec::Vec<Entitlement>,
}
/// Request message for [CloudChannelService.TransferEntitlementsToGoogle][google.cloud.channel.v1.CloudChannelService.TransferEntitlementsToGoogle].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEntitlementsToGoogleRequest {
    /// Required. The resource name of reseller's customer account where the entitlements
    /// transfer from.
    /// The parent takes the format: accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The entitlements to be transferred to Google.
    #[prost(message, repeated, tag = "2")]
    pub entitlements: ::prost::alloc::vec::Vec<Entitlement>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID so
    /// that if you must retry your request, the server will know to ignore the
    /// request if it has already been completed.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same
    /// request ID, the server can check if the original operation with the same
    /// request ID was received, and if so, will ignore the second request.
    ///
    /// The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for [CloudChannelService.ChangeParametersRequest][].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeParametersRequest {
    /// Required. The name of the entitlement to update.
    /// The name takes the format:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Entitlement parameters to update. Only editable parameters are allowed to
    /// be changed.
    #[prost(message, repeated, tag = "2")]
    pub parameters: ::prost::alloc::vec::Vec<Parameter>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID so
    /// that if you must retry your request, the server will know to ignore the
    /// request if it has already been completed.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same
    /// request ID, the server can check if the original operation with the same
    /// request ID was received, and if so, will ignore the second request.
    ///
    /// The request ID must be
    /// a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that
    /// zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. Purchase order ID provided by the reseller.
    #[prost(string, tag = "5")]
    pub purchase_order_id: ::prost::alloc::string::String,
}
/// Request message for [CloudChannelService.ChangeRenewalSettings][google.cloud.channel.v1.CloudChannelService.ChangeRenewalSettings].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeRenewalSettingsRequest {
    /// Required. The name of the entitlement to update.
    /// The name takes the format:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. New renewal settings.
    #[prost(message, optional, tag = "4")]
    pub renewal_settings: ::core::option::Option<RenewalSettings>,
    /// Optional. A request ID to identify requests. Specify a unique request ID so
    /// that if you must retry your request, the server will know to ignore the
    /// request if it has already been completed.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same
    /// request ID, the server can check if the original operation with the same
    /// request ID was received, and if so, will ignore the second request.
    ///
    /// The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "5")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for [CloudChannelService.ChangeOffer][google.cloud.channel.v1.CloudChannelService.ChangeOffer].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeOfferRequest {
    /// Required. The name of the entitlement to update.
    /// Format:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. New Offer.
    /// Format: accounts/{account_id}/offers/{offer_id}.
    #[prost(string, tag = "2")]
    pub offer: ::prost::alloc::string::String,
    /// Optional. Parameters needed to purchase the Offer.
    #[prost(message, repeated, tag = "3")]
    pub parameters: ::prost::alloc::vec::Vec<Parameter>,
    /// Optional. Purchase order id provided by the reseller.
    #[prost(string, tag = "5")]
    pub purchase_order_id: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID so
    /// that if you must retry your request, the server will know to ignore the
    /// request if it has already been completed.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same
    /// request ID, the server can check if the original operation with the same
    /// request ID was received, and if so, will ignore the second request.
    ///
    /// The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "6")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for [CloudChannelService.StartPaidService][google.cloud.channel.v1.CloudChannelService.StartPaidService].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartPaidServiceRequest {
    /// Required. The name of the entitlement for which paid service is being started.
    /// The name takes the format:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID so
    /// that if you must retry your request, the server will know to ignore the
    /// request if it has already been completed.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same
    /// request ID, the server can check if the original operation with the same
    /// request ID was received, and if so, will ignore the second request.
    ///
    /// The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for [CloudChannelService.CancelEntitlement][google.cloud.channel.v1.CloudChannelService.CancelEntitlement].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelEntitlementRequest {
    /// Required. The resource name of the entitlement to cancel.
    /// The name takes the format:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID so
    /// that if you must retry your request, the server will know to ignore the
    /// request if it has already been completed.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same
    /// request ID, the server can check if the original operation with the same
    /// request ID was received, and if so, will ignore the second request.
    ///
    /// The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for [CloudChannelService.SuspendEntitlement][google.cloud.channel.v1.CloudChannelService.SuspendEntitlement].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuspendEntitlementRequest {
    /// Required. The resource name of the entitlement to suspend.
    /// The name takes the format:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID so
    /// that if you must retry your request, the server will know to ignore the
    /// request if it has already been completed.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same
    /// request ID, the server can check if the original operation with the same
    /// request ID was received, and if so, will ignore the second request.
    ///
    /// The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for [CloudChannelService.ActivateEntitlement][google.cloud.channel.v1.CloudChannelService.ActivateEntitlement].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateEntitlementRequest {
    /// Required. The resource name of the entitlement to activate.
    /// The name takes the format:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID so
    /// that if you must retry your request, the server will know to ignore the
    /// request if it has already been completed.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same
    /// request ID, the server can check if the original operation with the same
    /// request ID was received, and if so, will ignore the second request.
    ///
    /// The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for ListProducts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductsRequest {
    /// Required. The resource name of the reseller account.
    /// Format: accounts/{account_id}.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server might return fewer results than requested.
    /// If unspecified, at most 100 Products will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results, if other than the first one.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The BCP-47 language code, such as "en-US".  If specified, the
    /// response will be localized to the corresponding language code. Default is
    /// "en-US".
    #[prost(string, tag = "4")]
    pub language_code: ::prost::alloc::string::String,
}
/// Response message for ListProducts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductsResponse {
    /// List of Products requested.
    #[prost(message, repeated, tag = "1")]
    pub products: ::prost::alloc::vec::Vec<Product>,
    /// A token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for ListSkus.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSkusRequest {
    /// Required. The resource name of the Product for which to list SKUs.
    /// The parent takes the format: products/{product_id}.
    /// Supports products/- to retrieve SKUs for all products.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Resource name of the reseller.
    /// Format: accounts/{account_id}.
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server might return fewer results than requested.
    /// If unspecified, at most 100 SKUs will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results, if other than the first one.
    /// Optional.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The BCP-47 language code, such as "en-US".  If specified, the
    /// response will be localized to the corresponding language code. Default is
    /// "en-US".
    #[prost(string, tag = "5")]
    pub language_code: ::prost::alloc::string::String,
}
/// Response message for ListSkus.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSkusResponse {
    /// The list of SKUs requested.
    #[prost(message, repeated, tag = "1")]
    pub skus: ::prost::alloc::vec::Vec<Sku>,
    /// A token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for ListOffers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOffersRequest {
    /// Required. The resource name of the reseller account from which to list Offers.
    /// The parent takes the format: accounts/{account_id}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server might return fewer results than requested.
    /// If unspecified, at most 500 Offers will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results, if other than the first one.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The expression to filter results by name (name of
    /// the Offer), sku.name (name of the SKU) or sku.product.name (name of the
    /// Product).
    /// Example 1: sku.product.name=products/p1 AND sku.name!=products/p1/skus/s1
    /// Example 2: name=accounts/a1/offers/o1
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The BCP-47 language code, such as "en-US".  If specified, the
    /// response will be localized to the corresponding language code. Default is
    /// "en-US".
    #[prost(string, tag = "5")]
    pub language_code: ::prost::alloc::string::String,
}
/// Response message for ListOffers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOffersResponse {
    /// The list of Offers requested.
    #[prost(message, repeated, tag = "1")]
    pub offers: ::prost::alloc::vec::Vec<Offer>,
    /// A token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for ListPurchasableSkus.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPurchasableSkusRequest {
    /// Required. The resource name of the customer for which to list SKUs.
    /// Format: accounts/{account_id}/customers/{customer_id}.
    #[prost(string, tag = "1")]
    pub customer: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server might return fewer results than requested.
    /// If unspecified, at most 100 SKUs will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results, if other than the first one.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The BCP-47 language code, such as "en-US".  If specified, the
    /// response will be localized to the corresponding language code. Default is
    /// "en-US".
    #[prost(string, tag = "6")]
    pub language_code: ::prost::alloc::string::String,
    /// Purchase option for the request. Defines the purchase for which the SKUs
    /// are being listed.
    #[prost(oneof = "list_purchasable_skus_request::PurchaseOption", tags = "2, 3")]
    pub purchase_option: ::core::option::Option<list_purchasable_skus_request::PurchaseOption>,
}
/// Nested message and enum types in `ListPurchasableSkusRequest`.
pub mod list_purchasable_skus_request {
    /// List SKUs for a new entitlement. Make the purchase using
    /// [CloudChannelService.CreateEntitlement][google.cloud.channel.v1.CloudChannelService.CreateEntitlement].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreateEntitlementPurchase {
        /// Required. List SKUs belonging to this Product.
        /// Format: products/{product_id}.
        /// Supports products/- to retrieve SKUs for all products.
        #[prost(string, tag = "1")]
        pub product: ::prost::alloc::string::String,
    }
    /// List SKUs for upgrading or downgrading an entitlement. Make the purchase
    /// using [CloudChannelService.ChangeOffer][google.cloud.channel.v1.CloudChannelService.ChangeOffer].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChangeOfferPurchase {
        /// Required. Resource name of the entitlement.
        /// Format:
        /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
        #[prost(string, tag = "1")]
        pub entitlement: ::prost::alloc::string::String,
        /// Required. Change Type for the entitlement.
        #[prost(enumeration = "change_offer_purchase::ChangeType", tag = "2")]
        pub change_type: i32,
    }
    /// Nested message and enum types in `ChangeOfferPurchase`.
    pub mod change_offer_purchase {
        /// Change Type enum.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum ChangeType {
            /// Not used.
            Unspecified = 0,
            /// SKU is an upgrade on the current entitlement.
            Upgrade = 1,
            /// SKU is a downgrade on the current entitlement.
            Downgrade = 2,
        }
    }
    /// Purchase option for the request. Defines the purchase for which the SKUs
    /// are being listed.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PurchaseOption {
        /// List SKUs for CreateEntitlement purchase.
        #[prost(message, tag = "2")]
        CreateEntitlementPurchase(CreateEntitlementPurchase),
        /// List SKUs for ChangeOffer purchase with a new SKU.
        #[prost(message, tag = "3")]
        ChangeOfferPurchase(ChangeOfferPurchase),
    }
}
/// Response message for ListPurchasableSkus.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPurchasableSkusResponse {
    /// The list of SKUs requested.
    #[prost(message, repeated, tag = "1")]
    pub purchasable_skus: ::prost::alloc::vec::Vec<PurchasableSku>,
    /// A token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// SKU that can be used for a puchase. This is used in ListPurchasableSku API
/// response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurchasableSku {
    /// SKU
    #[prost(message, optional, tag = "1")]
    pub sku: ::core::option::Option<Sku>,
}
/// Request message for ListPurchasableOffers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPurchasableOffersRequest {
    /// Required. The resource name of the customer for which to list Offers.
    /// Format: accounts/{account_id}/customers/{customer_id}.
    #[prost(string, tag = "1")]
    pub customer: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server might return fewer results than requested.
    /// If unspecified, at most 100 Offers will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results, if other than the first one.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The BCP-47 language code, such as "en-US".  If specified, the
    /// response will be localized to the corresponding language code. Default is
    /// "en-US".
    #[prost(string, tag = "6")]
    pub language_code: ::prost::alloc::string::String,
    /// Purchase option for the request. Defines the purchase for which the Offers
    /// are being listed.
    #[prost(
        oneof = "list_purchasable_offers_request::PurchaseOption",
        tags = "2, 3"
    )]
    pub purchase_option: ::core::option::Option<list_purchasable_offers_request::PurchaseOption>,
}
/// Nested message and enum types in `ListPurchasableOffersRequest`.
pub mod list_purchasable_offers_request {
    /// List Offers for CreateEntitlement purchase.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreateEntitlementPurchase {
        /// Required. SKU that the result should be restricted to.
        /// Format: products/{product_id}/skus/{sku_id}.
        #[prost(string, tag = "1")]
        pub sku: ::prost::alloc::string::String,
    }
    /// List Offers for ChangeOffer purchase.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChangeOfferPurchase {
        /// Required. Resource name of the entitlement.
        /// Format:
        /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
        #[prost(string, tag = "1")]
        pub entitlement: ::prost::alloc::string::String,
        /// Optional. Resource name of the SKU that is being changed to. Should be provided if
        /// upgrading or downgrading an entitlement. Format:
        /// products/{product_id}/skus/{sku_id}
        #[prost(string, tag = "2")]
        pub new_sku: ::prost::alloc::string::String,
    }
    /// Purchase option for the request. Defines the purchase for which the Offers
    /// are being listed.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PurchaseOption {
        /// List Offers for CreateEntitlement purchase.
        #[prost(message, tag = "2")]
        CreateEntitlementPurchase(CreateEntitlementPurchase),
        /// List Offers for ChangeOffer purchase.
        #[prost(message, tag = "3")]
        ChangeOfferPurchase(ChangeOfferPurchase),
    }
}
/// Response message for ListPurchasableOffers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPurchasableOffersResponse {
    /// The list of Offers requested.
    #[prost(message, repeated, tag = "1")]
    pub purchasable_offers: ::prost::alloc::vec::Vec<PurchasableOffer>,
    /// A token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Offer that can be puchased for a customer. This is used in
/// ListPurchasableOffer API response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurchasableOffer {
    /// Offer.
    #[prost(message, optional, tag = "1")]
    pub offer: ::core::option::Option<Offer>,
}
/// Request Message for RegisterSubscriber.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterSubscriberRequest {
    /// Required. Resource name of the account.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// Required. Service account which will provide subscriber access to the
    /// registered topic.
    #[prost(string, tag = "2")]
    pub service_account: ::prost::alloc::string::String,
}
/// Response Message for RegisterSubscriber.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterSubscriberResponse {
    /// Name of the topic to which the subscriber will listen to.
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
}
/// Request Message for UnregisterSubscriber.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnregisterSubscriberRequest {
    /// Required. Resource name of the account.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// Required. Service account which will be unregistered from getting subscriber access
    /// to the topic.
    #[prost(string, tag = "2")]
    pub service_account: ::prost::alloc::string::String,
}
/// Response Message for UnregisterSubscriber.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnregisterSubscriberResponse {
    /// Name of the topic from which the service account subscriber access has been
    /// removed.
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
}
/// Request Message for ListSubscribers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubscribersRequest {
    /// Required. Resource name of the account.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// Optional. The maximum number of service accounts to return. The service may return
    /// fewer than this value.
    /// If unspecified, at most 100 service accounts will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListSubscribers` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListSubscribers` must
    ///  match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response Message for ListSubscribers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubscribersResponse {
    /// Name of the topic registered with the reseller.
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    /// List of service accounts which have subscriber access to the topic.
    #[prost(string, repeated, tag = "2")]
    pub service_accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod cloud_channel_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " CloudChannelService enables Google cloud resellers and distributors to manage"]
    #[doc = " their customers, channel partners, entitlements and reports."]
    #[doc = ""]
    #[doc = " Using this service:"]
    #[doc = " 1. Resellers or distributors can manage a customer entity."]
    #[doc = " 2. Distributors can register an authorized reseller in their channel and then"]
    #[doc = "    enable delegated admin access for the reseller."]
    #[doc = " 3. Resellers or distributors can manage entitlements for their customers."]
    #[doc = ""]
    #[doc = " The service primarily exposes the following resources:"]
    #[doc = " - [Customer][google.cloud.channel.v1.Customer]s: A Customer represents an entity managed by a reseller or"]
    #[doc = " distributor. A customer typically represents an enterprise. In an n-tier"]
    #[doc = " resale channel hierarchy, customers are generally represented as leaf nodes."]
    #[doc = " Customers primarily have an Entitlement sub-resource discussed below."]
    #[doc = ""]
    #[doc = " - [Entitlement][google.cloud.channel.v1.Entitlement]s: An Entitlement represents an entity which provides a"]
    #[doc = " customer means to start using a service. Entitlements are created or updated"]
    #[doc = " as a result of a successful fulfillment."]
    #[doc = ""]
    #[doc = " - [ChannelPartnerLink][google.cloud.channel.v1.ChannelPartnerLink]s: A ChannelPartnerLink is an entity that identifies"]
    #[doc = " links between distributors and their indirect resellers in a channel."]
    pub struct CloudChannelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudChannelServiceClient<T>
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
        #[doc = " List downstream [Customer][google.cloud.channel.v1.Customer]s."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the reseller account making the request and the"]
        #[doc = " reseller account being queried for are different."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> List of [Customer][google.cloud.channel.v1.Customer]s pertaining to the reseller or empty list if"]
        #[doc = " there are none."]
        pub async fn list_customers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomersRequest>,
        ) -> Result<tonic::Response<super::ListCustomersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/ListCustomers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a requested [Customer][google.cloud.channel.v1.Customer] resource."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the reseller account making the request and the"]
        #[doc = " reseller account being queried for are different."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = " * NOT_FOUND: If the customer resource doesn't exist. Usually"]
        #[doc = " the result of an invalid name parameter."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> [Customer][google.cloud.channel.v1.Customer] resource if found, error otherwise."]
        pub async fn get_customer(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomerRequest>,
        ) -> Result<tonic::Response<super::Customer>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/GetCustomer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Confirms the existence of Cloud Identity accounts, based on the domain and"]
        #[doc = " whether the Cloud Identity accounts are owned by the reseller."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the reseller account making the request and the"]
        #[doc = " reseller account being queried for are different."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = " * INVALID_VALUE: Invalid domain value in the request."]
        #[doc = " * NOT_FOUND: If there is no [CloudIdentityCustomerAccount][google.cloud.channel.v1.CloudIdentityCustomerAccount] customer"]
        #[doc = " for the domain specified in the request."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> List of [CloudIdentityCustomerAccount][google.cloud.channel.v1.CloudIdentityCustomerAccount] resources if any exist for"]
        #[doc = " the domain, otherwise an error is returned."]
        pub async fn check_cloud_identity_accounts_exist(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckCloudIdentityAccountsExistRequest>,
        ) -> Result<tonic::Response<super::CheckCloudIdentityAccountsExistResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/CheckCloudIdentityAccountsExist",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new [Customer][google.cloud.channel.v1.Customer] resource under the reseller or distributor"]
        #[doc = " account."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = " <ul>"]
        #[doc = " <li>PERMISSION_DENIED: If the reseller account making the request and the"]
        #[doc = " reseller account being queried for are different.</li>"]
        #[doc = " <li> INVALID_ARGUMENT:"]
        #[doc = " <ul>"]
        #[doc = "  <li> Missing or invalid required parameters in the request. </li>"]
        #[doc = "  <li> Domain field value doesn't match the domain specified in primary"]
        #[doc = "  email.</li>"]
        #[doc = " </ul>"]
        #[doc = " </li>"]
        #[doc = " </ul>"]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> If successful, the newly created [Customer][google.cloud.channel.v1.Customer] resource, otherwise"]
        #[doc = " returns an error."]
        pub async fn create_customer(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCustomerRequest>,
        ) -> Result<tonic::Response<super::Customer>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/CreateCustomer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an existing [Customer][google.cloud.channel.v1.Customer] resource belonging to the reseller or"]
        #[doc = " distributor."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the reseller account making the request and the"]
        #[doc = " reseller account being queried for are different."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = " * NOT_FOUND: No [Customer][google.cloud.channel.v1.Customer] resource found for the name"]
        #[doc = " specified in the request."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> If successful, the updated [Customer][google.cloud.channel.v1.Customer] resource, otherwise returns"]
        #[doc = " an error."]
        pub async fn update_customer(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomerRequest>,
        ) -> Result<tonic::Response<super::Customer>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/UpdateCustomer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the given [Customer][google.cloud.channel.v1.Customer] permanently and irreversibly."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the account making the request does not own"]
        #[doc = " this customer."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = " * FAILED_PRECONDITION: If the customer has existing entitlements."]
        #[doc = " * NOT_FOUND: No [Customer][google.cloud.channel.v1.Customer] resource found for the name"]
        #[doc = " specified in the request."]
        pub async fn delete_customer(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCustomerRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/DeleteCustomer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a Cloud Identity for the given customer using the customer's"]
        #[doc = " information or the information provided here, if present."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " *  PERMISSION_DENIED: If the customer doesn't belong to the reseller."]
        #[doc = " *  INVALID_ARGUMENT: Missing or invalid required parameters in the request."]
        #[doc = " *  NOT_FOUND: If the customer is not found for the reseller."]
        #[doc = " *  ALREADY_EXISTS: If the customer's primary email already exists. In this"]
        #[doc = "    case, retry after changing the customer's primary contact email."]
        #[doc = " *  INTERNAL: Any non-user error related to a technical issue in the"]
        #[doc = "    backend. Contact Cloud Channel support in this case."]
        #[doc = " *  UNKNOWN: Any non-user error related to a technical issue in the backend."]
        #[doc = "    Contact Cloud Channel support in this case."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/>  Long Running Operation ID."]
        #[doc = ""]
        #[doc = " To get the results of the operation, call the GetOperation method of"]
        #[doc = " CloudChannelOperationsService. The Operation metadata will contain an"]
        #[doc = " instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata]."]
        pub async fn provision_cloud_identity(
            &mut self,
            request: impl tonic::IntoRequest<super::ProvisionCloudIdentityRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.channel.v1.CloudChannelService/ProvisionCloudIdentity",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List [Entitlement][google.cloud.channel.v1.Entitlement]s belonging to a customer."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the customer doesn't belong to the reseller."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the request."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> List of [Entitlement][google.cloud.channel.v1.Entitlement]s belonging to the customer, or empty list if"]
        #[doc = " there are none."]
        pub async fn list_entitlements(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEntitlementsRequest>,
        ) -> Result<tonic::Response<super::ListEntitlementsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/ListEntitlements",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List [TransferableSku][google.cloud.channel.v1.TransferableSku]s of a customer based on Cloud Identity ID or"]
        #[doc = " Customer Name in the request."]
        #[doc = ""]
        #[doc = " This method is used when a reseller lists the entitlements"]
        #[doc = " information of a customer that is not owned. The reseller should provide"]
        #[doc = " the customer's Cloud Identity ID or Customer Name."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = " <ul>"]
        #[doc = " <li>PERMISSION_DENIED, due to one of the following reasons:"]
        #[doc = " <ul>"]
        #[doc = "    <li> If the customer doesn't belong to the reseller and no auth token,"]
        #[doc = "    or an invalid auth token is supplied. </li> <li> If the reseller account"]
        #[doc = "    making the request and the reseller account being queried for are"]
        #[doc = "    different. </li>"]
        #[doc = " </ul>"]
        #[doc = " </li>"]
        #[doc = " <li> INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request.</li>"]
        #[doc = " </ul>"]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> List of [TransferableSku][google.cloud.channel.v1.TransferableSku] for the given customer."]
        pub async fn list_transferable_skus(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTransferableSkusRequest>,
        ) -> Result<tonic::Response<super::ListTransferableSkusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/ListTransferableSkus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List [TransferableOffer][google.cloud.channel.v1.TransferableOffer]s of a customer based on Cloud Identity ID or"]
        #[doc = " Customer Name in the request."]
        #[doc = ""]
        #[doc = " This method is used when a reseller gets the entitlement"]
        #[doc = " information of a customer that is not owned. The reseller should provide"]
        #[doc = " the customer's Cloud Identity ID or Customer Name."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED, due to one of the following reasons: (a) If the"]
        #[doc = " customer doesn't belong to the reseller and no auth token or invalid auth"]
        #[doc = " token is supplied. (b) If the reseller account making the request and the"]
        #[doc = " reseller account being queried for are different."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " List of [TransferableOffer][google.cloud.channel.v1.TransferableOffer] for the given customer and SKU."]
        pub async fn list_transferable_offers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTransferableOffersRequest>,
        ) -> Result<tonic::Response<super::ListTransferableOffersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/ListTransferableOffers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a requested [Entitlement][google.cloud.channel.v1.Entitlement] resource."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the customer doesn't belong to the reseller."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = " * NOT_FOUND: If the entitlement is not found for the customer."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> If found, the requested [Entitlement][google.cloud.channel.v1.Entitlement] resource, otherwise returns"]
        #[doc = " an error."]
        pub async fn get_entitlement(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEntitlementRequest>,
        ) -> Result<tonic::Response<super::Entitlement>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/GetEntitlement",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an entitlement for a customer."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = " <ul>"]
        #[doc = " <li> PERMISSION_DENIED: If the customer doesn't belong to the reseller."]
        #[doc = " </li> <li> INVALID_ARGUMENT: <ul>"]
        #[doc = "   <li> Missing or invalid required parameters in the request. </li>"]
        #[doc = "   <li> Cannot purchase an entitlement if there is already an"]
        #[doc = "    entitlement for customer, for a SKU from the same product family. </li>"]
        #[doc = "   <li> INVALID_VALUE: Offer passed in isn't valid. Make sure OfferId is"]
        #[doc = " valid. If it is valid, then contact Google Channel support for further"]
        #[doc = " troubleshooting. </li>"]
        #[doc = " </ul>"]
        #[doc = " </li>"]
        #[doc = " <li> NOT_FOUND: If the customer or offer resource is not found for the"]
        #[doc = " reseller. </li>"]
        #[doc = " <li> ALREADY_EXISTS: This failure can happen in the following cases:"]
        #[doc = "   <ul>"]
        #[doc = "     <li>If the SKU has been already purchased for the customer.</li>"]
        #[doc = "     <li>If the customer's primary email already exists. In this case retry"]
        #[doc = "         after changing the customer's primary contact email."]
        #[doc = "     </li>"]
        #[doc = "   </ul>"]
        #[doc = " </li>"]
        #[doc = " <li> CONDITION_NOT_MET or FAILED_PRECONDITION: This"]
        #[doc = " failure can happen in the following cases:"]
        #[doc = " <ul>"]
        #[doc = "    <li> Purchasing a SKU that requires domain verification and the"]
        #[doc = "    domain has not been verified. </li>"]
        #[doc = "    <li> Purchasing an Add-On SKU like Vault or Drive without purchasing"]
        #[doc = "    the pre-requisite SKU, such as Google Workspace Business Starter. </li>"]
        #[doc = "    <li> Applicable only for developer accounts: reseller and resold"]
        #[doc = "    domain. Must meet the following domain naming requirements:"]
        #[doc = "     <ul>"]
        #[doc = "       <li> Domain names must start with goog-test. </li>"]
        #[doc = "       <li> Resold domain names must include the reseller domain. </li>"]
        #[doc = "     </ul>"]
        #[doc = "    </li>"]
        #[doc = " </ul>"]
        #[doc = " </li>"]
        #[doc = " <li> INTERNAL: Any non-user error related to a technical issue in the"]
        #[doc = " backend. Contact Cloud Channel Support in this case. </li>"]
        #[doc = " <li> UNKNOWN: Any non-user error related to a technical issue in the"]
        #[doc = " backend. Contact Cloud Channel Support in this case. </li>"]
        #[doc = " </ul>"]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> Long Running Operation ID."]
        #[doc = ""]
        #[doc = " To get the results of the operation, call the GetOperation method of"]
        #[doc = " CloudChannelOperationsService. The Operation metadata will contain an"]
        #[doc = " instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata]."]
        pub async fn create_entitlement(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEntitlementRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.channel.v1.CloudChannelService/CreateEntitlement",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Change parameters of the entitlement"]
        #[doc = ""]
        #[doc = " An entitlement parameters update is a long-running operation and results in"]
        #[doc = " updates to the entitlement as a result of fulfillment."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the customer doesn't belong to the reseller."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request. For example, if the number of seats being changed to is greater"]
        #[doc = " than the allowed number of max seats for the resource. Or decreasing seats"]
        #[doc = " for a commitment based plan."]
        #[doc = " * NOT_FOUND: Entitlement resource not found."]
        #[doc = " * INTERNAL: Any non-user error related to a technical issue"]
        #[doc = " in the backend. In this case, contact Cloud Channel support."]
        #[doc = " * UNKNOWN: Any non-user error related to a technical issue in the backend."]
        #[doc = " In this case, contact Cloud Channel support."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> Long Running Operation ID."]
        #[doc = ""]
        #[doc = " To get the results of the operation, call the GetOperation method of"]
        #[doc = " CloudChannelOperationsService. The Operation metadata will contain an"]
        #[doc = " instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata]."]
        pub async fn change_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeParametersRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.channel.v1.CloudChannelService/ChangeParameters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the renewal settings for an existing customer entitlement."]
        #[doc = ""]
        #[doc = " An entitlement update is a long-running operation and results in updates to"]
        #[doc = " the entitlement as a result of fulfillment."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the customer doesn't belong to the reseller."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = " * NOT_FOUND: Entitlement resource not found."]
        #[doc = " * NOT_COMMITMENT_PLAN: Renewal Settings are only applicable for a"]
        #[doc = " commitment plan. Can't enable or disable renewal for non-commitment plans."]
        #[doc = " * INTERNAL: Any non user error related to a technical issue in the"]
        #[doc = " backend. In this case, contact Cloud Channel support."]
        #[doc = " * UNKNOWN: Any non user error related to a technical issue in the backend."]
        #[doc = " In this case, contact Cloud Channel support."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> Long Running Operation ID."]
        #[doc = ""]
        #[doc = " To get the results of the operation, call the GetOperation method of"]
        #[doc = " CloudChannelOperationsService. The Operation metadata will contain an"]
        #[doc = " instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata]."]
        pub async fn change_renewal_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeRenewalSettingsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.channel.v1.CloudChannelService/ChangeRenewalSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the Offer for an existing customer entitlement."]
        #[doc = ""]
        #[doc = " An entitlement update is a long-running operation and results in updates to"]
        #[doc = " the entitlement as a result of fulfillment."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the customer doesn't belong to the reseller."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = " * NOT_FOUND: Offer or Entitlement resource not found."]
        #[doc = " * INTERNAL: Any non-user error related to a technical issue in the backend."]
        #[doc = " In this case, contact Cloud Channel support."]
        #[doc = " * UNKNOWN: Any non-user error related to a technical issue in the backend."]
        #[doc = " In this case, contact Cloud Channel support."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> Long Running Operation ID."]
        #[doc = ""]
        #[doc = " To get the results of the operation, call the GetOperation method of"]
        #[doc = " CloudChannelOperationsService. The Operation metadata will contain an"]
        #[doc = " instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata]."]
        pub async fn change_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeOfferRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.channel.v1.CloudChannelService/ChangeOffer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Starts paid service for a trial entitlement."]
        #[doc = ""]
        #[doc = " Starts paid service for a trial entitlement immediately. This method is"]
        #[doc = " only applicable if a plan has already been set up for a trial entitlement"]
        #[doc = " but has some trial days remaining."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the customer doesn't belong to the reseller."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = " * NOT_FOUND: Entitlement resource not found."]
        #[doc = " * FAILED_PRECONDITION/NOT_IN_TRIAL: This method only works for"]
        #[doc = " entitlement on trial plans."]
        #[doc = " * INTERNAL: Any non-user error related to a technical issue in the backend."]
        #[doc = " In this case, contact Cloud Channel support."]
        #[doc = " * UNKNOWN: Any non-user error related to a technical issue"]
        #[doc = " in the backend. In this case, contact Cloud Channel support."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> Long Running Operation ID."]
        #[doc = ""]
        #[doc = " To get the results of the operation, call the GetOperation method of"]
        #[doc = " CloudChannelOperationsService. The Operation metadata will contain an"]
        #[doc = " instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata]."]
        pub async fn start_paid_service(
            &mut self,
            request: impl tonic::IntoRequest<super::StartPaidServiceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.channel.v1.CloudChannelService/StartPaidService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Suspends a previously fulfilled entitlement."]
        #[doc = " An entitlement suspension is a long-running operation."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the customer doesn't belong to the reseller."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = " * NOT_FOUND: Entitlement resource not found."]
        #[doc = " * NOT_ACTIVE: Entitlement is not active."]
        #[doc = " * INTERNAL: Any non-user error related to a technical issue in the backend."]
        #[doc = " In this case, contact Cloud Channel support."]
        #[doc = " * UNKNOWN: Any non-user error related to a technical issue in the backend."]
        #[doc = " In this case, contact Cloud Channel support."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> Long Running Operation ID."]
        #[doc = ""]
        #[doc = " To get the results of the operation, call the GetOperation method of"]
        #[doc = " CloudChannelOperationsService. The Operation metadata will contain an"]
        #[doc = " instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata]."]
        pub async fn suspend_entitlement(
            &mut self,
            request: impl tonic::IntoRequest<super::SuspendEntitlementRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.channel.v1.CloudChannelService/SuspendEntitlement",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Cancels a previously fulfilled entitlement."]
        #[doc = " An entitlement cancellation is a long-running operation."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the customer doesn't belong to the reseller or"]
        #[doc = " if the reseller account making the request and reseller account being"]
        #[doc = " queried for are different."]
        #[doc = " * FAILED_PRECONDITION: If there are any Google Cloud projects linked to the"]
        #[doc = " Google Cloud entitlement's Cloud Billing subaccount."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = " * NOT_FOUND: Entitlement resource not found."]
        #[doc = " * DELETION_TYPE_NOT_ALLOWED: Cancel is only allowed for Google Workspace"]
        #[doc = " add-ons or entitlements for Google Cloud's development platform."]
        #[doc = " * INTERNAL: Any non-user error related to a technical issue in the"]
        #[doc = " backend. In this case, contact Cloud Channel support."]
        #[doc = " * UNKNOWN: Any non-user error related to a technical issue in the backend."]
        #[doc = " In this case, contact Cloud Channel support."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> Long Running Operation ID."]
        #[doc = ""]
        #[doc = " To get the results of the operation, call the GetOperation method of"]
        #[doc = " CloudChannelOperationsService. The response will contain"]
        #[doc = " google.protobuf.Empty on success. The Operation metadata will contain an"]
        #[doc = " instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata]."]
        pub async fn cancel_entitlement(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelEntitlementRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.channel.v1.CloudChannelService/CancelEntitlement",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Activates a previously suspended entitlement. The entitlement must be in a"]
        #[doc = " suspended state for it to be activated. Entitlements suspended for pending"]
        #[doc = " ToS acceptance can't be activated using this method. An entitlement"]
        #[doc = " activation is a long-running operation and can result in updates to"]
        #[doc = " the state of the customer entitlement."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the customer doesn't belong to the reseller or"]
        #[doc = " if the reseller account making the request and reseller account being"]
        #[doc = " queried for are different."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = " * NOT_FOUND: Entitlement resource not found."]
        #[doc = " * SUSPENSION_NOT_RESELLER_INITIATED: Can't activate an"]
        #[doc = " entitlement that is pending TOS acceptance. Only reseller initiated"]
        #[doc = " suspensions can be activated."]
        #[doc = " * NOT_SUSPENDED: Can't activate entitlements that are already in ACTIVE"]
        #[doc = " state. Can only activate suspended entitlements."]
        #[doc = " * INTERNAL: Any non-user error related to a technical issue"]
        #[doc = " in the backend. In this case, contact Cloud Channel support."]
        #[doc = " * UNKNOWN: Any non-user error related to a technical issue in the backend."]
        #[doc = " In this case, contact Cloud Channel support."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> Long Running Operation ID."]
        #[doc = ""]
        #[doc = " To get the results of the operation, call the GetOperation method of"]
        #[doc = " CloudChannelOperationsService. The Operation metadata will contain an"]
        #[doc = " instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata]."]
        pub async fn activate_entitlement(
            &mut self,
            request: impl tonic::IntoRequest<super::ActivateEntitlementRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.channel.v1.CloudChannelService/ActivateEntitlement",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Transfers customer entitlements to new reseller."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = " <ul>"]
        #[doc = " <li> PERMISSION_DENIED: If the customer doesn't belong to the"]
        #[doc = " reseller.</li> <li> INVALID_ARGUMENT: Missing or invalid required"]
        #[doc = " parameters in the request. </li> <li> NOT_FOUND: If the customer or offer"]
        #[doc = " resource is not found for the reseller. </li> <li> ALREADY_EXISTS: If the"]
        #[doc = " SKU has been already transferred for the customer. </li> <li>"]
        #[doc = " CONDITION_NOT_MET or FAILED_PRECONDITION: This failure can happen in the"]
        #[doc = " following cases: <ul>"]
        #[doc = "    <li> Transferring a SKU that requires domain verification and the"]
        #[doc = " domain has not been verified. </li>"]
        #[doc = "    <li> Transferring an Add-On SKU like Vault or Drive without transferring"]
        #[doc = " the pre-requisite SKU, such as G Suite Basic </li> <li> Applicable only for"]
        #[doc = " developer accounts: reseller and resold domain must follow the domain"]
        #[doc = " naming convention as follows:"]
        #[doc = "      <ul>"]
        #[doc = "         <li> Domain names must start with goog-test. </li>"]
        #[doc = "         <li> Resold domain names must include the reseller domain. </li>"]
        #[doc = "      </ul>"]
        #[doc = "   </li>"]
        #[doc = "   <li> All transferring entitlements must be specified. </li>"]
        #[doc = " </ul>"]
        #[doc = " </li>"]
        #[doc = " <li> INTERNAL: Any non-user error related to a technical issue in the"]
        #[doc = " backend. Please contact Cloud Channel Support in this case. </li>"]
        #[doc = " <li> UNKNOWN: Any non-user error related to a technical issue in the"]
        #[doc = " backend. Please contact Cloud Channel Support in this case. </li>"]
        #[doc = " </ul>"]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> Long Running Operation ID."]
        #[doc = ""]
        #[doc = " To get the results of the operation, call the GetOperation method of"]
        #[doc = " CloudChannelOperationsService. The Operation metadata will contain an"]
        #[doc = " instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata]."]
        pub async fn transfer_entitlements(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferEntitlementsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.channel.v1.CloudChannelService/TransferEntitlements",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Transfers customer entitlements from current reseller to Google."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = " <ul>"]
        #[doc = " <li> PERMISSION_DENIED: If the customer doesn't belong to the reseller."]
        #[doc = " </li> <li> INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request. </li>"]
        #[doc = " <li> NOT_FOUND: If the customer or offer resource is not found"]
        #[doc = " for the reseller. </li>"]
        #[doc = " <li> ALREADY_EXISTS: If the SKU has been already"]
        #[doc = " transferred for the customer. </li>"]
        #[doc = " <li> CONDITION_NOT_MET or FAILED_PRECONDITION: This failure can happen in"]
        #[doc = " the following cases:"]
        #[doc = " <ul>"]
        #[doc = "    <li> Transferring a SKU that requires domain verification and the"]
        #[doc = " domain has not been verified. </li>"]
        #[doc = "    <li> Transferring an Add-On SKU like Vault or Drive without purchasing"]
        #[doc = " the pre-requisite SKU, such as G Suite Basic </li> <li> Applicable only for"]
        #[doc = " developer accounts: reseller and resold domain must follow the domain"]
        #[doc = " naming convention as follows:"]
        #[doc = "      <ul>"]
        #[doc = "         <li> Domain names must start with goog-test. </li>"]
        #[doc = "         <li> Resold domain names must include the reseller domain. </li>"]
        #[doc = "      </ul>"]
        #[doc = "    </li>"]
        #[doc = " </ul>"]
        #[doc = " </li>"]
        #[doc = " <li> INTERNAL: Any non-user error related to a technical issue in the"]
        #[doc = " backend. Please contact Cloud Channel Support in this case. </li>"]
        #[doc = " <li> UNKNOWN: Any non-user error related to a technical issue in the"]
        #[doc = " backend. Please contact Cloud Channel Support in this case.</li>"]
        #[doc = " </ul>"]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> Long Running Operation ID."]
        #[doc = ""]
        #[doc = " To get the results of the operation, call the GetOperation method of"]
        #[doc = " CloudChannelOperationsService. The response will contain"]
        #[doc = " google.protobuf.Empty on success. The Operation metadata will contain an"]
        #[doc = " instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata]."]
        pub async fn transfer_entitlements_to_google(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferEntitlementsToGoogleRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.channel.v1.CloudChannelService/TransferEntitlementsToGoogle",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List [ChannelPartnerLink][google.cloud.channel.v1.ChannelPartnerLink]s belonging to a distributor."]
        #[doc = " To call this method, you must be a distributor."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the reseller account making the request and the"]
        #[doc = " reseller account being queried for are different."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> If successful, returns the list of [ChannelPartnerLink][google.cloud.channel.v1.ChannelPartnerLink] resources"]
        #[doc = " for the distributor account, otherwise returns an error."]
        pub async fn list_channel_partner_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListChannelPartnerLinksRequest>,
        ) -> Result<tonic::Response<super::ListChannelPartnerLinksResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/ListChannelPartnerLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a requested [ChannelPartnerLink][google.cloud.channel.v1.ChannelPartnerLink] resource."]
        #[doc = " To call this method, you must be a distributor."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the reseller account making the request and the"]
        #[doc = " reseller account being queried for are different."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = " * NOT_FOUND: ChannelPartnerLink resource not found. Results"]
        #[doc = " due invalid channel partner link name."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> [ChannelPartnerLink][google.cloud.channel.v1.ChannelPartnerLink] resource if found, otherwise returns an error."]
        pub async fn get_channel_partner_link(
            &mut self,
            request: impl tonic::IntoRequest<super::GetChannelPartnerLinkRequest>,
        ) -> Result<tonic::Response<super::ChannelPartnerLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/GetChannelPartnerLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Initiates a channel partner link between a distributor and a reseller or"]
        #[doc = " between resellers in an n-tier reseller channel."]
        #[doc = " To accept the invite, the invited partner should follow the invite_link_uri"]
        #[doc = " provided in the response. If the link creation is accepted, a valid link is"]
        #[doc = " set up between the two involved parties."]
        #[doc = " To call this method, you must be a distributor."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the reseller account making the request and the"]
        #[doc = " reseller account being queried for are different."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = " * ALREADY_EXISTS: If the ChannelPartnerLink sent in the request already"]
        #[doc = " exists."]
        #[doc = " * NOT_FOUND: If no Cloud Identity customer exists for domain provided."]
        #[doc = " * INTERNAL: Any non-user error related to a technical issue in the"]
        #[doc = " backend. In this case, contact Cloud Channel support."]
        #[doc = " * UNKNOWN: Any non-user error related to a technical issue in"]
        #[doc = " the backend. In this case, contact Cloud Channel support."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> Newly created [ChannelPartnerLink][google.cloud.channel.v1.ChannelPartnerLink] resource if successful,"]
        #[doc = " otherwise error is returned."]
        pub async fn create_channel_partner_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateChannelPartnerLinkRequest>,
        ) -> Result<tonic::Response<super::ChannelPartnerLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/CreateChannelPartnerLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a channel partner link. A distributor calls this method to change a"]
        #[doc = " link's status. For example, suspend a partner link."]
        #[doc = " To call this method, you must be a distributor."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = " <ul>"]
        #[doc = " <li> PERMISSION_DENIED: If the reseller account making the request and the"]
        #[doc = " reseller account being queried for are different. </li>"]
        #[doc = " <li> INVALID_ARGUMENT:"]
        #[doc = " <ul>"]
        #[doc = "   <li> Missing or invalid required parameters in the request. </li>"]
        #[doc = "   <li> Updating link state from invited to active or suspended. </li>"]
        #[doc = "   <li> Sending reseller_cloud_identity_id, invite_url or name in update"]
        #[doc = "   mask. </li>"]
        #[doc = " </ul>"]
        #[doc = " </li>"]
        #[doc = " <li> NOT_FOUND: ChannelPartnerLink resource not found.</li>"]
        #[doc = " <li> INTERNAL: Any non-user error related to a technical issue in the"]
        #[doc = " backend. In this case, contact Cloud Channel support. </li>"]
        #[doc = " <li> UNKNOWN: Any non-user error related to a technical issue in the"]
        #[doc = " backend. In this case, contact Cloud Channel support.</li>"]
        #[doc = " </ul>"]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " <br/> If successful, the updated [ChannelPartnerLink][google.cloud.channel.v1.ChannelPartnerLink] resource, otherwise"]
        #[doc = " returns an error."]
        pub async fn update_channel_partner_link(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateChannelPartnerLinkRequest>,
        ) -> Result<tonic::Response<super::ChannelPartnerLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/UpdateChannelPartnerLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the Products the reseller is authorized to sell."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        pub async fn list_products(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProductsRequest>,
        ) -> Result<tonic::Response<super::ListProductsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/ListProducts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the SKUs for a product the reseller is authorized to sell."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
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
                "/google.cloud.channel.v1.CloudChannelService/ListSkus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the Offers the reseller can sell."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        pub async fn list_offers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOffersRequest>,
        ) -> Result<tonic::Response<super::ListOffersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/ListOffers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the Purchasable SKUs for following cases:"]
        #[doc = ""]
        #[doc = " * SKUs that can be newly purchased for a customer"]
        #[doc = " * SKUs that can be upgraded/downgraded to, for an entitlement."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the customer doesn't belong to the reseller"]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        pub async fn list_purchasable_skus(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPurchasableSkusRequest>,
        ) -> Result<tonic::Response<super::ListPurchasableSkusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/ListPurchasableSkus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the Purchasable Offers for the following cases:"]
        #[doc = ""]
        #[doc = " * Offers that can be newly purchased for a customer"]
        #[doc = " * Offers that can be changed to, for an entitlement."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the customer doesn't belong to the reseller"]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        pub async fn list_purchasable_offers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPurchasableOffersRequest>,
        ) -> Result<tonic::Response<super::ListPurchasableOffersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/ListPurchasableOffers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Registers a service account with subscriber privileges on the Cloud Pub/Sub"]
        #[doc = " topic created for this Channel Services account. Once you create a"]
        #[doc = " subscriber, you will get the events as per [SubscriberEvent][google.cloud.channel.v1.SubscriberEvent]"]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the reseller account making the request and the"]
        #[doc = " reseller account being provided are different, or if the impersonated user"]
        #[doc = " is not a super admin."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = " * INTERNAL: Any non-user error related to a technical issue in the"]
        #[doc = " backend. In this case, contact Cloud Channel support."]
        #[doc = " * UNKNOWN: Any non-user error related to a technical issue in"]
        #[doc = " the backend. In this case, contact Cloud Channel support."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " Topic name with service email address registered if successful,"]
        #[doc = " otherwise error is returned."]
        pub async fn register_subscriber(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterSubscriberRequest>,
        ) -> Result<tonic::Response<super::RegisterSubscriberResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/RegisterSubscriber",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Unregisters a service account with subscriber privileges on the Cloud"]
        #[doc = " Pub/Sub topic created for this Channel Services account. If there are no"]
        #[doc = " more service account left with sunbscriber privileges, the topic will be"]
        #[doc = " deleted. You can check this by calling ListSubscribers api."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the reseller account making the request and the"]
        #[doc = " reseller account being provided are different, or if the impersonated user"]
        #[doc = " is not a super admin."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = " * NOT_FOUND: If the topic resource doesn't exist."]
        #[doc = " * INTERNAL: Any non-user error related to a technical issue in the"]
        #[doc = " backend. In this case, contact Cloud Channel support."]
        #[doc = " * UNKNOWN: Any non-user error related to a technical issue in"]
        #[doc = " the backend. In this case, contact Cloud Channel support."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " Topic name from which service email address has been unregistered if"]
        #[doc = " successful, otherwise error is returned. If the service email was already"]
        #[doc = " not associated with the topic, the success response will be returned."]
        pub async fn unregister_subscriber(
            &mut self,
            request: impl tonic::IntoRequest<super::UnregisterSubscriberRequest>,
        ) -> Result<tonic::Response<super::UnregisterSubscriberResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/UnregisterSubscriber",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists service accounts with subscriber privileges on the Cloud Pub/Sub"]
        #[doc = " topic created for this Channel Services account."]
        #[doc = ""]
        #[doc = " Possible Error Codes:"]
        #[doc = ""]
        #[doc = " * PERMISSION_DENIED: If the reseller account making the request and the"]
        #[doc = " reseller account being provided are different, or if the account is not"]
        #[doc = " a super admin."]
        #[doc = " * INVALID_ARGUMENT: Missing or invalid required parameters in the"]
        #[doc = " request."]
        #[doc = " * NOT_FOUND: If the topic resource doesn't exist."]
        #[doc = " * INTERNAL: Any non-user error related to a technical issue in the"]
        #[doc = " backend. In this case, contact Cloud Channel support."]
        #[doc = " * UNKNOWN: Any non-user error related to a technical issue in"]
        #[doc = " the backend. In this case, contact Cloud Channel support."]
        #[doc = ""]
        #[doc = " Return Value:"]
        #[doc = " List of service email addresses if successful, otherwise error is"]
        #[doc = " returned."]
        pub async fn list_subscribers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSubscribersRequest>,
        ) -> Result<tonic::Response<super::ListSubscribersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.channel.v1.CloudChannelService/ListSubscribers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CloudChannelServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CloudChannelServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CloudChannelServiceClient {{ ... }}")
        }
    }
}
/// Represents Pub/Sub message content describing customer update.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerEvent {
    /// Resource name of the customer.
    /// Format: accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "1")]
    pub customer: ::prost::alloc::string::String,
    /// Type of event which happened on the customer.
    #[prost(enumeration = "customer_event::Type", tag = "2")]
    pub event_type: i32,
}
/// Nested message and enum types in `CustomerEvent`.
pub mod customer_event {
    /// Type of customer event.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Default value. This state doesn't show unless an error occurs.
        Unspecified = 0,
    }
}
/// Represents Pub/Sub message content describing entitlement update.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntitlementEvent {
    /// Resource name of an entitlement of the form:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    #[prost(string, tag = "1")]
    pub entitlement: ::prost::alloc::string::String,
    /// Type of event which happened on the entitlement.
    #[prost(enumeration = "entitlement_event::Type", tag = "2")]
    pub event_type: i32,
}
/// Nested message and enum types in `EntitlementEvent`.
pub mod entitlement_event {
    /// Type of entitlement event.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Default value. This state doesn't show unless an error occurs.
        Unspecified = 0,
        /// A new entitlement was created.
        Created = 1,
        /// The offer type associated with an entitlement was changed.
        /// This is not triggered if an entitlement converts from a commit offer to a
        /// flexible offer as part of a renewal.
        PricePlanSwitched = 3,
        /// Annual commitment for a commit plan was changed.
        CommitmentChanged = 4,
        /// An annual entitlement was renewed.
        Renewed = 5,
        /// Entitlement was suspended.
        Suspended = 6,
        /// Entitlement was unsuspended.
        Activated = 7,
        /// Entitlement was cancelled.
        Cancelled = 8,
        /// Entitlement was upgraded or downgraded (e.g. from Google Workspace
        /// Business Standard to Google Workspace Business Plus).
        SkuChanged = 9,
        /// The renewal settings of an entitlement has changed.
        RenewalSettingChanged = 10,
        /// Paid service has started on trial entitlement.
        PaidServiceStarted = 11,
        /// License was assigned to or revoked from a user.
        LicenseAssignmentChanged = 12,
    }
}
/// Represents information which resellers will get as part of notification from
/// Cloud Pub/Sub.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriberEvent {
    /// Specifies the Pub/Sub event provided to the partners.
    /// This is a required field.
    #[prost(oneof = "subscriber_event::Event", tags = "1, 2")]
    pub event: ::core::option::Option<subscriber_event::Event>,
}
/// Nested message and enum types in `SubscriberEvent`.
pub mod subscriber_event {
    /// Specifies the Pub/Sub event provided to the partners.
    /// This is a required field.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// Customer event send as part of Pub/Sub event to partners.
        #[prost(message, tag = "1")]
        CustomerEvent(super::CustomerEvent),
        /// Entitlement event send as part of Pub/Sub event to partners.
        #[prost(message, tag = "2")]
        EntitlementEvent(super::EntitlementEvent),
    }
}
