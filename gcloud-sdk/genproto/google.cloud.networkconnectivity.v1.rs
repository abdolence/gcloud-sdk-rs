/// Represents the metadata of the long-running operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have been cancelled successfully
    /// have \[Operation.error][\] value with a \[google.rpc.Status.code][google.rpc.Status.code\] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// A Network Connectivity Center hub is a collection of spokes. A single hub
/// can contain spokes from multiple regions. However, if any of a hub's spokes
/// use the data transfer feature, the resources associated with those spokes
/// must all reside in the same VPC network. Spokes that do not use data
/// transfer can be associated with any VPC network in your project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hub {
    /// Immutable. The name of the hub. Hub names must be unique. They use the
    /// following form:
    ///      `projects/{project_number}/locations/global/hubs/{hub_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time the hub was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the hub was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional labels in key:value format. For more information about labels, see
    /// [Requirements for
    /// labels](<https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements>).
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// An optional description of the hub.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The Google-generated UUID for the hub. This value is unique
    /// across all hub resources. If a hub is deleted and another with the same
    /// name is created, the new hub is assigned a different unique_id.
    #[prost(string, tag = "8")]
    pub unique_id: ::prost::alloc::string::String,
    /// Output only. The current lifecycle state of this hub.
    #[prost(enumeration = "State", tag = "9")]
    pub state: i32,
    /// The VPC networks associated with this hub's spokes.
    ///
    /// This field is read-only. Network Connectivity Center automatically
    /// populates it based on the set of spokes attached to the hub.
    #[prost(message, repeated, tag = "10")]
    pub routing_vpcs: ::prost::alloc::vec::Vec<RoutingVpc>,
}
/// RoutingVPC contains information about the VPC networks associated
/// with the spokes of a Network Connectivity Center hub.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingVpc {
    /// The URI of the VPC network.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// Output only. If true, indicates that this VPC network is currently
    /// associated with spokes that use the data transfer feature (spokes where the
    /// site_to_site_data_transfer field is set to true). If you create new spokes
    /// that use data transfer, they must be associated with this VPC network. At
    /// most, one VPC network will have this field set to true.
    #[prost(bool, tag = "2")]
    pub required_for_new_site_to_site_data_transfer_spokes: bool,
}
/// A Network Connectivity Center spoke represents a connection between your
/// Google Cloud network resources and a non-Google-Cloud network.
///
/// When you create a spoke, you associate it with a hub. You must also identify
/// a value for exactly one of the following fields:
///
/// * linked_vpn_tunnels
/// * linked_interconnect_attachments
/// * linked_router_appliance_instances
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spoke {
    /// Immutable. The name of the spoke. Spoke names must be unique. They use the
    /// following form:
    ///      `projects/{project_number}/locations/{region}/spokes/{spoke_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time the spoke was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the spoke was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional labels in key:value format. For more information about labels, see
    /// [Requirements for
    /// labels](<https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements>).
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// An optional description of the spoke.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Immutable. The name of the hub that this spoke is attached to.
    #[prost(string, tag = "6")]
    pub hub: ::prost::alloc::string::String,
    /// VPN tunnels that are associated with the spoke.
    #[prost(message, optional, tag = "17")]
    pub linked_vpn_tunnels: ::core::option::Option<LinkedVpnTunnels>,
    /// VLAN attachments that are associated with the spoke.
    #[prost(message, optional, tag = "18")]
    pub linked_interconnect_attachments: ::core::option::Option<
        LinkedInterconnectAttachments,
    >,
    /// Router appliance instances that are associated with the spoke.
    #[prost(message, optional, tag = "19")]
    pub linked_router_appliance_instances: ::core::option::Option<
        LinkedRouterApplianceInstances,
    >,
    /// Output only. The Google-generated UUID for the spoke. This value is unique
    /// across all spoke resources. If a spoke is deleted and another with the same
    /// name is created, the new spoke is assigned a different unique_id.
    #[prost(string, tag = "11")]
    pub unique_id: ::prost::alloc::string::String,
    /// Output only. The current lifecycle state of this spoke.
    #[prost(enumeration = "State", tag = "15")]
    pub state: i32,
}
/// Request for
/// \[HubService.ListHubs][google.cloud.networkconnectivity.v1.HubService.ListHubs\]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHubsRequest {
    /// Required. The parent resource's name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results per page that should be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that filters the results listed in the response.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sort the results by a certain order.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for
