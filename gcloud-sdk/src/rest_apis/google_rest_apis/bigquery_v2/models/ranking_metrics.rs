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

/// RankingMetrics : Evaluation metrics used by weighted-ALS models specified by feedback_type=implicit.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RankingMetrics {
    /// Determines the goodness of a ranking by computing the percentile rank from the predicted confidence and dividing it by the original rank.
    #[serde(rename = "averageRank", skip_serializing_if = "Option::is_none")]
    pub average_rank: Option<f64>,
    /// Calculates a precision per user for all the items by ranking them and then averages all the precisions across all the users.
    #[serde(
        rename = "meanAveragePrecision",
        skip_serializing_if = "Option::is_none"
    )]
    pub mean_average_precision: Option<f64>,
    /// Similar to the mean squared error computed in regression and explicit recommendation models except instead of computing the rating directly, the output from evaluate is computed against a preference which is 1 or 0 depending on if the rating exists or not.
    #[serde(rename = "meanSquaredError", skip_serializing_if = "Option::is_none")]
    pub mean_squared_error: Option<f64>,
    /// A metric to determine the goodness of a ranking calculated from the predicted confidence by comparing it to an ideal rank measured by the original ratings.
    #[serde(
        rename = "normalizedDiscountedCumulativeGain",
        skip_serializing_if = "Option::is_none"
    )]
    pub normalized_discounted_cumulative_gain: Option<f64>,
}

impl RankingMetrics {
    /// Evaluation metrics used by weighted-ALS models specified by feedback_type=implicit.
    pub fn new() -> RankingMetrics {
        RankingMetrics {
            average_rank: None,
            mean_average_precision: None,
            mean_squared_error: None,
            normalized_discounted_cumulative_gain: None,
        }
    }
}
