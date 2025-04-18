use serde::{Deserialize, Serialize}; /*
                                      * Cloud DNS API
                                      *
                                      * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::dns_v1::models;

/// ManagedZone : A zone is a subtree of the DNS namespace under one administrative responsibility. A ManagedZone is a resource that represents a DNS zone hosted by the Cloud DNS service.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedZone {
    #[serde(rename = "cloudLoggingConfig", skip_serializing_if = "Option::is_none")]
    pub cloud_logging_config: Option<Box<models::ManagedZoneCloudLoggingConfig>>,
    /// The time that this resource was created on the server. This is in RFC3339 text format. Output only.
    #[serde(rename = "creationTime", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the managed zone's function.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The DNS name of this managed zone, for instance \"example.com.\".
    #[serde(rename = "dnsName", skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[serde(rename = "dnssecConfig", skip_serializing_if = "Option::is_none")]
    pub dnssec_config: Option<Box<models::ManagedZoneDnsSecConfig>>,
    #[serde(rename = "forwardingConfig", skip_serializing_if = "Option::is_none")]
    pub forwarding_config: Option<Box<models::ManagedZoneForwardingConfig>>,
    /// Unique identifier for the resource; defined by the server (output only)
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// User labels.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// User assigned name for this resource. Must be unique within the project. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optionally specifies the NameServerSet for this ManagedZone. A NameServerSet is a set of DNS name servers that all host the same ManagedZones. Most users leave this field unset. If you need to use this field, contact your account team.
    #[serde(rename = "nameServerSet", skip_serializing_if = "Option::is_none")]
    pub name_server_set: Option<String>,
    /// Delegate your managed_zone to these virtual name servers; defined by the server (output only)
    #[serde(rename = "nameServers", skip_serializing_if = "Option::is_none")]
    pub name_servers: Option<Vec<String>>,
    #[serde(rename = "peeringConfig", skip_serializing_if = "Option::is_none")]
    pub peering_config: Option<Box<models::ManagedZonePeeringConfig>>,
    #[serde(
        rename = "privateVisibilityConfig",
        skip_serializing_if = "Option::is_none"
    )]
    pub private_visibility_config: Option<Box<models::ManagedZonePrivateVisibilityConfig>>,
    #[serde(
        rename = "reverseLookupConfig",
        skip_serializing_if = "Option::is_none"
    )]
    pub reverse_lookup_config: Option<Box<models::ManagedZoneReverseLookupConfig>>,
    #[serde(
        rename = "serviceDirectoryConfig",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_directory_config: Option<Box<models::ManagedZoneServiceDirectoryConfig>>,
    /// The zone's visibility: public zones are exposed to the Internet, while private zones are visible only to Virtual Private Cloud resources.
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

impl ManagedZone {
    /// A zone is a subtree of the DNS namespace under one administrative responsibility. A ManagedZone is a resource that represents a DNS zone hosted by the Cloud DNS service.
    pub fn new() -> ManagedZone {
        ManagedZone {
            cloud_logging_config: None,
            creation_time: None,
            description: None,
            dns_name: None,
            dnssec_config: None,
            forwarding_config: None,
            id: None,
            kind: None,
            labels: None,
            name: None,
            name_server_set: None,
            name_servers: None,
            peering_config: None,
            private_visibility_config: None,
            reverse_lookup_config: None,
            service_directory_config: None,
            visibility: None,
        }
    }
}
/// The zone's visibility: public zones are exposed to the Internet, while private zones are visible only to Virtual Private Cloud resources.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Visibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

impl Default for Visibility {
    fn default() -> Visibility {
        Self::Public
    }
}