/// \[HubService.ListHubs][google.cloud.networkconnectivity.v1.HubService.ListHubs\]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHubsResponse {
    /// The requested hubs.
    #[prost(message, repeated, tag = "1")]
    pub hubs: ::prost::alloc::vec::Vec<Hub>,
    /// The next pagination token in the List response. It should be used as
    /// page_token for the following request. An empty value means no more result.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for
/// \[HubService.GetHub][google.cloud.networkconnectivity.v1.HubService.GetHub\]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHubRequest {
    /// Required. The name of the hub resource to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for
/// \[HubService.CreateHub][google.cloud.networkconnectivity.v1.HubService.CreateHub\]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateHubRequest {
    /// Required. The parent resource.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A unique identifier for the hub.
    #[prost(string, tag = "2")]
    pub hub_id: ::prost::alloc::string::String,
    /// Required. The initial values for a new hub.
    #[prost(message, optional, tag = "3")]
    pub hub: ::core::option::Option<Hub>,
    /// Optional. A unique request ID (optional). If you specify this ID, you can
    /// use it in cases when you need to retry your request. When you need to
    /// retry, this ID lets the server know that it can ignore the request if it
    /// has already been completed. The server guarantees that for at least 60
    /// minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for
/// \[HubService.UpdateHub][google.cloud.networkconnectivity.v1.HubService.UpdateHub\]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHubRequest {
    /// Optional. In the case of an update to an existing hub, field mask is used
    /// to specify the fields to be overwritten. The fields specified in the
    /// update_mask are relative to the resource, not the full request. A field is
    /// overwritten if it is in the mask. If the user does not provide a mask, then
    /// all fields are overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The state that the hub should be in after the update.
    #[prost(message, optional, tag = "2")]
    pub hub: ::core::option::Option<Hub>,
    /// Optional. A unique request ID (optional). If you specify this ID, you can
    /// use it in cases when you need to retry your request. When you need to
    /// retry, this ID lets the server know that it can ignore the request if it
    /// has already been completed. The server guarantees that for at least 60
    /// minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// The request for
/// \[HubService.DeleteHub][google.cloud.networkconnectivity.v1.HubService.DeleteHub\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteHubRequest {
    /// Required. The name of the hub to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A unique request ID (optional). If you specify this ID, you can
    /// use it in cases when you need to retry your request. When you need to
    /// retry, this ID lets the server know that it can ignore the request if it
    /// has already been completed. The server guarantees that for at least 60
    /// minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// The request for
/// \[HubService.ListSpokes][google.cloud.networkconnectivity.v1.HubService.ListSpokes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpokesRequest {
    /// Required. The parent resource.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results per page that should be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that filters the results listed in the response.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sort the results by a certain order.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response for
/// \[HubService.ListSpokes][google.cloud.networkconnectivity.v1.HubService.ListSpokes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpokesResponse {
    /// The requested spokes.
    #[prost(message, repeated, tag = "1")]
    pub spokes: ::prost::alloc::vec::Vec<Spoke>,
    /// The next pagination token in the List response. It should be used as
    /// page_token for the following request. An empty value means no more result.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request for
/// \[HubService.GetSpoke][google.cloud.networkconnectivity.v1.HubService.GetSpoke\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSpokeRequest {
    /// Required. The name of the spoke resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for
/// \[HubService.CreateSpoke][google.cloud.networkconnectivity.v1.HubService.CreateSpoke\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSpokeRequest {
    /// Required. The parent resource.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Unique id for the spoke to create.
    #[prost(string, tag = "2")]
    pub spoke_id: ::prost::alloc::string::String,
    /// Required. The initial values for a new spoke.
    #[prost(message, optional, tag = "3")]
    pub spoke: ::core::option::Option<Spoke>,
    /// Optional. A unique request ID (optional). If you specify this ID, you can
    /// use it in cases when you need to retry your request. When you need to
    /// retry, this ID lets the server know that it can ignore the request if it
    /// has already been completed. The server guarantees that for at least 60
    /// minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for
