use serde::{Deserialize, Serialize}; /*
                                      * Cloud Storage JSON API
                                      *
                                      * Stores and retrieves potentially large, immutable data objects.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// BucketIamConfiguration : The bucket's IAM configuration.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BucketIamConfiguration {
    #[serde(rename = "bucketPolicyOnly", skip_serializing_if = "Option::is_none")]
    pub bucket_policy_only: Option<Box<crate::google_rest_apis::storage_v1::models::BucketIamConfigurationBucketPolicyOnly>>,
    /// The bucket's Public Access Prevention configuration. Currently, 'inherited' and 'enforced' are supported.
    #[serde(rename = "publicAccessPrevention", skip_serializing_if = "Option::is_none")]
    pub public_access_prevention: Option<String>,
    #[serde(rename = "uniformBucketLevelAccess", skip_serializing_if = "Option::is_none")]
    pub uniform_bucket_level_access: Option<Box<crate::google_rest_apis::storage_v1::models::BucketIamConfigurationUniformBucketLevelAccess>>,
}

impl BucketIamConfiguration {
    /// The bucket's IAM configuration.
    pub fn new() -> BucketIamConfiguration {
        BucketIamConfiguration {
            bucket_policy_only: None,
            public_access_prevention: None,
            uniform_bucket_level_access: None,
        }
    }
}
