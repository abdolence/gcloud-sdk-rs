/// Trace represents one simulated packet forwarding path.
///
///   * Each trace contains multiple ordered steps.
///   * Each step is in a particular state with associated configuration.
///   * State is categorized as final or non-final states.
///   * Each final state has a reason associated.
///   * Each trace must end with a final state (the last step).
/// ```
///   |---------------------Trace----------------------|
///   Step1(State) Step2(State) ---  StepN(State(final))
/// ```
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trace {
    /// Derived from the source and destination endpoints definition specified by
    /// user request, and validated by the data plane model.
    /// If there are multiple traces starting from different source locations, then
    /// the endpoint_info may be different between traces.
    #[prost(message, optional, tag = "1")]
    pub endpoint_info: ::core::option::Option<EndpointInfo>,
    /// A trace of a test contains multiple steps from the initial state to the
    /// final state (delivered, dropped, forwarded, or aborted).
    ///
    /// The steps are ordered by the processing sequence within the simulated
    /// network state machine. It is critical to preserve the order of the steps
    /// and avoid reordering or sorting them.
    #[prost(message, repeated, tag = "2")]
    pub steps: ::prost::alloc::vec::Vec<Step>,
}
/// A simulated forwarding path is composed of multiple steps.
/// Each step has a well-defined state and an associated configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Step {
    /// A description of the step. Usually this is a summary of the state.
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// Each step is in one of the pre-defined states.
    #[prost(enumeration = "step::State", tag = "2")]
    pub state: i32,
    /// This is a step that leads to the final state Drop.
    #[prost(bool, tag = "3")]
    pub causes_drop: bool,
    /// Project ID that contains the configuration this step is validating.
    #[prost(string, tag = "4")]
    pub project_id: ::prost::alloc::string::String,
    /// Configuration or metadata associated with each step.
    /// The configuration is filtered based on viewer's permission. If a viewer
    /// has no permission to view the configuration in this step, for non-final
    /// states a special state is populated (VIEWER_PERMISSION_MISSING), and for
    /// final state the configuration is cleared.
    #[prost(
        oneof = "step::StepInfo",
        tags = "5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19"
    )]
    pub step_info: ::core::option::Option<step::StepInfo>,
}
/// Nested message and enum types in `Step`.
pub mod step {
    /// Type of states that are defined in the network state machine.
    /// Each step in the packet trace is in a specific state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// Initial state: packet originating from a Compute Engine instance.
        /// An InstanceInfo is populated with starting instance information.
        StartFromInstance = 1,
        /// Initial state: packet originating from the internet.
        /// The endpoint information is populated.
        StartFromInternet = 2,
        /// Initial state: packet originating from a VPC or on-premises network
        /// with internal source IP.
        /// If the source is a VPC network visible to the user, a NetworkInfo
        /// is populated with details of the network.
        StartFromPrivateNetwork = 3,
        /// Initial state: packet originating from a Google Kubernetes Engine cluster
        /// master. A GKEMasterInfo is populated with starting instance information.
        StartFromGkeMaster = 21,
        /// Initial state: packet originating from a Cloud SQL instance.
        /// A CloudSQLInstanceInfo is populated with starting instance information.
        StartFromCloudSqlInstance = 22,
        /// Config checking state: verify ingress firewall rule.
        ApplyIngressFirewallRule = 4,
        /// Config checking state: verify egress firewall rule.
        ApplyEgressFirewallRule = 5,
        /// Config checking state: verify route.
        ApplyRoute = 6,
        /// Config checking state: match forwarding rule.
        ApplyForwardingRule = 7,
        /// Config checking state: packet sent or received under foreign IP
        /// address and allowed.
        SpoofingApproved = 8,
        /// Forwarding state: arriving at a Compute Engine instance.
        ArriveAtInstance = 9,
        /// Forwarding state: arriving at a Compute Engine internal load balancer.
        ArriveAtInternalLoadBalancer = 10,
        /// Forwarding state: arriving at a Compute Engine external load balancer.
        ArriveAtExternalLoadBalancer = 11,
        /// Forwarding state: arriving at a Cloud VPN gateway.
        ArriveAtVpnGateway = 12,
        /// Forwarding state: arriving at a Cloud VPN tunnel.
        ArriveAtVpnTunnel = 13,
        /// Transition state: packet header translated.
        Nat = 14,
        /// Transition state: original connection is terminated and a new proxied
        /// connection is initiated.
        ProxyConnection = 15,
        /// Final state: packet could be delivered.
        Deliver = 16,
        /// Final state: packet could be dropped.
        Drop = 17,
        /// Final state: packet could be forwarded to a network with an unknown
        /// configuration.
        Forward = 18,
        /// Final state: analysis is aborted.
        Abort = 19,
        /// Special state: viewer of the test result does not have permission to
        /// see the configuration in this step.
        ViewerPermissionMissing = 20,
    }
    /// Configuration or metadata associated with each step.
    /// The configuration is filtered based on viewer's permission. If a viewer
    /// has no permission to view the configuration in this step, for non-final
    /// states a special state is populated (VIEWER_PERMISSION_MISSING), and for
    /// final state the configuration is cleared.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StepInfo {
        /// Display information of a Compute Engine instance.
        #[prost(message, tag = "5")]
        Instance(super::InstanceInfo),
        /// Display information of a Compute Engine firewall rule.
        #[prost(message, tag = "6")]
        Firewall(super::FirewallInfo),
        /// Display information of a Compute Engine route.
        #[prost(message, tag = "7")]
        Route(super::RouteInfo),
        /// Display information of the source and destination under analysis.
        /// The endpoint information in an intermediate state may differ with the
        /// initial input, as it might be modified by state like NAT,
        /// or Connection Proxy.
        #[prost(message, tag = "8")]
        Endpoint(super::EndpointInfo),
        /// Display information of a Compute Engine forwarding rule.
        #[prost(message, tag = "9")]
        ForwardingRule(super::ForwardingRuleInfo),
        /// Display information of a Compute Engine VPN gateway.
        #[prost(message, tag = "10")]
        VpnGateway(super::VpnGatewayInfo),
        /// Display information of a Compute Engine VPN tunnel.
        #[prost(message, tag = "11")]
        VpnTunnel(super::VpnTunnelInfo),
        /// Display information of the final state "deliver" and reason.
        #[prost(message, tag = "12")]
        Deliver(super::DeliverInfo),
        /// Display information of the final state "forward" and reason.
        #[prost(message, tag = "13")]
        Forward(super::ForwardInfo),
        /// Display information of the final state "abort" and reason.
        #[prost(message, tag = "14")]
        Abort(super::AbortInfo),
        /// Display information of the final state "drop" and reason.
        #[prost(message, tag = "15")]
        Drop(super::DropInfo),
        /// Display information of the load balancers.
        #[prost(message, tag = "16")]
        LoadBalancer(super::LoadBalancerInfo),
        /// Display information of a Google Cloud network.
        #[prost(message, tag = "17")]
        Network(super::NetworkInfo),
        /// Display information of a Google Kubernetes Engine cluster master.
        #[prost(message, tag = "18")]
        GkeMaster(super::GkeMasterInfo),
        /// Display information of a Cloud SQL instance.
        #[prost(message, tag = "19")]
        CloudSqlInstance(super::CloudSqlInstanceInfo),
    }
}
/// For display only. Metadata associated with a Compute Engine instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceInfo {
    /// Name of a Compute Engine instance.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a Compute Engine instance.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Name of the network interface of a Compute Engine instance.
    #[prost(string, tag = "3")]
    pub interface: ::prost::alloc::string::String,
    /// URI of a Compute Engine network.
    #[prost(string, tag = "4")]
    pub network_uri: ::prost::alloc::string::String,
    /// Internal IP address of the network interface.
    #[prost(string, tag = "5")]
    pub internal_ip: ::prost::alloc::string::String,
    /// External IP address of the network interface.
    #[prost(string, tag = "6")]
    pub external_ip: ::prost::alloc::string::String,
    /// Network tags configured on the instance.
    #[prost(string, repeated, tag = "7")]
    pub network_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Service account authorized for the instance.
    #[deprecated]
    #[prost(string, tag = "8")]
    pub service_account: ::prost::alloc::string::String,
}
/// For display only. Metadata associated with a Compute Engine network.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkInfo {
    /// Name of a Compute Engine network.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a Compute Engine network.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// The IP range that matches the test.
    #[prost(string, tag = "4")]
    pub matched_ip_range: ::prost::alloc::string::String,
}
/// For display only. Metadata associated with a VPC firewall rule, an implied
/// VPC firewall rule, or a hierarchical firewall policy rule.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallInfo {
    /// The display name of the VPC firewall rule. This field is not applicable
    /// to hierarchical firewall policy rules.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// The URI of the VPC firewall rule. This field is not applicable to
    /// implied firewall rules or hierarchical firewall policy rules.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Possible values: INGRESS, EGRESS
    #[prost(string, tag = "3")]
    pub direction: ::prost::alloc::string::String,
    /// Possible values: ALLOW, DENY
    #[prost(string, tag = "4")]
    pub action: ::prost::alloc::string::String,
    /// The priority of the firewall rule.
    #[prost(int32, tag = "5")]
    pub priority: i32,
    /// The URI of the VPC network that the firewall rule is associated with.
    /// This field is not applicable to hierarchical firewall policy rules.
    #[prost(string, tag = "6")]
    pub network_uri: ::prost::alloc::string::String,
    /// The target tags defined by the VPC firewall rule. This field is not
    /// applicable to hierarchical firewall policy rules.
    #[prost(string, repeated, tag = "7")]
    pub target_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The target service accounts specified by the firewall rule.
    #[prost(string, repeated, tag = "8")]
    pub target_service_accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The hierarchical firewall policy that this rule is associated with.
    /// This field is not applicable to VPC firewall rules.
    #[prost(string, tag = "9")]
    pub policy: ::prost::alloc::string::String,
    /// The firewall rule's type.
    #[prost(enumeration = "firewall_info::FirewallRuleType", tag = "10")]
    pub firewall_rule_type: i32,
}
/// Nested message and enum types in `FirewallInfo`.
pub mod firewall_info {
    /// The firewall rule's type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FirewallRuleType {
        /// Unspecified type.
        Unspecified = 0,
        /// Hierarchical firewall policy rule. For details, see
        /// [Hierarchical firewall policies
        /// overview](<https://cloud.google.com/vpc/docs/firewall-policies>).
        HierarchicalFirewallPolicyRule = 1,
        /// VPC firewall rule. For details, see
        /// [VPC firewall rules
        /// overview](<https://cloud.google.com/vpc/docs/firewalls>).
        VpcFirewallRule = 2,
        /// Implied VPC firewall rule. For details, see
        /// [Implied
        /// rules](<https://cloud.google.com/vpc/docs/firewalls#default_firewall_rules>).
        ImpliedVpcFirewallRule = 3,
    }
}
/// For display only. Metadata associated with a Compute Engine route.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteInfo {
    /// Type of route.
    #[prost(enumeration = "route_info::RouteType", tag = "8")]
    pub route_type: i32,
    /// Type of next hop.
    #[prost(enumeration = "route_info::NextHopType", tag = "9")]
    pub next_hop_type: i32,
    /// Name of a Compute Engine route.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a Compute Engine route.
    /// Dynamic route from cloud router does not have a URI.
    /// Advertised route from Google Cloud VPC to on-premises network also does
    /// not have a URI.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Destination IP range of the route.
    #[prost(string, tag = "3")]
    pub dest_ip_range: ::prost::alloc::string::String,
    /// Next hop of the route.
    #[prost(string, tag = "4")]
    pub next_hop: ::prost::alloc::string::String,
    /// URI of a Compute Engine network.
    #[prost(string, tag = "5")]
    pub network_uri: ::prost::alloc::string::String,
    /// Priority of the route.
    #[prost(int32, tag = "6")]
    pub priority: i32,
    /// Instance tags of the route.
    #[prost(string, repeated, tag = "7")]
    pub instance_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `RouteInfo`.
