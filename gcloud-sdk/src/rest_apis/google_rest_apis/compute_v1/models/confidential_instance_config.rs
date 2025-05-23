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

/// ConfidentialInstanceConfig : A set of Confidential Instance options.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfidentialInstanceConfig {
    /// Defines whether the instance should have confidential compute enabled.
    #[serde(
        rename = "enableConfidentialCompute",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_confidential_compute: Option<bool>,
}

impl ConfidentialInstanceConfig {
    /// A set of Confidential Instance options.
    pub fn new() -> ConfidentialInstanceConfig {
        ConfidentialInstanceConfig {
            enable_confidential_compute: None,
        }
    }
}
