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
pub struct NetworkEndpointWithHealthStatus {
    /// [Output only] The health status of network endpoint;
    #[serde(rename = "healths", skip_serializing_if = "Option::is_none")]
    pub healths:
        Option<Vec<crate::google_rest_apis::compute_v1::models::HealthStatusForNetworkEndpoint>>,
    #[serde(rename = "networkEndpoint", skip_serializing_if = "Option::is_none")]
    pub network_endpoint: Option<Box<crate::google_rest_apis::compute_v1::models::NetworkEndpoint>>,
}

impl NetworkEndpointWithHealthStatus {
    pub fn new() -> NetworkEndpointWithHealthStatus {
        NetworkEndpointWithHealthStatus {
            healths: None,
            network_endpoint: None,
        }
    }
}
