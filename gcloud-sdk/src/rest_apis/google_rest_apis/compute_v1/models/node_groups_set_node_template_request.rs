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
pub struct NodeGroupsSetNodeTemplateRequest {
    /// Full or partial URL of the node template resource to be updated for this node group.
    #[serde(rename = "nodeTemplate", skip_serializing_if = "Option::is_none")]
    pub node_template: Option<String>,
}

impl NodeGroupsSetNodeTemplateRequest {
    pub fn new() -> NodeGroupsSetNodeTemplateRequest {
        NodeGroupsSetNodeTemplateRequest {
            node_template: None,
        }
    }
}
