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

/// TargetGrpcProxy : Represents a Target gRPC Proxy resource. A target gRPC proxy is a component of load balancers intended for load balancing gRPC traffic. Only global forwarding rules with load balancing scheme INTERNAL_SELF_MANAGED can reference a target gRPC proxy. The target gRPC Proxy references a URL map that specifies how traffic is routed to gRPC backend services.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetGrpcProxy {
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// An optional description of this resource. Provide this property when you create the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Fingerprint of this resource. A hash of the contents stored in this object. This field is used in optimistic locking. This field will be ignored when inserting a TargetGrpcProxy. An up-to-date fingerprint must be provided in order to patch/update the TargetGrpcProxy; otherwise, the request will fail with error 412 conditionNotMet. To see the latest fingerprint, make a get() request to retrieve the TargetGrpcProxy.
    #[serde(rename = "fingerprint", skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// [Output Only] The unique identifier for the resource type. The server generates this identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] Type of the resource. Always compute#targetGrpcProxy for target grpc proxies.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// [Output Only] Server-defined URL with id for the resource.
    #[serde(rename = "selfLinkWithId", skip_serializing_if = "Option::is_none")]
    pub self_link_with_id: Option<String>,
    /// URL to the UrlMap resource that defines the mapping from URL to the BackendService. The protocol field in the BackendService must be set to GRPC.
    #[serde(rename = "urlMap", skip_serializing_if = "Option::is_none")]
    pub url_map: Option<String>,
    /// If true, indicates that the BackendServices referenced by the urlMap may be accessed by gRPC applications without using a sidecar proxy. This will enable configuration checks on urlMap and its referenced BackendServices to not allow unsupported features. A gRPC application must use \"xds:///\" scheme in the target URI of the service it is connecting to. If false, indicates that the BackendServices referenced by the urlMap will be accessed by gRPC applications via a sidecar proxy. In this case, a gRPC application must not use \"xds:///\" scheme in the target URI of the service it is connecting to
    #[serde(
        rename = "validateForProxyless",
        skip_serializing_if = "Option::is_none"
    )]
    pub validate_for_proxyless: Option<bool>,
}

impl TargetGrpcProxy {
    /// Represents a Target gRPC Proxy resource. A target gRPC proxy is a component of load balancers intended for load balancing gRPC traffic. Only global forwarding rules with load balancing scheme INTERNAL_SELF_MANAGED can reference a target gRPC proxy. The target gRPC Proxy references a URL map that specifies how traffic is routed to gRPC backend services.
    pub fn new() -> TargetGrpcProxy {
        TargetGrpcProxy {
            creation_timestamp: None,
            description: None,
            fingerprint: None,
            id: None,
            kind: None,
            name: None,
            self_link: None,
            self_link_with_id: None,
            url_map: None,
            validate_for_proxyless: None,
        }
    }
}
