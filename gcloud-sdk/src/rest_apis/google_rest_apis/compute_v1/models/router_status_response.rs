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
pub struct RouterStatusResponse {
    /// Type of resource.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<crate::google_rest_apis::compute_v1::models::RouterStatus>>,
}

impl RouterStatusResponse {
    pub fn new() -> RouterStatusResponse {
        RouterStatusResponse {
            kind: None,
            result: None,
        }
    }
}