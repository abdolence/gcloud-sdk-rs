use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// BulkInsertInstanceResourcePerInstanceProperties : Per-instance properties to be set on individual instances. To be extended in the future.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BulkInsertInstanceResourcePerInstanceProperties {
    /// Specifies the hostname of the instance. More details in: https://cloud.google.com/compute/docs/instances/custom-hostname-vm#naming_convention
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// This field is only temporary. It will be removed. Do not use it.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl BulkInsertInstanceResourcePerInstanceProperties {
    /// Per-instance properties to be set on individual instances. To be extended in the future.
    pub fn new() -> BulkInsertInstanceResourcePerInstanceProperties {
        BulkInsertInstanceResourcePerInstanceProperties {
            hostname: None,
            name: None,
        }
    }
}