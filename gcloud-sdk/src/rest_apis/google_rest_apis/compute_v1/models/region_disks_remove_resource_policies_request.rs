use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RegionDisksRemoveResourcePoliciesRequest {
    /// Resource policies to be removed from this disk.
    #[serde(rename = "resourcePolicies", skip_serializing_if = "Option::is_none")]
    pub resource_policies: Option<Vec<String>>,
}

impl RegionDisksRemoveResourcePoliciesRequest {
    pub fn new() -> RegionDisksRemoveResourcePoliciesRequest {
        RegionDisksRemoveResourcePoliciesRequest {
            resource_policies: None,
        }
    }
}
