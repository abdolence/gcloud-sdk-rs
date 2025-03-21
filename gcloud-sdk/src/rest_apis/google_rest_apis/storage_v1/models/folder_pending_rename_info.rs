use serde::{Deserialize, Serialize}; /*
                                      * Cloud Storage JSON API
                                      *
                                      * Stores and retrieves potentially large, immutable data objects.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::storage_v1::models;

/// FolderPendingRenameInfo : Only present if the folder is part of an ongoing rename folder operation. Contains information which can be used to query the operation status.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FolderPendingRenameInfo {
    /// The ID of the rename folder operation.
    #[serde(rename = "operationId", skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

impl FolderPendingRenameInfo {
    /// Only present if the folder is part of an ongoing rename folder operation. Contains information which can be used to query the operation status.
    pub fn new() -> FolderPendingRenameInfo {
        FolderPendingRenameInfo { operation_id: None }
    }
}
