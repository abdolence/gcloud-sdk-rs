use serde::{Deserialize, Serialize}; /*
                                      * Service Control API
                                      *
                                      * Provides admission control and telemetry reporting for services integrated with Service Infrastructure.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// ThirdPartyPrincipal : Third party identity principal.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ThirdPartyPrincipal {
    /// Metadata about third party identity.
    #[serde(rename = "thirdPartyClaims", skip_serializing_if = "Option::is_none")]
    pub third_party_claims: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl ThirdPartyPrincipal {
    /// Third party identity principal.
    pub fn new() -> ThirdPartyPrincipal {
        ThirdPartyPrincipal {
            third_party_claims: None,
        }
    }
}
