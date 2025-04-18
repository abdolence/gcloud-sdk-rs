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

/// ArimaForecastingMetrics : Model evaluation metrics for ARIMA forecasting models.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArimaForecastingMetrics {
    /// Arima model fitting metrics.
    #[serde(
        rename = "arimaFittingMetrics",
        skip_serializing_if = "Option::is_none"
    )]
    pub arima_fitting_metrics: Option<Vec<models::ArimaFittingMetrics>>,
    /// Repeated as there can be many metric sets (one for each model) in auto-arima and the large-scale case.
    #[serde(
        rename = "arimaSingleModelForecastingMetrics",
        skip_serializing_if = "Option::is_none"
    )]
    pub arima_single_model_forecasting_metrics:
        Option<Vec<models::ArimaSingleModelForecastingMetrics>>,
    /// Whether Arima model fitted with drift or not. It is always false when d is not 1.
    #[serde(rename = "hasDrift", skip_serializing_if = "Option::is_none")]
    pub has_drift: Option<Vec<bool>>,
    /// Non-seasonal order.
    #[serde(rename = "nonSeasonalOrder", skip_serializing_if = "Option::is_none")]
    pub non_seasonal_order: Option<Vec<models::ArimaOrder>>,
    /// Seasonal periods. Repeated because multiple periods are supported for one time series.
    #[serde(rename = "seasonalPeriods", skip_serializing_if = "Option::is_none")]
    pub seasonal_periods: Option<Vec<SeasonalPeriods>>,
    /// Id to differentiate different time series for the large-scale case.
    #[serde(rename = "timeSeriesId", skip_serializing_if = "Option::is_none")]
    pub time_series_id: Option<Vec<String>>,
}

impl ArimaForecastingMetrics {
    /// Model evaluation metrics for ARIMA forecasting models.
    pub fn new() -> ArimaForecastingMetrics {
        ArimaForecastingMetrics {
            arima_fitting_metrics: None,
            arima_single_model_forecasting_metrics: None,
            has_drift: None,
            non_seasonal_order: None,
            seasonal_periods: None,
            time_series_id: None,
        }
    }
}
/// Seasonal periods. Repeated because multiple periods are supported for one time series.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SeasonalPeriods {
    #[serde(rename = "SEASONAL_PERIOD_TYPE_UNSPECIFIED")]
    SeasonalPeriodTypeUnspecified,
    #[serde(rename = "NO_SEASONALITY")]
    NoSeasonality,
    #[serde(rename = "DAILY")]
    Daily,
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(rename = "MONTHLY")]
    Monthly,
    #[serde(rename = "QUARTERLY")]
    Quarterly,
    #[serde(rename = "YEARLY")]
    Yearly,
}

impl Default for SeasonalPeriods {
    fn default() -> SeasonalPeriods {
        Self::SeasonalPeriodTypeUnspecified
    }
}
