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

/// ConsistentHashLoadBalancerSettings : This message defines settings for a consistent hash style load balancer.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsistentHashLoadBalancerSettings {
    #[serde(rename = "httpCookie", skip_serializing_if = "Option::is_none")]
    pub http_cookie: Option<Box<models::ConsistentHashLoadBalancerSettingsHttpCookie>>,
    /// The hash based on the value of the specified header field. This field is applicable if the sessionAffinity is set to HEADER_FIELD.
    #[serde(rename = "httpHeaderName", skip_serializing_if = "Option::is_none")]
    pub http_header_name: Option<String>,
    /// The minimum number of virtual nodes to use for the hash ring. Defaults to 1024. Larger ring sizes result in more granular load distributions. If the number of hosts in the load balancing pool is larger than the ring size, each host will be assigned a single virtual node.
    #[serde(rename = "minimumRingSize", skip_serializing_if = "Option::is_none")]
    pub minimum_ring_size: Option<String>,
}

impl ConsistentHashLoadBalancerSettings {
    /// This message defines settings for a consistent hash style load balancer.
    pub fn new() -> ConsistentHashLoadBalancerSettings {
        ConsistentHashLoadBalancerSettings {
            http_cookie: None,
            http_header_name: None,
            minimum_ring_size: None,
        }
    }
}
