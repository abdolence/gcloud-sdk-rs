/// **Multi-cluster Ingress**: The configuration for the MultiClusterIngress
/// feature.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureSpec {
    /// Fully-qualified Membership name which hosts the MultiClusterIngress CRD.
    /// Example: `projects/foo-proj/locations/global/memberships/bar`
    #[prost(string, tag="1")]
    pub config_membership: ::prost::alloc::string::String,
    /// Customer's billing structure
    #[prost(enumeration="Billing", tag="2")]
    pub billing: i32,
}
/// Billing identifies which billing structure the customer is using.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Billing {
    /// Unknown
    Unspecified = 0,
    /// User pays a fee per-endpoint.
    PayAsYouGo = 1,
    /// User is paying for Anthos as a whole.
    AnthosLicense = 2,
}
impl Billing {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Billing::Unspecified => "BILLING_UNSPECIFIED",
            Billing::PayAsYouGo => "PAY_AS_YOU_GO",
            Billing::AnthosLicense => "ANTHOS_LICENSE",
        }
    }
}
