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

/// BackendService : Represents a Backend Service resource. A backend service defines how Google Cloud load balancers distribute traffic. The backend service configuration contains a set of values, such as the protocol used to connect to backends, various distribution and session settings, health checks, and timeouts. These settings provide fine-grained control over how your load balancer behaves. Most of the settings have default values that allow for easy configuration if you need to get started quickly. Backend services in Google Compute Engine can be either regionally or globally scoped. * [Global](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices) * [Regional](https://cloud.google.com/compute/docs/reference/rest/v1/regionBackendServices) For more information, see Backend Services.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackendService {
    /// Lifetime of cookies in seconds. This setting is applicable to Application Load Balancers and Traffic Director and requires GENERATED_COOKIE or HTTP_COOKIE session affinity. If set to 0, the cookie is non-persistent and lasts only until the end of the browser session (or equivalent). The maximum allowed value is two weeks (1,209,600). Not supported when the backend service is referenced by a URL map that is bound to target gRPC proxy that has validateForProxyless field set to true.
    #[serde(
        rename = "affinityCookieTtlSec",
        skip_serializing_if = "Option::is_none"
    )]
    pub affinity_cookie_ttl_sec: Option<i32>,
    /// The list of backends that serve this BackendService.
    #[serde(rename = "backends", skip_serializing_if = "Option::is_none")]
    pub backends: Option<Vec<models::Backend>>,
    #[serde(rename = "cdnPolicy", skip_serializing_if = "Option::is_none")]
    pub cdn_policy: Option<Box<models::BackendServiceCdnPolicy>>,
    #[serde(rename = "circuitBreakers", skip_serializing_if = "Option::is_none")]
    pub circuit_breakers: Option<Box<models::CircuitBreakers>>,
    /// Compress text responses using Brotli or gzip compression, based on the client's Accept-Encoding header.
    #[serde(rename = "compressionMode", skip_serializing_if = "Option::is_none")]
    pub compression_mode: Option<CompressionMode>,
    #[serde(rename = "connectionDraining", skip_serializing_if = "Option::is_none")]
    pub connection_draining: Option<Box<models::ConnectionDraining>>,
    #[serde(
        rename = "connectionTrackingPolicy",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_tracking_policy: Option<Box<models::BackendServiceConnectionTrackingPolicy>>,
    #[serde(rename = "consistentHash", skip_serializing_if = "Option::is_none")]
    pub consistent_hash: Option<Box<models::ConsistentHashLoadBalancerSettings>>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Headers that the load balancer adds to proxied requests. See [Creating custom headers](https://cloud.google.com/load-balancing/docs/custom-headers).
    #[serde(
        rename = "customRequestHeaders",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_request_headers: Option<Vec<String>>,
    /// Headers that the load balancer adds to proxied responses. See [Creating custom headers](https://cloud.google.com/load-balancing/docs/custom-headers).
    #[serde(
        rename = "customResponseHeaders",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_response_headers: Option<Vec<String>>,
    /// An optional description of this resource. Provide this property when you create the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// [Output Only] The resource URL for the edge security policy associated with this backend service.
    #[serde(rename = "edgeSecurityPolicy", skip_serializing_if = "Option::is_none")]
    pub edge_security_policy: Option<String>,
    /// If true, enables Cloud CDN for the backend service of a global external Application Load Balancer.
    #[serde(rename = "enableCDN", skip_serializing_if = "Option::is_none")]
    pub enable_cdn: Option<bool>,
    #[serde(rename = "failoverPolicy", skip_serializing_if = "Option::is_none")]
    pub failover_policy: Option<Box<models::BackendServiceFailoverPolicy>>,
    /// Fingerprint of this resource. A hash of the contents stored in this object. This field is used in optimistic locking. This field will be ignored when inserting a BackendService. An up-to-date fingerprint must be provided in order to update the BackendService, otherwise the request will fail with error 412 conditionNotMet. To see the latest fingerprint, make a get() request to retrieve a BackendService.
    #[serde(rename = "fingerprint", skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// The list of URLs to the healthChecks, httpHealthChecks (legacy), or httpsHealthChecks (legacy) resource for health checking this backend service. Not all backend services support legacy health checks. See Load balancer guide. Currently, at most one health check can be specified for each backend service. Backend services with instance group or zonal NEG backends must have a health check. Backend services with internet or serverless NEG backends must not have a health check.
    #[serde(rename = "healthChecks", skip_serializing_if = "Option::is_none")]
    pub health_checks: Option<Vec<String>>,
    #[serde(rename = "iap", skip_serializing_if = "Option::is_none")]
    pub iap: Option<Box<models::BackendServiceIap>>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] Type of resource. Always compute#backendService for backend services.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Specifies the load balancer type. A backend service created for one type of load balancer cannot be used with another. For more information, refer to Choosing a load balancer.
    #[serde(
        rename = "loadBalancingScheme",
        skip_serializing_if = "Option::is_none"
    )]
    pub load_balancing_scheme: Option<LoadBalancingScheme>,
    /// A list of locality load-balancing policies to be used in order of preference. When you use localityLbPolicies, you must set at least one value for either the localityLbPolicies[].policy or the localityLbPolicies[].customPolicy field. localityLbPolicies overrides any value set in the localityLbPolicy field. For an example of how to use this field, see Define a list of preferred policies. Caution: This field and its children are intended for use in a service mesh that includes gRPC clients only. Envoy proxies can't use backend services that have this configuration.
    #[serde(rename = "localityLbPolicies", skip_serializing_if = "Option::is_none")]
    pub locality_lb_policies: Option<Vec<models::BackendServiceLocalityLoadBalancingPolicyConfig>>,
    /// The load balancing algorithm used within the scope of the locality. The possible values are: - ROUND_ROBIN: This is a simple policy in which each healthy backend is selected in round robin order. This is the default. - LEAST_REQUEST: An O(1) algorithm which selects two random healthy hosts and picks the host which has fewer active requests. - RING_HASH: The ring/modulo hash load balancer implements consistent hashing to backends. The algorithm has the property that the addition/removal of a host from a set of N hosts only affects 1/N of the requests. - RANDOM: The load balancer selects a random healthy host. - ORIGINAL_DESTINATION: Backend host is selected based on the client connection metadata, i.e., connections are opened to the same address as the destination address of the incoming connection before the connection was redirected to the load balancer. - MAGLEV: used as a drop in replacement for the ring hash load balancer. Maglev is not as stable as ring hash but has faster table lookup build times and host selection times. For more information about Maglev, see https://ai.google/research/pubs/pub44824 This field is applicable to either: - A regional backend service with the service_protocol set to HTTP, HTTPS, or HTTP2, and load_balancing_scheme set to INTERNAL_MANAGED. - A global backend service with the load_balancing_scheme set to INTERNAL_SELF_MANAGED, INTERNAL_MANAGED, or EXTERNAL_MANAGED. If sessionAffinity is not NONE, and this field is not set to MAGLEV or RING_HASH, session affinity settings will not take effect. Only ROUND_ROBIN and RING_HASH are supported when the backend service is referenced by a URL map that is bound to target gRPC proxy that has validateForProxyless field set to true.
    #[serde(rename = "localityLbPolicy", skip_serializing_if = "Option::is_none")]
    pub locality_lb_policy: Option<LocalityLbPolicy>,
    #[serde(rename = "logConfig", skip_serializing_if = "Option::is_none")]
    pub log_config: Option<Box<models::BackendServiceLogConfig>>,
    #[serde(rename = "maxStreamDuration", skip_serializing_if = "Option::is_none")]
    pub max_stream_duration: Option<Box<models::Duration>>,
    /// Deployment metadata associated with the resource to be set by a GKE hub controller and read by the backend RCTH
    #[serde(rename = "metadatas", skip_serializing_if = "Option::is_none")]
    pub metadatas: Option<std::collections::HashMap<String, String>>,
    /// Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the network to which this backend service belongs. This field can only be specified when the load balancing scheme is set to INTERNAL.
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(rename = "outlierDetection", skip_serializing_if = "Option::is_none")]
    pub outlier_detection: Option<Box<models::OutlierDetection>>,
    /// Deprecated in favor of portName. The TCP port to connect on the backend. The default value is 80. For internal passthrough Network Load Balancers and external passthrough Network Load Balancers, omit port.
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// A named port on a backend instance group representing the port for communication to the backend VMs in that group. The named port must be [defined on each backend instance group](https://cloud.google.com/load-balancing/docs/backend-service#named_ports). This parameter has no meaning if the backends are NEGs. For internal passthrough Network Load Balancers and external passthrough Network Load Balancers, omit port_name.
    #[serde(rename = "portName", skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
    /// The protocol this BackendService uses to communicate with backends. Possible values are HTTP, HTTPS, HTTP2, TCP, SSL, UDP or GRPC. depending on the chosen load balancer or Traffic Director configuration. Refer to the documentation for the load balancers or for Traffic Director for more information. Must be set to GRPC when the backend service is referenced by a URL map that is bound to target gRPC proxy.
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
    /// [Output Only] URL of the region where the regional backend service resides. This field is not applicable to global backend services. You must specify this field as part of the HTTP request URL. It is not settable as a field in the request body.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// [Output Only] The resource URL for the security policy associated with this backend service.
    #[serde(rename = "securityPolicy", skip_serializing_if = "Option::is_none")]
    pub security_policy: Option<String>,
    #[serde(rename = "securitySettings", skip_serializing_if = "Option::is_none")]
    pub security_settings: Option<Box<models::SecuritySettings>>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// URLs of networkservices.ServiceBinding resources. Can only be set if load balancing scheme is INTERNAL_SELF_MANAGED. If set, lists of backends and health checks must be both empty.
    #[serde(rename = "serviceBindings", skip_serializing_if = "Option::is_none")]
    pub service_bindings: Option<Vec<String>>,
    /// URL to networkservices.ServiceLbPolicy resource. Can only be set if load balancing scheme is EXTERNAL, EXTERNAL_MANAGED, INTERNAL_MANAGED or INTERNAL_SELF_MANAGED and the scope is global.
    #[serde(rename = "serviceLbPolicy", skip_serializing_if = "Option::is_none")]
    pub service_lb_policy: Option<String>,
    /// Type of session affinity to use. The default is NONE. Only NONE and HEADER_FIELD are supported when the backend service is referenced by a URL map that is bound to target gRPC proxy that has validateForProxyless field set to true. For more details, see: [Session Affinity](https://cloud.google.com/load-balancing/docs/backend-service#session_affinity).
    #[serde(rename = "sessionAffinity", skip_serializing_if = "Option::is_none")]
    pub session_affinity: Option<SessionAffinity>,
    #[serde(rename = "subsetting", skip_serializing_if = "Option::is_none")]
    pub subsetting: Option<Box<models::Subsetting>>,
    /// The backend service timeout has a different meaning depending on the type of load balancer. For more information see, Backend service settings. The default is 30 seconds. The full range of timeout values allowed goes from 1 through 2,147,483,647 seconds. This value can be overridden in the PathMatcher configuration of the UrlMap that references this backend service. Not supported when the backend service is referenced by a URL map that is bound to target gRPC proxy that has validateForProxyless field set to true. Instead, use maxStreamDuration.
    #[serde(rename = "timeoutSec", skip_serializing_if = "Option::is_none")]
    pub timeout_sec: Option<i32>,
    #[serde(rename = "usedBy", skip_serializing_if = "Option::is_none")]
    pub used_by: Option<Vec<models::BackendServiceUsedBy>>,
}

