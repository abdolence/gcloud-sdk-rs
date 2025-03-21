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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstanceManagedByIgmErrorManagedInstanceError {
    /// [Output Only] Error code.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// [Output Only] Error message.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl InstanceManagedByIgmErrorManagedInstanceError {
    pub fn new() -> InstanceManagedByIgmErrorManagedInstanceError {
        InstanceManagedByIgmErrorManagedInstanceError {
            code: None,
            message: None,
        }
    }
}
