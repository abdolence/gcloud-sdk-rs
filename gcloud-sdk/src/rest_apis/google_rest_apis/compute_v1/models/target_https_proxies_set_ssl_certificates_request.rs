use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TargetHttpsProxiesSetSslCertificatesRequest {
    /// New set of SslCertificate resources to associate with this TargetHttpsProxy resource. At least one SSL certificate must be specified. Currently, you may specify up to 15 SSL certificates.
    #[serde(rename = "sslCertificates", skip_serializing_if = "Option::is_none")]
    pub ssl_certificates: Option<Vec<String>>,
}

impl TargetHttpsProxiesSetSslCertificatesRequest {
    pub fn new() -> TargetHttpsProxiesSetSslCertificatesRequest {
        TargetHttpsProxiesSetSslCertificatesRequest {
            ssl_certificates: None,
        }
    }
}