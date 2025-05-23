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

/// GoogleLongrunningListOperationsResponse : The response message for storage.buckets.operations.list.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GoogleLongrunningListOperationsResponse {
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    #[serde(rename = "operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<models::GoogleLongrunningOperation>>,
}

impl GoogleLongrunningListOperationsResponse {
    /// The response message for storage.buckets.operations.list.
    pub fn new() -> GoogleLongrunningListOperationsResponse {
        GoogleLongrunningListOperationsResponse {
            next_page_token: None,
            operations: None,
        }
    }
}