pub mod route_info {
    /// Type of route:
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RouteType {
        /// Unspecified type. Default value.
        Unspecified = 0,
        /// Route is a subnet route automatically created by the system.
        Subnet = 1,
        /// Static route created by the user, including the default route to the
        /// internet.
        Static = 2,
        /// Dynamic route exchanged between BGP peers.
        Dynamic = 3,
        /// A subnet route received from peering network.
        PeeringSubnet = 4,
        /// A static route received from peering network.
        PeeringStatic = 5,
        /// A dynamic route received from peering network.
        PeeringDynamic = 6,
    }
    /// Type of next hop:
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum NextHopType {
        /// Unspecified type. Default value.
        Unspecified = 0,
        /// Next hop is an IP address.
        NextHopIp = 1,
        /// Next hop is a Compute Engine instance.
        NextHopInstance = 2,
        /// Next hop is a VPC network gateway.
        NextHopNetwork = 3,
        /// Next hop is a peering VPC.
        NextHopPeering = 4,
        /// Next hop is an interconnect.
        NextHopInterconnect = 5,
        /// Next hop is a VPN tunnel.
        NextHopVpnTunnel = 6,
        /// Next hop is a VPN gateway. This scenario only happens when tracing
        /// connectivity from an on-premises network to Google Cloud through a VPN.
        /// The analysis simulates a packet departing from the on-premises network
        /// through a VPN tunnel and arriving at a Cloud VPN gateway.
        NextHopVpnGateway = 7,
        /// Next hop is an internet gateway.
        NextHopInternetGateway = 8,
        /// Next hop is blackhole; that is, the next hop either does not exist or is
        /// not running.
        NextHopBlackhole = 9,
        /// Next hop is the forwarding rule of an Internal Load Balancer.
        NextHopIlb = 10,
    }
}
/// For display only. Metadata associated with a Compute Engine forwarding rule.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardingRuleInfo {
    /// Name of a Compute Engine forwarding rule.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a Compute Engine forwarding rule.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Protocol defined in the forwarding rule that matches the test.
    #[prost(string, tag = "3")]
    pub matched_protocol: ::prost::alloc::string::String,
    /// Port range defined in the forwarding rule that matches the test.
    #[prost(string, tag = "6")]
    pub matched_port_range: ::prost::alloc::string::String,
    /// VIP of the forwarding rule.
    #[prost(string, tag = "4")]
    pub vip: ::prost::alloc::string::String,
    /// Target type of the forwarding rule.
    #[prost(string, tag = "5")]
    pub target: ::prost::alloc::string::String,
    /// Network URI. Only valid for Internal Load Balancer.
    #[prost(string, tag = "7")]
    pub network_uri: ::prost::alloc::string::String,
}
/// For display only. Metadata associated with a load balancer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadBalancerInfo {
    /// Type of the load balancer.
    #[prost(enumeration = "load_balancer_info::LoadBalancerType", tag = "1")]
    pub load_balancer_type: i32,
    /// URI of the health check for the load balancer.
    #[prost(string, tag = "2")]
    pub health_check_uri: ::prost::alloc::string::String,
    /// Information for the loadbalancer backends.
    #[prost(message, repeated, tag = "3")]
    pub backends: ::prost::alloc::vec::Vec<LoadBalancerBackend>,
    /// Type of load balancer's backend configuration.
    #[prost(enumeration = "load_balancer_info::BackendType", tag = "4")]
    pub backend_type: i32,
    /// Backend configuration URI.
    #[prost(string, tag = "5")]
    pub backend_uri: ::prost::alloc::string::String,
}
/// Nested message and enum types in `LoadBalancerInfo`.
pub mod load_balancer_info {
    /// The type definition for a load balancer:
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LoadBalancerType {
        /// Type is unspecified.
        Unspecified = 0,
        /// Internal TCP/UDP load balancer.
        InternalTcpUdp = 1,
        /// Network TCP/UDP load balancer.
        NetworkTcpUdp = 2,
        /// HTTP(S) proxy load balancer.
        HttpProxy = 3,
        /// TCP proxy load balancer.
        TcpProxy = 4,
        /// SSL proxy load balancer.
        SslProxy = 5,
    }
    /// The type definition for a load balancer backend configuration:
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BackendType {
        /// Type is unspecified.
        Unspecified = 0,
        /// Backend Service as the load balancer's backend.
        BackendService = 1,
        /// Target Pool as the load balancer's backend.
        TargetPool = 2,
    }
}
/// For display only. Metadata associated with a specific load balancer backend.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadBalancerBackend {
    /// Name of a Compute Engine instance or network endpoint.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a Compute Engine instance or network endpoint.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// State of the health check firewall configuration.
    #[prost(enumeration = "load_balancer_backend::HealthCheckFirewallState", tag = "3")]
    pub health_check_firewall_state: i32,
    /// A list of firewall rule URIs allowing probes from health check IP ranges.
    #[prost(string, repeated, tag = "4")]
    pub health_check_allowing_firewall_rules:
        ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of firewall rule URIs blocking probes from health check IP ranges.
    #[prost(string, repeated, tag = "5")]
    pub health_check_blocking_firewall_rules:
        ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `LoadBalancerBackend`.
