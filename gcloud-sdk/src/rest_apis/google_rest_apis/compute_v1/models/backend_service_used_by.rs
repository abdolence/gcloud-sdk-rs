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
pub struct BackendServiceUsedBy {
    #[serde(rename = "reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

impl BackendServiceUsedBy {
    pub fn new() -> BackendServiceUsedBy {
        BackendServiceUsedBy { reference: None }
    }
}
