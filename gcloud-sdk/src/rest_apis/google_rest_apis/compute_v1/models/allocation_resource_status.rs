use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// AllocationResourceStatus : [Output Only] Contains output only fields.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AllocationResourceStatus {
    #[serde(rename = "specificSkuAllocation", skip_serializing_if = "Option::is_none")]
    pub specific_sku_allocation: Option<Box<crate::google_rest_apis::compute_v1::models::AllocationResourceStatusSpecificSkuAllocation>>,
}

impl AllocationResourceStatus {
    /// [Output Only] Contains output only fields.
    pub fn new() -> AllocationResourceStatus {
        AllocationResourceStatus {
            specific_sku_allocation: None,
        }
    }
}
