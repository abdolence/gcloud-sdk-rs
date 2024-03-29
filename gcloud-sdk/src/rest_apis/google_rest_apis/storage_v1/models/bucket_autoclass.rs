use serde::{Deserialize, Serialize}; /*
                                      * Cloud Storage JSON API
                                      *
                                      * Stores and retrieves potentially large, immutable data objects.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// BucketAutoclass : The bucket's Autoclass configuration.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BucketAutoclass {
    /// Whether or not Autoclass is enabled on this bucket
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// A date and time in RFC 3339 format representing the instant at which \"enabled\" was last toggled.
    #[serde(rename = "toggleTime", skip_serializing_if = "Option::is_none")]
    pub toggle_time: Option<String>,
}

impl BucketAutoclass {
    /// The bucket's Autoclass configuration.
    pub fn new() -> BucketAutoclass {
        BucketAutoclass {
            enabled: None,
            toggle_time: None,
        }
    }
}
