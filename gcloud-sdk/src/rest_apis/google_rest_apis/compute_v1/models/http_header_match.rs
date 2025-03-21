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

/// HttpHeaderMatch : matchRule criteria for request header matches.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpHeaderMatch {
    /// The value should exactly match contents of exactMatch. Only one of exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set.
    #[serde(rename = "exactMatch", skip_serializing_if = "Option::is_none")]
    pub exact_match: Option<String>,
    /// The name of the HTTP header to match. For matching against the HTTP request's authority, use a headerMatch with the header name \":authority\". For matching a request's method, use the headerName \":method\". When the URL map is bound to a target gRPC proxy that has the validateForProxyless field set to true, only non-binary user-specified custom metadata and the `content-type` header are supported. The following transport-level headers cannot be used in header matching rules: `:authority`, `:method`, `:path`, `:scheme`, `user-agent`, `accept-encoding`, `content-encoding`, `grpc-accept-encoding`, `grpc-encoding`, `grpc-previous-rpc-attempts`, `grpc-tags-bin`, `grpc-timeout` and `grpc-trace-bin`.
    #[serde(rename = "headerName", skip_serializing_if = "Option::is_none")]
    pub header_name: Option<String>,
    /// If set to false, the headerMatch is considered a match if the preceding match criteria are met. If set to true, the headerMatch is considered a match if the preceding match criteria are NOT met. The default setting is false.
    #[serde(rename = "invertMatch", skip_serializing_if = "Option::is_none")]
    pub invert_match: Option<bool>,
    /// The value of the header must start with the contents of prefixMatch. Only one of exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set.
    #[serde(rename = "prefixMatch", skip_serializing_if = "Option::is_none")]
    pub prefix_match: Option<String>,
    /// A header with the contents of headerName must exist. The match takes place whether or not the request's header has a value. Only one of exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set.
    #[serde(rename = "presentMatch", skip_serializing_if = "Option::is_none")]
    pub present_match: Option<bool>,
    #[serde(rename = "rangeMatch", skip_serializing_if = "Option::is_none")]
    pub range_match: Option<Box<models::Int64RangeMatch>>,
    /// The value of the header must match the regular expression specified in regexMatch. For more information about regular expression syntax, see Syntax. For matching against a port specified in the HTTP request, use a headerMatch with headerName set to PORT and a regular expression that satisfies the RFC2616 Host header's port specifier. Only one of exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set. Regular expressions can only be used when the loadBalancingScheme is set to INTERNAL_SELF_MANAGED.
    #[serde(rename = "regexMatch", skip_serializing_if = "Option::is_none")]
    pub regex_match: Option<String>,
    /// The value of the header must end with the contents of suffixMatch. Only one of exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set.
    #[serde(rename = "suffixMatch", skip_serializing_if = "Option::is_none")]
    pub suffix_match: Option<String>,
}

impl HttpHeaderMatch {
    /// matchRule criteria for request header matches.
    pub fn new() -> HttpHeaderMatch {
        HttpHeaderMatch {
            exact_match: None,
            header_name: None,
            invert_match: None,
            prefix_match: None,
            present_match: None,
            range_match: None,
            regex_match: None,
            suffix_match: None,
        }
    }
}
