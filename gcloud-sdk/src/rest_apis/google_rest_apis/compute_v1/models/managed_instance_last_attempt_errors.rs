use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::compute_v1::models;

/// ManagedInstanceLastAttemptErrors : [Output Only] Encountered errors during the last attempt to create or delete the instance.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedInstanceLastAttemptErrors {
    /// [Output Only] The array of errors encountered while processing this operation.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<models::ManagedInstanceLastAttemptErrorsErrorsInner>>,
}

impl ManagedInstanceLastAttemptErrors {
    /// [Output Only] Encountered errors during the last attempt to create or delete the instance.
    pub fn new() -> ManagedInstanceLastAttemptErrors {
        ManagedInstanceLastAttemptErrors { errors: None }
    }
}
