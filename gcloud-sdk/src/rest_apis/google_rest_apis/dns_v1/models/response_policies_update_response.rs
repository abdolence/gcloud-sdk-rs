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
pub struct ResponsePoliciesUpdateResponse {
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<Box<crate::google_rest_apis::dns_v1::models::ResponseHeader>>,
    #[serde(rename = "responsePolicy", skip_serializing_if = "Option::is_none")]
    pub response_policy: Option<Box<crate::google_rest_apis::dns_v1::models::ResponsePolicy>>,
}

impl ResponsePoliciesUpdateResponse {
    pub fn new() -> ResponsePoliciesUpdateResponse {
        ResponsePoliciesUpdateResponse {
            header: None,
            response_policy: None,
        }
    }
}