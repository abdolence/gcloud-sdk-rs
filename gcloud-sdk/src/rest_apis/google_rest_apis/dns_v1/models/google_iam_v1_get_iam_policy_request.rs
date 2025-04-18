use serde::{Deserialize, Serialize}; /*
                                      * Cloud DNS API
                                      *
                                      * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::dns_v1::models;

/// GoogleIamV1GetIamPolicyRequest : Request message for `GetIamPolicy` method.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GoogleIamV1GetIamPolicyRequest {
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<models::GoogleIamV1GetPolicyOptions>>,
}

impl GoogleIamV1GetIamPolicyRequest {
    /// Request message for `GetIamPolicy` method.
    pub fn new() -> GoogleIamV1GetIamPolicyRequest {
        GoogleIamV1GetIamPolicyRequest { options: None }
    }
}
