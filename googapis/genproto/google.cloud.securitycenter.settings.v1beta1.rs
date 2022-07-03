// If this field is populated and billing_tier is STANDARD, this is
// indication of a point in the _past_ when a PREMIUM access ended.

/// Billing settings
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BillingSettings {
    /// Output only. Billing tier selected by customer
    #[prost(enumeration = "BillingTier", tag = "1")]
    pub billing_tier: i32,
    /// Output only. Type of billing method
    #[prost(enumeration = "BillingType", tag = "2")]
    pub billing_type: i32,
    /// Output only. The absolute point in time when the subscription became effective.
    /// Can be compared to expire_time value to determine full contract duration
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The absolute point in time when the subscription expires.
    ///
    /// If this field is populated and billing_tier is STANDARD, this is
    /// indication of a point in the _past_ when a PREMIUM access ended.
    #[prost(message, optional, tag = "4")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Billing tier options
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BillingTier {
    /// Default value. This value is unused.
    Unspecified = 0,
    /// The standard billing tier.
    Standard = 1,
    /// The premium billing tier.
    Premium = 2,
}
/// Billing type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BillingType {
    /// Default billing type
    Unspecified = 0,
    /// Subscription for Premium billing tier
    Subscription = 1,
    /// Trial subscription for Premium billing tier
    TrialSubscription = 2,
    /// Alpha customer for Premium billing tier
    Alpha = 3,
}
/// Component Settings for Security Command Center
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComponentSettings {
    /// The relative resource name of the component settings.
    /// Formats:
    ///  * `organizations/{organization}/components/{component}/settings`
    ///  * `folders/{folder}/components/{component}/settings`
    ///  * `projects/{project}/components/{component}/settings`
    ///  * `projects/{project}/locations/{location}/clusters/{cluster}/components/{component}/settings`
    ///  * `projects/{project}/regions/{region}/clusters/{cluster}/components/{component}/settings`
    ///  * `projects/{project}/zones/{zone}/clusters/{cluster}/components/{component}/settings`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// ENABLE to enable component, DISABLE to disable and INHERIT to inherit
    /// setting from ancestors.
    #[prost(enumeration = "ComponentEnablementState", tag = "2")]
    pub state: i32,
    /// Output only. The service account to be used for security center component.
    /// The component must have permission to "act as" the service account.
    #[prost(string, tag = "3")]
    pub project_service_account: ::prost::alloc::string::String,
    /// Settings for detectors.  Not all detectors must have settings present at
    /// each and every level in the hierarchy.  If it is not present the setting
    /// will be inherited from its ancestors folders, organizations or the
    /// defaults.
    #[prost(map = "string, message", tag = "4")]
    pub detector_settings: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        component_settings::DetectorSettings,
    >,
    /// Output only. An fingerprint used for optimistic concurrency. If none is provided
    /// on updates then the existing metadata will be blindly overwritten.
    #[prost(string, tag = "5")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. The time these settings were last updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Component specific settings.  This must match the component value.
    #[prost(
        oneof = "component_settings::SpecificSettings",
        tags = "41, 42, 44, 40"
    )]
    pub specific_settings: ::core::option::Option<component_settings::SpecificSettings>,
}
/// Nested message and enum types in `ComponentSettings`.
pub mod component_settings {
    /// Settings for each detector.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DetectorSettings {
        /// ENABLE to enable component, DISABLE to disable and INHERIT to inherit
        /// setting from ancestors.
        #[prost(enumeration = "super::ComponentEnablementState", tag = "1")]
        pub state: i32,
    }
    /// Component specific settings.  This must match the component value.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SpecificSettings {
        /// Container Threate Detection specific settings
        /// For component, expect CONTAINER_THREAT_DETECTION
        #[prost(message, tag = "41")]
        ContainerThreatDetectionSettings(super::ContainerThreatDetectionSettings),
        /// Event Threat Detection specific settings
        /// For component, expect EVENT_THREAT_DETECTION
        #[prost(message, tag = "42")]
        EventThreatDetectionSettings(super::EventThreatDetectionSettings),
        /// Security Health Analytics specific settings
        /// For component, expect SECURITY_HEALTH_ANALYTICS
        #[prost(message, tag = "44")]
        SecurityHealthAnalyticsSettings(super::SecurityHealthAnalyticsSettings),
        /// Web Security Scanner specific settings
        /// For component, expect WEB_SECURITY_SCANNER
        #[prost(message, tag = "40")]
        WebSecurityScannerSettings(super::WebSecurityScanner),
    }
}
/// User specified settings for Web Security Scanner
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebSecurityScanner {}
/// User specified settings for KTD
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerThreatDetectionSettings {}
/// User specified settings for ETD
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventThreatDetectionSettings {}
/// User specified settings for Security Health Analytics
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityHealthAnalyticsSettings {
    /// Settings for "NON_ORG_IAM_MEMBER" scanner.
    #[prost(message, optional, tag = "1")]
    pub non_org_iam_member_settings:
        ::core::option::Option<security_health_analytics_settings::NonOrgIamMemberSettings>,
    /// Settings for "ADMIN_SERVICE_ACCOUNT" scanner.
    #[prost(message, optional, tag = "2")]
    pub admin_service_account_settings:
        ::core::option::Option<security_health_analytics_settings::AdminServiceAccountSettings>,
}
/// Nested message and enum types in `SecurityHealthAnalyticsSettings`.
pub mod security_health_analytics_settings {
    /// Settings for "NON_ORG_IAM_MEMBER" scanner.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NonOrgIamMemberSettings {
        /// User emails ending in the provided identities are allowed to have IAM
        /// permissions on a project or the organization. Otherwise a finding will
        /// be created.
        /// A valid identity can be:
        ///   *  a domain that starts with "@", e.g. "@yourdomain.com".
        ///   *  a fully specified email address that does not start with "@", e.g.
        ///   "abc@gmail.com"
        /// Regular expressions are not supported.
        /// Service accounts are not examined by the scanner and will be omitted if
        /// added to the list.
        /// If not specified, only Gmail accounts will be considered as non-approved.
        #[prost(string, repeated, tag = "1")]
        pub approved_identities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Settings for "ADMIN_SERVICE_ACCOUNT" scanner.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AdminServiceAccountSettings {
        /// User-created service accounts ending in the provided identities are
        /// allowed to have Admin, Owner or Editor roles granted to them. Otherwise
        /// a finding will be created.
        /// A valid identity can be:
        ///   *  a partilly specified service account that starts with "@", e.g.
        ///   "@myproject.iam.gserviceaccount.com". This approves all the service
        ///   accounts suffixed with the specified identity.
        ///   *  a fully specified service account that does not start with "@", e.g.
        ///   "myadmin@myproject.iam.gserviceaccount.com".
        /// Google-created service accounts are all approved.
        #[prost(string, repeated, tag = "1")]
        pub approved_identities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Valid states for a component
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ComponentEnablementState {
    /// No state specified, equivalent of INHERIT
    Unspecified = 0,
    /// Disable the component
    Disable = 1,
    /// Enable the component
    Enable = 2,
    /// Inherit the state from resources parent folder or organization.
    Inherit = 3,
}
/// Detector is a set of detectors or scanners act as individual checks done
/// within a component e.g. bad IP, bad domains, IAM anomaly, cryptomining, open
/// firewall, etc. Detector is independent of Organization, meaning each detector
/// must be defined for a given Security Center component under a specified
/// billing tier. Organizations can configure the list of detectors based on
/// their subscribed billing tier.
///
/// Defines a detector, its billing tier and any applicable labels.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Detector {
    /// Output only. Detector Identifier
    #[prost(string, tag = "1")]
    pub detector: ::prost::alloc::string::String,
    /// Output only. Component that supports detector type.  Multiple components may support the
    /// same detector.
    #[prost(string, tag = "2")]
    pub component: ::prost::alloc::string::String,
    /// Output only. The billing tier may be different for a detector of the same name in
    /// another component.
    #[prost(enumeration = "BillingTier", tag = "3")]
    pub billing_tier: i32,
    /// Output only. Google curated detector labels. These are alphanumeric tags that are not
    /// necessarily human readable. Labels can be used to group detectors together
    /// in the future. An example might be tagging all detectors “PCI” that help
    /// with PCI compliance.
    #[prost(string, repeated, tag = "4")]
    pub detector_labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Sink Settings for Security Command Center
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SinkSettings {
    /// The resource name of the project to send logs to. This project must be
    /// part of the same organization where the Security Center API is
    /// enabled. The format is `projects/{project}`. If it is empty, we do
    /// not output logs. If a project ID is provided it will be normalized to a
    /// project number.
    #[prost(string, tag = "1")]
    pub logging_sink_project: ::prost::alloc::string::String,
}
/// Common configuration settings for all of Security Center.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Settings {
    /// The relative resource name of the settings resource.
    /// Formats:
    ///  * `organizations/{organization}/settings`
    ///  * `folders/{folder}/settings`
    ///  * `projects/{project}/settings`
    ///  * `projects/{project}/locations/{location}/clusters/{cluster}/settings`
    ///  * `projects/{project}/regions/{region}/clusters/{cluster}/settings`
    ///  * `projects/{project}/zones/{zone}/clusters/{cluster}/settings`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Billing settings
    #[prost(message, optional, tag = "2")]
    pub billing_settings: ::core::option::Option<BillingSettings>,
    /// An enum representing the current on boarding state of SCC.
    #[prost(enumeration = "settings::OnboardingState", tag = "3")]
    pub state: i32,
    /// Output only. The organization-level service account to be used for security center
    /// components. The component must have permission to "act as" the service
    /// account.
    #[prost(string, tag = "5")]
    pub org_service_account: ::prost::alloc::string::String,
    /// Sink settings.
    #[prost(message, optional, tag = "6")]
    pub sink_settings: ::core::option::Option<SinkSettings>,
    /// The settings for detectors and/or scanners.
    #[prost(map = "string, message", tag = "7")]
    pub component_settings:
        ::std::collections::HashMap<::prost::alloc::string::String, ComponentSettings>,
    /// Detector group settings for all Security Center components.
    /// The key is the name of the detector group and the value is the settings for
    /// that group.
    #[prost(map = "string, message", tag = "8")]
    pub detector_group_settings: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        settings::DetectorGroupSettings,
    >,
    /// A fingerprint used for optimistic concurrency. If none is provided
    /// on updates then the existing metadata will be blindly overwritten.
    #[prost(string, tag = "9")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. The time these settings were last updated.
    #[prost(message, optional, tag = "10")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Settings`.
pub mod settings {
    /// The DetectorGroupSettings define the configuration for a detector group.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DetectorGroupSettings {
        /// The state determines if the group is enabled or not.
        #[prost(enumeration = "super::ComponentEnablementState", tag = "1")]
        pub state: i32,
    }
    /// Defines the onboarding states for SCC
    ///
    /// Potentially is just an indicator that a user has reviewed some subset of
    /// our configuration surface, even if it's still currently set to its
    /// API-default state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OnboardingState {
        /// No onboarding state has been set. Should not be seen in practice, but
        /// should be functionally equivalent to DISABLED.
        Unspecified = 0,
        /// SCC is fully on boarded
        Enabled = 1,
        /// SCC has been disabled after being on boarded
        Disabled = 2,
        /// SCC's onboarding tier has been explicitly set
        BillingSelected = 3,
        /// SCC's CTD FindingsProviders have been chosen
        ProvidersSelected = 4,
        /// SCC's Service-Resource mappings have been set
        ResourcesSelected = 5,
        /// SCC's core Service Account was created
        OrgServiceAccountCreated = 6,
    }
}
/// Request message for GetServiceAccount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceAccountRequest {
    /// Required. The relative resource name of the service account resource.
    /// Format:
    ///  * `organizations/{organization}/serviceAccount`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// An organization-level service account to be used by threat detection
