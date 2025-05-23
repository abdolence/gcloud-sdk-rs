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

/// BackendBucketCdnPolicyCacheKeyPolicy : Message containing what to include in the cache key for a request for Cloud CDN.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackendBucketCdnPolicyCacheKeyPolicy {
    /// Allows HTTP request headers (by name) to be used in the cache key.
    #[serde(rename = "includeHttpHeaders", skip_serializing_if = "Option::is_none")]
    pub include_http_headers: Option<Vec<String>>,
    /// Names of query string parameters to include in cache keys. Default parameters are always included. '&' and '=' will be percent encoded and not treated as delimiters.
    #[serde(
        rename = "queryStringWhitelist",
        skip_serializing_if = "Option::is_none"
    )]
    pub query_string_whitelist: Option<Vec<String>>,
}

impl BackendBucketCdnPolicyCacheKeyPolicy {
    /// Message containing what to include in the cache key for a request for Cloud CDN.
    pub fn new() -> BackendBucketCdnPolicyCacheKeyPolicy {
        BackendBucketCdnPolicyCacheKeyPolicy {
            include_http_headers: None,
            query_string_whitelist: None,
        }
    }
}