/// \[HubService.UpdateSpoke][google.cloud.networkconnectivity.v1.HubService.UpdateSpoke\]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSpokeRequest {
    /// Optional. In the case of an update to an existing spoke, field mask is used
    /// to specify the fields to be overwritten. The fields specified in the
    /// update_mask are relative to the resource, not the full request. A field is
    /// overwritten if it is in the mask. If the user does not provide a mask, then
    /// all fields are overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The state that the spoke should be in after the update.
    #[prost(message, optional, tag = "2")]
    pub spoke: ::core::option::Option<Spoke>,
    /// Optional. A unique request ID (optional). If you specify this ID, you can
    /// use it in cases when you need to retry your request. When you need to
    /// retry, this ID lets the server know that it can ignore the request if it
    /// has already been completed. The server guarantees that for at least 60
    /// minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// The request for
/// \[HubService.DeleteSpoke][google.cloud.networkconnectivity.v1.HubService.DeleteSpoke\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSpokeRequest {
    /// Required. The name of the spoke to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A unique request ID (optional). If you specify this ID, you can
    /// use it in cases when you need to retry your request. When you need to
    /// retry, this ID lets the server know that it can ignore the request if it
    /// has already been completed. The server guarantees that for at least 60
    /// minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// A collection of Cloud VPN tunnel resources. These resources should be
