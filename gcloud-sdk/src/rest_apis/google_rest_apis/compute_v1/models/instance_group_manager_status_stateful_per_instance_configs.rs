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
pub struct InstanceGroupManagerStatusStatefulPerInstanceConfigs {
    /// A bit indicating if all of the group's per-instance configurations (listed in the output of a listPerInstanceConfigs API call) have status EFFECTIVE or there are no per-instance-configs.
    #[serde(rename = "allEffective", skip_serializing_if = "Option::is_none")]
    pub all_effective: Option<bool>,
}

impl InstanceGroupManagerStatusStatefulPerInstanceConfigs {
    pub fn new() -> InstanceGroupManagerStatusStatefulPerInstanceConfigs {
        InstanceGroupManagerStatusStatefulPerInstanceConfigs {
            all_effective: None,
        }
    }
}
