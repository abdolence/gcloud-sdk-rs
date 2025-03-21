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

/// InstanceTemplate : Represents an Instance Template resource. Google Compute Engine has two Instance Template resources: * [Global](/compute/docs/reference/rest/v1/instanceTemplates) * [Regional](/compute/docs/reference/rest/v1/regionInstanceTemplates) You can reuse a global instance template in different regions whereas you can use a regional instance template in a specified region only. If you want to reduce cross-region dependency or achieve data residency, use a regional instance template. To create VMs, managed instance groups, and reservations, you can use either global or regional instance templates. For more information, read Instance Templates.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstanceTemplate {
    /// [Output Only] The creation timestamp for this instance template in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// An optional description of this resource. Provide this property when you create the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// [Output Only] A unique identifier for this instance template. The server defines this identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] The resource type, which is always compute#instanceTemplate for instance templates.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Box<models::InstanceProperties>>,
    /// [Output Only] URL of the region where the instance template resides. Only applicable for regional resources.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// [Output Only] The URL for this instance template. The server defines this URL.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// The source instance used to create the template. You can provide this as a partial or full URL to the resource. For example, the following are valid values: - https://www.googleapis.com/compute/v1/projects/project/zones/zone /instances/instance - projects/project/zones/zone/instances/instance
    #[serde(rename = "sourceInstance", skip_serializing_if = "Option::is_none")]
    pub source_instance: Option<String>,
    #[serde(
        rename = "sourceInstanceParams",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_instance_params: Option<Box<models::SourceInstanceParams>>,
}

impl InstanceTemplate {
    /// Represents an Instance Template resource. Google Compute Engine has two Instance Template resources: * [Global](/compute/docs/reference/rest/v1/instanceTemplates) * [Regional](/compute/docs/reference/rest/v1/regionInstanceTemplates) You can reuse a global instance template in different regions whereas you can use a regional instance template in a specified region only. If you want to reduce cross-region dependency or achieve data residency, use a regional instance template. To create VMs, managed instance groups, and reservations, you can use either global or regional instance templates. For more information, read Instance Templates.
    pub fn new() -> InstanceTemplate {
        InstanceTemplate {
            creation_timestamp: None,
            description: None,
            id: None,
            kind: None,
            name: None,
            properties: None,
            region: None,
            self_link: None,
            source_instance: None,
            source_instance_params: None,
        }
    }
}
