use serde::{Deserialize, Serialize}; /*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// InstancesRotateServerCaRequest : Rotate server CA request.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstancesRotateServerCaRequest {
    #[serde(
        rename = "rotateServerCaContext",
        skip_serializing_if = "Option::is_none"
    )]
    pub rotate_server_ca_context:
        Option<Box<crate::google_rest_apis::sqladmin_v1::models::RotateServerCaContext>>,
}

impl InstancesRotateServerCaRequest {
    /// Rotate server CA request.
    pub fn new() -> InstancesRotateServerCaRequest {
        InstancesRotateServerCaRequest {
            rotate_server_ca_context: None,
        }
    }
}
