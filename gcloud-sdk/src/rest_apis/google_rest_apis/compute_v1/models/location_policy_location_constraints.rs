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

/// LocationPolicyLocationConstraints : Per-zone constraints on location policy for this zone.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationPolicyLocationConstraints {
    /// Maximum number of items that are allowed to be placed in this zone. The value must be non-negative.
    #[serde(rename = "maxCount", skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i32>,
}

impl LocationPolicyLocationConstraints {
    /// Per-zone constraints on location policy for this zone.
    pub fn new() -> LocationPolicyLocationConstraints {
        LocationPolicyLocationConstraints { max_count: None }
    }
}
