use serde::{Deserialize, Serialize}; /*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::sqladmin_v1::models;

/// OnPremisesConfiguration : On-premises instance configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnPremisesConfiguration {
    /// PEM representation of the trusted CA's x509 certificate.
    #[serde(rename = "caCertificate", skip_serializing_if = "Option::is_none")]
    pub ca_certificate: Option<String>,
    /// PEM representation of the replica's x509 certificate.
    #[serde(rename = "clientCertificate", skip_serializing_if = "Option::is_none")]
    pub client_certificate: Option<String>,
    /// PEM representation of the replica's private key. The corresponsing public key is encoded in the client's certificate.
    #[serde(rename = "clientKey", skip_serializing_if = "Option::is_none")]
    pub client_key: Option<String>,
    /// The dump file to create the Cloud SQL replica.
    #[serde(rename = "dumpFilePath", skip_serializing_if = "Option::is_none")]
    pub dump_file_path: Option<String>,
    /// The host and port of the on-premises instance in host:port format
    #[serde(rename = "hostPort", skip_serializing_if = "Option::is_none")]
    pub host_port: Option<String>,
    /// This is always `sql#onPremisesConfiguration`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The password for connecting to on-premises instance.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "sourceInstance", skip_serializing_if = "Option::is_none")]
    pub source_instance: Option<Box<models::InstanceReference>>,
    /// The username for connecting to on-premises instance.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl OnPremisesConfiguration {
    /// On-premises instance configuration.
    pub fn new() -> OnPremisesConfiguration {
        OnPremisesConfiguration {
            ca_certificate: None,
            client_certificate: None,
            client_key: None,
            dump_file_path: None,
            host_port: None,
            kind: None,
            password: None,
            source_instance: None,
            username: None,
        }
    }
}
