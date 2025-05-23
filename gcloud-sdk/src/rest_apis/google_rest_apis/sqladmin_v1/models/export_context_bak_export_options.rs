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

/// ExportContextBakExportOptions : Options for exporting BAK files (SQL Server-only)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportContextBakExportOptions {
    /// Option for specifying how many stripes to use for the export. If blank, and the value of the striped field is true, the number of stripes is automatically chosen.
    #[serde(rename = "stripeCount", skip_serializing_if = "Option::is_none")]
    pub stripe_count: Option<i32>,
    /// Whether or not the export should be striped.
    #[serde(rename = "striped", skip_serializing_if = "Option::is_none")]
    pub striped: Option<bool>,
}

impl ExportContextBakExportOptions {
    /// Options for exporting BAK files (SQL Server-only)
    pub fn new() -> ExportContextBakExportOptions {
        ExportContextBakExportOptions {
            stripe_count: None,
            striped: None,
        }
    }
}