pub mod load_balancer_backend {
    /// State of a health check firewall configuration:
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HealthCheckFirewallState {
        /// State is unspecified. Default state if not populated.
        Unspecified = 0,
        /// There are configured firewall rules to allow health check probes to the
        /// backend.
        Configured = 1,
        /// There are firewall rules configured to allow partial health check ranges
        /// or block all health check ranges.
        /// If a health check probe is sent from denied IP ranges,
        /// the health check to the backend will fail. Then, the backend will be
        /// marked unhealthy and will not receive traffic sent to the load balancer.
        Misconfigured = 2,
    }
}
/// For display only. Metadata associated with a Compute Engine VPN gateway.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VpnGatewayInfo {
    /// Name of a VPN gateway.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a VPN gateway.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// URI of a Compute Engine network where the VPN gateway is configured.
    #[prost(string, tag = "3")]
    pub network_uri: ::prost::alloc::string::String,
    /// IP address of the VPN gateway.
    #[prost(string, tag = "4")]
    pub ip_address: ::prost::alloc::string::String,
    /// A VPN tunnel that is associated with this VPN gateway.
    /// There may be multiple VPN tunnels configured on a VPN gateway, and only
    /// the one relevant to the test is displayed.
    #[prost(string, tag = "5")]
    pub vpn_tunnel_uri: ::prost::alloc::string::String,
    /// Name of a Google Cloud region where this VPN gateway is configured.
    #[prost(string, tag = "6")]
    pub region: ::prost::alloc::string::String,
}
/// For display only. Metadata associated with a Compute Engine VPN tunnel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VpnTunnelInfo {
    /// Name of a VPN tunnel.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a VPN tunnel.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// URI of the VPN gateway at local end of the tunnel.
    #[prost(string, tag = "3")]
    pub source_gateway: ::prost::alloc::string::String,
    /// URI of a VPN gateway at remote end of the tunnel.
    #[prost(string, tag = "4")]
    pub remote_gateway: ::prost::alloc::string::String,
    /// Remote VPN gateway's IP address.
    #[prost(string, tag = "5")]
    pub remote_gateway_ip: ::prost::alloc::string::String,
    /// Local VPN gateway's IP address.
    #[prost(string, tag = "6")]
    pub source_gateway_ip: ::prost::alloc::string::String,
    /// URI of a Compute Engine network where the VPN tunnel is configured.
    #[prost(string, tag = "7")]
    pub network_uri: ::prost::alloc::string::String,
    /// Name of a Google Cloud region where this VPN tunnel is configured.
    #[prost(string, tag = "8")]
    pub region: ::prost::alloc::string::String,
    /// Type of the routing policy.
    #[prost(enumeration = "vpn_tunnel_info::RoutingType", tag = "9")]
    pub routing_type: i32,
}
/// Nested message and enum types in `VpnTunnelInfo`.
pub mod vpn_tunnel_info {
    /// Types of VPN routing policy. For details, refer to [Networks and Tunnel
    /// routing](<https://cloud.google.com/network-connectivity/docs/vpn/concepts/choosing-networks-routing/>).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RoutingType {
        /// Unspecified type. Default value.
        Unspecified = 0,
        /// Route based VPN.
        RouteBased = 1,
        /// Policy based routing.
        PolicyBased = 2,
        /// Dynamic (BGP) routing.
        Dynamic = 3,
    }
}
/// For display only. The specification of the endpoints for the test.
/// EndpointInfo is derived from source and destination Endpoint and validated
/// by the backend data plane model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointInfo {
    /// Source IP address.
    #[prost(string, tag = "1")]
    pub source_ip: ::prost::alloc::string::String,
    /// Destination IP address.
    #[prost(string, tag = "2")]
    pub destination_ip: ::prost::alloc::string::String,
    /// IP protocol in string format, for example: "TCP", "UDP", "ICMP".
    #[prost(string, tag = "3")]
    pub protocol: ::prost::alloc::string::String,
    /// Source port. Only valid when protocol is TCP or UDP.
    #[prost(int32, tag = "4")]
    pub source_port: i32,
    /// Destination port. Only valid when protocol is TCP or UDP.
    #[prost(int32, tag = "5")]
    pub destination_port: i32,
    /// URI of the network where this packet originates from.
    #[prost(string, tag = "6")]
    pub source_network_uri: ::prost::alloc::string::String,
    /// URI of the network where this packet is sent to.
    #[prost(string, tag = "7")]
    pub destination_network_uri: ::prost::alloc::string::String,
}
/// Details of the final state "deliver" and associated resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeliverInfo {
    /// Target type where the packet is delivered to.
    #[prost(enumeration = "deliver_info::Target", tag = "1")]
    pub target: i32,
    /// URI of the resource that the packet is delivered to.
    #[prost(string, tag = "2")]
    pub resource_uri: ::prost::alloc::string::String,
}
/// Nested message and enum types in `DeliverInfo`.
pub mod deliver_info {
    /// Deliver target types:
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Target {
        /// Target not specified.
        Unspecified = 0,
        /// Target is a Compute Engine instance.
        Instance = 1,
        /// Target is the internet.
        Internet = 2,
        /// Target is a Google API.
        GoogleApi = 3,
        /// Target is a Google Kubernetes Engine cluster master.
        GkeMaster = 4,
        /// Target is a Cloud SQL instance.
        CloudSqlInstance = 5,
    }
}
/// Details of the final state "forward" and associated resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardInfo {
    /// Target type where this packet is forwarded to.
    #[prost(enumeration = "forward_info::Target", tag = "1")]
    pub target: i32,
    /// URI of the resource that the packet is forwarded to.
    #[prost(string, tag = "2")]
    pub resource_uri: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ForwardInfo`.
pub mod forward_info {
    /// Forward target types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Target {
        /// Target not specified.
        Unspecified = 0,
        /// Forwarded to a VPC peering network.
        PeeringVpc = 1,
        /// Forwarded to a Cloud VPN gateway.
        VpnGateway = 2,
        /// Forwarded to a Cloud Interconnect connection.
        Interconnect = 3,
        /// Forwarded to a Google Kubernetes Engine Container cluster master.
        GkeMaster = 4,
        /// Forwarded to the next hop of a custom route imported from a peering VPC.
        ImportedCustomRouteNextHop = 5,
        /// Forwarded to a Cloud SQL instance.
        CloudSqlInstance = 6,
    }
}
/// Details of the final state "abort" and associated resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbortInfo {
    /// Causes that the analysis is aborted.
    #[prost(enumeration = "abort_info::Cause", tag = "1")]
    pub cause: i32,
    /// URI of the resource that caused the abort.
    #[prost(string, tag = "2")]
    pub resource_uri: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AbortInfo`.
