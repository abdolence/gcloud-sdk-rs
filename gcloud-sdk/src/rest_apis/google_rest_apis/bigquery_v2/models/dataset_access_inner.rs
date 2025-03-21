use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::bigquery_v2::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatasetAccessInner {
    #[serde(rename = "dataset", skip_serializing_if = "Option::is_none")]
    pub dataset: Option<Box<models::DatasetAccessEntry>>,
    /// [Pick one] A domain to grant access to. Any users signed in with the domain specified will be granted the specified access. Example: \"example.com\". Maps to IAM policy member \"domain:DOMAIN\".
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// [Pick one] An email address of a Google Group to grant access to. Maps to IAM policy member \"group:GROUP\".
    #[serde(rename = "groupByEmail", skip_serializing_if = "Option::is_none")]
    pub group_by_email: Option<String>,
    /// [Pick one] Some other type of member that appears in the IAM Policy but isn't a user, group, domain, or special group.
    #[serde(rename = "iamMember", skip_serializing_if = "Option::is_none")]
    pub iam_member: Option<String>,
    /// [Required] An IAM role ID that should be granted to the user, group, or domain specified in this access entry. The following legacy mappings will be applied: OWNER  roles/bigquery.dataOwner WRITER  roles/bigquery.dataEditor READER  roles/bigquery.dataViewer This field will accept any of the above formats, but will return only the legacy format. For example, if you set this field to \"roles/bigquery.dataOwner\", it will be returned back as \"OWNER\".
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "routine", skip_serializing_if = "Option::is_none")]
    pub routine: Option<Box<models::RoutineReference>>,
    /// [Pick one] A special group to grant access to. Possible values include: projectOwners: Owners of the enclosing project. projectReaders: Readers of the enclosing project. projectWriters: Writers of the enclosing project. allAuthenticatedUsers: All authenticated BigQuery users. Maps to similarly-named IAM members.
    #[serde(rename = "specialGroup", skip_serializing_if = "Option::is_none")]
    pub special_group: Option<String>,
    /// [Pick one] An email address of a user to grant access to. For example: fred@example.com. Maps to IAM policy member \"user:EMAIL\" or \"serviceAccount:EMAIL\".
    #[serde(rename = "userByEmail", skip_serializing_if = "Option::is_none")]
    pub user_by_email: Option<String>,
    #[serde(rename = "view", skip_serializing_if = "Option::is_none")]
    pub view: Option<Box<models::TableReference>>,
}

impl DatasetAccessInner {
    pub fn new() -> DatasetAccessInner {
        DatasetAccessInner {
            dataset: None,
            domain: None,
            group_by_email: None,
            iam_member: None,
            role: None,
            routine: None,
            special_group: None,
            user_by_email: None,
            view: None,
        }
    }
}
