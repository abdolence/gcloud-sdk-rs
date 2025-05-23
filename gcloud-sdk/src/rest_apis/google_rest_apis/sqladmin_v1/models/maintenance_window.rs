use serde::{Deserialize, Serialize}; /*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::sqladmin_v1::models;

/// MaintenanceWindow : Maintenance window. This specifies when a Cloud SQL instance is restarted for system maintenance purposes.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    /// day of week (1-7), starting on Monday.
    #[serde(rename = "day", skip_serializing_if = "Option::is_none")]
    pub day: Option<i32>,
    /// hour of day - 0 to 23.
    #[serde(rename = "hour", skip_serializing_if = "Option::is_none")]
    pub hour: Option<i32>,
    /// This is always `sql#maintenanceWindow`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Maintenance timing setting: `canary` (Earlier) or `stable` (Later). [Learn more](https://cloud.google.com/sql/docs/mysql/instance-settings#maintenance-timing-2ndgen).
    #[serde(rename = "updateTrack", skip_serializing_if = "Option::is_none")]
    pub update_track: Option<UpdateTrack>,
}

impl MaintenanceWindow {
    /// Maintenance window. This specifies when a Cloud SQL instance is restarted for system maintenance purposes.
    pub fn new() -> MaintenanceWindow {
        MaintenanceWindow {
            day: None,
            hour: None,
            kind: None,
            update_track: None,
        }
    }
}
/// Maintenance timing setting: `canary` (Earlier) or `stable` (Later). [Learn more](https://cloud.google.com/sql/docs/mysql/instance-settings#maintenance-timing-2ndgen).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UpdateTrack {
    #[serde(rename = "SQL_UPDATE_TRACK_UNSPECIFIED")]
    SqlUpdateTrackUnspecified,
    #[serde(rename = "canary")]
    Canary,
    #[serde(rename = "stable")]
    Stable,
}

impl Default for UpdateTrack {
    fn default() -> UpdateTrack {
        Self::SqlUpdateTrackUnspecified
    }
}
