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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityPolicyRuleRateLimitOptions {
    /// Can only be specified if the action for the rule is \"rate_based_ban\". If specified, determines the time (in seconds) the traffic will continue to be banned by the rate limit after the rate falls below the threshold.
    #[serde(rename = "banDurationSec", skip_serializing_if = "Option::is_none")]
    pub ban_duration_sec: Option<i32>,
    #[serde(rename = "banThreshold", skip_serializing_if = "Option::is_none")]
    pub ban_threshold: Option<Box<models::SecurityPolicyRuleRateLimitOptionsThreshold>>,
    /// Action to take for requests that are under the configured rate limit threshold. Valid option is \"allow\" only.
    #[serde(rename = "conformAction", skip_serializing_if = "Option::is_none")]
    pub conform_action: Option<String>,
    /// Determines the key to enforce the rate_limit_threshold on. Possible values are: - ALL: A single rate limit threshold is applied to all the requests matching this rule. This is the default value if \"enforceOnKey\" is not configured. - IP: The source IP address of the request is the key. Each IP has this limit enforced separately. - HTTP_HEADER: The value of the HTTP header whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the header value. If no such header is present in the request, the key type defaults to ALL. - XFF_IP: The first IP address (i.e. the originating client IP address) specified in the list of IPs under X-Forwarded-For HTTP header. If no such header is present or the value is not a valid IP, the key defaults to the source IP address of the request i.e. key type IP. - HTTP_COOKIE: The value of the HTTP cookie whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the cookie value. If no such cookie is present in the request, the key type defaults to ALL. - HTTP_PATH: The URL path of the HTTP request. The key value is truncated to the first 128 bytes. - SNI: Server name indication in the TLS session of the HTTPS request. The key value is truncated to the first 128 bytes. The key type defaults to ALL on a HTTP session. - REGION_CODE: The country/region from which the request originates. - TLS_JA3_FINGERPRINT: JA3 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL. - USER_IP: The IP address of the originating client, which is resolved based on \"userIpRequestHeaders\" configured with the security policy. If there is no \"userIpRequestHeaders\" configuration or an IP address cannot be resolved from it, the key type defaults to IP.
    #[serde(rename = "enforceOnKey", skip_serializing_if = "Option::is_none")]
    pub enforce_on_key: Option<EnforceOnKey>,
    /// If specified, any combination of values of enforce_on_key_type/enforce_on_key_name is treated as the key on which ratelimit threshold/action is enforced. You can specify up to 3 enforce_on_key_configs. If enforce_on_key_configs is specified, enforce_on_key must not be specified.
    #[serde(
        rename = "enforceOnKeyConfigs",
        skip_serializing_if = "Option::is_none"
    )]
    pub enforce_on_key_configs:
        Option<Vec<models::SecurityPolicyRuleRateLimitOptionsEnforceOnKeyConfig>>,
    /// Rate limit key name applicable only for the following key types: HTTP_HEADER -- Name of the HTTP header whose value is taken as the key value. HTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value.
    #[serde(rename = "enforceOnKeyName", skip_serializing_if = "Option::is_none")]
    pub enforce_on_key_name: Option<String>,
    /// Action to take for requests that are above the configured rate limit threshold, to either deny with a specified HTTP response code, or redirect to a different endpoint. Valid options are `deny(STATUS)`, where valid values for `STATUS` are 403, 404, 429, and 502, and `redirect`, where the redirect parameters come from `exceedRedirectOptions` below. The `redirect` action is only supported in Global Security Policies of type CLOUD_ARMOR.
    #[serde(rename = "exceedAction", skip_serializing_if = "Option::is_none")]
    pub exceed_action: Option<String>,
    #[serde(
        rename = "exceedRedirectOptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub exceed_redirect_options: Option<Box<models::SecurityPolicyRuleRedirectOptions>>,
    #[serde(rename = "rateLimitThreshold", skip_serializing_if = "Option::is_none")]
    pub rate_limit_threshold: Option<Box<models::SecurityPolicyRuleRateLimitOptionsThreshold>>,
}

impl SecurityPolicyRuleRateLimitOptions {
    pub fn new() -> SecurityPolicyRuleRateLimitOptions {
        SecurityPolicyRuleRateLimitOptions {
            ban_duration_sec: None,
            ban_threshold: None,
            conform_action: None,
            enforce_on_key: None,
            enforce_on_key_configs: None,
            enforce_on_key_name: None,
            exceed_action: None,
            exceed_redirect_options: None,
            rate_limit_threshold: None,
        }
    }
}
/// Determines the key to enforce the rate_limit_threshold on. Possible values are: - ALL: A single rate limit threshold is applied to all the requests matching this rule. This is the default value if \"enforceOnKey\" is not configured. - IP: The source IP address of the request is the key. Each IP has this limit enforced separately. - HTTP_HEADER: The value of the HTTP header whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the header value. If no such header is present in the request, the key type defaults to ALL. - XFF_IP: The first IP address (i.e. the originating client IP address) specified in the list of IPs under X-Forwarded-For HTTP header. If no such header is present or the value is not a valid IP, the key defaults to the source IP address of the request i.e. key type IP. - HTTP_COOKIE: The value of the HTTP cookie whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the cookie value. If no such cookie is present in the request, the key type defaults to ALL. - HTTP_PATH: The URL path of the HTTP request. The key value is truncated to the first 128 bytes. - SNI: Server name indication in the TLS session of the HTTPS request. The key value is truncated to the first 128 bytes. The key type defaults to ALL on a HTTP session. - REGION_CODE: The country/region from which the request originates. - TLS_JA3_FINGERPRINT: JA3 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL. - USER_IP: The IP address of the originating client, which is resolved based on \"userIpRequestHeaders\" configured with the security policy. If there is no \"userIpRequestHeaders\" configuration or an IP address cannot be resolved from it, the key type defaults to IP.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EnforceOnKey {
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "HTTP_COOKIE")]
    HttpCookie,
    #[serde(rename = "HTTP_HEADER")]
    HttpHeader,
    #[serde(rename = "HTTP_PATH")]
    HttpPath,
    #[serde(rename = "IP")]
    Ip,
    #[serde(rename = "REGION_CODE")]
    RegionCode,
    #[serde(rename = "SNI")]
    Sni,
    #[serde(rename = "TLS_JA3_FINGERPRINT")]
    TlsJa3Fingerprint,
    #[serde(rename = "USER_IP")]
    UserIp,
    #[serde(rename = "XFF_IP")]
    XffIp,
}

impl Default for EnforceOnKey {
    fn default() -> EnforceOnKey {
        Self::All
    }
}
