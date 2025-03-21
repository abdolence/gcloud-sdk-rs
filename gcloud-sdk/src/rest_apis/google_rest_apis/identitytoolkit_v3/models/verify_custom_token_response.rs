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

/// VerifyCustomTokenResponse : Response from verifying a custom token
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VerifyCustomTokenResponse {
    /// If idToken is STS id token, then this field will be expiration time of STS id token in seconds.
    #[serde(rename = "expiresIn", skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<String>,
    /// The GITKit token for authenticated user.
    #[serde(rename = "idToken", skip_serializing_if = "Option::is_none")]
    pub id_token: Option<String>,
    /// True if it's a new user sign-in, false if it's a returning user.
    #[serde(rename = "isNewUser", skip_serializing_if = "Option::is_none")]
    pub is_new_user: Option<bool>,
    /// The fixed string \"identitytoolkit#VerifyCustomTokenResponse\".
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// If idToken is STS id token, then this field will be refresh token.
    #[serde(rename = "refreshToken", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

impl VerifyCustomTokenResponse {
    /// Response from verifying a custom token
    pub fn new() -> VerifyCustomTokenResponse {
        VerifyCustomTokenResponse {
            expires_in: None,
            id_token: None,
            is_new_user: None,
            kind: None,
            refresh_token: None,
        }
    }
}
