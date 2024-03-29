use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// CategoryCount : Represents the count of a single category within the cluster.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CategoryCount {
    /// The name of category.
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The count of training samples matching the category within the cluster.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
}

impl CategoryCount {
    /// Represents the count of a single category within the cluster.
    pub fn new() -> CategoryCount {
        CategoryCount {
            category: None,
            count: None,
        }
    }
}
