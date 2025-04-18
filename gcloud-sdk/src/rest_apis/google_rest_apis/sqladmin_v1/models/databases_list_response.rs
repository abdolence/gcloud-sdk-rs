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

/// DatabasesListResponse : Database list response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabasesListResponse {
    /// List of database resources in the instance.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::Database>>,
    /// This is always `sql#databasesList`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl DatabasesListResponse {
    /// Database list response.
    pub fn new() -> DatabasesListResponse {
        DatabasesListResponse {
            items: None,
            kind: None,
        }
    }
}
