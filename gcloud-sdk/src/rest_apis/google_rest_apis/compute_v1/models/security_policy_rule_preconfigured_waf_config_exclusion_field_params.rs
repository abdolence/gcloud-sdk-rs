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
pub struct SecurityPolicyRulePreconfiguredWafConfigExclusionFieldParams {
    /// The match operator for the field.
    #[serde(rename = "op", skip_serializing_if = "Option::is_none")]
    pub op: Option<Op>,
    /// The value of the field.
    #[serde(rename = "val", skip_serializing_if = "Option::is_none")]
    pub val: Option<String>,
}

impl SecurityPolicyRulePreconfiguredWafConfigExclusionFieldParams {
    pub fn new() -> SecurityPolicyRulePreconfiguredWafConfigExclusionFieldParams {
        SecurityPolicyRulePreconfiguredWafConfigExclusionFieldParams {
            op: None,
            val: None,
        }
    }
}
/// The match operator for the field.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Op {
    #[serde(rename = "CONTAINS")]
    Contains,
    #[serde(rename = "ENDS_WITH")]
    EndsWith,
    #[serde(rename = "EQUALS")]
    Equals,
    #[serde(rename = "EQUALS_ANY")]
    EqualsAny,
    #[serde(rename = "STARTS_WITH")]
    StartsWith,
}

impl Default for Op {
    fn default() -> Op {
        Self::Contains
    }
}
