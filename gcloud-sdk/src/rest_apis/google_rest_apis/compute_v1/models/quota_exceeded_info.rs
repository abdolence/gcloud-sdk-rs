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

/// QuotaExceededInfo : Additional details for quota exceeded error for resource quota.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaExceededInfo {
    /// The map holding related quota dimensions.
    #[serde(rename = "dimensions", skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<std::collections::HashMap<String, String>>,
    /// Future quota limit being rolled out. The limit's unit depends on the quota type or metric.
    #[serde(rename = "futureLimit", skip_serializing_if = "Option::is_none")]
    pub future_limit: Option<f64>,
    /// Current effective quota limit. The limit's unit depends on the quota type or metric.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    /// The name of the quota limit.
    #[serde(rename = "limitName", skip_serializing_if = "Option::is_none")]
    pub limit_name: Option<String>,
    /// The Compute Engine quota metric name.
    #[serde(rename = "metricName", skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// Rollout status of the future quota limit.
    #[serde(rename = "rolloutStatus", skip_serializing_if = "Option::is_none")]
    pub rollout_status: Option<RolloutStatus>,
}

impl QuotaExceededInfo {
    /// Additional details for quota exceeded error for resource quota.
    pub fn new() -> QuotaExceededInfo {
        QuotaExceededInfo {
            dimensions: None,
            future_limit: None,
            limit: None,
            limit_name: None,
            metric_name: None,
            rollout_status: None,
        }
    }
}
/// Rollout status of the future quota limit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RolloutStatus {
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "ROLLOUT_STATUS_UNSPECIFIED")]
    RolloutStatusUnspecified,
}

impl Default for RolloutStatus {
    fn default() -> RolloutStatus {
        Self::InProgress
    }
}
