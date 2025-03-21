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
pub struct ExchangedPeeringRoute {
    /// The destination range of the route.
    #[serde(rename = "destRange", skip_serializing_if = "Option::is_none")]
    pub dest_range: Option<String>,
    /// True if the peering route has been imported from a peer. The actual import happens if the field networkPeering.importCustomRoutes is true for this network, and networkPeering.exportCustomRoutes is true for the peer network, and the import does not result in a route conflict.
    #[serde(rename = "imported", skip_serializing_if = "Option::is_none")]
    pub imported: Option<bool>,
    /// The region of peering route next hop, only applies to dynamic routes.
    #[serde(rename = "nextHopRegion", skip_serializing_if = "Option::is_none")]
    pub next_hop_region: Option<String>,
    /// The priority of the peering route.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// The type of the peering route.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl ExchangedPeeringRoute {
    pub fn new() -> ExchangedPeeringRoute {
        ExchangedPeeringRoute {
            dest_range: None,
            imported: None,
            next_hop_region: None,
            priority: None,
            r#type: None,
        }
    }
}
/// The type of the peering route.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "DYNAMIC_PEERING_ROUTE")]
    DynamicPeeringRoute,
    #[serde(rename = "STATIC_PEERING_ROUTE")]
    StaticPeeringRoute,
    #[serde(rename = "SUBNET_PEERING_ROUTE")]
    SubnetPeeringRoute,
}

impl Default for Type {
    fn default() -> Type {
        Self::DynamicPeeringRoute
    }
}
