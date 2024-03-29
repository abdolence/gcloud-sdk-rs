use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// InterconnectsGetDiagnosticsResponse : Response for the InterconnectsGetDiagnosticsRequest.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InterconnectsGetDiagnosticsResponse {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<crate::google_rest_apis::compute_v1::models::InterconnectDiagnostics>>,
}

impl InterconnectsGetDiagnosticsResponse {
    /// Response for the InterconnectsGetDiagnosticsRequest.
    pub fn new() -> InterconnectsGetDiagnosticsResponse {
        InterconnectsGetDiagnosticsResponse { result: None }
    }
}
