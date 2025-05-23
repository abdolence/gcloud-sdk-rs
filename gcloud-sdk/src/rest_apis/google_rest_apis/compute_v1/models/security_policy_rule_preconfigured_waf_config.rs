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
pub struct SecurityPolicyRulePreconfiguredWafConfig {
    /// A list of exclusions to apply during preconfigured WAF evaluation.
    #[serde(rename = "exclusions", skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<models::SecurityPolicyRulePreconfiguredWafConfigExclusion>>,
}

impl SecurityPolicyRulePreconfiguredWafConfig {
    pub fn new() -> SecurityPolicyRulePreconfiguredWafConfig {
        SecurityPolicyRulePreconfiguredWafConfig { exclusions: None }
    }
}
