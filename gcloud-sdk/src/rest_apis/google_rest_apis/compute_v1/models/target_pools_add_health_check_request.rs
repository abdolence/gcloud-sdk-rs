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
pub struct TargetPoolsAddHealthCheckRequest {
    /// The HttpHealthCheck to add to the target pool.
    #[serde(rename = "healthChecks", skip_serializing_if = "Option::is_none")]
    pub health_checks: Option<Vec<models::HealthCheckReference>>,
}

impl TargetPoolsAddHealthCheckRequest {
    pub fn new() -> TargetPoolsAddHealthCheckRequest {
        TargetPoolsAddHealthCheckRequest {
            health_checks: None,
        }
    }
}
