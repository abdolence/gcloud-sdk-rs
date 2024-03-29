use serde::{Deserialize, Serialize}; /*
                                      * Cloud Storage JSON API
                                      *
                                      * Stores and retrieves potentially large, immutable data objects.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BucketLifecycleRuleInner {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action:
        Option<Box<crate::google_rest_apis::storage_v1::models::BucketLifecycleRuleInnerAction>>,
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition:
        Option<Box<crate::google_rest_apis::storage_v1::models::BucketLifecycleRuleInnerCondition>>,
}

impl BucketLifecycleRuleInner {
    pub fn new() -> BucketLifecycleRuleInner {
        BucketLifecycleRuleInner {
            action: None,
            condition: None,
        }
    }
}
