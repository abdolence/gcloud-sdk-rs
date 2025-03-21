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

/// LocationMetadata : BigQuery-specific metadata about a location. This will be set on google.cloud.location.Location.metadata in Cloud Location API responses.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationMetadata {
    /// The legacy BigQuery location ID, e.g. “EU” for the “europe” location. This is for any API consumers that need the legacy “US” and “EU” locations.
    #[serde(rename = "legacyLocationId", skip_serializing_if = "Option::is_none")]
    pub legacy_location_id: Option<String>,
}

impl LocationMetadata {
    /// BigQuery-specific metadata about a location. This will be set on google.cloud.location.Location.metadata in Cloud Location API responses.
    pub fn new() -> LocationMetadata {
        LocationMetadata {
            legacy_location_id: None,
        }
    }
}
