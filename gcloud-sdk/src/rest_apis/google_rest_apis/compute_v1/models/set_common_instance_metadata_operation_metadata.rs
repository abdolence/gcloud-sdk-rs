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
pub struct SetCommonInstanceMetadataOperationMetadata {
    /// [Output Only] The client operation id.
    #[serde(rename = "clientOperationId", skip_serializing_if = "Option::is_none")]
    pub client_operation_id: Option<String>,
    /// [Output Only] Status information per location (location name is key). Example key: zones/us-central1-a
    #[serde(
        rename = "perLocationOperations",
        skip_serializing_if = "Option::is_none"
    )]
    pub per_location_operations: Option<
        std::collections::HashMap<
            String,
            models::SetCommonInstanceMetadataOperationMetadataPerLocationOperationInfo,
        >,
    >,
}

impl SetCommonInstanceMetadataOperationMetadata {
    pub fn new() -> SetCommonInstanceMetadataOperationMetadata {
        SetCommonInstanceMetadataOperationMetadata {
            client_operation_id: None,
            per_location_operations: None,
        }
    }
}
