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
