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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDnsKeyContext {
    #[serde(rename = "newValue", skip_serializing_if = "Option::is_none")]
    pub new_value: Option<Box<models::DnsKey>>,
    #[serde(rename = "oldValue", skip_serializing_if = "Option::is_none")]
    pub old_value: Option<Box<models::DnsKey>>,
}

impl OperationDnsKeyContext {
    pub fn new() -> OperationDnsKeyContext {
        OperationDnsKeyContext {
            new_value: None,
            old_value: None,
        }
    }
}
