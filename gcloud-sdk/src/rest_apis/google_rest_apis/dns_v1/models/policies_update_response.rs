use serde::{Deserialize, Serialize}; /*
                                      * Cloud DNS API
                                      *
                                      * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoliciesUpdateResponse {
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<Box<crate::google_rest_apis::dns_v1::models::ResponseHeader>>,
    #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<Box<crate::google_rest_apis::dns_v1::models::Policy>>,
}

impl PoliciesUpdateResponse {
    pub fn new() -> PoliciesUpdateResponse {
        PoliciesUpdateResponse {
            header: None,
            policy: None,
        }
    }
}
