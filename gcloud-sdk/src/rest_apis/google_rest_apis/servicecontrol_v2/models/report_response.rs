use serde::{Deserialize, Serialize}; /*
                                      * Service Control API
                                      *
                                      * Provides admission control and telemetry reporting for services integrated with Service Infrastructure.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::servicecontrol_v2::models;

/// ReportResponse : Response message for the Report method.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportResponse {
    /// The extension field to store serialized OTel responses. e.g. ExportLogsServiceResponse, ExportMetricsServiceResponse.
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl ReportResponse {
    /// Response message for the Report method.
    pub fn new() -> ReportResponse {
        ReportResponse { extensions: None }
    }
}
