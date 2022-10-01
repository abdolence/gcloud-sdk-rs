use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// RangePartitioningRange : [TrustedTester] [Required] Defines the ranges for range partitioning.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RangePartitioningRange {
    /// [TrustedTester] [Required] The end of range partitioning, exclusive.
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// [TrustedTester] [Required] The width of each interval.
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// [TrustedTester] [Required] The start of range partitioning, inclusive.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

impl RangePartitioningRange {
    /// [TrustedTester] [Required] Defines the ranges for range partitioning.
    pub fn new() -> RangePartitioningRange {
        RangePartitioningRange {
            end: None,
            interval: None,
            start: None,
        }
    }
}