use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::compute_v1::models;

/// InterconnectOutageNotification : Description of a planned outage on this Interconnect.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterconnectOutageNotification {
    /// If issue_type is IT_PARTIAL_OUTAGE, a list of the Google-side circuit IDs that will be affected.
    #[serde(rename = "affectedCircuits", skip_serializing_if = "Option::is_none")]
    pub affected_circuits: Option<Vec<String>>,
    /// A description about the purpose of the outage.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Scheduled end time for the outage (milliseconds since Unix epoch).
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Form this outage is expected to take, which can take one of the following values: - OUTAGE: The Interconnect may be completely out of service for some or all of the specified window. - PARTIAL_OUTAGE: Some circuits comprising the Interconnect as a whole should remain up, but with reduced bandwidth. Note that the versions of this enum prefixed with \"IT_\" have been deprecated in favor of the unprefixed values.
    #[serde(rename = "issueType", skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<IssueType>,
    /// Unique identifier for this outage notification.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The party that generated this notification, which can take the following value: - GOOGLE: this notification as generated by Google. Note that the value of NSRC_GOOGLE has been deprecated in favor of GOOGLE.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// Scheduled start time for the outage (milliseconds since Unix epoch).
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// State of this notification, which can take one of the following values: - ACTIVE: This outage notification is active. The event could be in the past, present, or future. See start_time and end_time for scheduling. - CANCELLED: The outage associated with this notification was cancelled before the outage was due to start. - COMPLETED: The outage associated with this notification is complete. Note that the versions of this enum prefixed with \"NS_\" have been deprecated in favor of the unprefixed values.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

impl InterconnectOutageNotification {
    /// Description of a planned outage on this Interconnect.
    pub fn new() -> InterconnectOutageNotification {
        InterconnectOutageNotification {
            affected_circuits: None,
            description: None,
            end_time: None,
            issue_type: None,
            name: None,
            source: None,
            start_time: None,
            state: None,
        }
    }
}
/// Form this outage is expected to take, which can take one of the following values: - OUTAGE: The Interconnect may be completely out of service for some or all of the specified window. - PARTIAL_OUTAGE: Some circuits comprising the Interconnect as a whole should remain up, but with reduced bandwidth. Note that the versions of this enum prefixed with \"IT_\" have been deprecated in favor of the unprefixed values.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IssueType {
    #[serde(rename = "IT_OUTAGE")]
    ItOutage,
    #[serde(rename = "IT_PARTIAL_OUTAGE")]
    ItPartialOutage,
    #[serde(rename = "OUTAGE")]
    Outage,
    #[serde(rename = "PARTIAL_OUTAGE")]
    PartialOutage,
}

impl Default for IssueType {
    fn default() -> IssueType {
        Self::ItOutage
    }
}
/// The party that generated this notification, which can take the following value: - GOOGLE: this notification as generated by Google. Note that the value of NSRC_GOOGLE has been deprecated in favor of GOOGLE.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "GOOGLE")]
    Google,
    #[serde(rename = "NSRC_GOOGLE")]
    NsrcGoogle,
}

impl Default for Source {
    fn default() -> Source {
        Self::Google
    }
}
/// State of this notification, which can take one of the following values: - ACTIVE: This outage notification is active. The event could be in the past, present, or future. See start_time and end_time for scheduling. - CANCELLED: The outage associated with this notification was cancelled before the outage was due to start. - COMPLETED: The outage associated with this notification is complete. Note that the versions of this enum prefixed with \"NS_\" have been deprecated in favor of the unprefixed values.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "NS_ACTIVE")]
    NsActive,
    #[serde(rename = "NS_CANCELED")]
    NsCanceled,
}

impl Default for State {
    fn default() -> State {
        Self::Active
    }
}