impl BackendService {
    /// Represents a Backend Service resource. A backend service defines how Google Cloud load balancers distribute traffic. The backend service configuration contains a set of values, such as the protocol used to connect to backends, various distribution and session settings, health checks, and timeouts. These settings provide fine-grained control over how your load balancer behaves. Most of the settings have default values that allow for easy configuration if you need to get started quickly. Backend services in Google Compute Engine can be either regionally or globally scoped. * [Global](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices) * [Regional](https://cloud.google.com/compute/docs/reference/rest/v1/regionBackendServices) For more information, see Backend Services.
    pub fn new() -> BackendService {
        BackendService {
            affinity_cookie_ttl_sec: None,
            backends: None,
            cdn_policy: None,
            circuit_breakers: None,
            compression_mode: None,
            connection_draining: None,
            connection_tracking_policy: None,
            consistent_hash: None,
            creation_timestamp: None,
            custom_request_headers: None,
            custom_response_headers: None,
            description: None,
            edge_security_policy: None,
            enable_cdn: None,
            failover_policy: None,
            fingerprint: None,
            health_checks: None,
            iap: None,
            id: None,
            kind: None,
            load_balancing_scheme: None,
            locality_lb_policies: None,
            locality_lb_policy: None,
            log_config: None,
            max_stream_duration: None,
            metadatas: None,
            name: None,
            network: None,
            outlier_detection: None,
            port: None,
            port_name: None,
            protocol: None,
            region: None,
            security_policy: None,
            security_settings: None,
            self_link: None,
            service_bindings: None,
            service_lb_policy: None,
            session_affinity: None,
            subsetting: None,
            timeout_sec: None,
            used_by: None,
        }
    }
}
/// Compress text responses using Brotli or gzip compression, based on the client's Accept-Encoding header.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CompressionMode {
    #[serde(rename = "AUTOMATIC")]
    Automatic,
    #[serde(rename = "DISABLED")]
    Disabled,
}

