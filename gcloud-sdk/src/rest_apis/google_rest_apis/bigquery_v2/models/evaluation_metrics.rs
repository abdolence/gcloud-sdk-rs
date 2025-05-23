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

/// EvaluationMetrics : Evaluation metrics of a model. These are either computed on all training data or just the eval data based on whether eval data was used during training. These are not present for imported models.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvaluationMetrics {
    #[serde(
        rename = "arimaForecastingMetrics",
        skip_serializing_if = "Option::is_none"
    )]
    pub arima_forecasting_metrics: Option<Box<models::ArimaForecastingMetrics>>,
    #[serde(
        rename = "binaryClassificationMetrics",
        skip_serializing_if = "Option::is_none"
    )]
    pub binary_classification_metrics: Option<Box<models::BinaryClassificationMetrics>>,
    #[serde(rename = "clusteringMetrics", skip_serializing_if = "Option::is_none")]
    pub clustering_metrics: Option<Box<models::ClusteringMetrics>>,
    #[serde(
        rename = "dimensionalityReductionMetrics",
        skip_serializing_if = "Option::is_none"
    )]
    pub dimensionality_reduction_metrics: Option<Box<models::DimensionalityReductionMetrics>>,
    #[serde(
        rename = "multiClassClassificationMetrics",
        skip_serializing_if = "Option::is_none"
    )]
    pub multi_class_classification_metrics: Option<Box<models::MultiClassClassificationMetrics>>,
    #[serde(rename = "rankingMetrics", skip_serializing_if = "Option::is_none")]
    pub ranking_metrics: Option<Box<models::RankingMetrics>>,
    #[serde(rename = "regressionMetrics", skip_serializing_if = "Option::is_none")]
    pub regression_metrics: Option<Box<models::RegressionMetrics>>,
}

impl EvaluationMetrics {
    /// Evaluation metrics of a model. These are either computed on all training data or just the eval data based on whether eval data was used during training. These are not present for imported models.
    pub fn new() -> EvaluationMetrics {
        EvaluationMetrics {
            arima_forecasting_metrics: None,
            binary_classification_metrics: None,
            clustering_metrics: None,
            dimensionality_reduction_metrics: None,
            multi_class_classification_metrics: None,
            ranking_metrics: None,
            regression_metrics: None,
        }
    }
}
