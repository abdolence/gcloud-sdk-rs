/// A resource message representing a Google Analytics account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// Output only. Resource name of this account.
    /// Format: accounts/{account}
    /// Example: "accounts/100"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Time when this account was originally created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when account payload fields were last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. Human-readable display name for this account.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// Country of business. Must be a Unicode CLDR region code.
    #[prost(string, tag = "5")]
    pub region_code: ::prost::alloc::string::String,
    /// Output only. Indicates whether this Account is soft-deleted or not. Deleted
    /// accounts are excluded from List results unless specifically requested.
    #[prost(bool, tag = "6")]
    pub deleted: bool,
}
/// A resource message representing a Google Analytics GA4 property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Property {
    /// Output only. Resource name of this property.
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Time when the entity was originally created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when entity payload fields were last updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Immutable. Resource name of this property's logical parent.
    ///
    /// Note: The Property-Moving UI can be used to change the parent.
    /// Format: accounts/{account}
    /// Example: "accounts/100"
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Human-readable display name for this property.
    ///
    /// The max allowed display name length is 100 UTF-16 code units.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// Industry associated with this property
    /// Example: AUTOMOTIVE, FOOD_AND_DRINK
    #[prost(enumeration = "IndustryCategory", tag = "6")]
    pub industry_category: i32,
    /// Reporting Time Zone, used as the day boundary for reports, regardless of
    /// where the data originates. If the time zone honors DST, Analytics will
    /// automatically adjust for the changes.
    ///
    /// NOTE: Changing the time zone only affects data going forward, and is not
    /// applied retroactively.
    ///
    /// Format: https://www.iana.org/time-zones
    /// Example: "America/Los_Angeles"
    #[prost(string, tag = "7")]
    pub time_zone: ::prost::alloc::string::String,
    /// The currency type used in reports involving monetary values.
    ///
    ///
    /// Format: https://en.wikipedia.org/wiki/ISO_4217
    /// Examples: "USD", "EUR", "JPY"
    #[prost(string, tag = "8")]
    pub currency_code: ::prost::alloc::string::String,
    /// Output only. Indicates whether this Property is soft-deleted or not. Deleted properties
    /// are excluded from List results unless specifically requested.
    #[prost(bool, tag = "9")]
    pub deleted: bool,
}
/// A resource message representing a Google Analytics Android app stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidAppDataStream {
    /// Output only. Resource name of this Data Stream.
    /// Format: properties/{property_id}/androidAppDataStreams/{stream_id}
    /// Example: "properties/1000/androidAppDataStreams/2000"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. ID of the corresponding Android app in Firebase, if any.
    /// This ID can change if the Android app is deleted and recreated.
    #[prost(string, tag = "2")]
    pub firebase_app_id: ::prost::alloc::string::String,
    /// Output only. Time when this stream was originally created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when stream payload fields were last updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Immutable. The package name for the app being measured.
    /// Example: "com.example.myandroidapp"
    #[prost(string, tag = "5")]
    pub package_name: ::prost::alloc::string::String,
    /// Human-readable display name for the Data Stream.
    ///
    /// The max allowed display name length is 255 UTF-16 code units.
    #[prost(string, tag = "6")]
    pub display_name: ::prost::alloc::string::String,
}
/// A resource message representing a Google Analytics IOS app stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosAppDataStream {
    /// Output only. Resource name of this Data Stream.
    /// Format: properties/{property_id}/iosAppDataStreams/{stream_id}
    /// Example: "properties/1000/iosAppDataStreams/2000"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. ID of the corresponding iOS app in Firebase, if any.
    /// This ID can change if the iOS app is deleted and recreated.
    #[prost(string, tag = "2")]
    pub firebase_app_id: ::prost::alloc::string::String,
    /// Output only. Time when this stream was originally created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when stream payload fields were last updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. Immutable. The Apple App Store Bundle ID for the app
    /// Example: "com.example.myiosapp"
    #[prost(string, tag = "5")]
    pub bundle_id: ::prost::alloc::string::String,
    /// Human-readable display name for the Data Stream.
    ///
    /// The max allowed display name length is 255 UTF-16 code units.
    #[prost(string, tag = "6")]
    pub display_name: ::prost::alloc::string::String,
}
/// A resource message representing a Google Analytics web stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebDataStream {
    /// Output only. Resource name of this Data Stream.
    /// Format: properties/{property_id}/webDataStreams/{stream_id}
    /// Example: "properties/1000/webDataStreams/2000"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Analytics "Measurement ID", without the "G-" prefix.
    /// Example: "G-1A2BCD345E" would just be "1A2BCD345E"
    #[prost(string, tag = "2")]
    pub measurement_id: ::prost::alloc::string::String,
    /// Output only. ID of the corresponding web app in Firebase, if any.
    /// This ID can change if the web app is deleted and recreated.
    #[prost(string, tag = "3")]
    pub firebase_app_id: ::prost::alloc::string::String,
    /// Output only. Time when this stream was originally created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when stream payload fields were last updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Immutable. Domain name of the web app being measured, or empty.
    /// Example: "http://www.google.com", "https://www.google.com"
    #[prost(string, tag = "6")]
    pub default_uri: ::prost::alloc::string::String,
    /// Required. Human-readable display name for the Data Stream.
    ///
    /// The max allowed display name length is 100 UTF-16 code units.
    #[prost(string, tag = "7")]
    pub display_name: ::prost::alloc::string::String,
}
/// A resource message representing a user's permissions on an Account or
/// Property resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserLink {
    /// Example format: properties/1234/userLinks/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Email address of the user to link
    #[prost(string, tag = "2")]
    pub email_address: ::prost::alloc::string::String,
    /// Roles directly assigned to this user for this account or property.
    ///
    /// Valid values:
    /// predefinedRoles/read
    /// predefinedRoles/collaborate
    /// predefinedRoles/edit
    /// predefinedRoles/manage-users
    ///
    /// Excludes roles that are inherited from a higher-level entity, group,
    /// or organization admin role.
    ///
    /// A UserLink that is updated to have an empty list of direct_roles will be
    /// deleted.
    #[prost(string, repeated, tag = "3")]
    pub direct_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Read-only resource used to summarize a principal's effective roles.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditUserLink {
    /// Example format: properties/1234/userLinks/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Email address of the linked user
    #[prost(string, tag = "2")]
    pub email_address: ::prost::alloc::string::String,
    /// Roles directly assigned to this user for this entity.
    ///
    /// Format: predefinedRoles/read
    ///
    /// Excludes roles that are inherited from an account (if this is for a
    /// property), group, or organization admin role.
    #[prost(string, repeated, tag = "3")]
    pub direct_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Union of all permissions a user has at this account or property (includes
    /// direct permissions, group-inherited permissions, etc.).
    ///
    /// Format: predefinedRoles/read
    #[prost(string, repeated, tag = "4")]
    pub effective_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Singleton resource under a WebDataStream, configuring measurement of
/// additional site interactions and content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnhancedMeasurementSettings {
    /// Output only. Resource name of this Data Stream.
    /// Format:
    /// properties/{property_id}/webDataStreams/{stream_id}/enhancedMeasurementSettings
    /// Example: "properties/1000/webDataStreams/2000/enhancedMeasurementSettings"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Indicates whether Enhanced Measurement Settings will be used to
    /// automatically measure interactions and content on this web stream.
    ///
    /// Changing this value does not affect the settings themselves, but determines
    /// whether they are respected.
    #[prost(bool, tag = "2")]
    pub stream_enabled: bool,
    /// Output only. If enabled, capture a page view event each time a page loads or the
    /// website changes the browser history state.
    #[prost(bool, tag = "3")]
    pub page_views_enabled: bool,
    /// If enabled, capture scroll events each time a visitor gets to the bottom of
    /// a page.
    #[prost(bool, tag = "4")]
    pub scrolls_enabled: bool,
    /// If enabled, capture an outbound click event each time a visitor clicks a
    /// link that leads them away from your domain.
    #[prost(bool, tag = "5")]
    pub outbound_clicks_enabled: bool,
    /// If enabled, capture a view search results event each time a visitor
    /// performs a search on your site (based on a query parameter).
    #[prost(bool, tag = "7")]
    pub site_search_enabled: bool,
    /// If enabled, capture video play, progress, and complete events as visitors
    /// view embedded videos on your site.
    #[prost(bool, tag = "9")]
    pub video_engagement_enabled: bool,
    /// If enabled, capture a file download event each time a link is clicked with
    /// a common document, compressed file, application, video, or audio extension.
    #[prost(bool, tag = "10")]
    pub file_downloads_enabled: bool,
    /// Output only. If enabled, capture a page view event each time a page loads.
    #[prost(bool, tag = "12")]
    pub page_loads_enabled: bool,
    /// If enabled, capture a page view event each time the website changes the
    /// browser history state.
    #[prost(bool, tag = "13")]
    pub page_changes_enabled: bool,
    /// Required. URL query parameters to interpret as site search parameters.
    /// Max length is 1024 characters. Must not be empty.
    #[prost(string, tag = "16")]
    pub search_query_parameter: ::prost::alloc::string::String,
    /// Additional URL query parameters.
    /// Max length is 1024 characters.
    #[prost(string, tag = "17")]
    pub uri_query_parameter: ::prost::alloc::string::String,
}
/// A link between an GA4 property and a Firebase project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirebaseLink {
    /// Output only. Example format: properties/1234/firebaseLinks/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. Firebase project resource name. When creating a FirebaseLink, you may
    /// provide this resource name using either a project number or project ID.
    /// Once this resource has been created, returned FirebaseLinks will always
    /// have a project_name that contains a project number.
    ///
    /// Format: 'projects/{project number}'
    /// Example: 'projects/1234'
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    /// Output only. Time when this FirebaseLink was originally created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Maximum user access to the GA4 property allowed to admins of
    /// the linked Firebase project.
    #[prost(enumeration = "MaximumUserAccess", tag = "4")]
    pub maximum_user_access: i32,
}
/// Read-only resource with the tag for sending data from a website to a
/// WebDataStream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalSiteTag {
    /// Output only. Resource name for this GlobalSiteTag resource.
    /// Format: properties/{propertyId}/globalSiteTag
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. JavaScript code snippet to be pasted as the first item into the head tag of
    /// every webpage to measure.
    #[prost(string, tag = "2")]
    pub snippet: ::prost::alloc::string::String,
}
/// A link between an GA4 property and a Google Ads account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleAdsLink {
    /// Output only. Format: properties/{propertyId}/googleAdsLinks/{googleAdsLinkId}
    ///
    /// Note: googleAdsLinkId is not the Google Ads customer ID.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. Google Ads customer ID.
    #[prost(string, tag = "3")]
    pub customer_id: ::prost::alloc::string::String,
    /// Output only. If true, this link is for a Google Ads manager account.
    #[prost(bool, tag = "4")]
    pub can_manage_clients: bool,
    /// Enable personalized advertising features with this integration.
    /// Automatically publish my Google Analytics audience lists and Google
    /// Analytics remarketing events/parameters to the linked Google Ads account.
    /// If this field is not set on create/update it will be defaulted to true.
    #[prost(message, optional, tag = "5")]
    pub ads_personalization_enabled: ::core::option::Option<bool>,
    /// Output only. Email address of the user that created the link.
    /// An empty string will be returned if the email address can't be retrieved.
    #[prost(string, tag = "6")]
    pub email_address: ::prost::alloc::string::String,
    /// Output only. Time when this link was originally created.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this link was last updated.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A resource message representing data sharing settings of a Google Analytics
