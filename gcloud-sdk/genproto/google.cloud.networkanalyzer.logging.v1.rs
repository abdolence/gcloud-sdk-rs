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
    /// Type of an report.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        ReportTypeUnspecified = 0,
        Info = 1,
        Warning = 2,
        Error = 3,
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
}
