use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// RegionInstanceGroupManagersCreateInstancesRequest : RegionInstanceGroupManagers.createInstances

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RegionInstanceGroupManagersCreateInstancesRequest {
    /// [Required] List of specifications of per-instance configs.
    #[serde(rename = "instances", skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<crate::google_rest_apis::compute_v1::models::PerInstanceConfig>>,
}

impl RegionInstanceGroupManagersCreateInstancesRequest {
    /// RegionInstanceGroupManagers.createInstances
    pub fn new() -> RegionInstanceGroupManagersCreateInstancesRequest {
        RegionInstanceGroupManagersCreateInstancesRequest { instances: None }
    }
}
