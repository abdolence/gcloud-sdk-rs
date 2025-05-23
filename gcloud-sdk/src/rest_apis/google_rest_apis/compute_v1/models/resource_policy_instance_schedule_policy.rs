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

/// ResourcePolicyInstanceSchedulePolicy : An InstanceSchedulePolicy specifies when and how frequent certain operations are performed on the instance.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcePolicyInstanceSchedulePolicy {
    /// The expiration time of the schedule. The timestamp is an RFC3339 string.
    #[serde(rename = "expirationTime", skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// The start time of the schedule. The timestamp is an RFC3339 string.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// Specifies the time zone to be used in interpreting Schedule.schedule. The value of this field must be a time zone name from the tz database: https://wikipedia.org/wiki/Tz_database.
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(rename = "vmStartSchedule", skip_serializing_if = "Option::is_none")]
    pub vm_start_schedule: Option<Box<models::ResourcePolicyInstanceSchedulePolicySchedule>>,
    #[serde(rename = "vmStopSchedule", skip_serializing_if = "Option::is_none")]
    pub vm_stop_schedule: Option<Box<models::ResourcePolicyInstanceSchedulePolicySchedule>>,
}

impl ResourcePolicyInstanceSchedulePolicy {
    /// An InstanceSchedulePolicy specifies when and how frequent certain operations are performed on the instance.
    pub fn new() -> ResourcePolicyInstanceSchedulePolicy {
        ResourcePolicyInstanceSchedulePolicy {
            expiration_time: None,
            start_time: None,
            time_zone: None,
            vm_start_schedule: None,
            vm_stop_schedule: None,
        }
    }
}
