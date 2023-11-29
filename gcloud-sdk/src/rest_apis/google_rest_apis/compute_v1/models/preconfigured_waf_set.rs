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
pub struct PreconfiguredWafSet {
    /// List of entities that are currently supported for WAF rules.
    #[serde(rename = "expressionSets", skip_serializing_if = "Option::is_none")]
    pub expression_sets: Option<Vec<crate::google_rest_apis::compute_v1::models::WafExpressionSet>>,
}

impl PreconfiguredWafSet {
    pub fn new() -> PreconfiguredWafSet {
        PreconfiguredWafSet {
            expression_sets: None,
        }
    }
}