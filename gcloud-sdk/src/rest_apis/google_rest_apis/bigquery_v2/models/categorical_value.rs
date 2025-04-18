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

/// CategoricalValue : Representative value of a categorical feature.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CategoricalValue {
    /// Counts of all categories for the categorical feature. If there are more than ten categories, we return top ten (by count) and return one more CategoryCount with category \"_OTHER_\" and count as aggregate counts of remaining categories.
    #[serde(rename = "categoryCounts", skip_serializing_if = "Option::is_none")]
    pub category_counts: Option<Vec<models::CategoryCount>>,
}

impl CategoricalValue {
    /// Representative value of a categorical feature.
    pub fn new() -> CategoricalValue {
        CategoricalValue {
            category_counts: None,
        }
    }
}