pub mod abort_info {
    /// Abort cause types:
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Cause {
        /// Cause is unspecified.
        Unspecified = 0,
        /// Aborted due to unknown network.
        /// The reachability analysis cannot proceed because the user does not have
        /// access to the host project's network configurations, including firewall
        /// rules and routes. This happens when the project is a service project and
        /// the endpoints being traced are in the host project's network.
        UnknownNetwork = 1,
        /// Aborted because the IP address(es) are unknown.
        UnknownIp = 2,
        /// Aborted because no project information can be derived from the test
        /// input.
        UnknownProject = 3,
        /// Aborted because the user lacks the permission to access all or part of
        /// the network configurations required to run the test.
        PermissionDenied = 4,
        /// Aborted because no valid source endpoint is derived from the input test
        /// request.
        NoSourceLocation = 5,
        /// Aborted because the source and/or destination endpoint specified in
        /// the test are invalid. The possible reasons that an endpoint is
        /// invalid include: malformed IP address; nonexistent instance or
        /// network URI; IP address not in the range of specified network URI; and
        /// instance not owning the network interface in the specified network.
        InvalidArgument = 6,
        /// Aborted because traffic is sent from a public IP to an instance without
        /// an external IP.
        NoExternalIp = 7,
        /// Aborted because none of the traces matches destination information
        /// specified in the input test request.
        UnintendedDestination = 8,
        /// Aborted because the number of steps in the trace exceeding a certain
        /// limit which may be caused by routing loop.
        TraceTooLong = 9,
        /// Aborted due to internal server error.
        InternalError = 10,
        /// Aborted because the source endpoint could not be found.
        SourceEndpointNotFound = 11,
        /// Aborted because the source network does not match the source endpoint.
        MismatchedSourceNetwork = 12,
        /// Aborted because the destination endpoint could not be found.
        DestinationEndpointNotFound = 13,
        /// Aborted because the destination network does not match the destination
        /// endpoint.
        MismatchedDestinationNetwork = 14,
    }
}
/// Details of the final state "drop" and associated resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropInfo {
    /// Cause that the packet is dropped.
    #[prost(enumeration = "drop_info::Cause", tag = "1")]
    pub cause: i32,
    /// URI of the resource that caused the drop.
    #[prost(string, tag = "2")]
    pub resource_uri: ::prost::alloc::string::String,
}
/// Nested message and enum types in `DropInfo`.
pub mod drop_info {
    /// Drop cause types:
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Cause {
        /// Cause is unspecified.
        Unspecified = 0,
        /// Destination external address cannot be resolved to a known target. If
        /// the address is used in a Google Cloud project, provide the project ID
        /// as test input.
        UnknownExternalAddress = 1,
        /// A Compute Engine instance can only send or receive a packet with a
        /// foreign IP address if ip_forward is enabled.
        ForeignIpDisallowed = 2,
        /// Dropped due to a firewall rule, unless allowed due to connection
        /// tracking.
        FirewallRule = 3,
        /// Dropped due to no routes.
        NoRoute = 4,
        /// Dropped due to invalid route. Route's next hop is a blackhole.
        RouteBlackhole = 5,
        /// Packet is sent to a wrong (unintended) network. Example: you trace a
        /// packet from VM1:Network1 to VM2:Network2, however, the route configured
        /// in Network1 sends the packet destined for VM2's IP addresss to Network3.
        RouteWrongNetwork = 6,
        /// Packet with internal destination address sent to the internet gateway.
        PrivateTrafficToInternet = 7,
        /// Instance with only an internal IP address tries to access Google API and
        /// services, but private Google access is not enabled.
        PrivateGoogleAccessDisallowed = 8,
        /// Instance with only an internal IP address tries to access external hosts,
        /// but Cloud NAT is not enabled in the subnet, unless special configurations
        /// on a VM allow this connection. For more details, see [Special
        /// configurations for VM
        /// instances](<https://cloud.google.com/vpc/docs/special-configurations>).
        NoExternalAddress = 9,
        /// Destination internal address cannot be resolved to a known target. If
        /// this is a shared VPC scenario, verify if the service project ID is
        /// provided as test input. Otherwise, verify if the IP address is being
        /// used in the project.
        UnknownInternalAddress = 10,
        /// Forwarding rule's protocol and ports do not match the packet header.
        ForwardingRuleMismatch = 11,
        /// Forwarding rule does not have backends configured.
        ForwardingRuleNoInstances = 12,
        /// Firewalls block the health check probes to the backends and cause
        /// the backends to be unavailable for traffic from the load balancer.
        /// For more details, see [Health check firewall
        /// rules](<https://cloud.google.com/load-balancing/docs/health-checks#firewall_rules>).
        FirewallBlockingLoadBalancerBackendHealthCheck = 13,
        /// Packet is sent from or to a Compute Engine instance that is not in a
        /// running state.
        InstanceNotRunning = 14,
        /// The type of traffic is blocked and the user cannot configure a firewall
        /// rule to enable it. See [Always blocked
        /// traffic](<https://cloud.google.com/vpc/docs/firewalls#blockedtraffic>) for
        /// more details.
        TrafficTypeBlocked = 15,
        /// Access to Google Kubernetes Engine cluster master's endpoint is not
        /// authorized. See [Access to the cluster
        /// endpoints](<https://cloud.google.com/kubernetes-engine/docs/how-to/private-clusters#access_to_the_cluster_endpoints>)
        /// for more details.
        GkeMasterUnauthorizedAccess = 16,
        /// Access to the Cloud SQL instance endpoint is not authorized.
        /// See [Authorizing with authorized
        /// networks](<https://cloud.google.com/sql/docs/mysql/authorize-networks>) for
        /// more details.
        CloudSqlInstanceUnauthorizedAccess = 17,
        /// Packet was dropped inside Google Kubernetes Engine Service.
        DroppedInsideGkeService = 18,
        /// Packet was dropped inside Cloud SQL Service.
        DroppedInsideCloudSqlService = 19,
        /// Packet was dropped because there is no peering between the originating
        /// network and the Google Managed Services Network.
        GoogleManagedServiceNoPeering = 20,
        /// Packet was dropped because the Cloud SQL instance has neither a private
        /// nor a public IP address.
        CloudSqlInstanceNoIpAddress = 21,
    }
}
/// For display only. Metadata associated with a Google Kubernetes Engine (GKE)
/// cluster master.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GkeMasterInfo {
    /// URI of a GKE cluster.
    #[prost(string, tag = "2")]
    pub cluster_uri: ::prost::alloc::string::String,
    /// URI of a GKE cluster network.
    #[prost(string, tag = "4")]
    pub cluster_network_uri: ::prost::alloc::string::String,
    /// Internal IP address of a GKE cluster master.
    #[prost(string, tag = "5")]
    pub internal_ip: ::prost::alloc::string::String,
    /// External IP address of a GKE cluster master.
    #[prost(string, tag = "6")]
    pub external_ip: ::prost::alloc::string::String,
}
/// For display only. Metadata associated with a Cloud SQL instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSqlInstanceInfo {
    /// Name of a Cloud SQL instance.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a Cloud SQL instance.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// URI of a Cloud SQL instance network or empty string if the instance does
    /// not have one.
    #[prost(string, tag = "4")]
    pub network_uri: ::prost::alloc::string::String,
    /// Internal IP address of a Cloud SQL instance.
    #[prost(string, tag = "5")]
    pub internal_ip: ::prost::alloc::string::String,
    /// External IP address of a Cloud SQL instance.
    #[prost(string, tag = "6")]
    pub external_ip: ::prost::alloc::string::String,
    /// Region in which the Cloud SQL instance is running.
    #[prost(string, tag = "7")]
    pub region: ::prost::alloc::string::String,
}
/// A Connectivity Test for a network reachability analysis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectivityTest {
    /// Required. Unique name of the resource using the form:
    ///     `projects/{project_id}/locations/global/connectivityTests/{test_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The user-supplied description of the Connectivity Test.
    /// Maximum of 512 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. Source specification of the Connectivity Test.
    ///
    /// You can use a combination of source IP address, virtual machine
    /// (VM) instance, or Compute Engine network to uniquely identify
    /// the source location.
    ///
    /// Examples:
    /// If the source IP address is an internal IP address within a Google Cloud
    /// Virtual Private Cloud (VPC) network, then you must also specify the VPC
    /// network. Otherwise, specify the VM instance, which already contains its
    /// internal IP address and VPC network information.
    ///
    /// If the source of the test is within an on-premises network, then you must
    /// provide the destination VPC network.
    ///
    /// If the source endpoint is a Compute Engine VM instance with multiple
    /// network interfaces, the instance itself is not sufficient to identify the
    /// endpoint. So, you must also specify the source IP address or VPC network.
    ///
    /// A reachability analysis proceeds even if the source location is
    /// ambiguous. However, the test result may include endpoints that you don't
    /// intend to test.
    #[prost(message, optional, tag = "3")]
    pub source: ::core::option::Option<Endpoint>,
    /// Required. Destination specification of the Connectivity Test.
    ///
    /// You can use a combination of destination IP address, Compute Engine
    /// VM instance, or VPC network to uniquely identify the destination
    /// location.
    ///
    /// Even if the destination IP address is not unique, the source IP
    /// location is unique. Usually, the analysis can infer the destination
    /// endpoint from route information.
    ///
    /// If the destination you specify is a VM instance and the instance has
    /// multiple network interfaces, then you must also specify either
    /// a destination IP address  or VPC network to identify the destination
    /// interface.
    ///
    /// A reachability analysis proceeds even if the destination location is
    /// ambiguous. However, the result can include endpoints that you don't
    /// intend to test.
    #[prost(message, optional, tag = "4")]
    pub destination: ::core::option::Option<Endpoint>,
    /// IP Protocol of the test. When not provided, "TCP" is assumed.
    #[prost(string, tag = "5")]
    pub protocol: ::prost::alloc::string::String,
    /// Other projects that may be relevant for reachability analysis.
    /// This is applicable to scenarios where a test can cross project boundaries.
    #[prost(string, repeated, tag = "6")]
    pub related_projects: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The display name of a Connectivity Test.
    #[prost(string, tag = "7")]
    pub display_name: ::prost::alloc::string::String,
    /// Resource labels to represent user-provided metadata.
    #[prost(map = "string, string", tag = "8")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. The time the test was created.
    #[prost(message, optional, tag = "10")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the test's configuration was updated.
    #[prost(message, optional, tag = "11")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The reachability details of this test from the latest run.
    /// The details are updated when creating a new test, updating an
    /// existing test, or triggering a one-time rerun of an existing test.
    #[prost(message, optional, tag = "12")]
    pub reachability_details: ::core::option::Option<ReachabilityDetails>,
}
/// Source or destination of the Connectivity Test.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Endpoint {
    /// The IP address of the endpoint, which can be an external or internal IP.
    /// An IPv6 address is only allowed when the test's destination is a
    /// [global load balancer VIP](/load-balancing/docs/load-balancing-overview).
    #[prost(string, tag = "1")]
    pub ip_address: ::prost::alloc::string::String,
    /// The IP protocol port of the endpoint.
    /// Only applicable when protocol is TCP or UDP.
    #[prost(int32, tag = "2")]
    pub port: i32,
    /// A Compute Engine instance URI.
    #[prost(string, tag = "3")]
    pub instance: ::prost::alloc::string::String,
    /// A cluster URI for [Google Kubernetes Engine
    /// master](<https://cloud.google.com/kubernetes-engine/docs/concepts/cluster-architecture>).
    #[prost(string, tag = "7")]
    pub gke_master_cluster: ::prost::alloc::string::String,
    /// A [Cloud SQL](<https://cloud.google.com/sql>) instance URI.
    #[prost(string, tag = "8")]
    pub cloud_sql_instance: ::prost::alloc::string::String,
    /// A Compute Engine network URI.
    #[prost(string, tag = "4")]
    pub network: ::prost::alloc::string::String,
    /// Type of the network where the endpoint is located.
    /// Applicable only to source endpoint, as destination network type can be
    /// inferred from the source.
    #[prost(enumeration = "endpoint::NetworkType", tag = "5")]
    pub network_type: i32,
    /// Project ID where the endpoint is located.
    /// The Project ID can be derived from the URI if you provide a VM instance or
    /// network URI.
    /// The following are two cases where you must provide the project ID:
    /// 1. Only the IP address is specified, and the IP address is within a GCP
    /// project.
    /// 2. When you are using Shared VPC and the IP address that you provide is
    /// from the service project. In this case, the network that the IP address
    /// resides in is defined in the host project.
    #[prost(string, tag = "6")]
    pub project_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Endpoint`.
pub mod endpoint {
    /// The type definition of an endpoint's network. Use one of the
    /// following choices:
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum NetworkType {
        /// Default type if unspecified.
        Unspecified = 0,
        /// A network hosted within Google Cloud Platform.
        /// To receive more detailed output, specify the URI for the source or
        /// destination network.
        GcpNetwork = 1,
        /// A network hosted outside of Google Cloud Platform.
        /// This can be an on-premises network, or a network hosted by another cloud
        /// provider.
        NonGcpNetwork = 2,
    }
}
/// Results of the configuration analysis from the last run of the test.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReachabilityDetails {
    /// The overall result of the test's configuration analysis.
    #[prost(enumeration = "reachability_details::Result", tag = "1")]
    pub result: i32,
    /// The time of the configuration analysis.
    #[prost(message, optional, tag = "2")]
    pub verify_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The details of a failure or a cancellation of reachability analysis.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// Result may contain a list of traces if a test has multiple possible
    /// paths in the network, such as when destination endpoint is a load balancer
    /// with multiple backends.
    #[prost(message, repeated, tag = "5")]
    pub traces: ::prost::alloc::vec::Vec<Trace>,
}
/// Nested message and enum types in `ReachabilityDetails`.
pub mod reachability_details {
    /// The overall result of the test's configuration analysis.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// No result was specified.
        Unspecified = 0,
        /// Possible scenarios are:
        ///
        /// * The configuration analysis determined that a packet originating from
        ///   the source is expected to reach the destination.
        /// * The analysis didn't complete because the user lacks permission for
        ///   some of the resources in the trace. However, at the time the user's
        ///   permission became insufficient, the trace had been successful so far.
        Reachable = 1,
        /// A packet originating from the source is expected to be dropped before
        /// reaching the destination.
        Unreachable = 2,
        /// The source and destination endpoints do not uniquely identify
        /// the test location in the network, and the reachability result contains
        /// multiple traces. For some traces, a packet could be delivered, and for
        /// others, it would not be.
        Ambiguous = 4,
        /// The configuration analysis did not complete. Possible reasons are:
        ///
        /// * A permissions error occurred--for example, the user might not have
        ///   read permission for all of the resources named in the test.
        /// * An internal error occurred.
        /// * The analyzer received an invalid or unsupported argument or was unable
        ///   to identify a known endpoint.
        Undetermined = 5,
    }
}
/// Request for the `ListConnectivityTests` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectivityTestsRequest {
    /// Required. The parent resource of the Connectivity Tests:
    ///     `projects/{project_id}/locations/global`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Number of `ConnectivityTests` to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token from an earlier query, as returned in `next_page_token`.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Lists the `ConnectivityTests` that match the filter expression. A filter
    /// expression filters the resources listed in the response. The expression
    /// must be of the form `<field> <operator> <value>` where operators: `<`, `>`,
    /// `<=`,
    /// `>=`,
    /// `!=`, `=`, `:` are supported (colon `:` represents a HAS operator which is
    /// roughly synonymous with equality). <field> can refer to a proto or JSON
    /// field, or a synthetic field. Field names can be camelCase or snake_case.
    ///
    /// Examples:
    /// - Filter by name:
    ///   name = "projects/proj-1/locations/global/connectivityTests/test-1
    ///
    /// - Filter by labels:
    ///   - Resources that have a key called `foo`
    ///     labels.foo:*
    ///   - Resources that have a key called `foo` whose value is `bar`
    ///     labels.foo = bar
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field to use to sort the list.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for the `ListConnectivityTests` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectivityTestsResponse {
    /// List of Connectivity Tests.
    #[prost(message, repeated, tag = "1")]
    pub resources: ::prost::alloc::vec::Vec<ConnectivityTest>,
    /// Page token to fetch the next set of Connectivity Tests.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached (when querying all locations with `-`).
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for the `GetConnectivityTest` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConnectivityTestRequest {
    /// Required. `ConnectivityTest` resource name using the form:
    ///     `projects/{project_id}/locations/global/connectivityTests/{test_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `CreateConnectivityTest` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConnectivityTestRequest {
    /// Required. The parent resource of the Connectivity Test to create:
    ///     `projects/{project_id}/locations/global`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The logical name of the Connectivity Test in your project
    /// with the following restrictions:
    ///
    /// * Must contain only lowercase letters, numbers, and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-40 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the customer project
    #[prost(string, tag = "2")]
    pub test_id: ::prost::alloc::string::String,
    /// Required. A `ConnectivityTest` resource
    #[prost(message, optional, tag = "3")]
    pub resource: ::core::option::Option<ConnectivityTest>,
}
/// Request for the `UpdateConnectivityTest` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConnectivityTestRequest {
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "2")]
    pub resource: ::core::option::Option<ConnectivityTest>,
}
/// Request for the `DeleteConnectivityTest` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConnectivityTestRequest {
    /// Required. Connectivity Test resource name using the form:
    ///     `projects/{project_id}/locations/global/connectivityTests/{test_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `RerunConnectivityTest` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RerunConnectivityTestRequest {
    /// Required. Connectivity Test resource name using the form:
    ///     `projects/{project_id}/locations/global/connectivityTests/{test_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Metadata describing an \[Operation][google.longrunning.Operation\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Target of the operation - for example
    /// projects/project-1/locations/global/connectivityTests/test-1
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_detail: ::prost::alloc::string::String,
    /// Specifies if cancellation was requested for the operation.
    #[prost(bool, tag = "6")]
    pub cancel_requested: bool,
    /// API version.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod reachability_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The Reachability service in the Google Cloud Network Management API provides"]
    #[doc = " services that analyze the reachability within a single Google Virtual Private"]
    #[doc = " Cloud (VPC) network, between peered VPC networks, between VPC and on-premises"]
    #[doc = " networks, or between VPC networks and internet hosts. A reachability analysis"]
    #[doc = " is based on Google Cloud network configurations."]
    #[doc = ""]
    #[doc = " You can use the analysis results to verify these configurations and"]
    #[doc = " to troubleshoot connectivity issues."]
    #[derive(Debug, Clone)]
    pub struct ReachabilityServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ReachabilityServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ReachabilityServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ReachabilityServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Lists all Connectivity Tests owned by a project."]
        pub async fn list_connectivity_tests(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConnectivityTestsRequest>,
        ) -> Result<tonic::Response<super::ListConnectivityTestsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkmanagement.v1.ReachabilityService/ListConnectivityTests",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the details of a specific Connectivity Test."]
        pub async fn get_connectivity_test(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConnectivityTestRequest>,
        ) -> Result<tonic::Response<super::ConnectivityTest>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkmanagement.v1.ReachabilityService/GetConnectivityTest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Connectivity Test."]
        #[doc = " After you create a test, the reachability analysis is performed as part"]
        #[doc = " of the long running operation, which completes when the analysis completes."]
        #[doc = ""]
        #[doc = " If the endpoint specifications in `ConnectivityTest` are invalid"]
        #[doc = " (for example, containing non-existent resources in the network, or you"]
        #[doc = " don't have read permissions to the network configurations of listed"]
        #[doc = " projects), then the reachability result returns a value of `UNKNOWN`."]
        #[doc = ""]
        #[doc = " If the endpoint specifications in `ConnectivityTest` are"]
        #[doc = " incomplete, the reachability result returns a value of"]
        #[doc = " <code>AMBIGUOUS</code>. For more information,"]
        #[doc = " see the Connectivity Test documentation."]
        pub async fn create_connectivity_test(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConnectivityTestRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkmanagement.v1.ReachabilityService/CreateConnectivityTest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the configuration of an existing `ConnectivityTest`."]
        #[doc = " After you update a test, the reachability analysis is performed as part"]
        #[doc = " of the long running operation, which completes when the analysis completes."]
        #[doc = " The Reachability state in the test resource is updated with the new result."]
        #[doc = ""]
        #[doc = " If the endpoint specifications in `ConnectivityTest` are invalid"]
        #[doc = " (for example, they contain non-existent resources in the network, or the"]
        #[doc = " user does not have read permissions to the network configurations of"]
        #[doc = " listed projects), then the reachability result returns a value of"]
        #[doc = " <code>UNKNOWN</code>."]
        #[doc = ""]
        #[doc = " If the endpoint specifications in `ConnectivityTest` are incomplete, the"]
        #[doc = " reachability result returns a value of `AMBIGUOUS`. See the documentation"]
        #[doc = " in `ConnectivityTest` for for more details."]
        pub async fn update_connectivity_test(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConnectivityTestRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkmanagement.v1.ReachabilityService/UpdateConnectivityTest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Rerun an existing `ConnectivityTest`."]
        #[doc = " After the user triggers the rerun, the reachability analysis is performed"]
        #[doc = " as part of the long running operation, which completes when the analysis"]
        #[doc = " completes."]
        #[doc = ""]
        #[doc = " Even though the test configuration remains the same, the reachability"]
        #[doc = " result may change due to underlying network configuration changes."]
        #[doc = ""]
        #[doc = " If the endpoint specifications in `ConnectivityTest` become invalid (for"]
        #[doc = " example, specified resources are deleted in the network, or you lost"]
        #[doc = " read permissions to the network configurations of listed projects), then"]
        #[doc = " the reachability result returns a value of `UNKNOWN`."]
        pub async fn rerun_connectivity_test(
            &mut self,
            request: impl tonic::IntoRequest<super::RerunConnectivityTestRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkmanagement.v1.ReachabilityService/RerunConnectivityTest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a specific `ConnectivityTest`."]
        pub async fn delete_connectivity_test(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConnectivityTestRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkmanagement.v1.ReachabilityService/DeleteConnectivityTest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
