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
pub struct HttpRouteAction {
    #[serde(rename = "corsPolicy", skip_serializing_if = "Option::is_none")]
    pub cors_policy: Option<Box<crate::google_rest_apis::compute_v1::models::CorsPolicy>>,
    #[serde(
        rename = "faultInjectionPolicy",
        skip_serializing_if = "Option::is_none"
    )]
    pub fault_injection_policy:
        Option<Box<crate::google_rest_apis::compute_v1::models::HttpFaultInjection>>,
    #[serde(rename = "maxStreamDuration", skip_serializing_if = "Option::is_none")]
    pub max_stream_duration: Option<Box<crate::google_rest_apis::compute_v1::models::Duration>>,
    #[serde(
        rename = "requestMirrorPolicy",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_mirror_policy:
        Option<Box<crate::google_rest_apis::compute_v1::models::RequestMirrorPolicy>>,
    #[serde(rename = "retryPolicy", skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<Box<crate::google_rest_apis::compute_v1::models::HttpRetryPolicy>>,
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Box<crate::google_rest_apis::compute_v1::models::Duration>>,
    #[serde(rename = "urlRewrite", skip_serializing_if = "Option::is_none")]
    pub url_rewrite: Option<Box<crate::google_rest_apis::compute_v1::models::UrlRewrite>>,
    /// A list of weighted backend services to send traffic to when a route match occurs. The weights determine the fraction of traffic that flows to their corresponding backend service. If all traffic needs to go to a single backend service, there must be one weightedBackendService with weight set to a non-zero number. After a backend service is identified and before forwarding the request to the backend service, advanced routing actions such as URL rewrites and header transformations are applied depending on additional settings specified in this HttpRouteAction.
    #[serde(
        rename = "weightedBackendServices",
        skip_serializing_if = "Option::is_none"
    )]
    pub weighted_backend_services:
        Option<Vec<crate::google_rest_apis::compute_v1::models::WeightedBackendService>>,
}

impl HttpRouteAction {
    pub fn new() -> HttpRouteAction {
        HttpRouteAction {
            cors_policy: None,
            fault_injection_policy: None,
            max_stream_duration: None,
            request_mirror_policy: None,
            retry_policy: None,
            timeout: None,
            url_rewrite: None,
            weighted_backend_services: None,
        }
    }
}
