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
pub struct RouterNatRule {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<models::RouterNatRuleAction>>,
    /// An optional description of this rule.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// CEL expression that specifies the match condition that egress traffic from a VM is evaluated against. If it evaluates to true, the corresponding `action` is enforced. The following examples are valid match expressions for public NAT: \"inIpRange(destination.ip, '1.1.0.0/16') || inIpRange(destination.ip, '2.2.0.0/16')\" \"destination.ip == '1.1.0.1' || destination.ip == '8.8.8.8'\" The following example is a valid match expression for private NAT: \"nexthop.hub == '//networkconnectivity.googleapis.com/projects/my-project/locations/global/hubs/hub-1'\"
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub r#match: Option<String>,
    /// An integer uniquely identifying a rule in the list. The rule number must be a positive value between 0 and 65000, and must be unique among rules within a NAT.
    #[serde(rename = "ruleNumber", skip_serializing_if = "Option::is_none")]
    pub rule_number: Option<i32>,
}

impl RouterNatRule {
    pub fn new() -> RouterNatRule {
        RouterNatRule {
            action: None,
            description: None,
            r#match: None,
            rule_number: None,
        }
    }
}
