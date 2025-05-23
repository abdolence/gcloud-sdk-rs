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

/// BackendBucketCdnPolicyBypassCacheOnRequestHeader : Bypass the cache when the specified request headers are present, e.g. Pragma or Authorization headers. Values are case insensitive. The presence of such a header overrides the cache_mode setting.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackendBucketCdnPolicyBypassCacheOnRequestHeader {
    /// The header field name to match on when bypassing cache. Values are case-insensitive.
    #[serde(rename = "headerName", skip_serializing_if = "Option::is_none")]
    pub header_name: Option<String>,
}

impl BackendBucketCdnPolicyBypassCacheOnRequestHeader {
    /// Bypass the cache when the specified request headers are present, e.g. Pragma or Authorization headers. Values are case insensitive. The presence of such a header overrides the cache_mode setting.
    pub fn new() -> BackendBucketCdnPolicyBypassCacheOnRequestHeader {
        BackendBucketCdnPolicyBypassCacheOnRequestHeader { header_name: None }
    }
}
