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
pub struct GlobalNetworkEndpointGroupsDetachEndpointsRequest {
    /// The list of network endpoints to be detached.
    #[serde(rename = "networkEndpoints", skip_serializing_if = "Option::is_none")]
    pub network_endpoints: Option<Vec<models::NetworkEndpoint>>,
}

impl GlobalNetworkEndpointGroupsDetachEndpointsRequest {
    pub fn new() -> GlobalNetworkEndpointGroupsDetachEndpointsRequest {
        GlobalNetworkEndpointGroupsDetachEndpointsRequest {
            network_endpoints: None,
        }
    }
}