impl Default for CompressionMode {
    fn default() -> CompressionMode {
        Self::Automatic
    }
}
/// Specifies the load balancer type. A backend service created for one type of load balancer cannot be used with another. For more information, refer to Choosing a load balancer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LoadBalancingScheme {
    #[serde(rename = "EXTERNAL")]
    External,
    #[serde(rename = "EXTERNAL_MANAGED")]
    ExternalManaged,
    #[serde(rename = "INTERNAL")]
    Internal,
    #[serde(rename = "INTERNAL_MANAGED")]
    InternalManaged,
    #[serde(rename = "INTERNAL_SELF_MANAGED")]
    InternalSelfManaged,
    #[serde(rename = "INVALID_LOAD_BALANCING_SCHEME")]
    InvalidLoadBalancingScheme,
}

impl Default for LoadBalancingScheme {
    fn default() -> LoadBalancingScheme {
        Self::External
    }
}
/// The load balancing algorithm used within the scope of the locality. The possible values are: - ROUND_ROBIN: This is a simple policy in which each healthy backend is selected in round robin order. This is the default. - LEAST_REQUEST: An O(1) algorithm which selects two random healthy hosts and picks the host which has fewer active requests. - RING_HASH: The ring/modulo hash load balancer implements consistent hashing to backends. The algorithm has the property that the addition/removal of a host from a set of N hosts only affects 1/N of the requests. - RANDOM: The load balancer selects a random healthy host. - ORIGINAL_DESTINATION: Backend host is selected based on the client connection metadata, i.e., connections are opened to the same address as the destination address of the incoming connection before the connection was redirected to the load balancer. - MAGLEV: used as a drop in replacement for the ring hash load balancer. Maglev is not as stable as ring hash but has faster table lookup build times and host selection times. For more information about Maglev, see https://ai.google/research/pubs/pub44824 This field is applicable to either: - A regional backend service with the service_protocol set to HTTP, HTTPS, or HTTP2, and load_balancing_scheme set to INTERNAL_MANAGED. - A global backend service with the load_balancing_scheme set to INTERNAL_SELF_MANAGED, INTERNAL_MANAGED, or EXTERNAL_MANAGED. If sessionAffinity is not NONE, and this field is not set to MAGLEV or RING_HASH, session affinity settings will not take effect. Only ROUND_ROBIN and RING_HASH are supported when the backend service is referenced by a URL map that is bound to target gRPC proxy that has validateForProxyless field set to true.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LocalityLbPolicy {
    #[serde(rename = "INVALID_LB_POLICY")]
    InvalidLbPolicy,
    #[serde(rename = "LEAST_REQUEST")]
    LeastRequest,
    #[serde(rename = "MAGLEV")]
    Maglev,
    #[serde(rename = "ORIGINAL_DESTINATION")]
    OriginalDestination,
    #[serde(rename = "RANDOM")]
    Random,
    #[serde(rename = "RING_HASH")]
    RingHash,
    #[serde(rename = "ROUND_ROBIN")]
    RoundRobin,
    #[serde(rename = "WEIGHTED_MAGLEV")]
    WeightedMaglev,
}

