use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// Tags : A set of instance tags.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Tags {
    /// Specifies a fingerprint for this request, which is essentially a hash of the tags' contents and used for optimistic locking. The fingerprint is initially generated by Compute Engine and changes after every request to modify or update tags. You must always provide an up-to-date fingerprint hash in order to update or change tags. To see the latest fingerprint, make get() request to the instance.
    #[serde(rename = "fingerprint", skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// An array of tags. Each tag must be 1-63 characters long, and comply with RFC1035.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
}

impl Tags {
    /// A set of instance tags.
    pub fn new() -> Tags {
        Tags {
            fingerprint: None,
            items: None,
        }
    }
}