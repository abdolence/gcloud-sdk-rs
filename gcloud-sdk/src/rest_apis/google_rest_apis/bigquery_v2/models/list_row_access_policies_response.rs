use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::bigquery_v2::models;

/// ListRowAccessPoliciesResponse : Response message for the ListRowAccessPolicies method.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListRowAccessPoliciesResponse {
    /// A token to request the next page of results.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// Row access policies on the requested table.
    #[serde(rename = "rowAccessPolicies", skip_serializing_if = "Option::is_none")]
    pub row_access_policies: Option<Vec<models::RowAccessPolicy>>,
}

impl ListRowAccessPoliciesResponse {
    /// Response message for the ListRowAccessPolicies method.
    pub fn new() -> ListRowAccessPoliciesResponse {
        ListRowAccessPoliciesResponse {
            next_page_token: None,
            row_access_policies: None,
        }
    }
}
