use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReservationsScopedList {
    /// A list of reservations contained in this scope.
    #[serde(rename = "reservations", skip_serializing_if = "Option::is_none")]
    pub reservations: Option<Vec<crate::google_rest_apis::compute_v1::models::Reservation>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning:
        Option<Box<crate::google_rest_apis::compute_v1::models::ReservationsScopedListWarning>>,
}

impl ReservationsScopedList {
    pub fn new() -> ReservationsScopedList {
        ReservationsScopedList {
            reservations: None,
            warning: None,
        }
    }
}
