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
pub struct VpnGatewayStatus {
    /// List of VPN connection for this VpnGateway.
    #[serde(rename = "vpnConnections", skip_serializing_if = "Option::is_none")]
    pub vpn_connections:
        Option<Vec<crate::google_rest_apis::compute_v1::models::VpnGatewayStatusVpnConnection>>,
}

impl VpnGatewayStatus {
    pub fn new() -> VpnGatewayStatus {
        VpnGatewayStatus {
            vpn_connections: None,
        }
    }
}
