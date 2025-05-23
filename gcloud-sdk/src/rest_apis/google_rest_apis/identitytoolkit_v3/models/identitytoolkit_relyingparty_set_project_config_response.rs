use serde::{Deserialize, Serialize}; /*
                                      * Google Identity Toolkit API
                                      *
                                      * Help the third party sites to implement federated login.
                                      *
                                      * The version of the OpenAPI document: v3
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::identitytoolkit_v3::models;

/// IdentitytoolkitRelyingpartySetProjectConfigResponse : Response of setting the project configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartySetProjectConfigResponse {
    /// Project ID of the relying party.
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl IdentitytoolkitRelyingpartySetProjectConfigResponse {
    /// Response of setting the project configuration.
    pub fn new() -> IdentitytoolkitRelyingpartySetProjectConfigResponse {
        IdentitytoolkitRelyingpartySetProjectConfigResponse { project_id: None }
    }
}
