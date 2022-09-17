#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpUtilizationInfo {
    #[prost(message, repeated, tag="1")]
    pub subnet_ip_utilization: ::prost::alloc::vec::Vec<ip_utilization_info::SubnetIpUtilization>,
}
/// Nested message and enum types in `IpUtilizationInfo`.
pub mod ip_utilization_info {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SubnetIpUtilization {
        /// URI of subnet.
        #[prost(string, tag="1")]
        pub subnet_uri: ::prost::alloc::string::String,
        /// Secondary range name. If the range is the primary range of the subnet,
        /// this field is empty.
        #[prost(string, tag="2")]
        pub secondary_range_name: ::prost::alloc::string::String,
        /// Total number of usable IP addresses in the IP range.
        #[prost(uint64, tag="3")]
        pub total_usable_addresses: u64,
        /// The ratio of allocated IP addresses from the total usable addresses.
        #[prost(double, tag="4")]
        pub allocation_ratio: f64,
    }
}
/// Log entry that describes a report from Network Analyzer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Report {
    /// The unique identifier of the report.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Priority of the report.
    #[prost(enumeration="report::Priority", tag="2")]
    pub priority: i32,
    /// Type of the report.
    #[prost(enumeration="report::Type", tag="3")]
    pub r#type: i32,
    /// Status of the report.
    #[prost(enumeration="report::ReportStatus", tag="4")]
    pub status: i32,
    /// The timestamp when the report was first discovered by Network Analyzer.
    #[prost(message, optional, tag="9")]
    pub first_report_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Cause code of the report.
    #[prost(enumeration="ReportCauseCode", tag="12")]
    pub cause_code: i32,
    /// The resource that are reported with the report.
    /// Contains the fully qualified resource name.
    /// e.g.,
    /// `//compute.googleapis.com/projects/{project_id}/global/networks/{network}`
    #[prost(string, tag="15")]
    pub resource_name: ::prost::alloc::string::String,
    /// Location associated with the report. It can be global or GCP regions
    /// and zones. e.g., <https://cloud.google.com/compute/docs/regions-zones/>
    #[prost(string, tag="16")]
    pub location: ::prost::alloc::string::String,
    /// URI to the documentation of the report.
    #[prost(string, tag="17")]
    pub report_documentation_uri: ::prost::alloc::string::String,
    /// The groups of the report. One report may be present in multiple groups.
    #[prost(enumeration="report::ReportGroup", repeated, tag="18")]
    pub report_groups: ::prost::alloc::vec::Vec<i32>,
    #[prost(oneof="report::Content", tags="19")]
    pub content: ::core::option::Option<report::Content>,
}
/// Nested message and enum types in `Report`.
pub mod report {
    /// Priority level of an report.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Priority {
        SeverityUnspecified = 0,
        Critical = 1,
        High = 2,
        Medium = 3,
        Low = 4,
    }
    impl Priority {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Priority::SeverityUnspecified => "SEVERITY_UNSPECIFIED",
                Priority::Critical => "CRITICAL",
                Priority::High => "HIGH",
                Priority::Medium => "MEDIUM",
                Priority::Low => "LOW",
            }
        }
    }
    /// Type of an report.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        ReportTypeUnspecified = 0,
        Info = 1,
        Warning = 2,
        Error = 3,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::ReportTypeUnspecified => "REPORT_TYPE_UNSPECIFIED",
                Type::Info => "INFO",
                Type::Warning => "WARNING",
                Type::Error => "ERROR",
            }
        }
    }
    /// Status of an report.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReportStatus {
        Unspecified = 0,
        Active = 1,
        Fixed = 2,
        Dismissed = 3,
    }
    impl ReportStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReportStatus::Unspecified => "REPORT_STATUS_UNSPECIFIED",
                ReportStatus::Active => "ACTIVE",
                ReportStatus::Fixed => "FIXED",
                ReportStatus::Dismissed => "DISMISSED",
            }
        }
    }
    /// Groups of an report.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReportGroup {
        CategoryUnspecified = 0,
        VpcNetwork = 1,
        NetworkServices = 2,
        KubernetesEngine = 3,
        HybridConnectivity = 4,
        ManagedServices = 5,
    }
    impl ReportGroup {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReportGroup::CategoryUnspecified => "CATEGORY_UNSPECIFIED",
                ReportGroup::VpcNetwork => "VPC_NETWORK",
                ReportGroup::NetworkServices => "NETWORK_SERVICES",
                ReportGroup::KubernetesEngine => "KUBERNETES_ENGINE",
                ReportGroup::HybridConnectivity => "HYBRID_CONNECTIVITY",
                ReportGroup::ManagedServices => "MANAGED_SERVICES",
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag="19")]
        IpUtilizationInfo(super::IpUtilizationInfo),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReportCauseCode {
    Unspecified = 0,
    /// VPC Basics
    RouteInvalidNextHopVmIpForwardingDisabled = 1,
    RouteInvalidNextHopVmDeleted = 2,
    RouteInvalidNextHopVmStopped = 3,
    RouteInvalidNextHopIlbMisconfigured = 4,
    RouteInvalidNextHopVpnTunnelDeleted = 5,
    RouteInvalidNextHopIlbBackendIpForwardingDisabled = 6,
    IpUtilizationIpAllocationRatioHigh = 20,
    /// Kubernetes Engine
    GkeNodeToControlPlaneBlockedByRoutingIssue = 201,
    GkeNodeToControlPlanePublicEndpointBlockedByEgressFirewall = 202,
    GkeNodeToControlPlanePrivateEndpointBlockedByEgressFirewall = 203,
    GkeControlPlaneToNodeBlockedByRoutingIssue = 211,
    GkeControlPlaneToNodeBlockedByIngressFirewallOnNode = 212,
    GkeIpUtilizationPodRangesAllocationHigh = 221,
    GkeIpUtilizationPodRangesAllocationLimitesAutoscaling = 222,
    /// Managed Services
    CloudSqlPrivateIpBlockedByEgressFirewall = 601,
    CloudSqlPrivateIpBlockedByRoutingIssue = 602,
    CloudSqlPrivateIpInstanceNotRunning = 603,
    /// Hybrid Connectivity
    DynamicRouteShadowedFullyShadowedBySubnetRoute = 801,
    DynamicRouteShadowedFullyShadowedByPeeringSubnetRoute = 802,
    DynamicRouteShadowedFullyShadowedByStaticRoute = 803,
    DynamicRouteShadowedFullyShadowedByPeeringStaticRoute = 804,
    DynamicRouteShadowedPartiallyShadowedBySubnetRoute = 805,
    DynamicRouteShadowedPartiallyShadowedByPeeringSubnetRoute = 806,
    DynamicRouteShadowedPartiallyShadowedByStaticRoute = 807,
    DynamicRouteShadowedPartiallyShadowedByPeeringStaticRoute = 808,
    /// Network Services
    LoadBalancerHealthCheckFirewallHealthCheckFirewallNotConfigured = 1001,
    LoadBalancerHealthCheckFirewallHealthCheckRangeBlocked = 1002,
    LoadBalancerHealthCheckFirewallFirewallConfigInconsistent = 1003,
    LoadBalancerHealthCheckFirewallHealthCheckRangePartiallyBlocked = 1004,
    LoadBalancerBestPracticesBackendServiceBalancingModeBreaksSessionAffinity = 1021,
    LoadBalancerBestPracticesBackendServiceHealthCheckPortMismatch = 1024,
}
impl ReportCauseCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReportCauseCode::Unspecified => "REPORT_CAUSE_CODE_UNSPECIFIED",
            ReportCauseCode::RouteInvalidNextHopVmIpForwardingDisabled => "ROUTE_INVALID_NEXT_HOP_VM_IP_FORWARDING_DISABLED",
            ReportCauseCode::RouteInvalidNextHopVmDeleted => "ROUTE_INVALID_NEXT_HOP_VM_DELETED",
            ReportCauseCode::RouteInvalidNextHopVmStopped => "ROUTE_INVALID_NEXT_HOP_VM_STOPPED",
            ReportCauseCode::RouteInvalidNextHopIlbMisconfigured => "ROUTE_INVALID_NEXT_HOP_ILB_MISCONFIGURED",
            ReportCauseCode::RouteInvalidNextHopVpnTunnelDeleted => "ROUTE_INVALID_NEXT_HOP_VPN_TUNNEL_DELETED",
            ReportCauseCode::RouteInvalidNextHopIlbBackendIpForwardingDisabled => "ROUTE_INVALID_NEXT_HOP_ILB_BACKEND_IP_FORWARDING_DISABLED",
            ReportCauseCode::IpUtilizationIpAllocationRatioHigh => "IP_UTILIZATION_IP_ALLOCATION_RATIO_HIGH",
            ReportCauseCode::GkeNodeToControlPlaneBlockedByRoutingIssue => "GKE_NODE_TO_CONTROL_PLANE_BLOCKED_BY_ROUTING_ISSUE",
            ReportCauseCode::GkeNodeToControlPlanePublicEndpointBlockedByEgressFirewall => "GKE_NODE_TO_CONTROL_PLANE_PUBLIC_ENDPOINT_BLOCKED_BY_EGRESS_FIREWALL",
            ReportCauseCode::GkeNodeToControlPlanePrivateEndpointBlockedByEgressFirewall => "GKE_NODE_TO_CONTROL_PLANE_PRIVATE_ENDPOINT_BLOCKED_BY_EGRESS_FIREWALL",
            ReportCauseCode::GkeControlPlaneToNodeBlockedByRoutingIssue => "GKE_CONTROL_PLANE_TO_NODE_BLOCKED_BY_ROUTING_ISSUE",
            ReportCauseCode::GkeControlPlaneToNodeBlockedByIngressFirewallOnNode => "GKE_CONTROL_PLANE_TO_NODE_BLOCKED_BY_INGRESS_FIREWALL_ON_NODE",
            ReportCauseCode::GkeIpUtilizationPodRangesAllocationHigh => "GKE_IP_UTILIZATION_POD_RANGES_ALLOCATION_HIGH",
            ReportCauseCode::GkeIpUtilizationPodRangesAllocationLimitesAutoscaling => "GKE_IP_UTILIZATION_POD_RANGES_ALLOCATION_LIMITES_AUTOSCALING",
            ReportCauseCode::CloudSqlPrivateIpBlockedByEgressFirewall => "CLOUD_SQL_PRIVATE_IP_BLOCKED_BY_EGRESS_FIREWALL",
            ReportCauseCode::CloudSqlPrivateIpBlockedByRoutingIssue => "CLOUD_SQL_PRIVATE_IP_BLOCKED_BY_ROUTING_ISSUE",
            ReportCauseCode::CloudSqlPrivateIpInstanceNotRunning => "CLOUD_SQL_PRIVATE_IP_INSTANCE_NOT_RUNNING",
            ReportCauseCode::DynamicRouteShadowedFullyShadowedBySubnetRoute => "DYNAMIC_ROUTE_SHADOWED_FULLY_SHADOWED_BY_SUBNET_ROUTE",
            ReportCauseCode::DynamicRouteShadowedFullyShadowedByPeeringSubnetRoute => "DYNAMIC_ROUTE_SHADOWED_FULLY_SHADOWED_BY_PEERING_SUBNET_ROUTE",
            ReportCauseCode::DynamicRouteShadowedFullyShadowedByStaticRoute => "DYNAMIC_ROUTE_SHADOWED_FULLY_SHADOWED_BY_STATIC_ROUTE",
            ReportCauseCode::DynamicRouteShadowedFullyShadowedByPeeringStaticRoute => "DYNAMIC_ROUTE_SHADOWED_FULLY_SHADOWED_BY_PEERING_STATIC_ROUTE",
            ReportCauseCode::DynamicRouteShadowedPartiallyShadowedBySubnetRoute => "DYNAMIC_ROUTE_SHADOWED_PARTIALLY_SHADOWED_BY_SUBNET_ROUTE",
            ReportCauseCode::DynamicRouteShadowedPartiallyShadowedByPeeringSubnetRoute => "DYNAMIC_ROUTE_SHADOWED_PARTIALLY_SHADOWED_BY_PEERING_SUBNET_ROUTE",
            ReportCauseCode::DynamicRouteShadowedPartiallyShadowedByStaticRoute => "DYNAMIC_ROUTE_SHADOWED_PARTIALLY_SHADOWED_BY_STATIC_ROUTE",
            ReportCauseCode::DynamicRouteShadowedPartiallyShadowedByPeeringStaticRoute => "DYNAMIC_ROUTE_SHADOWED_PARTIALLY_SHADOWED_BY_PEERING_STATIC_ROUTE",
            ReportCauseCode::LoadBalancerHealthCheckFirewallHealthCheckFirewallNotConfigured => "LOAD_BALANCER_HEALTH_CHECK_FIREWALL_HEALTH_CHECK_FIREWALL_NOT_CONFIGURED",
            ReportCauseCode::LoadBalancerHealthCheckFirewallHealthCheckRangeBlocked => "LOAD_BALANCER_HEALTH_CHECK_FIREWALL_HEALTH_CHECK_RANGE_BLOCKED",
            ReportCauseCode::LoadBalancerHealthCheckFirewallFirewallConfigInconsistent => "LOAD_BALANCER_HEALTH_CHECK_FIREWALL_FIREWALL_CONFIG_INCONSISTENT",
            ReportCauseCode::LoadBalancerHealthCheckFirewallHealthCheckRangePartiallyBlocked => "LOAD_BALANCER_HEALTH_CHECK_FIREWALL_HEALTH_CHECK_RANGE_PARTIALLY_BLOCKED",
            ReportCauseCode::LoadBalancerBestPracticesBackendServiceBalancingModeBreaksSessionAffinity => "LOAD_BALANCER_BEST_PRACTICES_BACKEND_SERVICE_BALANCING_MODE_BREAKS_SESSION_AFFINITY",
            ReportCauseCode::LoadBalancerBestPracticesBackendServiceHealthCheckPortMismatch => "LOAD_BALANCER_BEST_PRACTICES_BACKEND_SERVICE_HEALTH_CHECK_PORT_MISMATCH",
        }
    }
}
