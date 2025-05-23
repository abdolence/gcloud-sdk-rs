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
pub struct SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfo {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<models::Status>>,
    /// [Output Only] Status of the action, which can be one of the following: `PROPAGATING`, `PROPAGATED`, `ABANDONED`, `FAILED`, or `DONE`.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

impl SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfo {
    pub fn new() -> SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfo {
        SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfo {
            error: None,
            state: None,
        }
    }
}
/// [Output Only] Status of the action, which can be one of the following: `PROPAGATING`, `PROPAGATED`, `ABANDONED`, `FAILED`, or `DONE`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "ABANDONED")]
    Abandoned,
    #[serde(rename = "DONE")]
    Done,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "PROPAGATED")]
    Propagated,
    #[serde(rename = "PROPAGATING")]
    Propagating,
    #[serde(rename = "UNSPECIFIED")]
    Unspecified,
}

impl Default for State {
    fn default() -> State {
        Self::Abandoned
    }
}
