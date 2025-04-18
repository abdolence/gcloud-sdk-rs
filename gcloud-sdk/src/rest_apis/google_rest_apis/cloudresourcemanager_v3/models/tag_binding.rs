use serde::{Deserialize, Serialize}; /*
                                      * Cloud Resource Manager API
                                      *
                                      * Creates, reads, and updates metadata for Google Cloud Platform resource containers.
                                      *
                                      * The version of the OpenAPI document: v3
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::cloudresourcemanager_v3::models;

/// TagBinding : A TagBinding represents a connection between a TagValue and a cloud resource Once a TagBinding is created, the TagValue is applied to all the descendants of the Google Cloud resource.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagBinding {
    /// Output only. The name of the TagBinding. This is a String of the form: `tagBindings/{full-resource-name}/{tag-value-name}` (e.g. `tagBindings/%2F%2Fcloudresourcemanager.googleapis.com%2Fprojects%2F123/tagValues/456`).
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The full resource name of the resource the TagValue is bound to. E.g. `//cloudresourcemanager.googleapis.com/projects/123`
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// The TagValue of the TagBinding. Must be of the form `tagValues/456`.
    #[serde(rename = "tagValue", skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// The namespaced name for the TagValue of the TagBinding. Must be in the format `{parent_id}/{tag_key_short_name}/{short_name}`. For methods that support TagValue namespaced name, only one of tag_value_namespaced_name or tag_value may be filled. Requests with both fields will be rejected.
    #[serde(
        rename = "tagValueNamespacedName",
        skip_serializing_if = "Option::is_none"
    )]
    pub tag_value_namespaced_name: Option<String>,
}

impl TagBinding {
    /// A TagBinding represents a connection between a TagValue and a cloud resource Once a TagBinding is created, the TagValue is applied to all the descendants of the Google Cloud resource.
    pub fn new() -> TagBinding {
        TagBinding {
            name: None,
            parent: None,
            tag_value: None,
            tag_value_namespaced_name: None,
        }
    }
}
