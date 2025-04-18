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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetHttpsProxiesSetCertificateMapRequest {
    /// URL of the Certificate Map to associate with this TargetHttpsProxy. Accepted format is //certificatemanager.googleapis.com/projects/{project }/locations/{location}/certificateMaps/{resourceName}.
    #[serde(rename = "certificateMap", skip_serializing_if = "Option::is_none")]
    pub certificate_map: Option<String>,
}

impl TargetHttpsProxiesSetCertificateMapRequest {
    pub fn new() -> TargetHttpsProxiesSetCertificateMapRequest {
        TargetHttpsProxiesSetCertificateMapRequest {
            certificate_map: None,
        }
    }
}