/// redundant HA VPN tunnels that all advertise the same prefixes to Google
/// Cloud. Alternatively, in a passive/active configuration, all tunnels
/// should be capable of advertising the same prefixes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkedVpnTunnels {
    /// The URIs of linked VPN tunnel resources.
    #[prost(string, repeated, tag = "1")]
    pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A value that controls whether site-to-site data transfer is enabled for
    /// these resources. Data transfer is available only in [supported
    /// locations](<https://cloud.google.com/network-connectivity/docs/network-connectivity-center/concepts/locations>).
    #[prost(bool, tag = "2")]
    pub site_to_site_data_transfer: bool,
    /// Output only. The VPC network where these VPN tunnels are located.
    #[prost(string, tag = "3")]
    pub vpc_network: ::prost::alloc::string::String,
}
/// A collection of VLAN attachment resources. These resources should
/// be redundant attachments that all advertise the same prefixes to Google
/// Cloud. Alternatively, in active/passive configurations, all attachments
/// should be capable of advertising the same prefixes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkedInterconnectAttachments {
    /// The URIs of linked interconnect attachment resources
    #[prost(string, repeated, tag = "1")]
    pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A value that controls whether site-to-site data transfer is enabled for
    /// these resources. Data transfer is available only in [supported
    /// locations](<https://cloud.google.com/network-connectivity/docs/network-connectivity-center/concepts/locations>).
    #[prost(bool, tag = "2")]
    pub site_to_site_data_transfer: bool,
    /// Output only. The VPC network where these VLAN attachments are located.
    #[prost(string, tag = "3")]
    pub vpc_network: ::prost::alloc::string::String,
}
/// A collection of router appliance instances. If you configure multiple router
/// appliance instances to receive data from the same set of sites outside of
/// Google Cloud, we recommend that you associate those instances with the same
/// spoke.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkedRouterApplianceInstances {
    /// The list of router appliance instances.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<RouterApplianceInstance>,
    /// A value that controls whether site-to-site data transfer is enabled for
    /// these resources. Data transfer is available only in [supported
    /// locations](<https://cloud.google.com/network-connectivity/docs/network-connectivity-center/concepts/locations>).
    #[prost(bool, tag = "2")]
    pub site_to_site_data_transfer: bool,
    /// Output only. The VPC network where these router appliance instances are
    /// located.
    #[prost(string, tag = "3")]
    pub vpc_network: ::prost::alloc::string::String,
}
/// A router appliance instance is a Compute Engine virtual machine (VM) instance
/// that acts as a BGP speaker. A router appliance instance is specified by the
/// URI of the VM and the internal IP address of one of the VM's network
/// interfaces.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouterApplianceInstance {
    /// The URI of the VM.
    #[prost(string, tag = "1")]
    pub virtual_machine: ::prost::alloc::string::String,
    /// The IP address on the VM to use for peering.
    #[prost(string, tag = "3")]
    pub ip_address: ::prost::alloc::string::String,
}
/// Metadata about locations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMetadata {
    /// List of supported features
    #[prost(enumeration = "LocationFeature", repeated, tag = "1")]
    pub location_features: ::prost::alloc::vec::Vec<i32>,
}
/// Supported features for a location
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LocationFeature {
    /// No publicly supported feature in this location
    Unspecified = 0,
    /// Site-to-cloud spokes are supported in this location
    SiteToCloudSpokes = 1,
    /// Site-to-site spokes are supported in this location
    SiteToSiteSpokes = 2,
}
impl LocationFeature {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LocationFeature::Unspecified => "LOCATION_FEATURE_UNSPECIFIED",
            LocationFeature::SiteToCloudSpokes => "SITE_TO_CLOUD_SPOKES",
            LocationFeature::SiteToSiteSpokes => "SITE_TO_SITE_SPOKES",
        }
    }
}
/// The State enum represents the lifecycle stage of a Network Connectivity
/// Center resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    /// No state information available
    Unspecified = 0,
    /// The resource's create operation is in progress
    Creating = 1,
    /// The resource is active
    Active = 2,
    /// The resource's Delete operation is in progress
    Deleting = 3,
    /// The resource's Update operation is in progress
    Updating = 6,
}
impl State {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            State::Unspecified => "STATE_UNSPECIFIED",
            State::Creating => "CREATING",
            State::Active => "ACTIVE",
            State::Deleting => "DELETING",
            State::Updating => "UPDATING",
        }
    }
}
/// Generated client implementations.
pub mod hub_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Network Connectivity Center is a hub-and-spoke abstraction for network
    /// connectivity management in Google Cloud. It reduces operational complexity
    /// through a simple, centralized connectivity management model.
    #[derive(Debug, Clone)]
    pub struct HubServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl HubServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> HubServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> HubServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            HubServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Lists the Network Connectivity Center hubs associated with a given project.
        pub async fn list_hubs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListHubsRequest>,
        ) -> Result<tonic::Response<super::ListHubsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkconnectivity.v1.HubService/ListHubs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details about a Network Connectivity Center hub.
        pub async fn get_hub(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHubRequest>,
        ) -> Result<tonic::Response<super::Hub>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkconnectivity.v1.HubService/GetHub",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Network Connectivity Center hub in the specified project.
        pub async fn create_hub(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateHubRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkconnectivity.v1.HubService/CreateHub",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the description and/or labels of a Network Connectivity Center
        /// hub.
        pub async fn update_hub(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateHubRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkconnectivity.v1.HubService/UpdateHub",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a Network Connectivity Center hub.
        pub async fn delete_hub(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteHubRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkconnectivity.v1.HubService/DeleteHub",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists the Network Connectivity Center spokes in a specified project and
        /// location.
        pub async fn list_spokes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSpokesRequest>,
        ) -> Result<tonic::Response<super::ListSpokesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkconnectivity.v1.HubService/ListSpokes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details about a Network Connectivity Center spoke.
        pub async fn get_spoke(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSpokeRequest>,
        ) -> Result<tonic::Response<super::Spoke>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkconnectivity.v1.HubService/GetSpoke",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a Network Connectivity Center spoke.
        pub async fn create_spoke(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSpokeRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkconnectivity.v1.HubService/CreateSpoke",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a Network Connectivity Center spoke.
        pub async fn update_spoke(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSpokeRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkconnectivity.v1.HubService/UpdateSpoke",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a Network Connectivity Center spoke.
        pub async fn delete_spoke(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSpokeRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkconnectivity.v1.HubService/DeleteSpoke",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Policy Based Routes (PBR) are more powerful routes that allows GCP customers
/// to route their L4 network traffic based on not just destination IP, but also
/// source IP, protocol and more. A PBR always take precedence when it conflicts
/// with other types of routes.
/// Next id: 19
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyBasedRoute {
    /// Immutable. A unique name of the resource in the form of
    /// `projects/{project_number}/locations/global/PolicyBasedRoutes/{policy_based_route_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Time when the PolicyBasedRoute was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the PolicyBasedRoute was updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User-defined labels.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. An optional description of this resource. Provide this field when you
    /// create the resource.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Required. Fully-qualified URL of the network that this route applies to. e.g.
    /// projects/my-project/global/networks/my-network.
    #[prost(string, tag = "6")]
    pub network: ::prost::alloc::string::String,
    /// Required. The filter to match L4 traffic.
    #[prost(message, optional, tag = "10")]
    pub filter: ::core::option::Option<policy_based_route::Filter>,
    /// Optional. The priority of this policy based route. Priority is used to break ties in
    /// cases where there are more than one matching policy based routes found. In
    /// cases where multiple policy based routes are matched, the one with the
    /// lowest-numbered priority value wins. The default value is 1000. The
    /// priority value must be from 1 to 65535, inclusive.
    #[prost(int32, tag = "11")]
    pub priority: i32,
    /// Output only. If potential misconfigurations are detected for this route,
    /// this field will be populated with warning messages.
    #[prost(message, repeated, tag = "14")]
    pub warnings: ::prost::alloc::vec::Vec<policy_based_route::Warnings>,
    /// Output only. Server-defined fully-qualified URL for this resource.
    #[prost(string, tag = "15")]
    pub self_link: ::prost::alloc::string::String,
    /// Output only. Type of this resource. Always networkconnectivity#policyBasedRoute for
    /// Policy Based Route resources.
    #[prost(string, tag = "16")]
    pub kind: ::prost::alloc::string::String,
    /// Target specifies network endpoints to which this policy based route applies
    /// to. If none of the target is specified, the PBR will be installed on all
    /// network endpoints (e.g. VMs, VPNs, and Interconnects) in the VPC.
    #[prost(oneof = "policy_based_route::Target", tags = "18, 9")]
    pub target: ::core::option::Option<policy_based_route::Target>,
    #[prost(oneof = "policy_based_route::NextHop", tags = "12")]
    pub next_hop: ::core::option::Option<policy_based_route::NextHop>,
}
/// Nested message and enum types in `PolicyBasedRoute`.
pub mod policy_based_route {
    /// VM instances to which this policy based route applies to.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VirtualMachine {
        /// Optional. A list of VM instance tags to which this policy based route applies to.
        /// VM instances that have ANY of tags specified here will install this
        /// PBR.
        #[prost(string, repeated, tag = "1")]
        pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// InterconnectAttachment to which this route applies to.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InterconnectAttachment {
        /// Optional. Cloud region to install this policy based route on interconnect
        /// attachment. Use `all` to install it on all interconnect attachments.
        #[prost(string, tag = "1")]
        pub region: ::prost::alloc::string::String,
    }
    /// Filter matches L4 traffic.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Filter {
        /// Optional. The IP protocol that this policy based route applies to. Valid values are
        /// 'TCP', 'UDP', and 'ALL'. Default is 'ALL'.
        #[prost(string, tag = "1")]
        pub ip_protocol: ::prost::alloc::string::String,
        /// Optional. The source IP range of outgoing packets that this policy based route
        /// applies to. Default is "0.0.0.0/0" if protocol version is IPv4.
        #[prost(string, tag = "2")]
        pub src_range: ::prost::alloc::string::String,
        /// Optional. The destination IP range of outgoing packets that this policy based route
        /// applies to. Default is "0.0.0.0/0" if protocol version is IPv4.
        #[prost(string, tag = "3")]
        pub dest_range: ::prost::alloc::string::String,
        /// Required. Internet protocol versions this policy based route applies to. For this
        /// version, only IPV4 is supported.
        #[prost(enumeration = "filter::ProtocolVersion", tag = "6")]
        pub protocol_version: i32,
    }
    /// Nested message and enum types in `Filter`.
    pub mod filter {
        /// The internet protocol version.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum ProtocolVersion {
            /// Default value.
            Unspecified = 0,
            /// The PBR is for IPv4 internet protocol traffic.
            Ipv4 = 1,
        }
        impl ProtocolVersion {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ProtocolVersion::Unspecified => "PROTOCOL_VERSION_UNSPECIFIED",
                    ProtocolVersion::Ipv4 => "IPV4",
                }
            }
        }
    }
    /// Informational warning message.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Warnings {
        /// Output only. A warning code, if applicable.
        #[prost(enumeration = "warnings::Code", tag = "1")]
        pub code: i32,
        /// Output only. Metadata about this warning in key: value format. The key should provides
        /// more detail on the warning being returned. For example, for warnings
        /// where there are no results in a list request for a particular zone, this
        /// key might be scope and the key value might be the zone name. Other
        /// examples might be a key indicating a deprecated resource and a suggested
        /// replacement.
        #[prost(map = "string, string", tag = "2")]
        pub data: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// Output only. A human-readable description of the warning code.
        #[prost(string, tag = "3")]
        pub warning_message: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Warnings`.
    pub mod warnings {
        /// Warning code for Policy Based Routing. Expect to add values in the
        /// future.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum Code {
            /// Default value.
            WarningUnspecified = 0,
            /// The policy based route is not active and functioning. Common causes are
            /// the dependent network was deleted or the resource project was turned
            /// off.
            ResourceNotActive = 1,
            /// The policy based route is being modified (e.g. created/deleted) at this
            /// time.
            ResourceBeingModified = 2,
        }
        impl Code {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Code::WarningUnspecified => "WARNING_UNSPECIFIED",
                    Code::ResourceNotActive => "RESOURCE_NOT_ACTIVE",
                    Code::ResourceBeingModified => "RESOURCE_BEING_MODIFIED",
                }
            }
        }
    }
    /// Target specifies network endpoints to which this policy based route applies
    /// to. If none of the target is specified, the PBR will be installed on all
    /// network endpoints (e.g. VMs, VPNs, and Interconnects) in the VPC.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// Optional. VM instances to which this policy based route applies to.
        #[prost(message, tag = "18")]
        VirtualMachine(VirtualMachine),
        /// Optional. The interconnect attachments to which this route applies to.
        #[prost(message, tag = "9")]
        InterconnectAttachment(InterconnectAttachment),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum NextHop {
        /// Optional. The IP of a global access enabled L4 ILB that should be the next hop to
        /// handle matching packets. For this version, only next_hop_ilb_ip is
        /// supported.
        #[prost(string, tag = "12")]
        NextHopIlbIp(::prost::alloc::string::String),
    }
}
/// Request for \[PolicyBasedRouting.ListPolicyBasedRoutes][\] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPolicyBasedRoutesRequest {
    /// Required. The parent resource's name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results per page that should be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters the results listed in the response.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sort the results by a certain order.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for \[PolicyBasedRouting.ListPolicyBasedRoutes][\] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPolicyBasedRoutesResponse {
    /// Policy based routes to be returned.
    #[prost(message, repeated, tag = "1")]
    pub policy_based_routes: ::prost::alloc::vec::Vec<PolicyBasedRoute>,
    /// The next pagination token in the List response. It should be used as
    /// page_token for the following request. An empty value means no more result.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for \[PolicyBasedRouting.GetPolicyBasedRoute][\] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicyBasedRouteRequest {
    /// Required. Name of the PolicyBasedRoute resource to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for \[PolicyBasedRouting.CreatePolicyBasedRoute][\] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePolicyBasedRouteRequest {
    /// Required. The parent resource's name of the PolicyBasedRoute.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Unique id for the Policy Based Route to create.
    #[prost(string, tag = "2")]
    pub policy_based_route_id: ::prost::alloc::string::String,
    /// Required. Initial values for a new Policy Based Route.
    #[prost(message, optional, tag = "3")]
    pub policy_based_route: ::core::option::Option<PolicyBasedRoute>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for \[PolicyBasedRouting.DeletePolicyBasedRoute][\] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePolicyBasedRouteRequest {
    /// Required. Name of the PolicyBasedRoute resource to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod policy_based_routing_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Policy-Based Routing allows GCP customers to specify flexibile routing
    /// policies for Layer 4 traffic traversing through the connected service.
    #[derive(Debug, Clone)]
    pub struct PolicyBasedRoutingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PolicyBasedRoutingServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PolicyBasedRoutingServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PolicyBasedRoutingServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            PolicyBasedRoutingServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Lists PolicyBasedRoutes in a given project and location.
        pub async fn list_policy_based_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPolicyBasedRoutesRequest>,
        ) -> Result<
            tonic::Response<super::ListPolicyBasedRoutesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkconnectivity.v1.PolicyBasedRoutingService/ListPolicyBasedRoutes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single PolicyBasedRoute.
        pub async fn get_policy_based_route(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPolicyBasedRouteRequest>,
        ) -> Result<tonic::Response<super::PolicyBasedRoute>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkconnectivity.v1.PolicyBasedRoutingService/GetPolicyBasedRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new PolicyBasedRoute in a given project and location.
        pub async fn create_policy_based_route(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePolicyBasedRouteRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkconnectivity.v1.PolicyBasedRoutingService/CreatePolicyBasedRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single PolicyBasedRoute.
        pub async fn delete_policy_based_route(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePolicyBasedRouteRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkconnectivity.v1.PolicyBasedRoutingService/DeletePolicyBasedRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
