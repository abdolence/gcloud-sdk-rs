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

/// ComposeRequestSourceObjectsInnerObjectPreconditions : Conditions that must be met for this operation to execute.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComposeRequestSourceObjectsInnerObjectPreconditions {
    /// Only perform the composition if the generation of the source object that would be used matches this value. If this value and a generation are both specified, they must be the same value or the call will fail.
    #[serde(rename = "ifGenerationMatch", skip_serializing_if = "Option::is_none")]
    pub if_generation_match: Option<String>,
}

impl ComposeRequestSourceObjectsInnerObjectPreconditions {
    /// Conditions that must be met for this operation to execute.
    pub fn new() -> ComposeRequestSourceObjectsInnerObjectPreconditions {
        ComposeRequestSourceObjectsInnerObjectPreconditions {
            if_generation_match: None,
        }
    }
}