/// account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSharingSettings {
    /// Output only. Resource name.
    /// Format: accounts/{account}/dataSharingSettings
    /// Example: "accounts/1000/dataSharingSettings"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Allows Google support to access the data in order to help troubleshoot
    /// issues.
    #[prost(bool, tag = "2")]
    pub sharing_with_google_support_enabled: bool,
    /// Allows Google sales teams that are assigned to the customer to access the
    /// data in order to suggest configuration changes to improve results.
    /// Sales team restrictions still apply when enabled.
    #[prost(bool, tag = "3")]
    pub sharing_with_google_assigned_sales_enabled: bool,
    /// Allows any of Google sales to access the data in order to suggest
    /// configuration changes to improve results.
    #[prost(bool, tag = "4")]
    pub sharing_with_google_any_sales_enabled: bool,
    /// Allows Google to use the data to improve other Google products or services.
    #[prost(bool, tag = "5")]
    pub sharing_with_google_products_enabled: bool,
    /// Allows Google to share the data anonymously in aggregate form with others.
    #[prost(bool, tag = "6")]
    pub sharing_with_others_enabled: bool,
}
/// A virtual resource representing an overview of an account and
/// all its child GA4 properties.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountSummary {
    /// Resource name for this account summary.
    /// Format: accountSummaries/{account_id}
    /// Example: "accountSummaries/1000"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Resource name of account referred to by this account summary
    /// Format: accounts/{account_id}
    /// Example: "accounts/1000"
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
    /// Display name for the account referred to in this account summary.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// List of summaries for child accounts of this account.
    #[prost(message, repeated, tag = "4")]
    pub property_summaries: ::prost::alloc::vec::Vec<PropertySummary>,
}
/// A virtual resource representing metadata for an GA4 property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertySummary {
    /// Resource name of property referred to by this property summary
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
    /// Display name for the property referred to in this account summary.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
}
/// Maximum access settings that Firebase user receive on the linked Analytics
/// property.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MaximumUserAccess {
    /// Unspecified maximum user access.
    Unspecified = 0,
    /// Firebase users have no access to the Analytics property.
    NoAccess = 1,
    /// Firebase users have Read & Analyze access to the Analytics property.
    ReadAndAnalyze = 2,
    /// Firebase users have edit access to the Analytics property, but may not
    /// manage the Firebase link.
    EditorWithoutLinkManagement = 3,
    /// Firebase users have edit access to the Analytics property and may manage
    /// the Firebase link.
    EditorIncludingLinkManagement = 4,
}
/// The category selected for this property, used for industry benchmarking.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IndustryCategory {
    /// Industry category unspecified
    Unspecified = 0,
    /// Automotive
    Automotive = 1,
    /// Business and industrial markets
    BusinessAndIndustrialMarkets = 2,
    /// Finance
    Finance = 3,
    /// Healthcare
    Healthcare = 4,
    /// Technology
    Technology = 5,
    /// Travel
    Travel = 6,
    /// Other
    Other = 7,
    /// Arts and entertainment
    ArtsAndEntertainment = 8,
    /// Beauty and fitness
    BeautyAndFitness = 9,
    /// Books and literature
    BooksAndLiterature = 10,
    /// Food and drink
    FoodAndDrink = 11,
    /// Games
    Games = 12,
    /// Hobbies and leisure
    HobbiesAndLeisure = 13,
    /// Home and garden
    HomeAndGarden = 14,
    /// Internet and telecom
    InternetAndTelecom = 15,
    /// Law and government
    LawAndGovernment = 16,
    /// News
    News = 17,
    /// Online communities
    OnlineCommunities = 18,
    /// People and society
    PeopleAndSociety = 19,
    /// Pets and animals
    PetsAndAnimals = 20,
    /// Real estate
    RealEstate = 21,
    /// Reference
    Reference = 22,
    /// Science
    Science = 23,
    /// Sports
    Sports = 24,
    /// Jobs and education
    JobsAndEducation = 25,
    /// Shopping
    Shopping = 26,
}
/// Request message for GetAccount RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountRequest {
    /// Required. The name of the account to lookup.
    /// Format: accounts/{account}
    /// Example: "accounts/100"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListAccounts RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountsRequest {
    /// The maximum number of resources to return. The service may return
    /// fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// A page token, received from a previous `ListAccounts` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListAccounts` must
    /// match the call that provided the page token.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Whether to include soft-deleted (ie: "trashed") Accounts in the
    /// results. Accounts can be inspected to determine whether they are deleted or
    /// not.
    #[prost(bool, tag = "3")]
    pub show_deleted: bool,
}
/// Request message for ListAccounts RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountsResponse {
    /// Results that were accessible to the caller.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<Account>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for DeleteAccount RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAccountRequest {
    /// Required. The name of the Account to soft-delete.
    /// Format: accounts/{account}
    /// Example: "accounts/100"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateAccount RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccountRequest {
    /// Required. The account to update.
    /// The account's `name` field is used to identify the account.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<Account>,
    /// Required. The list of fields to be updated. Field names must be in snake case
    /// (e.g., "field_to_update"). Omitted fields will not be updated. To replace
    /// the entire entity, use one path with the string "*" to match all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ProvisionAccountTicket RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisionAccountTicketRequest {
    /// The account to create.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<Account>,
    /// Redirect URI where the user will be sent after accepting Terms of Service.
    /// Must be configured in Developers Console as a Redirect URI
    #[prost(string, tag = "2")]
    pub redirect_uri: ::prost::alloc::string::String,
}
/// Response message for ProvisionAccountTicket RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisionAccountTicketResponse {
    /// The param to be passed in the ToS link.
    #[prost(string, tag = "1")]
    pub account_ticket_id: ::prost::alloc::string::String,
}
/// Request message for GetProperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertyRequest {
    /// Required. The name of the property to lookup.
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListProperties RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPropertiesRequest {
    /// Required. An expression for filtering the results of the request.
    /// Fields eligible for filtering are:
    /// `parent:`(The resource name of the parent account) or
    /// `firebase_project:`(The id or number of the linked firebase project).
    /// Some examples of filters:
    ///
    /// ```
    /// | Filter                      | Description                               |
    /// |-----------------------------|-------------------------------------------|
    /// | parent:accounts/123         | The account with account id: 123.         |
    /// | firebase_project:project-id | The firebase project with id: project-id. |
    /// | firebase_project:123        | The firebase project with number: 123.    |
    /// ```
    #[prost(string, tag = "1")]
    pub filter: ::prost::alloc::string::String,
    /// The maximum number of resources to return. The service may return
    /// fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListProperties` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListProperties` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Whether to include soft-deleted (ie: "trashed") Properties in the
    /// results. Properties can be inspected to determine whether they are deleted
    /// or not.
    #[prost(bool, tag = "4")]
    pub show_deleted: bool,
}
/// Response message for ListProperties RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPropertiesResponse {
    /// Results that matched the filter criteria and were accessible to the caller.
    #[prost(message, repeated, tag = "1")]
    pub properties: ::prost::alloc::vec::Vec<Property>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for UpdateProperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePropertyRequest {
    /// Required. The property to update.
    /// The property's `name` field is used to identify the property to be
    /// updated.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<Property>,
    /// Required. The list of fields to be updated. Field names must be in snake case
    /// (e.g., "field_to_update"). Omitted fields will not be updated. To replace
    /// the entire entity, use one path with the string "*" to match all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateProperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePropertyRequest {
    /// Required. The property to create.
    /// Note: the supplied property must specify its parent.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<Property>,
}
/// Request message for DeleteProperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePropertyRequest {
    /// Required. The name of the Property to soft-delete.
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetUserLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserLinkRequest {
    /// Required. Example format: accounts/1234/userLinks/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for BatchGetUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetUserLinksRequest {
    /// Required. The account or property that all user links in the request are
    /// for. The parent of all provided values for the 'names' field must match
    /// this field.
    /// Example format: accounts/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The names of the user links to retrieve.
    /// A maximum of 1000 user links can be retrieved in a batch.
    /// Format: accounts/{accountId}/userLinks/{userLinkId}
    #[prost(string, repeated, tag = "2")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response message for BatchGetUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetUserLinksResponse {
    /// The requested user links.
    #[prost(message, repeated, tag = "1")]
    pub user_links: ::prost::alloc::vec::Vec<UserLink>,
}
/// Request message for ListUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserLinksRequest {
    /// Required. Example format: accounts/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of user links to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 200 user links will be returned.
    /// The maximum value is 500; values above 500 will be coerced to 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListUserLinks` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListUserLinks` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserLinksResponse {
    /// List of UserLinks. These will be ordered stably, but in an arbitrary order.
    #[prost(message, repeated, tag = "1")]
    pub user_links: ::prost::alloc::vec::Vec<UserLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for AuditUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditUserLinksRequest {
    /// Required. Example format: accounts/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of user links to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 1000 user links will be returned.
    /// The maximum value is 5000; values above 5000 will be coerced to 5000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `AuditUserLinks` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `AuditUserLinks` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for AuditUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditUserLinksResponse {
    /// List of AuditUserLinks. These will be ordered stably, but in an arbitrary
    /// order.
    #[prost(message, repeated, tag = "1")]
    pub user_links: ::prost::alloc::vec::Vec<AuditUserLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CreateUserLink RPC.
///
/// Users can have multiple email addresses associated with their Google
/// account, and one of these email addresses is the "primary" email address.
/// Any of the email addresses associated with a Google account may be used
/// for a new UserLink, but the returned UserLink will always contain the
/// "primary" email address. As a result, the input and output email address
/// for this request may differ.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserLinkRequest {
    /// Required. Example format: accounts/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. If set, then email the new user notifying them that they've been granted
    /// permissions to the resource.
    #[prost(bool, tag = "2")]
    pub notify_new_user: bool,
    /// Required. The user link to create.
    #[prost(message, optional, tag = "3")]
    pub user_link: ::core::option::Option<UserLink>,
}
/// Request message for BatchCreateUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateUserLinksRequest {
    /// Required. The account or property that all user links in the request are for.
    /// This field is required. The parent field in the CreateUserLinkRequest
    /// messages must either be empty or match this field.
    /// Example format: accounts/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. If set, then email the new users notifying them that they've been granted
    /// permissions to the resource. Regardless of whether this is set or not,
    /// notify_new_user field inside each individual request is ignored.
    #[prost(bool, tag = "2")]
    pub notify_new_users: bool,
    /// Required. The requests specifying the user links to create.
    /// A maximum of 1000 user links can be created in a batch.
    #[prost(message, repeated, tag = "3")]
    pub requests: ::prost::alloc::vec::Vec<CreateUserLinkRequest>,
}
/// Response message for BatchCreateUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateUserLinksResponse {
    /// The user links created.
    #[prost(message, repeated, tag = "1")]
    pub user_links: ::prost::alloc::vec::Vec<UserLink>,
}
/// Request message for UpdateUserLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserLinkRequest {
    /// Required. The user link to update.
    #[prost(message, optional, tag = "1")]
    pub user_link: ::core::option::Option<UserLink>,
}
/// Request message for BatchUpdateUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateUserLinksRequest {
    /// Required. The account or property that all user links in the request are
    /// for. The parent field in the UpdateUserLinkRequest messages must either be
    /// empty or match this field.
    /// Example format: accounts/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The requests specifying the user links to update.
    /// A maximum of 1000 user links can be updated in a batch.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<UpdateUserLinkRequest>,
}
/// Response message for BatchUpdateUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateUserLinksResponse {
    /// The user links updated.
    #[prost(message, repeated, tag = "1")]
    pub user_links: ::prost::alloc::vec::Vec<UserLink>,
}
/// Request message for DeleteUserLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserLinkRequest {
    /// Required. Example format: accounts/1234/userLinks/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for BatchDeleteUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteUserLinksRequest {
    /// Required. The account or property that all user links in the request are
    /// for. The parent of all values for user link names to delete must match this
    /// field.
    /// Example format: accounts/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The requests specifying the user links to update.
    /// A maximum of 1000 user links can be updated in a batch.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<DeleteUserLinkRequest>,
}
/// Request message for GetWebDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWebDataStreamRequest {
    /// Required. The name of the web data stream to lookup.
    /// Format: properties/{property_id}/webDataStreams/{stream_id}
    /// Example: "properties/123/webDataStreams/456"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for DeleteWebDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWebDataStreamRequest {
    /// Required. The name of the web data stream to delete.
    /// Format: properties/{property_id}/webDataStreams/{stream_id}
    /// Example: "properties/123/webDataStreams/456"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateWebDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWebDataStreamRequest {
    /// Required. The web stream to update.
    /// The `name` field is used to identify the web stream to be updated.
    #[prost(message, optional, tag = "1")]
    pub web_data_stream: ::core::option::Option<WebDataStream>,
    /// Required. The list of fields to be updated. Field names must be in snake case
    /// (e.g., "field_to_update"). Omitted fields will not be updated. To replace
    /// the entire entity, use one path with the string "*" to match all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateWebDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWebDataStreamRequest {
    /// Required. The web stream to create.
    #[prost(message, optional, tag = "1")]
    pub web_data_stream: ::core::option::Option<WebDataStream>,
    /// Required. The parent resource where this web data stream will be created.
    /// Format: properties/123
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
}
/// Request message for ListWebDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWebDataStreamsRequest {
    /// Required. The name of the parent property.
    /// For example, to list results of web streams under the property with Id
    /// 123: "properties/123"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListWebDataStreams` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListWebDataStreams` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Request message for ListWebDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWebDataStreamsResponse {
    /// Results that matched the filter criteria and were accessible to the caller.
    #[prost(message, repeated, tag = "1")]
    pub web_data_streams: ::prost::alloc::vec::Vec<WebDataStream>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetIosAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIosAppDataStreamRequest {
    /// Required. The name of the iOS app data stream to lookup.
    /// Format: properties/{property_id}/iosAppDataStreams/{stream_id}
    /// Example: "properties/123/iosAppDataStreams/456"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for DeleteIosAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteIosAppDataStreamRequest {
    /// Required. The name of the iOS app data stream to delete.
    /// Format: properties/{property_id}/iosAppDataStreams/{stream_id}
    /// Example: "properties/123/iosAppDataStreams/456"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateIosAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIosAppDataStreamRequest {
    /// Required. The iOS app stream to update.
    /// The `name` field is used to identify the iOS app stream to be updated.
    #[prost(message, optional, tag = "1")]
    pub ios_app_data_stream: ::core::option::Option<IosAppDataStream>,
    /// Required. The list of fields to be updated. Field names must be in snake case
    /// (e.g., "field_to_update"). Omitted fields will not be updated. To replace
    /// the entire entity, use one path with the string "*" to match all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateIosAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIosAppDataStreamRequest {
    /// Required. The iOS app data stream to create.
    #[prost(message, optional, tag = "1")]
    pub ios_app_data_stream: ::core::option::Option<IosAppDataStream>,
    /// Required. The parent resource where this ios app data stream will be created.
    /// Format: properties/123
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
}
/// Request message for ListIosAppDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIosAppDataStreamsRequest {
    /// Required. The name of the parent property.
    /// For example, to list results of app streams under the property with Id
    /// 123: "properties/123"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListIosAppDataStreams`
    /// call. Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListIosAppDataStreams`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Request message for ListIosAppDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIosAppDataStreamsResponse {
    /// Results that matched the filter criteria and were accessible to the caller.
    #[prost(message, repeated, tag = "1")]
    pub ios_app_data_streams: ::prost::alloc::vec::Vec<IosAppDataStream>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetAndroidAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAndroidAppDataStreamRequest {
    /// Required. The name of the android app data stream to lookup.
    /// Format: properties/{property_id}/androidAppDataStreams/{stream_id}
    /// Example: "properties/123/androidAppDataStreams/456"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for DeleteAndroidAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAndroidAppDataStreamRequest {
    /// Required. The name of the android app data stream to delete.
    /// Format: properties/{property_id}/androidAppDataStreams/{stream_id}
    /// Example: "properties/123/androidAppDataStreams/456"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateAndroidAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAndroidAppDataStreamRequest {
    /// Required. The android app stream to update.
    /// The `name` field is used to identify the android app stream to be updated.
    #[prost(message, optional, tag = "1")]
    pub android_app_data_stream: ::core::option::Option<AndroidAppDataStream>,
    /// Required. The list of fields to be updated. Field names must be in snake case
    /// (e.g., "field_to_update"). Omitted fields will not be updated. To replace
    /// the entire entity, use one path with the string "*" to match all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateAndroidAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAndroidAppDataStreamRequest {
    /// Required. The android app stream to create.
    #[prost(message, optional, tag = "1")]
    pub android_app_data_stream: ::core::option::Option<AndroidAppDataStream>,
    /// Required. The parent resource where this android app data stream will be created.
    /// Format: properties/123
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
}
/// Request message for ListAndroidAppDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAndroidAppDataStreamsRequest {
    /// Required. The name of the parent property.
    /// For example, to limit results to app streams under the property with Id
    /// 123: "properties/123"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    ///
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous call. Provide this to
    /// retrieve the subsequent page.
    /// When paginating, all other parameters provided to
    /// `ListAndroidAppDataStreams` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Request message for ListAndroidDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAndroidAppDataStreamsResponse {
    /// Results that matched the filter criteria and were accessible to the caller.
    #[prost(message, repeated, tag = "1")]
    pub android_app_data_streams: ::prost::alloc::vec::Vec<AndroidAppDataStream>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetEnhancedMeasurementSettings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEnhancedMeasurementSettingsRequest {
    /// Required. The name of the settings to lookup.
    /// Format:
    /// properties/{property_id}/webDataStreams/{stream_id}/enhancedMeasurementSettings
    /// Example: "properties/1000/webDataStreams/2000/enhancedMeasurementSettings"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateEnhancedMeasurementSettings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEnhancedMeasurementSettingsRequest {
    /// Required. The settings to update.
    /// The `name` field is used to identify the settings to be updated.
    #[prost(message, optional, tag = "1")]
    pub enhanced_measurement_settings: ::core::option::Option<EnhancedMeasurementSettings>,
    /// Required. The list of fields to be updated. Field names must be in snake case
    /// (e.g., "field_to_update"). Omitted fields will not be updated. To replace
    /// the entire entity, use one path with the string "*" to match all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateFirebaseLink RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFirebaseLinkRequest {
    /// Required. Format: properties/{property_id}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Firebase link to create.
    #[prost(message, optional, tag = "2")]
    pub firebase_link: ::core::option::Option<FirebaseLink>,
}
/// Request message for UpdateFirebaseLink RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFirebaseLinkRequest {
    /// Required. The Firebase link to update.
    #[prost(message, optional, tag = "1")]
    pub firebase_link: ::core::option::Option<FirebaseLink>,
    /// Required. The list of fields to be updated. Field names must be in snake case
    /// (e.g., "field_to_update"). Omitted fields will not be updated. To replace
    /// the entire entity, use one path with the string "*" to match all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteFirebaseLink RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFirebaseLinkRequest {
    /// Required. Format: properties/{property_id}/firebaseLinks/{firebase_link_id}
    /// Example: properties/1234/firebaseLinks/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListFirebaseLinks RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFirebaseLinksRequest {
    /// Required. Format: properties/{property_id}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return. The service may return
    /// fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListFirebaseLinks` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListProperties` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListFirebaseLinks RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFirebaseLinksResponse {
    /// List of FirebaseLinks. This will have at most one value.
    #[prost(message, repeated, tag = "1")]
    pub firebase_links: ::prost::alloc::vec::Vec<FirebaseLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    /// Currently, Google Analytics supports only one FirebaseLink per property,
    /// so this will never be populated.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetGlobalSiteTag RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGlobalSiteTagRequest {
    /// Required. The name of the site tag to lookup.
    /// Note that site tags are singletons and do not have unique IDs.
    /// Format: properties/{property_id}/webDataStreams/{stream_id}/globalSiteTag
    /// Example: "properties/123/webDataStreams/456/globalSiteTag"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateGoogleAdsLink RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGoogleAdsLinkRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The GoogleAdsLink to create.
    #[prost(message, optional, tag = "2")]
    pub google_ads_link: ::core::option::Option<GoogleAdsLink>,
}
/// Request message for UpdateGoogleAdsLink RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGoogleAdsLinkRequest {
    /// The GoogleAdsLink to update
    #[prost(message, optional, tag = "1")]
    pub google_ads_link: ::core::option::Option<GoogleAdsLink>,
    /// Required. The list of fields to be updated. Field names must be in snake case
    /// (e.g., "field_to_update"). Omitted fields will not be updated. To replace
    /// the entire entity, use one path with the string "*" to match all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteGoogleAdsLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGoogleAdsLinkRequest {
    /// Required. Example format: properties/1234/googleAdsLinks/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListGoogleAdsLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGoogleAdsLinksRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListGoogleAdsLinks` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListGoogleAdsLinks` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListGoogleAdsLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGoogleAdsLinksResponse {
    /// List of GoogleAdsLinks.
    #[prost(message, repeated, tag = "1")]
    pub google_ads_links: ::prost::alloc::vec::Vec<GoogleAdsLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetDataSharingSettings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataSharingSettingsRequest {
    /// Required. The name of the settings to lookup.
    /// Format: accounts/{account}/dataSharingSettings
    /// Example: "accounts/1000/dataSharingSettings"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListAccountSummaries RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountSummariesRequest {
    /// The maximum number of AccountSummary resources to return. The service may
    /// return fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// A page token, received from a previous `ListAccountSummaries` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListAccountSummaries`
    /// must match the call that provided the page token.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListAccountSummaries RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountSummariesResponse {
    /// Account summaries of all accounts the caller has access to.
    #[prost(message, repeated, tag = "1")]
    pub account_summaries: ::prost::alloc::vec::Vec<AccountSummary>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod analytics_admin_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service Interface for the Analytics Admin API (GA4)."]
    pub struct AnalyticsAdminServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AnalyticsAdminServiceClient<T>
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
        #[doc = " Lookup for a single Account."]
        pub async fn get_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountRequest>,
        ) -> Result<tonic::Response<super::Account>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns all accounts accessible by the caller."]
        #[doc = ""]
        #[doc = " Note that these accounts might not currently have GA4 properties."]
        #[doc = " Soft-deleted (ie: \"trashed\") accounts are excluded by default."]
        #[doc = " Returns an empty list if no relevant accounts are found."]
        pub async fn list_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccountsRequest>,
        ) -> Result<tonic::Response<super::ListAccountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListAccounts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Marks target Account as soft-deleted (ie: \"trashed\") and returns it."]
        #[doc = ""]
        #[doc = " This API does not have a method to restore soft-deleted accounts."]
        #[doc = " However, they can be restored using the Trash Can UI."]
        #[doc = ""]
        #[doc = " If the accounts are not restored before the expiration time, the account"]
        #[doc = " and all child resources (eg: Properties, GoogleAdsLinks, Streams,"]
        #[doc = " UserLinks) will be permanently purged."]
        #[doc = " https://support.google.com/analytics/answer/6154772"]
        #[doc = ""]
        #[doc = " Returns an error if the target is not found."]
        pub async fn delete_account(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAccountRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an account."]
        pub async fn update_account(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAccountRequest>,
        ) -> Result<tonic::Response<super::Account>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Requests a ticket for creating an account."]
        pub async fn provision_account_ticket(
            &mut self,
            request: impl tonic::IntoRequest<super::ProvisionAccountTicketRequest>,
        ) -> Result<tonic::Response<super::ProvisionAccountTicketResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ProvisionAccountTicket",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns summaries of all accounts accessible by the caller."]
        pub async fn list_account_summaries(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccountSummariesRequest>,
        ) -> Result<tonic::Response<super::ListAccountSummariesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListAccountSummaries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single \"GA4\" Property."]
        pub async fn get_property(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPropertyRequest>,
        ) -> Result<tonic::Response<super::Property>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetProperty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns child Properties under the specified parent Account."]
        #[doc = ""]
        #[doc = " Only \"GA4\" properties will be returned."]
        #[doc = " Properties will be excluded if the caller does not have access."]
        #[doc = " Soft-deleted (ie: \"trashed\") properties are excluded by default."]
        #[doc = " Returns an empty list if no relevant properties are found."]
        pub async fn list_properties(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPropertiesRequest>,
        ) -> Result<tonic::Response<super::ListPropertiesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListProperties",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an \"GA4\" property with the specified location and attributes."]
        pub async fn create_property(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePropertyRequest>,
        ) -> Result<tonic::Response<super::Property>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateProperty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Marks target Property as soft-deleted (ie: \"trashed\") and returns it."]
        #[doc = ""]
        #[doc = " This API does not have a method to restore soft-deleted properties."]
        #[doc = " However, they can be restored using the Trash Can UI."]
        #[doc = ""]
        #[doc = " If the properties are not restored before the expiration time, the Property"]
        #[doc = " and all child resources (eg: GoogleAdsLinks, Streams, UserLinks)"]
        #[doc = " will be permanently purged."]
        #[doc = " https://support.google.com/analytics/answer/6154772"]
        #[doc = ""]
        #[doc = " Returns an error if the target is not found, or is not an GA4 Property."]
        pub async fn delete_property(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePropertyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteProperty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a property."]
        pub async fn update_property(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePropertyRequest>,
        ) -> Result<tonic::Response<super::Property>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateProperty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about a user's link to an account or property."]
        pub async fn get_user_link(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserLinkRequest>,
        ) -> Result<tonic::Response<super::UserLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetUserLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about multiple users' links to an account or property."]
        pub async fn batch_get_user_links(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetUserLinksRequest>,
        ) -> Result<tonic::Response<super::BatchGetUserLinksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/BatchGetUserLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all user links on an account or property."]
        pub async fn list_user_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserLinksRequest>,
        ) -> Result<tonic::Response<super::ListUserLinksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListUserLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all user links on an account or property, including implicit ones"]
        #[doc = " that come from effective permissions granted by groups or organization"]
        #[doc = " admin roles."]
        #[doc = ""]
        #[doc = " If a returned user link does not have direct permissions, they cannot"]
        #[doc = " be removed from the account or property directly with the DeleteUserLink"]
        #[doc = " command. They have to be removed from the group/etc that gives them"]
        #[doc = " permissions, which is currently only usable/discoverable in the GA or GMP"]
        #[doc = " UIs."]
        pub async fn audit_user_links(
            &mut self,
            request: impl tonic::IntoRequest<super::AuditUserLinksRequest>,
        ) -> Result<tonic::Response<super::AuditUserLinksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/AuditUserLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a user link on an account or property."]
        #[doc = ""]
        #[doc = " If the user with the specified email already has permissions on the"]
        #[doc = " account or property, then the user's existing permissions will be unioned"]
        #[doc = " with the permissions specified in the new UserLink."]
        pub async fn create_user_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUserLinkRequest>,
        ) -> Result<tonic::Response<super::UserLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateUserLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates information about multiple users' links to an account or property."]
        #[doc = ""]
        #[doc = " This method is transactional. If any UserLink cannot be created, none of"]
        #[doc = " the UserLinks will be created."]
        pub async fn batch_create_user_links(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateUserLinksRequest>,
        ) -> Result<tonic::Response<super::BatchCreateUserLinksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/BatchCreateUserLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a user link on an account or property."]
        pub async fn update_user_link(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserLinkRequest>,
        ) -> Result<tonic::Response<super::UserLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateUserLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates information about multiple users' links to an account or property."]
        pub async fn batch_update_user_links(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdateUserLinksRequest>,
        ) -> Result<tonic::Response<super::BatchUpdateUserLinksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/BatchUpdateUserLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a user link on an account or property."]
        pub async fn delete_user_link(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteUserLinkRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteUserLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes information about multiple users' links to an account or property."]
        pub async fn batch_delete_user_links(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteUserLinksRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/BatchDeleteUserLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single WebDataStream"]
        pub async fn get_web_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWebDataStreamRequest>,
        ) -> Result<tonic::Response<super::WebDataStream>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetWebDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a web stream on a property."]
        pub async fn delete_web_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWebDataStreamRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteWebDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a web stream on a property."]
        pub async fn update_web_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWebDataStreamRequest>,
        ) -> Result<tonic::Response<super::WebDataStream>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateWebDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a web stream with the specified location and attributes."]
        pub async fn create_web_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWebDataStreamRequest>,
        ) -> Result<tonic::Response<super::WebDataStream>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateWebDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns child web data streams under the specified parent property."]
        #[doc = ""]
        #[doc = " Web data streams will be excluded if the caller does not have access."]
        #[doc = " Returns an empty list if no relevant web data streams are found."]
        pub async fn list_web_data_streams(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWebDataStreamsRequest>,
        ) -> Result<tonic::Response<super::ListWebDataStreamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListWebDataStreams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single IosAppDataStream"]
        pub async fn get_ios_app_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIosAppDataStreamRequest>,
        ) -> Result<tonic::Response<super::IosAppDataStream>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetIosAppDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an iOS app stream on a property."]
        pub async fn delete_ios_app_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteIosAppDataStreamRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteIosAppDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an iOS app stream on a property."]
        pub async fn update_ios_app_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateIosAppDataStreamRequest>,
        ) -> Result<tonic::Response<super::IosAppDataStream>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateIosAppDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an iOS app stream with the specified location and attributes."]
        #[doc = ""]
        #[doc = " Note that an iOS app stream must be linked to a Firebase app to receive"]
        #[doc = " traffic."]
        #[doc = ""]
        #[doc = " To create a working app stream, make sure your property is linked to a"]
        #[doc = " Firebase project. Then, use the Firebase API to create a Firebase app,"]
        #[doc = " which will also create an appropriate data stream in Analytics (may take up"]
        #[doc = " to 24 hours)."]
        pub async fn create_ios_app_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateIosAppDataStreamRequest>,
        ) -> Result<tonic::Response<super::IosAppDataStream>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateIosAppDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns child iOS app data streams under the specified parent property."]
        #[doc = ""]
        #[doc = " iOS app data streams will be excluded if the caller does not have access."]
        #[doc = " Returns an empty list if no relevant iOS app data streams are found."]
        pub async fn list_ios_app_data_streams(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIosAppDataStreamsRequest>,
        ) -> Result<tonic::Response<super::ListIosAppDataStreamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListIosAppDataStreams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single AndroidAppDataStream"]
        pub async fn get_android_app_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAndroidAppDataStreamRequest>,
        ) -> Result<tonic::Response<super::AndroidAppDataStream>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetAndroidAppDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an android app stream on a property."]
        pub async fn delete_android_app_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAndroidAppDataStreamRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteAndroidAppDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an android app stream on a property."]
        pub async fn update_android_app_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAndroidAppDataStreamRequest>,
        ) -> Result<tonic::Response<super::AndroidAppDataStream>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateAndroidAppDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an Android app stream with the specified location and attributes."]
        #[doc = ""]
        #[doc = " Note that an Android app stream must be linked to a Firebase app to receive"]
        #[doc = " traffic."]
        #[doc = ""]
        #[doc = " To create a working app stream, make sure your property is linked to a"]
        #[doc = " Firebase project. Then, use the Firebase API to create a Firebase app,"]
        #[doc = " which will also create an appropriate data stream in Analytics (may take up"]
        #[doc = " to 24 hours)."]
        pub async fn create_android_app_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAndroidAppDataStreamRequest>,
        ) -> Result<tonic::Response<super::AndroidAppDataStream>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateAndroidAppDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns child android app streams under the specified parent property."]
        #[doc = ""]
        #[doc = " Android app streams will be excluded if the caller does not have access."]
        #[doc = " Returns an empty list if no relevant android app streams are found."]
        pub async fn list_android_app_data_streams(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAndroidAppDataStreamsRequest>,
        ) -> Result<tonic::Response<super::ListAndroidAppDataStreamsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListAndroidAppDataStreams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the singleton enhanced measurement settings for this web stream."]
        #[doc = " Note that the stream must enable enhanced measurement for these settings to"]
        #[doc = " take effect."]
        pub async fn get_enhanced_measurement_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEnhancedMeasurementSettingsRequest>,
        ) -> Result<tonic::Response<super::EnhancedMeasurementSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/GetEnhancedMeasurementSettings") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the singleton enhanced measurement settings for this web stream."]
        #[doc = " Note that the stream must enable enhanced measurement for these settings to"]
        #[doc = " take effect."]
        pub async fn update_enhanced_measurement_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEnhancedMeasurementSettingsRequest>,
        ) -> Result<tonic::Response<super::EnhancedMeasurementSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateEnhancedMeasurementSettings") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a FirebaseLink."]
        #[doc = ""]
        #[doc = " Properties can have at most one FirebaseLink."]
        pub async fn create_firebase_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFirebaseLinkRequest>,
        ) -> Result<tonic::Response<super::FirebaseLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateFirebaseLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a FirebaseLink on a property"]
        pub async fn update_firebase_link(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFirebaseLinkRequest>,
        ) -> Result<tonic::Response<super::FirebaseLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateFirebaseLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a FirebaseLink on a property"]
        pub async fn delete_firebase_link(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFirebaseLinkRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteFirebaseLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists FirebaseLinks on a property."]
        #[doc = " Properties can have at most one FirebaseLink."]
        pub async fn list_firebase_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFirebaseLinksRequest>,
        ) -> Result<tonic::Response<super::ListFirebaseLinksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListFirebaseLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the Site Tag for the specified web stream."]
        #[doc = " Site Tags are immutable singletons."]
        pub async fn get_global_site_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGlobalSiteTagRequest>,
        ) -> Result<tonic::Response<super::GlobalSiteTag>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetGlobalSiteTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a GoogleAdsLink."]
        pub async fn create_google_ads_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGoogleAdsLinkRequest>,
        ) -> Result<tonic::Response<super::GoogleAdsLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateGoogleAdsLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a GoogleAdsLink on a property"]
        pub async fn update_google_ads_link(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGoogleAdsLinkRequest>,
        ) -> Result<tonic::Response<super::GoogleAdsLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateGoogleAdsLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a GoogleAdsLink on a property"]
        pub async fn delete_google_ads_link(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGoogleAdsLinkRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteGoogleAdsLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists GoogleAdsLinks on a property."]
        pub async fn list_google_ads_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGoogleAdsLinksRequest>,
        ) -> Result<tonic::Response<super::ListGoogleAdsLinksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListGoogleAdsLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get data sharing settings on an account."]
        #[doc = " Data sharing settings are singletons."]
        pub async fn get_data_sharing_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataSharingSettingsRequest>,
        ) -> Result<tonic::Response<super::DataSharingSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetDataSharingSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AnalyticsAdminServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AnalyticsAdminServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AnalyticsAdminServiceClient {{ ... }}")
        }
    }
}
