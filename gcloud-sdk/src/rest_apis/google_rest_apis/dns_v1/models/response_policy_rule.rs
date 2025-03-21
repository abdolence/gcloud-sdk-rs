use serde::{Deserialize, Serialize}; /*
                                      * Cloud DNS API
                                      *
                                      * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::dns_v1::models;

/// ResponsePolicyRule : A Response Policy Rule is a selector that applies its behavior to queries that match the selector. Selectors are DNS names, which may be wildcards or exact matches. Each DNS query subject to a Response Policy matches at most one ResponsePolicyRule, as identified by the dns_name field with the longest matching suffix.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponsePolicyRule {
    /// Answer this query with a behavior rather than DNS data.
    #[serde(rename = "behavior", skip_serializing_if = "Option::is_none")]
    pub behavior: Option<Behavior>,
    /// The DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule.
    #[serde(rename = "dnsName", skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "localData", skip_serializing_if = "Option::is_none")]
    pub local_data: Option<Box<models::ResponsePolicyRuleLocalData>>,
    /// An identifier for this rule. Must be unique with the ResponsePolicy.
    #[serde(rename = "ruleName", skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
}

impl ResponsePolicyRule {
    /// A Response Policy Rule is a selector that applies its behavior to queries that match the selector. Selectors are DNS names, which may be wildcards or exact matches. Each DNS query subject to a Response Policy matches at most one ResponsePolicyRule, as identified by the dns_name field with the longest matching suffix.
    pub fn new() -> ResponsePolicyRule {
        ResponsePolicyRule {
            behavior: None,
            dns_name: None,
            kind: None,
            local_data: None,
            rule_name: None,
        }
    }
}
/// Answer this query with a behavior rather than DNS data.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Behavior {
    #[serde(rename = "behaviorUnspecified")]
    BehaviorUnspecified,
    #[serde(rename = "bypassResponsePolicy")]
    BypassResponsePolicy,
}

impl Default for Behavior {
    fn default() -> Behavior {
        Self::BehaviorUnspecified
    }
}
