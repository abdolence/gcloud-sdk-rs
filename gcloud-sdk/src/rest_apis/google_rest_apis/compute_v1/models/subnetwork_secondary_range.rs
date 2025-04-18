use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::compute_v1::models;

/// SubnetworkSecondaryRange : Represents a secondary IP range of a subnetwork.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubnetworkSecondaryRange {
    /// The range of IP addresses belonging to this subnetwork secondary range. Provide this property when you create the subnetwork. Ranges must be unique and non-overlapping with all primary and secondary IP ranges within a network. Only IPv4 is supported. The range can be any range listed in the Valid ranges list.
    #[serde(rename = "ipCidrRange", skip_serializing_if = "Option::is_none")]
    pub ip_cidr_range: Option<String>,
    /// The name associated with this subnetwork secondary range, used when adding an alias IP range to a VM instance. The name must be 1-63 characters long, and comply with RFC1035. The name must be unique within the subnetwork.
    #[serde(rename = "rangeName", skip_serializing_if = "Option::is_none")]
    pub range_name: Option<String>,
    /// The URL of the reserved internal range.
    #[serde(
        rename = "reservedInternalRange",
        skip_serializing_if = "Option::is_none"
    )]
    pub reserved_internal_range: Option<String>,
}

impl SubnetworkSecondaryRange {
    /// Represents a secondary IP range of a subnetwork.
    pub fn new() -> SubnetworkSecondaryRange {
        SubnetworkSecondaryRange {
            ip_cidr_range: None,
            range_name: None,
            reserved_internal_range: None,
        }
    }
}