impl Default for LocalityLbPolicy {
    fn default() -> LocalityLbPolicy {
        Self::InvalidLbPolicy
    }
}
/// The protocol this BackendService uses to communicate with backends. Possible values are HTTP, HTTPS, HTTP2, TCP, SSL, UDP or GRPC. depending on the chosen load balancer or Traffic Director configuration. Refer to the documentation for the load balancers or for Traffic Director for more information. Must be set to GRPC when the backend service is referenced by a URL map that is bound to target gRPC proxy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "GRPC")]
    Grpc,
    #[serde(rename = "HTTP")]
    Http,
    #[serde(rename = "HTTP2")]
    Http2,
    #[serde(rename = "HTTPS")]
    Https,
    #[serde(rename = "SSL")]
    Ssl,
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
    #[serde(rename = "UNSPECIFIED")]
    Unspecified,
}

impl Default for Protocol {
    fn default() -> Protocol {
        Self::Grpc
    }
}
/// Type of session affinity to use. The default is NONE. Only NONE and HEADER_FIELD are supported when the backend service is referenced by a URL map that is bound to target gRPC proxy that has validateForProxyless field set to true. For more details, see: [Session Affinity](https://cloud.google.com/load-balancing/docs/backend-service#session_affinity).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SessionAffinity {
    #[serde(rename = "CLIENT_IP")]
    ClientIp,
    #[serde(rename = "CLIENT_IP_NO_DESTINATION")]
    ClientIpNoDestination,
    #[serde(rename = "CLIENT_IP_PORT_PROTO")]
    ClientIpPortProto,
    #[serde(rename = "CLIENT_IP_PROTO")]
    ClientIpProto,
    #[serde(rename = "GENERATED_COOKIE")]
    GeneratedCookie,
    #[serde(rename = "HEADER_FIELD")]
    HeaderField,
    #[serde(rename = "HTTP_COOKIE")]
    HttpCookie,
    #[serde(rename = "NONE")]
    None,
}

impl Default for SessionAffinity {
    fn default() -> SessionAffinity {
        Self::ClientIp
    }
}
