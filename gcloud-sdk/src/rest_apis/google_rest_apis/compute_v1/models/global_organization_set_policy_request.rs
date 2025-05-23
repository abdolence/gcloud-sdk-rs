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
pub struct GlobalOrganizationSetPolicyRequest {
    /// Flatten Policy to create a backward compatible wire-format. Deprecated. Use 'policy' to specify bindings.
    #[serde(rename = "bindings", skip_serializing_if = "Option::is_none")]
    pub bindings: Option<Vec<models::Binding>>,
    /// Flatten Policy to create a backward compatible wire-format. Deprecated. Use 'policy' to specify the etag.
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<Box<models::Policy>>,
}

impl GlobalOrganizationSetPolicyRequest {
    pub fn new() -> GlobalOrganizationSetPolicyRequest {
        GlobalOrganizationSetPolicyRequest {
            bindings: None,
            etag: None,
            policy: None,
        }
    }
}
