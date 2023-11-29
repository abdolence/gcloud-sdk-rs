use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// PublicDelegatedPrefixPublicDelegatedSubPrefix : Represents a sub PublicDelegatedPrefix.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PublicDelegatedPrefixPublicDelegatedSubPrefix {
    /// Name of the project scoping this PublicDelegatedSubPrefix.
    #[serde(rename = "delegateeProject", skip_serializing_if = "Option::is_none")]
    pub delegatee_project: Option<String>,
    /// An optional description of this resource. Provide this property when you create the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The IP address range, in CIDR format, represented by this sub public delegated prefix.
    #[serde(rename = "ipCidrRange", skip_serializing_if = "Option::is_none")]
    pub ip_cidr_range: Option<String>,
    /// Whether the sub prefix is delegated to create Address resources in the delegatee project.
    #[serde(rename = "isAddress", skip_serializing_if = "Option::is_none")]
    pub is_address: Option<bool>,
    /// The name of the sub public delegated prefix.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [Output Only] The region of the sub public delegated prefix if it is regional. If absent, the sub prefix is global.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// [Output Only] The status of the sub public delegated prefix.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl PublicDelegatedPrefixPublicDelegatedSubPrefix {
    /// Represents a sub PublicDelegatedPrefix.
    pub fn new() -> PublicDelegatedPrefixPublicDelegatedSubPrefix {
        PublicDelegatedPrefixPublicDelegatedSubPrefix {
            delegatee_project: None,
            description: None,
            ip_cidr_range: None,
            is_address: None,
            name: None,
            region: None,
            status: None,
        }
    }
}

/// [Output Only] The status of the sub public delegated prefix.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "INACTIVE")]
    Inactive,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}