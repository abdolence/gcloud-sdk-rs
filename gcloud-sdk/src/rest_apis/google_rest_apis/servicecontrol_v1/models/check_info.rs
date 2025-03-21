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

/// CheckInfo : Contains additional information about the check operation.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckInfo {
    /// The unique id of the api key in the format of \"apikey:\". This field will be populated when the consumer passed to Chemist is an API key and all the API key related validations are successful.
    #[serde(rename = "apiKeyUid", skip_serializing_if = "Option::is_none")]
    pub api_key_uid: Option<String>,
    #[serde(rename = "consumerInfo", skip_serializing_if = "Option::is_none")]
    pub consumer_info: Option<Box<models::ConsumerInfo>>,
    /// A list of fields and label keys that are ignored by the server. The client doesn't need to send them for following requests to improve performance and allow better aggregation.
    #[serde(rename = "unusedArguments", skip_serializing_if = "Option::is_none")]
    pub unused_arguments: Option<Vec<String>>,
}

impl CheckInfo {
    /// Contains additional information about the check operation.
    pub fn new() -> CheckInfo {
        CheckInfo {
            api_key_uid: None,
            consumer_info: None,
            unused_arguments: None,
        }
    }
}
