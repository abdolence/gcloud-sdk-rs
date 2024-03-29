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
pub struct InstancesScopedList {
    /// [Output Only] A list of instances contained in this scope.
    #[serde(rename = "instances", skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<crate::google_rest_apis::compute_v1::models::Instance>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning:
        Option<Box<crate::google_rest_apis::compute_v1::models::InstancesScopedListWarning>>,
}

impl InstancesScopedList {
    pub fn new() -> InstancesScopedList {
        InstancesScopedList {
            instances: None,
            warning: None,
        }
    }
}