/// components.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccount {
    /// The relative resource name of the service account resource.
    /// Format:
    ///  * `organizations/{organization}/serviceAccount`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Security Center managed service account for the organization
    /// example service-org-1234@scc.iam.gserviceaccount.com
    /// This service_account will be stored in the ComponentSettings field for the
    /// SCC, SHA, and Infra Automation components.
    #[prost(string, tag = "2")]
    pub service_account: ::prost::alloc::string::String,
}
/// Request message for GetSettings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSettingsRequest {
    /// Required. The name of the settings to retrieve.
    /// Formats:
    ///  * `organizations/{organization}/settings`
    ///  * `folders/{folder}/settings`
    ///  * `projects/{project}/settings`
    ///  * `projects/{project}/locations/{location}/clusters/{cluster}/settings`
    ///  * `projects/{project}/regions/{region}/clusters/{cluster}/settings`
    ///  * `projects/{project}/zones/{zone}/clusters/{cluster}/settings`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateSettings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSettingsRequest {
    /// Required. The settings to update.
    ///
    /// The settings' `name` field is used to identify the settings to be updated.
    /// Formats:
    ///  * `organizations/{organization}/settings`
    ///  * `folders/{folder}/settings`
    ///  * `projects/{project}/settings`
    ///  * `projects/{project}/locations/{location}/clusters/{cluster}/settings`
    ///  * `projects/{project}/regions/{region}/clusters/{cluster}/settings`
    ///  * `projects/{project}/zones/{zone}/clusters/{cluster}/settings`
    #[prost(message, optional, tag = "1")]
    pub settings: ::core::option::Option<Settings>,
    /// The list of fields to be updated on the settings.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ResetSettings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetSettingsRequest {
    /// Required. The name of the settings to reset.
    /// Formats:
    ///  * `organizations/{organization}/settings`
    ///  * `folders/{folder}/settings`
    ///  * `projects/{project}/settings`
    ///  * `projects/{project}/locations/{location}/clusters/{cluster}/settings`
    ///  * `projects/{project}/regions/{region}/clusters/{cluster}/settings`
    ///  * `projects/{project}/zones/{zone}/clusters/{cluster}/settings`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A fingerprint used for optimistic concurrency. If none is provided,
    /// then the existing settings will be blindly overwritten.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for BatchGetSettings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetSettingsRequest {
    /// Required. The relative resource name of the organization shared by all of the
    /// settings being retrieved.
    /// Format:
    ///  * `organizations/{organization}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The names of the settings to retrieve.
    /// A maximum of 1000 settings can be retrieved in a batch.
    /// Formats:
    ///  * `organizations/{organization}/settings`
    ///  * `folders/{folder}/settings`
    ///  * `projects/{project}/settings`
    ///  * `projects/{project}/locations/{location}/clusters/{cluster}/settings`
    ///  * `projects/{project}/regions/{region}/clusters/{cluster}/settings`
    ///  * `projects/{project}/zones/{zone}/clusters/{cluster}/settings`
    #[prost(string, repeated, tag = "2")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response message for BatchGetSettings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetSettingsResponse {
    /// Settings requested.
    #[prost(message, repeated, tag = "1")]
    pub settings: ::prost::alloc::vec::Vec<Settings>,
}
/// Request message for CalculateEffectiveSettings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculateEffectiveSettingsRequest {
    /// Required. The name of the effective settings to retrieve.
    /// Formats:
    ///  * `organizations/{organization}/effectiveSettings`
    ///  * `folders/{folder}/effectiveSettings`
    ///  * `projects/{project}/effectiveSettings`
    ///  * `projects/{project}/locations/{location}/clusters/{cluster}/effectiveSettings`
    ///  * `projects/{project}/regions/{region}/clusters/{cluster}/effectiveSettings`
    ///  * `projects/{project}/zones/{zone}/clusters/{cluster}/effectiveSettings`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for BatchGetEffectiveSettings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCalculateEffectiveSettingsRequest {
    /// Required. The relative resource name of the organization shared by all of the
    /// settings being retrieved.
    /// Format:
    ///  * `organizations/{organization}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The requests specifying the effective settings to retrieve.
    /// A maximum of 1000 effective settings can be retrieved in a batch.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<CalculateEffectiveSettingsRequest>,
}
/// Response message for BatchGetEffectiveSettings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCalculateEffectiveSettingsResponse {
    /// Settings requested.
    #[prost(message, repeated, tag = "1")]
    pub settings: ::prost::alloc::vec::Vec<Settings>,
}
/// Request message for GetComponentSettings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetComponentSettingsRequest {
    /// Required. The component settings to retrieve.
    ///
    /// Formats:
    ///  * `organizations/{organization}/components/{component}/settings`
    ///  * `folders/{folder}/components/{component}/settings`
    ///  * `projects/{project}/components/{component}/settings`
    ///  * `projects/{project}/locations/{location}/clusters/{cluster}/components/{component}/settings`
    ///  * `projects/{project}/regions/{region}/clusters/{cluster}/components/{component}/settings`
    ///  * `projects/{project}/zones/{zone}/clusters/{cluster}/components/{component}/settings`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateComponentSettings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateComponentSettingsRequest {
    /// Required. The component settings to update.
    ///
    /// The component settings' `name` field is used to identify the component
    /// settings to be updated. Formats:
    ///  * `organizations/{organization}/components/{component}/settings`
    ///  * `folders/{folder}/components/{component}/settings`
    ///  * `projects/{project}/components/{component}/settings`
    ///  * `projects/{project}/locations/{location}/clusters/{cluster}/components/{component}/settings`
    ///  * `projects/{project}/regions/{region}/clusters/{cluster}/components/{component}/settings`
    ///  * `projects/{project}/zones/{zone}/clusters/{cluster}/components/{component}/settings`
    #[prost(message, optional, tag = "1")]
    pub component_settings: ::core::option::Option<ComponentSettings>,
    /// The list of fields to be updated on the component settings resource.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ResetComponentSettings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetComponentSettingsRequest {
    /// Required. The component settings to reset.
    ///
    /// Formats:
    ///  * `organizations/{organization}/components/{component}/settings`
    ///  * `folders/{folder}/components/{component}/settings`
    ///  * `projects/{project}/components/{component}/settings`
    ///  * `projects/{project}/locations/{location}/clusters/{cluster}/components/{component}/settings`
    ///  * `projects/{project}/regions/{region}/clusters/{cluster}/components/{component}/settings`
    ///  * `projects/{project}/zones/{zone}/clusters/{cluster}/components/{component}/settings`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// An fingerprint used for optimistic concurrency. If none is provided,
    /// then the existing settings will be blindly overwritten.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for CalculateEffectiveComponentSettings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculateEffectiveComponentSettingsRequest {
    /// Required. The effective component settings to retrieve.
    ///
    /// Formats:
    ///  * `organizations/{organization}/components/{component}/settings`
    ///  * `folders/{folder}/components/{component}/settings`
    ///  * `projects/{project}/components/{component}/settings`
    ///  * `projects/{project}/locations/{location}/clusters/{cluster}/components/{component}/settings`
    ///  * `projects/{project}/regions/{region}/clusters/{cluster}/components/{component}/settings`
    ///  * `projects/{project}/zones/{zone}/clusters/{cluster}/components/{component}/settings`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListDetectors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDetectorsRequest {
    /// Required. The parent, which owns this collection of detectors.
    /// Format:
    ///  * `organizations/{organization}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Filters to apply on the response. Filters can be applied on:
    ///  * components
    ///  * labels
    ///  * billing tiers
    ///
    /// Component filters will retrieve only detectors for the components
    /// specified. Label filters will retrieve only detectors that match one of the
    /// labels specified. Billing tier filters will retrieve only detectors for
    /// that billing tier.
    ///
    /// The filters
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The maximum number of detectors to return. The service may return fewer
    /// than this value. If unspecified, at most 100 detectors will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// A page token, received from a previous `ListDetectors` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListDetectors` must
    /// match the call that provided the page token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListDetectors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDetectorsResponse {
    /// The detectors from the specified organization.
    #[prost(message, repeated, tag = "1")]
    pub detectors: ::prost::alloc::vec::Vec<Detector>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for ListComponents.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListComponentsRequest {
    /// Required. The parent, which owns this collection of components.
    /// Format:
    ///  * `organizations/{organization}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of components to return. The service may return fewer
    /// than this value. If unspecified, at most 100 components will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListComponents` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListComponents` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListComponents.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListComponentsResponse {
    /// The components from the specified organization.
    #[prost(string, repeated, tag = "1")]
    pub components: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod security_center_settings_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " ## API Overview"]
    #[doc = ""]
    #[doc = " The SecurityCenterSettingsService is a sub-api of"]
    #[doc = " `securitycenter.googleapis.com`. The service provides methods to manage"]
    #[doc = " Security Center Settings, and Component Settings for GCP organizations,"]
    #[doc = " folders, projects, and clusters."]
    #[derive(Debug, Clone)]
    pub struct SecurityCenterSettingsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SecurityCenterSettingsServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SecurityCenterSettingsServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            SecurityCenterSettingsServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Retrieves the organizations service account, if it exists, otherwise it"]
        #[doc = " creates the organization service account. This API is idempotent and"]
        #[doc = " will only create a service account once. On subsequent calls it will"]
        #[doc = " return the previously created service account.  SHA, SCC and CTD Infra"]
        #[doc = " Automation will use this SA.  This SA will not have any permissions when"]
        #[doc = " created.  The UI will provision this via IAM or the user will using"]
        #[doc = " their own internal process. This API only creates SAs on the organization."]
        #[doc = " Folders are not supported and projects will use per-project SAs associated"]
        #[doc = " with APIs enabled on a project. This API will be called by the UX"]
        #[doc = " onboarding workflow."]
        pub async fn get_service_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceAccountRequest>,
        ) -> Result<tonic::Response<super::ServiceAccount>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.securitycenter.settings.v1beta1.SecurityCenterSettingsService/GetServiceAccount") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the Settings."]
        pub async fn get_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSettingsRequest>,
        ) -> Result<tonic::Response<super::Settings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.securitycenter.settings.v1beta1.SecurityCenterSettingsService/GetSettings") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the Settings."]
        pub async fn update_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSettingsRequest>,
        ) -> Result<tonic::Response<super::Settings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.securitycenter.settings.v1beta1.SecurityCenterSettingsService/UpdateSettings") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Reset the organization, folder or project's settings and return"]
        #[doc = " the settings of just that resource to the default."]
        #[doc = ""]
        #[doc = " Settings are present at the organization, folder, project, and cluster"]
        #[doc = " levels. Using Reset on a sub-organization level will remove that resource's"]
        #[doc = " override and result in the parent's settings being used (eg: if Reset on a"]
        #[doc = " cluster, project settings will be used)."]
        #[doc = ""]
        #[doc = " Using Reset on organization will remove the override that was set and"]
        #[doc = " result in default settings being used."]
        pub async fn reset_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetSettingsRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.securitycenter.settings.v1beta1.SecurityCenterSettingsService/ResetSettings") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a list of settings."]
        pub async fn batch_get_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetSettingsRequest>,
        ) -> Result<tonic::Response<super::BatchGetSettingsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.securitycenter.settings.v1beta1.SecurityCenterSettingsService/BatchGetSettings") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " CalculateEffectiveSettings looks up all of the Security Center"]
        #[doc = " Settings resources in the GCP resource hierarchy, and calculates the"]
        #[doc = " effective settings on that resource by applying the following rules:"]
        #[doc = "  * Settings provided closer to the target resource take precedence over"]
        #[doc = "    those further away (e.g. folder will override organization level"]
        #[doc = "    settings)."]
        #[doc = "  * Product defaults can be overridden at org, folder, project, and cluster"]
        #[doc = "  levels."]
        #[doc = "  * Detectors will be filtered out if they belong to a billing tier the"]
        #[doc = "  customer"]
        #[doc = "    has not configured."]
        pub async fn calculate_effective_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::CalculateEffectiveSettingsRequest>,
        ) -> Result<tonic::Response<super::Settings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.securitycenter.settings.v1beta1.SecurityCenterSettingsService/CalculateEffectiveSettings") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a list of effective settings."]
        pub async fn batch_calculate_effective_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCalculateEffectiveSettingsRequest>,
        ) -> Result<tonic::Response<super::BatchCalculateEffectiveSettingsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.securitycenter.settings.v1beta1.SecurityCenterSettingsService/BatchCalculateEffectiveSettings") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the Component Settings."]
        pub async fn get_component_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetComponentSettingsRequest>,
        ) -> Result<tonic::Response<super::ComponentSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.securitycenter.settings.v1beta1.SecurityCenterSettingsService/GetComponentSettings") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the Component Settings."]
        pub async fn update_component_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateComponentSettingsRequest>,
        ) -> Result<tonic::Response<super::ComponentSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.securitycenter.settings.v1beta1.SecurityCenterSettingsService/UpdateComponentSettings") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Reset the organization, folder or project's component settings and return"]
        #[doc = " the settings to the default. Settings are present at the"]
        #[doc = " organization, folder and project levels. Using Reset for a folder or"]
        #[doc = " project will remove the override that was set and result in the"]
        #[doc = " organization-level settings being used."]
        pub async fn reset_component_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetComponentSettingsRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.securitycenter.settings.v1beta1.SecurityCenterSettingsService/ResetComponentSettings") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the Effective Component Settings."]
        pub async fn calculate_effective_component_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::CalculateEffectiveComponentSettingsRequest>,
        ) -> Result<tonic::Response<super::ComponentSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.securitycenter.settings.v1beta1.SecurityCenterSettingsService/CalculateEffectiveComponentSettings") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves an unordered list of available detectors."]
        pub async fn list_detectors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDetectorsRequest>,
        ) -> Result<tonic::Response<super::ListDetectorsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.securitycenter.settings.v1beta1.SecurityCenterSettingsService/ListDetectors") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves an unordered list of available SCC components."]
        pub async fn list_components(
            &mut self,
            request: impl tonic::IntoRequest<super::ListComponentsRequest>,
        ) -> Result<tonic::Response<super::ListComponentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.securitycenter.settings.v1beta1.SecurityCenterSettingsService/ListComponents") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
