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

/// IdentitytoolkitRelyingpartyVerifyAssertionRequest : Request to verify the IDP assertion.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartyVerifyAssertionRequest {
    /// When it's true, automatically creates a new account if the user doesn't exist. When it's false, allows existing user to sign in normally and throws exception if the user doesn't exist.
    #[serde(rename = "autoCreate", skip_serializing_if = "Option::is_none")]
    pub auto_create: Option<bool>,
    /// GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration.
    #[serde(
        rename = "delegatedProjectNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub delegated_project_number: Option<String>,
    /// The GITKit token of the authenticated user.
    #[serde(rename = "idToken", skip_serializing_if = "Option::is_none")]
    pub id_token: Option<String>,
    /// Instance id token of the app.
    #[serde(rename = "instanceId", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// The GITKit token for the non-trusted IDP pending to be confirmed by the user.
    #[serde(rename = "pendingIdToken", skip_serializing_if = "Option::is_none")]
    pub pending_id_token: Option<String>,
    /// The post body if the request is a HTTP POST.
    #[serde(rename = "postBody", skip_serializing_if = "Option::is_none")]
    pub post_body: Option<String>,
    /// The URI to which the IDP redirects the user back. It may contain federated login result params added by the IDP.
    #[serde(rename = "requestUri", skip_serializing_if = "Option::is_none")]
    pub request_uri: Option<String>,
    /// Whether return 200 and IDP credential rather than throw exception when federated id is already linked.
    #[serde(
        rename = "returnIdpCredential",
        skip_serializing_if = "Option::is_none"
    )]
    pub return_idp_credential: Option<bool>,
    /// Whether to return refresh tokens.
    #[serde(rename = "returnRefreshToken", skip_serializing_if = "Option::is_none")]
    pub return_refresh_token: Option<bool>,
    /// Whether return sts id token and refresh token instead of gitkit token.
    #[serde(rename = "returnSecureToken", skip_serializing_if = "Option::is_none")]
    pub return_secure_token: Option<bool>,
    /// Session ID, which should match the one in previous createAuthUri request.
    #[serde(rename = "sessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from.
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// Tenant project number to be used for idp discovery.
    #[serde(
        rename = "tenantProjectNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub tenant_project_number: Option<String>,
}

impl IdentitytoolkitRelyingpartyVerifyAssertionRequest {
    /// Request to verify the IDP assertion.
    pub fn new() -> IdentitytoolkitRelyingpartyVerifyAssertionRequest {
        IdentitytoolkitRelyingpartyVerifyAssertionRequest {
            auto_create: None,
            delegated_project_number: None,
            id_token: None,
            instance_id: None,
            pending_id_token: None,
            post_body: None,
            request_uri: None,
            return_idp_credential: None,
            return_refresh_token: None,
            return_secure_token: None,
            session_id: None,
            tenant_id: None,
            tenant_project_number: None,
        }
    }
}
