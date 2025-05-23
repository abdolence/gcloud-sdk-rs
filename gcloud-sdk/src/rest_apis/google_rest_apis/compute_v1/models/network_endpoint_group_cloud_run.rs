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

/// NetworkEndpointGroupCloudRun : Configuration for a Cloud Run network endpoint group (NEG). The service must be provided explicitly or in the URL mask. The tag is optional, may be provided explicitly or in the URL mask. Note: Cloud Run service must be in the same project and located in the same region as the Serverless NEG.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkEndpointGroupCloudRun {
    /// Cloud Run service is the main resource of Cloud Run. The service must be 1-63 characters long, and comply with RFC1035. Example value: \"run-service\".
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// Optional Cloud Run tag represents the \"named-revision\" to provide additional fine-grained traffic routing information. The tag must be 1-63 characters long, and comply with RFC1035. Example value: \"revision-0010\".
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// An URL mask is one of the main components of the Cloud Function. A template to parse <service> and <tag> fields from a request URL. URL mask allows for routing to multiple Run services without having to create multiple network endpoint groups and backend services. For example, request URLs foo1.domain.com/bar1 and foo1.domain.com/bar2 can be backed by the same Serverless Network Endpoint Group (NEG) with URL mask <tag>.domain.com/<service>. The URL mask will parse them to { service=\"bar1\", tag=\"foo1\" } and { service=\"bar2\", tag=\"foo2\" } respectively.
    #[serde(rename = "urlMask", skip_serializing_if = "Option::is_none")]
    pub url_mask: Option<String>,
}

impl NetworkEndpointGroupCloudRun {
    /// Configuration for a Cloud Run network endpoint group (NEG). The service must be provided explicitly or in the URL mask. The tag is optional, may be provided explicitly or in the URL mask. Note: Cloud Run service must be in the same project and located in the same region as the Serverless NEG.
    pub fn new() -> NetworkEndpointGroupCloudRun {
        NetworkEndpointGroupCloudRun {
            service: None,
            tag: None,
            url_mask: None,
        }
    }
}
