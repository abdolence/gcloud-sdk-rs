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
pub struct InstanceGroupManagerStatusAllInstancesConfig {
    /// [Output Only] Current all-instances configuration revision. This value is in RFC3339 text format.
    #[serde(rename = "currentRevision", skip_serializing_if = "Option::is_none")]
    pub current_revision: Option<String>,
    /// [Output Only] A bit indicating whether this configuration has been applied to all managed instances in the group.
    #[serde(rename = "effective", skip_serializing_if = "Option::is_none")]
    pub effective: Option<bool>,
}

impl InstanceGroupManagerStatusAllInstancesConfig {
    pub fn new() -> InstanceGroupManagerStatusAllInstancesConfig {
        InstanceGroupManagerStatusAllInstancesConfig {
            current_revision: None,
            effective: None,
        }
    }
}
