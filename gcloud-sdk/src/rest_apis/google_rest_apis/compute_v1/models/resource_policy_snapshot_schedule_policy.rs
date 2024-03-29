use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// ResourcePolicySnapshotSchedulePolicy : A snapshot schedule policy specifies when and how frequently snapshots are to be created for the target disk. Also specifies how many and how long these scheduled snapshots should be retained.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResourcePolicySnapshotSchedulePolicy {
    #[serde(rename = "retentionPolicy", skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<Box<crate::google_rest_apis::compute_v1::models::ResourcePolicySnapshotSchedulePolicyRetentionPolicy>>,
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Box<crate::google_rest_apis::compute_v1::models::ResourcePolicySnapshotSchedulePolicySchedule>>,
    #[serde(rename = "snapshotProperties", skip_serializing_if = "Option::is_none")]
    pub snapshot_properties: Option<Box<crate::google_rest_apis::compute_v1::models::ResourcePolicySnapshotSchedulePolicySnapshotProperties>>,
}

impl ResourcePolicySnapshotSchedulePolicy {
    /// A snapshot schedule policy specifies when and how frequently snapshots are to be created for the target disk. Also specifies how many and how long these scheduled snapshots should be retained.
    pub fn new() -> ResourcePolicySnapshotSchedulePolicy {
        ResourcePolicySnapshotSchedulePolicy {
            retention_policy: None,
            schedule: None,
            snapshot_properties: None,
        }
    }
}
