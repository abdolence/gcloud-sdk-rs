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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComposeRequestSourceObjectsInner {
    /// The generation of this object to use as the source.
    #[serde(rename = "generation", skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    /// The source object's name. All source objects must reside in the same bucket.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "objectPreconditions",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_preconditions:
        Option<Box<models::ComposeRequestSourceObjectsInnerObjectPreconditions>>,
}

impl ComposeRequestSourceObjectsInner {
    pub fn new() -> ComposeRequestSourceObjectsInner {
        ComposeRequestSourceObjectsInner {
            generation: None,
            name: None,
            object_preconditions: None,
        }
    }
}
