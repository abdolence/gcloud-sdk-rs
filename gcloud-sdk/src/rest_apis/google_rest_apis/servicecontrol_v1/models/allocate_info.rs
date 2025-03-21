use serde::{Deserialize, Serialize}; /*
                                      * Service Control API
                                      *
                                      * Provides admission control and telemetry reporting for services integrated with Service Infrastructure.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::servicecontrol_v1::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllocateInfo {
    /// A list of label keys that were unused by the server in processing the request. Thus, for similar requests repeated in a certain future time window, the caller can choose to ignore these labels in the requests to achieve better client-side cache hits and quota aggregation for rate quota. This field is not populated for allocation quota checks.
    #[serde(rename = "unusedArguments", skip_serializing_if = "Option::is_none")]
    pub unused_arguments: Option<Vec<String>>,
}

impl AllocateInfo {
    pub fn new() -> AllocateInfo {
        AllocateInfo {
            unused_arguments: None,
        }
    }
}
