/// User specified security marks that are attached to the parent Security
/// Command Center resource. Security marks are scoped within a Security Command
/// Center organization -- they can be modified and viewed by all users who have
/// proper permissions on the organization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityMarks {
    /// The relative resource name of the SecurityMarks. See:
    /// https://cloud.google.com/apis/design/resource_names#relative_resource_name
    /// Examples:
    /// "organizations/{organization_id}/assets/{asset_id}/securityMarks"
    /// "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Mutable user specified security marks belonging to the parent resource.
    /// Constraints are as follows:
    ///
    ///   * Keys and values are treated as case insensitive
    ///   * Keys must be between 1 - 256 characters (inclusive)
    ///   * Keys must be letters, numbers, underscores, or dashes
    ///   * Values have leading and trailing whitespace trimmed, remaining
    ///     characters must be between 1 - 4096 characters (inclusive)
    #[prost(map = "string, string", tag = "2")]
    pub marks: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// Security Command Center representation of a Google Cloud
/// resource.
///
/// The Asset is a Security Command Center resource that captures information
/// about a single Google Cloud resource. All modifications to an Asset are only
/// within the context of Security Command Center and don't affect the referenced
/// Google Cloud resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// The relative resource name of this asset. See:
    /// https://cloud.google.com/apis/design/resource_names#relative_resource_name
    /// Example:
    /// "organizations/{organization_id}/assets/{asset_id}".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Security Command Center managed properties. These properties are managed by
    /// Security Command Center and cannot be modified by the user.
    #[prost(message, optional, tag = "2")]
    pub security_center_properties: ::std::option::Option<asset::SecurityCenterProperties>,
    /// Resource managed properties. These properties are managed and defined by
    /// the Google Cloud resource and cannot be modified by the user.
    #[prost(map = "string, message", tag = "7")]
    pub resource_properties: ::std::collections::HashMap<std::string::String, ::prost_types::Value>,
    /// User specified security marks. These marks are entirely managed by the user
    /// and come from the SecurityMarks resource that belongs to the asset.
    #[prost(message, optional, tag = "8")]
    pub security_marks: ::std::option::Option<SecurityMarks>,
    /// The time at which the asset was created in Security Command Center.
    #[prost(message, optional, tag = "9")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time at which the asset was last updated, added, or deleted in Cloud
    /// SCC.
    #[prost(message, optional, tag = "10")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Cloud IAM Policy information associated with the Google Cloud resource
    /// described by the Security Command Center asset. This information is managed
    /// and defined by the Google Cloud resource and cannot be modified by the
    /// user.
    #[prost(message, optional, tag = "11")]
    pub iam_policy: ::std::option::Option<asset::IamPolicy>,
}
pub mod asset {
    /// Security Command Center managed properties. These properties are managed by
    /// Security Command Center and cannot be modified by the user.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SecurityCenterProperties {
        /// The full resource name of the Google Cloud resource this asset
        /// represents. This field is immutable after create time. See:
        /// https://cloud.google.com/apis/design/resource_names#full_resource_name
        #[prost(string, tag = "1")]
        pub resource_name: std::string::String,
        /// The type of the Google Cloud resource. Examples include: APPLICATION,
        /// PROJECT, and ORGANIZATION. This is a case insensitive field defined by
        /// Security Command Center and/or the producer of the resource and is
        /// immutable after create time.
        #[prost(string, tag = "2")]
        pub resource_type: std::string::String,
        /// The full resource name of the immediate parent of the resource. See:
        /// https://cloud.google.com/apis/design/resource_names#full_resource_name
        #[prost(string, tag = "3")]
        pub resource_parent: std::string::String,
        /// The full resource name of the project the resource belongs to. See:
        /// https://cloud.google.com/apis/design/resource_names#full_resource_name
        #[prost(string, tag = "4")]
        pub resource_project: std::string::String,
        /// Owners of the Google Cloud resource.
        #[prost(string, repeated, tag = "5")]
        pub resource_owners: ::std::vec::Vec<std::string::String>,
        /// The user defined display name for this resource.
        #[prost(string, tag = "6")]
        pub resource_display_name: std::string::String,
        /// The user defined display name for the parent of this resource.
        #[prost(string, tag = "7")]
        pub resource_parent_display_name: std::string::String,
        /// The user defined display name for the project of this resource.
        #[prost(string, tag = "8")]
        pub resource_project_display_name: std::string::String,
    }
    /// Cloud IAM Policy information associated with the Google Cloud resource
    /// described by the Security Command Center asset. This information is managed
    /// and defined by the Google Cloud resource and cannot be modified by the
    /// user.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IamPolicy {
        /// The JSON representation of the Policy associated with the asset.
        /// See https://cloud.google.com/iam/docs/reference/rest/v1/Policy for
        /// format details.
        #[prost(string, tag = "1")]
        pub policy_blob: std::string::String,
    }
}
/// Security Command Center finding.
///
/// A finding is a record of assessment data (security, risk, health or privacy)
/// ingested into Security Command Center for presentation, notification,
/// analysis, policy testing, and enforcement. For example, an XSS vulnerability
/// in an App Engine application is a finding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Finding {
    /// The relative resource name of this finding. See:
    /// https://cloud.google.com/apis/design/resource_names#relative_resource_name
    /// Example:
    /// "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The relative resource name of the source the finding belongs to. See:
    /// https://cloud.google.com/apis/design/resource_names#relative_resource_name
    /// This field is immutable after creation time.
    /// For example:
    /// "organizations/{organization_id}/sources/{source_id}"
    #[prost(string, tag = "2")]
    pub parent: std::string::String,
    /// For findings on Google Cloud resources, the full resource
    /// name of the Google Cloud resource this finding is for. See:
    /// https://cloud.google.com/apis/design/resource_names#full_resource_name
    /// When the finding is for a non-Google Cloud resource, the resourceName can
    /// be a customer or partner defined string. This field is immutable after
    /// creation time.
    #[prost(string, tag = "3")]
    pub resource_name: std::string::String,
    /// The state of the finding.
    #[prost(enumeration = "finding::State", tag = "4")]
    pub state: i32,
    /// The additional taxonomy group within findings from a given source.
    /// This field is immutable after creation time.
    /// Example: "XSS_FLASH_INJECTION"
    #[prost(string, tag = "5")]
    pub category: std::string::String,
    /// The URI that, if available, points to a web page outside of Security
    /// Command Center where additional information about the finding can be found.
    /// This field is guaranteed to be either empty or a well formed URL.
    #[prost(string, tag = "6")]
    pub external_uri: std::string::String,
    /// Source specific properties. These properties are managed by the source
    /// that writes the finding. The key names in the source_properties map must be
    /// between 1 and 255 characters, and must start with a letter and contain
    /// alphanumeric characters or underscores only.
    #[prost(map = "string, message", tag = "7")]
    pub source_properties: ::std::collections::HashMap<std::string::String, ::prost_types::Value>,
    /// Output only. User specified security marks. These marks are entirely
    /// managed by the user and come from the SecurityMarks resource that belongs
    /// to the finding.
    #[prost(message, optional, tag = "8")]
    pub security_marks: ::std::option::Option<SecurityMarks>,
    /// The time at which the event took place, or when an update to the finding
    /// occurred. For example, if the finding represents an open firewall it would
    /// capture the time the detector believes the firewall became open. The
    /// accuracy is determined by the detector. If the finding were to be resolved
    /// afterward, this time would reflect when the finding was resolved.
    #[prost(message, optional, tag = "9")]
    pub event_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time at which the finding was created in Security Command Center.
    #[prost(message, optional, tag = "10")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The severity of the finding. This field is managed by the source that
    /// writes the finding.
    #[prost(enumeration = "finding::Severity", tag = "13")]
    pub severity: i32,
}
pub mod finding {
    /// The state of the finding.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// The finding requires attention and has not been addressed yet.
        Active = 1,
        /// The finding has been fixed, triaged as a non-issue or otherwise addressed
        /// and is no longer active.
        Inactive = 2,
    }
    /// The severity of the finding. This field is managed by the source that
    /// writes the finding.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        /// No severity specified. The default value.
        Unspecified = 0,
        /// Critical severity.
        Critical = 1,
        /// High severity.
        High = 2,
        /// Medium severity.
        Medium = 3,
        /// Low severity.
        Low = 4,
    }
}
/// Security Command Center notification configs.
///
/// A notification config is a Security Command Center resource that contains the
/// configuration to send notifications for create/update events of findings,
/// assets and etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationConfig {
    /// The relative resource name of this notification config. See:
    /// https://cloud.google.com/apis/design/resource_names#relative_resource_name
    /// Example:
    /// "organizations/{organization_id}/notificationConfigs/notify_public_bucket".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The description of the notification config (max of 1024 characters).
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    /// The type of events the config is for, e.g. FINDING.
    #[prost(enumeration = "notification_config::EventType", tag = "3")]
    pub event_type: i32,
    /// The Pub/Sub topic to send notifications to. Its format is
    /// "projects/[project_id]/topics/[topic]".
    #[prost(string, tag = "4")]
    pub pubsub_topic: std::string::String,
    /// Output only. The service account that needs "pubsub.topics.publish"
    /// permission to publish to the Pub/Sub topic.
    #[prost(string, tag = "5")]
    pub service_account: std::string::String,
    /// The config for triggering notifications.
    #[prost(oneof = "notification_config::NotifyConfig", tags = "6")]
    pub notify_config: ::std::option::Option<notification_config::NotifyConfig>,
}
pub mod notification_config {
    /// The config for streaming-based notifications, which send each event as soon
    /// as it is detected.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StreamingConfig {
        /// Expression that defines the filter to apply across create/update events
        /// of assets or findings as specified by the event type. The expression is a
        /// list of zero or more restrictions combined via logical operators `AND`
        /// and `OR`. Parentheses are supported, and `OR` has higher precedence than
        /// `AND`.
        ///
        /// Restrictions have the form `<field> <operator> <value>` and may have a
        /// `-` character in front of them to indicate negation. The fields map to
        /// those defined in the corresponding resource.
        ///
        /// The supported operators are:
        ///
        /// * `=` for all value types.
        /// * `>`, `<`, `>=`, `<=` for integer values.
        /// * `:`, meaning substring matching, for strings.
        ///
        /// The supported value types are:
        ///
        /// * string literals in quotes.
        /// * integer literals without quotes.
        /// * boolean literals `true` and `false` without quotes.
        #[prost(string, tag = "1")]
        pub filter: std::string::String,
    }
    /// The type of events.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventType {
        /// Unspecified event type.
        Unspecified = 0,
        /// Events for findings.
        Finding = 1,
    }
    /// The config for triggering notifications.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum NotifyConfig {
        /// The config for triggering streaming-based notifications.
        #[prost(message, tag = "6")]
        StreamingConfig(StreamingConfig),
    }
}
/// Information related to the Google Cloud resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    /// The full resource name of the resource. See:
    /// https://cloud.google.com/apis/design/resource_names#full_resource_name
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The full resource name of project that the resource belongs to.
    #[prost(string, tag = "2")]
    pub project: std::string::String,
    /// The human readable name of project that the resource belongs to.
    #[prost(string, tag = "3")]
    pub project_display_name: std::string::String,
    /// The full resource name of resource's parent.
    #[prost(string, tag = "4")]
    pub parent: std::string::String,
    /// The human readable name of resource's parent.
    #[prost(string, tag = "5")]
    pub parent_display_name: std::string::String,
}
/// Security Command Center's Notification
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationMessage {
    /// Name of the notification config that generated current notification.
    #[prost(string, tag = "1")]
    pub notification_config_name: std::string::String,
    /// The Cloud resource tied to the notification.
    #[prost(message, optional, tag = "3")]
    pub resource: ::std::option::Option<Resource>,
    /// Notification Event.
    #[prost(oneof = "notification_message::Event", tags = "2")]
    pub event: ::std::option::Option<notification_message::Event>,
}
pub mod notification_message {
    /// Notification Event.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// If it's a Finding based notification config, this field will be
        /// populated.
        #[prost(message, tag = "2")]
        Finding(super::Finding),
    }
}
/// User specified settings that are attached to the Security Command
/// Center organization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationSettings {
    /// The relative resource name of the settings. See:
    /// https://cloud.google.com/apis/design/resource_names#relative_resource_name
    /// Example:
    /// "organizations/{organization_id}/organizationSettings".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// A flag that indicates if Asset Discovery should be enabled. If the flag is
    /// set to `true`, then discovery of assets will occur. If it is set to `false,
    /// all historical assets will remain, but discovery of future assets will not
    /// occur.
    #[prost(bool, tag = "2")]
    pub enable_asset_discovery: bool,
    /// The configuration used for Asset Discovery runs.
    #[prost(message, optional, tag = "3")]
    pub asset_discovery_config: ::std::option::Option<organization_settings::AssetDiscoveryConfig>,
}
pub mod organization_settings {
    /// The configuration used for Asset Discovery runs.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AssetDiscoveryConfig {
        /// The project ids to use for filtering asset discovery.
        #[prost(string, repeated, tag = "1")]
        pub project_ids: ::std::vec::Vec<std::string::String>,
        /// The mode to use for filtering asset discovery.
        #[prost(enumeration = "asset_discovery_config::InclusionMode", tag = "2")]
        pub inclusion_mode: i32,
    }
    pub mod asset_discovery_config {
        /// The mode of inclusion when running Asset Discovery.
        /// Asset discovery can be limited by explicitly identifying projects to be
        /// included or excluded. If INCLUDE_ONLY is set, then only those projects
        /// within the organization and their children are discovered during asset
        /// discovery. If EXCLUDE is set, then projects that don't match those
        /// projects are discovered during asset discovery. If neither are set, then
        /// all projects within the organization are discovered during asset
        /// discovery.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum InclusionMode {
            /// Unspecified. Setting the mode with this value will disable
            /// inclusion/exclusion filtering for Asset Discovery.
            Unspecified = 0,
            /// Asset Discovery will capture only the resources within the projects
            /// specified. All other resources will be ignored.
            IncludeOnly = 1,
            /// Asset Discovery will ignore all resources under the projects specified.
            /// All other resources will be retrieved.
            Exclude = 2,
        }
    }
}
/// Response of asset discovery run
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunAssetDiscoveryResponse {
    /// The state of an asset discovery run.
    #[prost(enumeration = "run_asset_discovery_response::State", tag = "1")]
    pub state: i32,
    /// The duration between asset discovery run start and end
    #[prost(message, optional, tag = "2")]
    pub duration: ::std::option::Option<::prost_types::Duration>,
}
pub mod run_asset_discovery_response {
    /// The state of an asset discovery run.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Asset discovery run state was unspecified.
        Unspecified = 0,
        /// Asset discovery run completed successfully.
        Completed = 1,
        /// Asset discovery run was cancelled with tasks still pending, as another
        /// run for the same organization was started with a higher priority.
        Superseded = 2,
        /// Asset discovery run was killed and terminated.
        Terminated = 3,
    }
}
/// Security Command Center finding source. A finding source
/// is an entity or a mechanism that can produce a finding. A source is like a
/// container of findings that come from the same scanner, logger, monitor, etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Source {
    /// The relative resource name of this source. See:
    /// https://cloud.google.com/apis/design/resource_names#relative_resource_name
    /// Example:
    /// "organizations/{organization_id}/sources/{source_id}"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The source's display name.
    /// A source's display name must be unique amongst its siblings, for example,
    /// two sources with the same parent can't share the same display name.
    /// The display name must have a length between 1 and 64 characters
    /// (inclusive).
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// The description of the source (max of 1024 characters).
    /// Example:
    /// "Web Security Scanner is a web security scanner for common
    /// vulnerabilities in App Engine applications. It can automatically
    /// scan and detect four common vulnerabilities, including cross-site-scripting
    /// (XSS), Flash injection, mixed content (HTTP in HTTPS), and
    /// outdated/insecure libraries."
    #[prost(string, tag = "3")]
    pub description: std::string::String,
}
/// Request message for creating a finding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFindingRequest {
    /// Required. Resource name of the new finding's parent. Its format should be
    /// "organizations/[organization_id]/sources/[source_id]".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. Unique identifier provided by the client within the parent scope.
    #[prost(string, tag = "2")]
    pub finding_id: std::string::String,
    /// Required. The Finding being created. The name and security_marks will be ignored as
    /// they are both output only fields on this resource.
    #[prost(message, optional, tag = "3")]
    pub finding: ::std::option::Option<Finding>,
}
/// Request message for creating a notification config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNotificationConfigRequest {
    /// Required. Resource name of the new notification config's parent. Its format is
    /// "organizations/[organization_id]".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. Unique identifier provided by the client within the parent scope.
    /// It must be between 1 and 128 characters, and contains alphanumeric
    /// characters, underscores or hyphens only.
    #[prost(string, tag = "2")]
    pub config_id: std::string::String,
    /// Required. The notification config being created. The name and the service account
    /// will be ignored as they are both output only fields on this resource.
    #[prost(message, optional, tag = "3")]
    pub notification_config: ::std::option::Option<NotificationConfig>,
}
/// Request message for creating a source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSourceRequest {
    /// Required. Resource name of the new source's parent. Its format should be
    /// "organizations/[organization_id]".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The Source being created, only the display_name and description will be
    /// used. All other fields will be ignored.
    #[prost(message, optional, tag = "2")]
    pub source: ::std::option::Option<Source>,
}
/// Request message for deleting a notification config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNotificationConfigRequest {
    /// Required. Name of the notification config to delete. Its format is
    /// "organizations/[organization_id]/notificationConfigs/[config_id]".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for getting a notification config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationConfigRequest {
    /// Required. Name of the notification config to get. Its format is
    /// "organizations/[organization_id]/notificationConfigs/[config_id]".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for getting organization settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrganizationSettingsRequest {
    /// Required. Name of the organization to get organization settings for. Its format is
    /// "organizations/[organization_id]/organizationSettings".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for getting a source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSourceRequest {
    /// Required. Relative resource name of the source. Its format is
    /// "organizations/[organization_id]/source/[source_id]".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for grouping by assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAssetsRequest {
    /// Required. Name of the organization to groupBy. Its format is
    /// "organizations/[organization_id]".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Expression that defines the filter to apply across assets.
    /// The expression is a list of zero or more restrictions combined via logical
    /// operators `AND` and `OR`.
    /// Parentheses are supported, and `OR` has higher precedence than `AND`.
    ///
    /// Restrictions have the form `<field> <operator> <value>` and may have a `-`
    /// character in front of them to indicate negation. The fields map to those
    /// defined in the Asset resource. Examples include:
    ///
    /// * name
    /// * security_center_properties.resource_name
    /// * resource_properties.a_property
    /// * security_marks.marks.marka
    ///
    /// The supported operators are:
    ///
    /// * `=` for all value types.
    /// * `>`, `<`, `>=`, `<=` for integer values.
    /// * `:`, meaning substring matching, for strings.
    ///
    /// The supported value types are:
    ///
    /// * string literals in quotes.
    /// * integer literals without quotes.
    /// * boolean literals `true` and `false` without quotes.
    ///
    /// The following field and operator combinations are supported:
    ///
    /// * name: `=`
    /// * update_time: `=`, `>`, `<`, `>=`, `<=`
    ///
    ///   Usage: This should be milliseconds since epoch or an RFC3339 string.
    ///   Examples:
    ///     `update_time = "2019-06-10T16:07:18-07:00"`
    ///     `update_time = 1560208038000`
    ///
    /// * create_time: `=`, `>`, `<`, `>=`, `<=`
    ///
    ///   Usage: This should be milliseconds since epoch or an RFC3339 string.
    ///   Examples:
    ///     `create_time = "2019-06-10T16:07:18-07:00"`
    ///     `create_time = 1560208038000`
    ///
    /// * iam_policy.policy_blob: `=`, `:`
    /// * resource_properties: `=`, `:`, `>`, `<`, `>=`, `<=`
    /// * security_marks.marks: `=`, `:`
    /// * security_center_properties.resource_name: `=`, `:`
    /// * security_center_properties.resource_name_display_name: `=`, `:`
    /// * security_center_properties.resource_type: `=`, `:`
    /// * security_center_properties.resource_parent: `=`, `:`
    /// * security_center_properties.resource_parent_display_name: `=`, `:`
    /// * security_center_properties.resource_project: `=`, `:`
    /// * security_center_properties.resource_project_display_name: `=`, `:`
    /// * security_center_properties.resource_owners: `=`, `:`
    ///
    /// For example, `resource_properties.size = 100` is a valid filter string.
    ///
    /// Use a partial match on the empty string to filter based on a property
    /// existing: `resource_properties.my_property : ""`
    ///
    /// Use a negated partial match on the empty string to filter based on a
    /// property not existing: `-resource_properties.my_property : ""`
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// Required. Expression that defines what assets fields to use for grouping. The string
    /// value should follow SQL syntax: comma separated list of fields. For
    /// example:
    /// "security_center_properties.resource_project,security_center_properties.project".
    ///
    /// The following fields are supported when compare_duration is not set:
    ///
    /// * security_center_properties.resource_project
    /// * security_center_properties.resource_project_display_name
    /// * security_center_properties.resource_type
    /// * security_center_properties.resource_parent
    /// * security_center_properties.resource_parent_display_name
    ///
    /// The following fields are supported when compare_duration is set:
    ///
    /// * security_center_properties.resource_type
    /// * security_center_properties.resource_project_display_name
    /// * security_center_properties.resource_parent_display_name
    #[prost(string, tag = "3")]
    pub group_by: std::string::String,
    /// When compare_duration is set, the GroupResult's "state_change" property is
    /// updated to indicate whether the asset was added, removed, or remained
    /// present during the compare_duration period of time that precedes the
    /// read_time. This is the time between (read_time - compare_duration) and
    /// read_time.
    ///
    /// The state change value is derived based on the presence of the asset at the
    /// two points in time. Intermediate state changes between the two times don't
    /// affect the result. For example, the results aren't affected if the asset is
    /// removed and re-created again.
    ///
    /// Possible "state_change" values when compare_duration is specified:
    ///
    /// * "ADDED":   indicates that the asset was not present at the start of
    ///                compare_duration, but present at reference_time.
    /// * "REMOVED": indicates that the asset was present at the start of
    ///                compare_duration, but not present at reference_time.
    /// * "ACTIVE":  indicates that the asset was present at both the
    ///                start and the end of the time period defined by
    ///                compare_duration and reference_time.
    ///
    /// If compare_duration is not specified, then the only possible state_change
    /// is "UNUSED", which will be the state_change set for all assets present at
    /// read_time.
    ///
    /// If this field is set then `state_change` must be a specified field in
    /// `group_by`.
    #[prost(message, optional, tag = "4")]
    pub compare_duration: ::std::option::Option<::prost_types::Duration>,
    /// Time used as a reference point when filtering assets. The filter is limited
    /// to assets existing at the supplied time and their values are those at that
    /// specific time. Absence of this field will default to the API's version of
    /// NOW.
    #[prost(message, optional, tag = "5")]
    pub read_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The value returned by the last `GroupAssetsResponse`; indicates
    /// that this is a continuation of a prior `GroupAssets` call, and that the
    /// system should return the next page of data.
    #[prost(string, tag = "7")]
    pub page_token: std::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag = "8")]
    pub page_size: i32,
}
/// Response message for grouping by assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAssetsResponse {
    /// Group results. There exists an element for each existing unique
    /// combination of property/values. The element contains a count for the number
    /// of times those specific property/values appear.
    #[prost(message, repeated, tag = "1")]
    pub group_by_results: ::std::vec::Vec<GroupResult>,
    /// Time used for executing the groupBy request.
    #[prost(message, optional, tag = "2")]
    pub read_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag = "3")]
    pub next_page_token: std::string::String,
    /// The total number of results matching the query.
    #[prost(int32, tag = "4")]
    pub total_size: i32,
}
/// Request message for grouping by findings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupFindingsRequest {
    /// Required. Name of the source to groupBy. Its format is
    /// "organizations/[organization_id]/sources/[source_id]". To groupBy across
    /// all sources provide a source_id of `-`. For example:
    /// organizations/{organization_id}/sources/-
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Expression that defines the filter to apply across findings.
    /// The expression is a list of one or more restrictions combined via logical
    /// operators `AND` and `OR`.
    /// Parentheses are supported, and `OR` has higher precedence than `AND`.
    ///
    /// Restrictions have the form `<field> <operator> <value>` and may have a `-`
    /// character in front of them to indicate negation. Examples include:
    ///
    ///  * name
    ///  * source_properties.a_property
    ///  * security_marks.marks.marka
    ///
    /// The supported operators are:
    ///
    /// * `=` for all value types.
    /// * `>`, `<`, `>=`, `<=` for integer values.
    /// * `:`, meaning substring matching, for strings.
    ///
    /// The supported value types are:
    ///
    /// * string literals in quotes.
    /// * integer literals without quotes.
    /// * boolean literals `true` and `false` without quotes.
    ///
    /// The following field and operator combinations are supported:
    ///
    /// * name: `=`
    /// * parent: `=`, `:`
    /// * resource_name: `=`, `:`
    /// * state: `=`, `:`
    /// * category: `=`, `:`
    /// * external_uri: `=`, `:`
    /// * event_time: `=`, `>`, `<`, `>=`, `<=`
    /// * severity: `=`, `:`
    ///
    ///   Usage: This should be milliseconds since epoch or an RFC3339 string.
    ///   Examples:
    ///     `event_time = "2019-06-10T16:07:18-07:00"`
    ///     `event_time = 1560208038000`
    ///
    /// * security_marks.marks: `=`, `:`
    /// * source_properties: `=`, `:`, `>`, `<`, `>=`, `<=`
    ///
    /// For example, `source_properties.size = 100` is a valid filter string.
    ///
    /// Use a partial match on the empty string to filter based on a property
    /// existing: `source_properties.my_property : ""`
    ///
    /// Use a negated partial match on the empty string to filter based on a
    /// property not existing: `-source_properties.my_property : ""`
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// Required. Expression that defines what assets fields to use for grouping (including
    /// `state_change`). The string value should follow SQL syntax: comma separated
    /// list of fields. For example: "parent,resource_name".
    ///
    /// The following fields are supported:
    ///
    /// * resource_name
    /// * category
    /// * state
    /// * parent
    /// * severity
    ///
    /// The following fields are supported when compare_duration is set:
    ///
    /// * state_change
    #[prost(string, tag = "3")]
    pub group_by: std::string::String,
    /// Time used as a reference point when filtering findings. The filter is
    /// limited to findings existing at the supplied time and their values are
    /// those at that specific time. Absence of this field will default to the
    /// API's version of NOW.
    #[prost(message, optional, tag = "4")]
    pub read_time: ::std::option::Option<::prost_types::Timestamp>,
    /// When compare_duration is set, the GroupResult's "state_change" attribute is
    /// updated to indicate whether the finding had its state changed, the
    /// finding's state remained unchanged, or if the finding was added during the
    /// compare_duration period of time that precedes the read_time. This is the
    /// time between (read_time - compare_duration) and read_time.
    ///
    /// The state_change value is derived based on the presence and state of the
    /// finding at the two points in time. Intermediate state changes between the
    /// two times don't affect the result. For example, the results aren't affected
    /// if the finding is made inactive and then active again.
    ///
    /// Possible "state_change" values when compare_duration is specified:
    ///
    /// * "CHANGED":   indicates that the finding was present and matched the given
    ///                  filter at the start of compare_duration, but changed its
    ///                  state at read_time.
    /// * "UNCHANGED": indicates that the finding was present and matched the given
    ///                  filter at the start of compare_duration and did not change
    ///                  state at read_time.
    /// * "ADDED":     indicates that the finding did not match the given filter or
    ///                  was not present at the start of compare_duration, but was
    ///                  present at read_time.
    /// * "REMOVED":   indicates that the finding was present and matched the
    ///                  filter at the start of compare_duration, but did not match
    ///                  the filter at read_time.
    ///
    /// If compare_duration is not specified, then the only possible state_change
    /// is "UNUSED",  which will be the state_change set for all findings present
    /// at read_time.
    ///
    /// If this field is set then `state_change` must be a specified field in
    /// `group_by`.
    #[prost(message, optional, tag = "5")]
    pub compare_duration: ::std::option::Option<::prost_types::Duration>,
    /// The value returned by the last `GroupFindingsResponse`; indicates
    /// that this is a continuation of a prior `GroupFindings` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag = "7")]
    pub page_token: std::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag = "8")]
    pub page_size: i32,
}
/// Response message for group by findings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupFindingsResponse {
    /// Group results. There exists an element for each existing unique
    /// combination of property/values. The element contains a count for the number
    /// of times those specific property/values appear.
    #[prost(message, repeated, tag = "1")]
    pub group_by_results: ::std::vec::Vec<GroupResult>,
    /// Time used for executing the groupBy request.
    #[prost(message, optional, tag = "2")]
    pub read_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag = "3")]
    pub next_page_token: std::string::String,
    /// The total number of results matching the query.
    #[prost(int32, tag = "4")]
    pub total_size: i32,
}
/// Result containing the properties and count of a groupBy request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupResult {
    /// Properties matching the groupBy fields in the request.
    #[prost(map = "string, message", tag = "1")]
    pub properties: ::std::collections::HashMap<std::string::String, ::prost_types::Value>,
    /// Total count of resources for the given properties.
    #[prost(int64, tag = "2")]
    pub count: i64,
}
/// Request message for listing notification configs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationConfigsRequest {
    /// Required. Name of the organization to list notification configs.
    /// Its format is "organizations/[organization_id]".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The value returned by the last `ListNotificationConfigsResponse`; indicates
    /// that this is a continuation of a prior `ListNotificationConfigs` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag = "2")]
    pub page_token: std::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// Response message for listing notification configs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationConfigsResponse {
    /// Notification configs belonging to the requested parent.
    #[prost(message, repeated, tag = "1")]
    pub notification_configs: ::std::vec::Vec<NotificationConfig>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for listing sources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSourcesRequest {
    /// Required. Resource name of the parent of sources to list. Its format should be
    /// "organizations/[organization_id]".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The value returned by the last `ListSourcesResponse`; indicates
    /// that this is a continuation of a prior `ListSources` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag = "2")]
    pub page_token: std::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag = "7")]
    pub page_size: i32,
}
/// Response message for listing sources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSourcesResponse {
    /// Sources belonging to the requested parent.
    #[prost(message, repeated, tag = "1")]
    pub sources: ::std::vec::Vec<Source>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for listing assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsRequest {
    /// Required. Name of the organization assets should belong to. Its format is
    /// "organizations/[organization_id]".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Expression that defines the filter to apply across assets.
    /// The expression is a list of zero or more restrictions combined via logical
    /// operators `AND` and `OR`.
    /// Parentheses are supported, and `OR` has higher precedence than `AND`.
    ///
    /// Restrictions have the form `<field> <operator> <value>` and may have a `-`
    /// character in front of them to indicate negation. The fields map to those
    /// defined in the Asset resource. Examples include:
    ///
    /// * name
    /// * security_center_properties.resource_name
    /// * resource_properties.a_property
    /// * security_marks.marks.marka
    ///
    /// The supported operators are:
    ///
    /// * `=` for all value types.
    /// * `>`, `<`, `>=`, `<=` for integer values.
    /// * `:`, meaning substring matching, for strings.
    ///
    /// The supported value types are:
    ///
    /// * string literals in quotes.
    /// * integer literals without quotes.
    /// * boolean literals `true` and `false` without quotes.
    ///
    /// The following are the allowed field and operator combinations:
    ///
    /// * name: `=`
    /// * update_time: `=`, `>`, `<`, `>=`, `<=`
    ///
    ///   Usage: This should be milliseconds since epoch or an RFC3339 string.
    ///   Examples:
    ///     `update_time = "2019-06-10T16:07:18-07:00"`
    ///     `update_time = 1560208038000`
    ///
    /// * create_time: `=`, `>`, `<`, `>=`, `<=`
    ///
    ///   Usage: This should be milliseconds since epoch or an RFC3339 string.
    ///   Examples:
    ///     `create_time = "2019-06-10T16:07:18-07:00"`
    ///     `create_time = 1560208038000`
    ///
    /// * iam_policy.policy_blob: `=`, `:`
    /// * resource_properties: `=`, `:`, `>`, `<`, `>=`, `<=`
    /// * security_marks.marks: `=`, `:`
    /// * security_center_properties.resource_name: `=`, `:`
    /// * security_center_properties.resource_display_name: `=`, `:`
    /// * security_center_properties.resource_type: `=`, `:`
    /// * security_center_properties.resource_parent: `=`, `:`
    /// * security_center_properties.resource_parent_display_name: `=`, `:`
    /// * security_center_properties.resource_project: `=`, `:`
    /// * security_center_properties.resource_project_display_name: `=`, `:`
    /// * security_center_properties.resource_owners: `=`, `:`
    ///
    /// For example, `resource_properties.size = 100` is a valid filter string.
    ///
    /// Use a partial match on the empty string to filter based on a property
    /// existing: `resource_properties.my_property : ""`
    ///
    /// Use a negated partial match on the empty string to filter based on a
    /// property not existing: `-resource_properties.my_property : ""`
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// Expression that defines what fields and order to use for sorting. The
    /// string value should follow SQL syntax: comma separated list of fields. For
    /// example: "name,resource_properties.a_property". The default sorting order
    /// is ascending. To specify descending order for a field, a suffix " desc"
    /// should be appended to the field name. For example: "name
    /// desc,resource_properties.a_property". Redundant space characters in the
    /// syntax are insignificant. "name desc,resource_properties.a_property" and "
    /// name     desc  ,   resource_properties.a_property  " are equivalent.
    ///
    /// The following fields are supported:
    /// name
    /// update_time
    /// resource_properties
    /// security_marks.marks
    /// security_center_properties.resource_name
    /// security_center_properties.resource_display_name
    /// security_center_properties.resource_parent
    /// security_center_properties.resource_parent_display_name
    /// security_center_properties.resource_project
    /// security_center_properties.resource_project_display_name
    /// security_center_properties.resource_type
    #[prost(string, tag = "3")]
    pub order_by: std::string::String,
    /// Time used as a reference point when filtering assets. The filter is limited
    /// to assets existing at the supplied time and their values are those at that
    /// specific time. Absence of this field will default to the API's version of
    /// NOW.
    #[prost(message, optional, tag = "4")]
    pub read_time: ::std::option::Option<::prost_types::Timestamp>,
    /// When compare_duration is set, the ListAssetsResult's "state_change"
    /// attribute is updated to indicate whether the asset was added, removed, or
    /// remained present during the compare_duration period of time that precedes
    /// the read_time. This is the time between (read_time - compare_duration) and
    /// read_time.
    ///
    /// The state_change value is derived based on the presence of the asset at the
    /// two points in time. Intermediate state changes between the two times don't
    /// affect the result. For example, the results aren't affected if the asset is
    /// removed and re-created again.
    ///
    /// Possible "state_change" values when compare_duration is specified:
    ///
    /// * "ADDED":   indicates that the asset was not present at the start of
    ///                compare_duration, but present at read_time.
    /// * "REMOVED": indicates that the asset was present at the start of
    ///                compare_duration, but not present at read_time.
    /// * "ACTIVE":  indicates that the asset was present at both the
    ///                start and the end of the time period defined by
    ///                compare_duration and read_time.
    ///
    /// If compare_duration is not specified, then the only possible state_change
    /// is "UNUSED",  which will be the state_change set for all assets present at
    /// read_time.
    #[prost(message, optional, tag = "5")]
    pub compare_duration: ::std::option::Option<::prost_types::Duration>,
    /// A field mask to specify the ListAssetsResult fields to be listed in the
    /// response.
    /// An empty field mask will list all fields.
    #[prost(message, optional, tag = "7")]
    pub field_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The value returned by the last `ListAssetsResponse`; indicates
    /// that this is a continuation of a prior `ListAssets` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag = "8")]
    pub page_token: std::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag = "9")]
    pub page_size: i32,
}
/// Response message for listing assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsResponse {
    /// Assets matching the list request.
    #[prost(message, repeated, tag = "1")]
    pub list_assets_results: ::std::vec::Vec<list_assets_response::ListAssetsResult>,
    /// Time used for executing the list request.
    #[prost(message, optional, tag = "2")]
    pub read_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag = "3")]
    pub next_page_token: std::string::String,
    /// The total number of assets matching the query.
    #[prost(int32, tag = "4")]
    pub total_size: i32,
}
pub mod list_assets_response {
    /// Result containing the Asset and its State.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListAssetsResult {
        /// Asset matching the search request.
        #[prost(message, optional, tag = "1")]
        pub asset: ::std::option::Option<super::Asset>,
        /// State change of the asset between the points in time.
        #[prost(enumeration = "list_assets_result::StateChange", tag = "2")]
        pub state_change: i32,
    }
    pub mod list_assets_result {
        /// The change in state of the asset.
        ///
        /// When querying across two points in time this describes
        /// the change between the two points: ADDED, REMOVED, or ACTIVE.
        /// If there was no compare_duration supplied in the request the state change
        /// will be: UNUSED
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum StateChange {
            /// State change is unused, this is the canonical default for this enum.
            Unused = 0,
            /// Asset was added between the points in time.
            Added = 1,
            /// Asset was removed between the points in time.
            Removed = 2,
            /// Asset was present at both point(s) in time.
            Active = 3,
        }
    }
}
/// Request message for listing findings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFindingsRequest {
    /// Required. Name of the source the findings belong to. Its format is
    /// "organizations/[organization_id]/sources/[source_id]". To list across all
    /// sources provide a source_id of `-`. For example:
    /// organizations/{organization_id}/sources/-
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Expression that defines the filter to apply across findings.
    /// The expression is a list of one or more restrictions combined via logical
    /// operators `AND` and `OR`.
    /// Parentheses are supported, and `OR` has higher precedence than `AND`.
    ///
    /// Restrictions have the form `<field> <operator> <value>` and may have a `-`
    /// character in front of them to indicate negation. Examples include:
    ///
    ///  * name
    ///  * source_properties.a_property
    ///  * security_marks.marks.marka
    ///
    /// The supported operators are:
    ///
    /// * `=` for all value types.
    /// * `>`, `<`, `>=`, `<=` for integer values.
    /// * `:`, meaning substring matching, for strings.
    ///
    /// The supported value types are:
    ///
    /// * string literals in quotes.
    /// * integer literals without quotes.
    /// * boolean literals `true` and `false` without quotes.
    ///
    /// The following field and operator combinations are supported:
    ///
    /// * name: `=`
    /// * parent: `=`, `:`
    /// * resource_name: `=`, `:`
    /// * state: `=`, `:`
    /// * category: `=`, `:`
    /// * external_uri: `=`, `:`
    /// * event_time: `=`, `>`, `<`, `>=`, `<=`
    /// * severity: `=`, `:`
    ///
    ///   Usage: This should be milliseconds since epoch or an RFC3339 string.
    ///   Examples:
    ///     `event_time = "2019-06-10T16:07:18-07:00"`
    ///     `event_time = 1560208038000`
    ///
    /// security_marks.marks: `=`, `:`
    /// source_properties: `=`, `:`, `>`, `<`, `>=`, `<=`
    ///
    /// For example, `source_properties.size = 100` is a valid filter string.
    ///
    /// Use a partial match on the empty string to filter based on a property
    /// existing: `source_properties.my_property : ""`
    ///
    /// Use a negated partial match on the empty string to filter based on a
    /// property not existing: `-source_properties.my_property : ""`
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// Expression that defines what fields and order to use for sorting. The
    /// string value should follow SQL syntax: comma separated list of fields. For
    /// example: "name,resource_properties.a_property". The default sorting order
    /// is ascending. To specify descending order for a field, a suffix " desc"
    /// should be appended to the field name. For example: "name
    /// desc,source_properties.a_property". Redundant space characters in the
    /// syntax are insignificant. "name desc,source_properties.a_property" and "
    /// name     desc  ,   source_properties.a_property  " are equivalent.
    ///
    /// The following fields are supported:
    /// name
    /// parent
    /// state
    /// category
    /// resource_name
    /// event_time
    /// source_properties
    /// security_marks.marks
    #[prost(string, tag = "3")]
    pub order_by: std::string::String,
    /// Time used as a reference point when filtering findings. The filter is
    /// limited to findings existing at the supplied time and their values are
    /// those at that specific time. Absence of this field will default to the
    /// API's version of NOW.
    #[prost(message, optional, tag = "4")]
    pub read_time: ::std::option::Option<::prost_types::Timestamp>,
    /// When compare_duration is set, the ListFindingsResult's "state_change"
    /// attribute is updated to indicate whether the finding had its state changed,
    /// the finding's state remained unchanged, or if the finding was added in any
    /// state during the compare_duration period of time that precedes the
    /// read_time. This is the time between (read_time - compare_duration) and
    /// read_time.
    ///
    /// The state_change value is derived based on the presence and state of the
    /// finding at the two points in time. Intermediate state changes between the
    /// two times don't affect the result. For example, the results aren't affected
    /// if the finding is made inactive and then active again.
    ///
    /// Possible "state_change" values when compare_duration is specified:
    ///
    /// * "CHANGED":   indicates that the finding was present and matched the given
    ///                  filter at the start of compare_duration, but changed its
    ///                  state at read_time.
    /// * "UNCHANGED": indicates that the finding was present and matched the given
    ///                  filter at the start of compare_duration and did not change
    ///                  state at read_time.
    /// * "ADDED":     indicates that the finding did not match the given filter or
    ///                  was not present at the start of compare_duration, but was
    ///                  present at read_time.
    /// * "REMOVED":   indicates that the finding was present and matched the
    ///                  filter at the start of compare_duration, but did not match
    ///                  the filter at read_time.
    ///
    /// If compare_duration is not specified, then the only possible state_change
    /// is "UNUSED", which will be the state_change set for all findings present at
    /// read_time.
    #[prost(message, optional, tag = "5")]
    pub compare_duration: ::std::option::Option<::prost_types::Duration>,
    /// A field mask to specify the Finding fields to be listed in the response.
    /// An empty field mask will list all fields.
    #[prost(message, optional, tag = "7")]
    pub field_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The value returned by the last `ListFindingsResponse`; indicates
    /// that this is a continuation of a prior `ListFindings` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag = "8")]
    pub page_token: std::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag = "9")]
    pub page_size: i32,
}
/// Response message for listing findings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFindingsResponse {
    /// Findings matching the list request.
    #[prost(message, repeated, tag = "1")]
    pub list_findings_results: ::std::vec::Vec<list_findings_response::ListFindingsResult>,
    /// Time used for executing the list request.
    #[prost(message, optional, tag = "2")]
    pub read_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag = "3")]
    pub next_page_token: std::string::String,
    /// The total number of findings matching the query.
    #[prost(int32, tag = "4")]
    pub total_size: i32,
}
pub mod list_findings_response {
    /// Result containing the Finding and its StateChange.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListFindingsResult {
        /// Finding matching the search request.
        #[prost(message, optional, tag = "1")]
        pub finding: ::std::option::Option<super::Finding>,
        /// State change of the finding between the points in time.
        #[prost(enumeration = "list_findings_result::StateChange", tag = "2")]
        pub state_change: i32,
        /// Output only. Resource that is associated with this finding.
        #[prost(message, optional, tag = "3")]
        pub resource: ::std::option::Option<list_findings_result::Resource>,
    }
    pub mod list_findings_result {
        /// Information related to the Google Cloud resource that is
        /// associated with this finding.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Resource {
            /// The full resource name of the resource. See:
            /// https://cloud.google.com/apis/design/resource_names#full_resource_name
            #[prost(string, tag = "1")]
            pub name: std::string::String,
            /// The full resource name of project that the resource belongs to.
            #[prost(string, tag = "2")]
            pub project_name: std::string::String,
            /// The human readable name of project that the resource belongs to.
            #[prost(string, tag = "3")]
            pub project_display_name: std::string::String,
            /// The full resource name of resource's parent.
            #[prost(string, tag = "4")]
            pub parent_name: std::string::String,
            /// The human readable name of resource's parent.
            #[prost(string, tag = "5")]
            pub parent_display_name: std::string::String,
        }
        /// The change in state of the finding.
        ///
        /// When querying across two points in time this describes
        /// the change in the finding between the two points: CHANGED, UNCHANGED,
        /// ADDED, or REMOVED. Findings can not be deleted, so REMOVED implies that
        /// the finding at timestamp does not match the filter specified, but it did
        /// at timestamp - compare_duration. If there was no compare_duration
        /// supplied in the request the state change will be: UNUSED
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum StateChange {
            /// State change is unused, this is the canonical default for this enum.
            Unused = 0,
            /// The finding has changed state in some way between the points in time
            /// and existed at both points.
            Changed = 1,
            /// The finding has not changed state between the points in time and
            /// existed at both points.
            Unchanged = 2,
            /// The finding was created between the points in time.
            Added = 3,
            /// The finding at timestamp does not match the filter specified, but it
            /// did at timestamp - compare_duration.
            Removed = 4,
        }
    }
}
/// Request message for updating a finding's state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetFindingStateRequest {
    /// Required. The relative resource name of the finding. See:
    /// https://cloud.google.com/apis/design/resource_names#relative_resource_name
    /// Example:
    /// "organizations/{organization_id}/sources/{source_id}/finding/{finding_id}".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The desired State of the finding.
    #[prost(enumeration = "finding::State", tag = "2")]
    pub state: i32,
    /// Required. The time at which the updated state takes effect.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Request message for running asset discovery for an organization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunAssetDiscoveryRequest {
    /// Required. Name of the organization to run asset discovery for. Its format is
    /// "organizations/[organization_id]".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
}
/// Request message for updating or creating a finding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFindingRequest {
    /// Required. The finding resource to update or create if it does not already exist.
    /// parent, security_marks, and update_time will be ignored.
    ///
    /// In the case of creation, the finding id portion of the name must be
    /// alphanumeric and less than or equal to 32 characters and greater than 0
    /// characters in length.
    #[prost(message, optional, tag = "1")]
    pub finding: ::std::option::Option<Finding>,
    /// The FieldMask to use when updating the finding resource. This field should
    /// not be specified when creating a finding.
    ///
    /// When updating a finding, an empty mask is treated as updating all mutable
    /// fields and replacing source_properties.  Individual source_properties can
    /// be added/updated by using "source_properties.<property key>" in the field
    /// mask.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating a notification config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNotificationConfigRequest {
    /// Required. The notification config to update.
    #[prost(message, optional, tag = "1")]
    pub notification_config: ::std::option::Option<NotificationConfig>,
    /// The FieldMask to use when updating the notification config.
    ///
    /// If empty all mutable fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating an organization's settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrganizationSettingsRequest {
    /// Required. The organization settings resource to update.
    #[prost(message, optional, tag = "1")]
    pub organization_settings: ::std::option::Option<OrganizationSettings>,
    /// The FieldMask to use when updating the settings resource.
    ///
    ///  If empty all mutable fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating a source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSourceRequest {
    /// Required. The source resource to update.
    #[prost(message, optional, tag = "1")]
    pub source: ::std::option::Option<Source>,
    /// The FieldMask to use when updating the source resource.
    ///
    /// If empty all mutable fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating a SecurityMarks resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSecurityMarksRequest {
    /// Required. The security marks resource to update.
    #[prost(message, optional, tag = "1")]
    pub security_marks: ::std::option::Option<SecurityMarks>,
    /// The FieldMask to use when updating the security marks resource.
    ///
    /// The field mask must not contain duplicate fields.
    /// If empty or set to "marks", all marks will be replaced.  Individual
    /// marks can be updated using "marks.<mark_key>".
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The time at which the updated SecurityMarks take effect.
    /// If not set uses current server time.  Updates will be applied to the
    /// SecurityMarks that are active immediately preceding this time.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
}
#[doc = r" Generated client implementations."]
pub mod security_center_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " V1p1Beta1 APIs for Security Center service."]
    pub struct SecurityCenterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SecurityCenterClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Creates a source."]
        pub async fn create_source(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSourceRequest>,
        ) -> Result<tonic::Response<super::Source>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/CreateSource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a finding. The corresponding source must exist for finding"]
        #[doc = " creation to succeed."]
        pub async fn create_finding(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFindingRequest>,
        ) -> Result<tonic::Response<super::Finding>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/CreateFinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a notification config."]
        pub async fn create_notification_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNotificationConfigRequest>,
        ) -> Result<tonic::Response<super::NotificationConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/CreateNotificationConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a notification config."]
        pub async fn delete_notification_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNotificationConfigRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/DeleteNotificationConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy on the specified Source."]
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a notification config."]
        pub async fn get_notification_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNotificationConfigRequest>,
        ) -> Result<tonic::Response<super::NotificationConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/GetNotificationConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the settings for an organization."]
        pub async fn get_organization_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrganizationSettingsRequest>,
        ) -> Result<tonic::Response<super::OrganizationSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/GetOrganizationSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a source."]
        pub async fn get_source(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSourceRequest>,
        ) -> Result<tonic::Response<super::Source>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/GetSource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Filters an organization's assets and  groups them by their specified"]
        #[doc = " properties."]
        pub async fn group_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupAssetsRequest>,
        ) -> Result<tonic::Response<super::GroupAssetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/GroupAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Filters an organization or source's findings and  groups them by their"]
        #[doc = " specified properties."]
        #[doc = ""]
        #[doc = " To group across all sources provide a `-` as the source id."]
        #[doc = " Example: /v1p1beta1/organizations/{organization_id}/sources/-/findings"]
        pub async fn group_findings(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupFindingsRequest>,
        ) -> Result<tonic::Response<super::GroupFindingsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/GroupFindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists an organization's assets."]
        pub async fn list_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAssetsRequest>,
        ) -> Result<tonic::Response<super::ListAssetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/ListAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists an organization or source's findings."]
        #[doc = ""]
        #[doc = " To list across all sources provide a `-` as the source id."]
        #[doc = " Example: /v1p1beta1/organizations/{organization_id}/sources/-/findings"]
        pub async fn list_findings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFindingsRequest>,
        ) -> Result<tonic::Response<super::ListFindingsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/ListFindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists notification configs."]
        pub async fn list_notification_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNotificationConfigsRequest>,
        ) -> Result<tonic::Response<super::ListNotificationConfigsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/ListNotificationConfigs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all sources belonging to an organization."]
        pub async fn list_sources(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSourcesRequest>,
        ) -> Result<tonic::Response<super::ListSourcesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/ListSources",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Runs asset discovery. The discovery is tracked with a long-running"]
        #[doc = " operation."]
        #[doc = ""]
        #[doc = " This API can only be called with limited frequency for an organization. If"]
        #[doc = " it is called too frequently the caller will receive a TOO_MANY_REQUESTS"]
        #[doc = " error."]
        pub async fn run_asset_discovery(
            &mut self,
            request: impl tonic::IntoRequest<super::RunAssetDiscoveryRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/RunAssetDiscovery",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the state of a finding."]
        pub async fn set_finding_state(
            &mut self,
            request: impl tonic::IntoRequest<super::SetFindingStateRequest>,
        ) -> Result<tonic::Response<super::Finding>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/SetFindingState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy on the specified Source."]
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the permissions that a caller has on the specified source."]
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates or updates a finding. The corresponding source must exist for a"]
        #[doc = " finding creation to succeed."]
        pub async fn update_finding(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFindingRequest>,
        ) -> Result<tonic::Response<super::Finding>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/UpdateFinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a notification config. The following update"]
        #[doc = " fields are allowed: description, pubsub_topic, streaming_config.filter"]
        pub async fn update_notification_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNotificationConfigRequest>,
        ) -> Result<tonic::Response<super::NotificationConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/UpdateNotificationConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an organization's settings."]
        pub async fn update_organization_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOrganizationSettingsRequest>,
        ) -> Result<tonic::Response<super::OrganizationSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/UpdateOrganizationSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a source."]
        pub async fn update_source(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSourceRequest>,
        ) -> Result<tonic::Response<super::Source>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/UpdateSource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates security marks."]
        pub async fn update_security_marks(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSecurityMarksRequest>,
        ) -> Result<tonic::Response<super::SecurityMarks>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securitycenter.v1p1beta1.SecurityCenter/UpdateSecurityMarks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for SecurityCenterClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SecurityCenterClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SecurityCenterClient {{ ... }}")
        }
    }
}
