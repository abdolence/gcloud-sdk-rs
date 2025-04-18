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
pub struct NodeGroupNode {
    /// Accelerators for this node.
    #[serde(rename = "accelerators", skip_serializing_if = "Option::is_none")]
    pub accelerators: Option<Vec<models::AcceleratorConfig>>,
    #[serde(rename = "consumedResources", skip_serializing_if = "Option::is_none")]
    pub consumed_resources: Option<Box<models::InstanceConsumptionInfo>>,
    /// CPU overcommit.
    #[serde(rename = "cpuOvercommitType", skip_serializing_if = "Option::is_none")]
    pub cpu_overcommit_type: Option<CpuOvercommitType>,
    /// Local disk configurations.
    #[serde(rename = "disks", skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<models::LocalDisk>>,
    /// Instance data that shows consumed resources on the node.
    #[serde(
        rename = "instanceConsumptionData",
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_consumption_data: Option<Vec<models::InstanceConsumptionData>>,
    /// Instances scheduled on this node.
    #[serde(rename = "instances", skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<String>>,
    /// The name of the node.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of this node.
    #[serde(rename = "nodeType", skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// [Output Only] Reserved for future use.
    #[serde(rename = "satisfiesPzs", skip_serializing_if = "Option::is_none")]
    pub satisfies_pzs: Option<bool>,
    #[serde(rename = "serverBinding", skip_serializing_if = "Option::is_none")]
    pub server_binding: Option<Box<models::ServerBinding>>,
    /// Server ID associated with this node.
    #[serde(rename = "serverId", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "totalResources", skip_serializing_if = "Option::is_none")]
    pub total_resources: Option<Box<models::InstanceConsumptionInfo>>,
}

impl NodeGroupNode {
    pub fn new() -> NodeGroupNode {
        NodeGroupNode {
            accelerators: None,
            consumed_resources: None,
            cpu_overcommit_type: None,
            disks: None,
            instance_consumption_data: None,
            instances: None,
            name: None,
            node_type: None,
            satisfies_pzs: None,
            server_binding: None,
            server_id: None,
            status: None,
            total_resources: None,
        }
    }
}
/// CPU overcommit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CpuOvercommitType {
    #[serde(rename = "CPU_OVERCOMMIT_TYPE_UNSPECIFIED")]
    CpuOvercommitTypeUnspecified,
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "NONE")]
    None,
}

impl Default for CpuOvercommitType {
    fn default() -> CpuOvercommitType {
        Self::CpuOvercommitTypeUnspecified
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "CREATING")]
    Creating,
    #[serde(rename = "DELETING")]
    Deleting,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "READY")]
    Ready,
    #[serde(rename = "REPAIRING")]
    Repairing,
}

impl Default for Status {
    fn default() -> Status {
        Self::Creating
    }
}
