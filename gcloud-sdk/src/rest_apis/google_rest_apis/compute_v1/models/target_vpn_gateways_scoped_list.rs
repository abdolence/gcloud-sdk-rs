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
pub struct TargetVpnGatewaysScopedList {
    /// [Output Only] A list of target VPN gateways contained in this scope.
    #[serde(rename = "targetVpnGateways", skip_serializing_if = "Option::is_none")]
    pub target_vpn_gateways:
        Option<Vec<crate::google_rest_apis::compute_v1::models::TargetVpnGateway>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning:
        Option<Box<crate::google_rest_apis::compute_v1::models::AddressesScopedListWarning>>,
}

impl TargetVpnGatewaysScopedList {
    pub fn new() -> TargetVpnGatewaysScopedList {
        TargetVpnGatewaysScopedList {
            target_vpn_gateways: None,
            warning: None,
        }
    }
}
