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

/// TargetHttpProxy : Represents a Target HTTP Proxy resource. Google Compute Engine has two Target HTTP Proxy resources: * [Global](/compute/docs/reference/rest/v1/targetHttpProxies) * [Regional](/compute/docs/reference/rest/v1/regionTargetHttpProxies) A target HTTP proxy is a component of Google Cloud HTTP load balancers. * targetHttpProxies are used by global external Application Load Balancers, classic Application Load Balancers, cross-region internal Application Load Balancers, and Traffic Director. * regionTargetHttpProxies are used by regional internal Application Load Balancers and regional external Application Load Balancers. Forwarding rules reference a target HTTP proxy, and the target proxy then references a URL map. For more information, read Using Target Proxies and Forwarding rule concepts.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetHttpProxy {
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// An optional description of this resource. Provide this property when you create the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Fingerprint of this resource. A hash of the contents stored in this object. This field is used in optimistic locking. This field will be ignored when inserting a TargetHttpProxy. An up-to-date fingerprint must be provided in order to patch/update the TargetHttpProxy; otherwise, the request will fail with error 412 conditionNotMet. To see the latest fingerprint, make a get() request to retrieve the TargetHttpProxy.
    #[serde(rename = "fingerprint", skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// Specifies how long to keep a connection open, after completing a response, while there is no matching traffic (in seconds). If an HTTP keep-alive is not specified, a default value (610 seconds) will be used. For global external Application Load Balancers, the minimum allowed value is 5 seconds and the maximum allowed value is 1200 seconds. For classic Application Load Balancers, this option is not supported.
    #[serde(
        rename = "httpKeepAliveTimeoutSec",
        skip_serializing_if = "Option::is_none"
    )]
    pub http_keep_alive_timeout_sec: Option<i32>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] Type of resource. Always compute#targetHttpProxy for target HTTP proxies.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This field only applies when the forwarding rule that references this target proxy has a loadBalancingScheme set to INTERNAL_SELF_MANAGED. When this field is set to true, Envoy proxies set up inbound traffic interception and bind to the IP address and port specified in the forwarding rule. This is generally useful when using Traffic Director to configure Envoy as a gateway or middle proxy (in other words, not a sidecar proxy). The Envoy proxy listens for inbound requests and handles requests when it receives them. The default is false.
    #[serde(rename = "proxyBind", skip_serializing_if = "Option::is_none")]
    pub proxy_bind: Option<bool>,
    /// [Output Only] URL of the region where the regional Target HTTP Proxy resides. This field is not applicable to global Target HTTP Proxies.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// URL to the UrlMap resource that defines the mapping from URL to the BackendService.
    #[serde(rename = "urlMap", skip_serializing_if = "Option::is_none")]
    pub url_map: Option<String>,
}

impl TargetHttpProxy {
    /// Represents a Target HTTP Proxy resource. Google Compute Engine has two Target HTTP Proxy resources: * [Global](/compute/docs/reference/rest/v1/targetHttpProxies) * [Regional](/compute/docs/reference/rest/v1/regionTargetHttpProxies) A target HTTP proxy is a component of Google Cloud HTTP load balancers. * targetHttpProxies are used by global external Application Load Balancers, classic Application Load Balancers, cross-region internal Application Load Balancers, and Traffic Director. * regionTargetHttpProxies are used by regional internal Application Load Balancers and regional external Application Load Balancers. Forwarding rules reference a target HTTP proxy, and the target proxy then references a URL map. For more information, read Using Target Proxies and Forwarding rule concepts.
    pub fn new() -> TargetHttpProxy {
        TargetHttpProxy {
            creation_timestamp: None,
            description: None,
            fingerprint: None,
            http_keep_alive_timeout_sec: None,
            id: None,
            kind: None,
            name: None,
            proxy_bind: None,
            region: None,
            self_link: None,
            url_map: None,
        }
    }
}
