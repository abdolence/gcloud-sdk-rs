/// NEXT TAG: 7
/// A resource message representing a Google Analytics account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// Output only. Resource name of this account.
    /// Format: accounts/{account_id}
    /// Example: "accounts/100"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. Time when this account was originally created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when account payload fields were last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Human-readable display name for this account.
    #[prost(string, tag = "4")]
    pub display_name: std::string::String,
    /// Country of business. Must be a non-deprecated code for a UN M.49 region.
    /// https://unicode.org/cldr/charts/latest/supplemental/territory_containment_un_m_49.html
    #[prost(string, tag = "5")]
    pub country_code: std::string::String,
    /// Output only. Indicates whether this Account is soft-deleted or not. Deleted
    /// accounts are excluded from List results unless specifically requested.
    #[prost(bool, tag = "6")]
    pub deleted: bool,
}
/// NEXT TAG: 10
/// A resource message representing a Google Analytics App+Web property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Property {
    /// Output only. Resource name of this property.
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. Time when the entity was originally created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when entity payload fields were last updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Immutable. Resource name of this property's logical parent.
    ///
    /// Note: The Property-Moving UI can be used to change the parent.
    /// Format: accounts/{account_id}
    /// Example: "accounts/100"
    #[prost(string, tag = "2")]
    pub parent: std::string::String,
    /// Human-readable display name for this property.
    #[prost(string, tag = "5")]
    pub display_name: std::string::String,
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
    pub time_zone: std::string::String,
    /// The currency type used in reports involving monetary values.
    ///
    ///
    /// Format: https://en.wikipedia.org/wiki/ISO_4217
    /// Examples: "USD", "EUR", "JPY"
    #[prost(string, tag = "8")]
    pub currency_code: std::string::String,
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
    pub name: std::string::String,
    /// Output only. Analytics "Measurement ID", without the "G-" prefix.
    /// ex: "G-1A2BCD345E" would just be "1A2BCD345E"
    #[prost(string, tag = "2")]
    pub measurement_id: std::string::String,
    /// Output only. ID of the corresponding Android app in Firebase, if any.
    /// This ID can change if the Android app is deleted and recreated.
    #[prost(string, tag = "3")]
    pub firebase_app_id: std::string::String,
    /// Output only. Time when this stream was originally created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when stream payload fields were last updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Immutable. The package name for the app being measured.
    /// Example: "com.example.myandroidapp"
    #[prost(string, tag = "6")]
    pub package_name: std::string::String,
    /// Human-readable display name for the Data Stream.
    #[prost(string, tag = "7")]
    pub display_name: std::string::String,
}
/// A resource message representing a Google Analytics IOS app stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosAppDataStream {
    /// Output only. Resource name of this Data Stream.
    /// Format: properties/{property_id}/iosAppDataStreams/{stream_id}
    /// Example: "properties/1000/iosAppDataStreams/2000"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. Analytics "Measurement ID", without the "G-" prefix.
    /// Example: "G-1A2BCD345E" would just be "1A2BCD345E"
    #[prost(string, tag = "2")]
    pub measurement_id: std::string::String,
    /// Output only. ID of the corresponding iOS app in Firebase, if any.
    /// This ID can change if the iOS app is deleted and recreated.
    #[prost(string, tag = "3")]
    pub firebase_app_id: std::string::String,
    /// Output only. Time when this stream was originally created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when stream payload fields were last updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Immutable. The Apple App Store Bundle ID for the app
    /// Example: "com.example.myiosapp"
    #[prost(string, tag = "6")]
    pub bundle_id: std::string::String,
    /// Human-readable display name for the Data Stream.
    #[prost(string, tag = "7")]
    pub display_name: std::string::String,
}
/// A resource message representing a Google Analytics web stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebDataStream {
    /// Output only. Resource name of this Data Stream.
    /// Format: properties/{property_id}/webDataStreams/{stream_id}
    /// Example: "properties/1000/webDataStreams/2000"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. Analytics "Measurement ID", without the "G-" prefix.
    /// Example: "G-1A2BCD345E" would just be "1A2BCD345E"
    #[prost(string, tag = "2")]
    pub measurement_id: std::string::String,
    /// Output only. ID of the corresponding web app in Firebase, if any.
    /// This ID can change if the web app is deleted and recreated.
    #[prost(string, tag = "3")]
    pub firebase_app_id: std::string::String,
    /// Output only. Time when this stream was originally created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when stream payload fields were last updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Immutable. Domain name of the web app being measured, or empty.
    /// Example: "http://www.google.com", "https://www.google.com"
    #[prost(string, tag = "6")]
    pub default_uri: std::string::String,
    /// Human-readable display name for the Data Stream.
    #[prost(string, tag = "7")]
    pub display_name: std::string::String,
}
/// A resource message representing a user's permissions on an Account or
/// Property resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserLink {
    /// Example format: properties/1234/userLinks/5678
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Email address of the user to link
    #[prost(string, tag = "2")]
    pub email_address: std::string::String,
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
    pub direct_roles: ::std::vec::Vec<std::string::String>,
}
/// Read-only resource used to summarize a principal's effective roles.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditUserLink {
    /// Example format: properties/1234/userLinks/5678
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Email address of the linked user
    #[prost(string, tag = "2")]
    pub email_address: std::string::String,
    /// Roles directly assigned to this user for this entity.
    ///
    /// Format: predefinedRoles/read
    ///
    /// Excludes roles that are inherited from an account (if this is for a
    /// property), group, or organization admin role.
    #[prost(string, repeated, tag = "3")]
    pub direct_roles: ::std::vec::Vec<std::string::String>,
    /// Union of all permissions a user has at this account or property (includes
    /// direct permissions, group-inherited permissions, etc.).
    ///
    /// Format: predefinedRoles/read
    #[prost(string, repeated, tag = "4")]
    pub effective_roles: ::std::vec::Vec<std::string::String>,
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
    pub name: std::string::String,
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
    /// Capture events when your visitors view content on your site that has
    /// structured data (eg, articles, blog posts, product details screens, etc.).
    #[prost(bool, tag = "6")]
    pub content_views_enabled: bool,
    /// If enabled, capture a view search results event each time a visitor
    /// performs a search on your site (based on a query parameter).
    #[prost(bool, tag = "7")]
    pub site_search_enabled: bool,
    /// If enabled, capture a view search results event each time a visitor
    /// interacts with a form on your site.
    #[prost(bool, tag = "8")]
    pub form_interactions_enabled: bool,
    /// If enabled, capture video play, progress, and complete events as visitors
    /// view embedded videos on your site.
    #[prost(bool, tag = "9")]
    pub video_engagement_enabled: bool,
    /// If enabled, capture a file download event each time a link is clicked with
    /// a common document, compressed file, application, video, or audio extension.
    #[prost(bool, tag = "10")]
    pub file_downloads_enabled: bool,
    /// If enabled, capture a click event each time a visitor clicks a link or
    /// element that has data attributes beginning with "data-ga".
    #[prost(bool, tag = "11")]
    pub data_tagged_element_clicks_enabled: bool,
    /// If enabled, capture a page view event each time a page loads.
    #[prost(bool, tag = "12")]
    pub page_loads_enabled: bool,
    /// If enabled, capture a page view event each time the website changes the
    /// browser history state.
    #[prost(bool, tag = "13")]
    pub page_changes_enabled: bool,
    /// Capture events when your visitors view content on your site that has
    /// articles or blog posts.
    #[prost(bool, tag = "14")]
    pub articles_and_blogs_enabled: bool,
    /// Capture events when your visitors view content on your site that has
    /// product details screens, etc.
    #[prost(bool, tag = "15")]
    pub products_and_ecommerce_enabled: bool,
    /// Required. URL query parameters to interpret as site search parameters.
    /// Max length is 1024 characters. Must not be empty.
    #[prost(string, tag = "16")]
    pub search_query_parameter: std::string::String,
    /// Additional URL query parameters.
    /// Max length is 1024 characters.
    #[prost(string, tag = "17")]
    pub url_query_parameter: std::string::String,
    /// Domains to exclude from measurement. Max length is 1024 characters.
    #[prost(string, tag = "18")]
    pub excluded_domains: std::string::String,
}
/// A link between an App+Web property and a Firebase project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirebaseLink {
    /// Output only. Example format: properties/1234/firebaseLinks/5678
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Immutable. Firebase project resource name. When creating a FirebaseLink, you may
    /// provide this resource name using either a project number or project ID.
    /// Once this resource has been created, returned FirebaseLinks will always
    /// have a project_name that contains a project number.
    ///
    /// Format: 'projects/{project number}'
    /// Example: 'projects/1234'
    #[prost(string, tag = "2")]
    pub project: std::string::String,
    /// Output only. Time when this FirebaseLink was originally created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Maximum user access to the App + Web property allowed to admins of
    /// the linked Firebase project.
    #[prost(enumeration = "MaximumUserAccess", tag = "4")]
    pub maximum_user_access: i32,
}
/// Read-only resource with the tag for sending data from a website to a
/// WebDataStream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalSiteTag {
    /// Immutable. JavaScript code snippet to be pasted as the first item into the head tag of
    /// every webpage to measure.
    #[prost(string, tag = "1")]
    pub snippet: std::string::String,
}
/// A link between an App+Web property and a Google Ads account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleAdsLink {
    /// Output only. Format: properties/{propertyId}/googleAdsLinks/{googleAdsLinkId}
    ///
    /// Note: googleAdsLinkId is not the Google Ads customer ID.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Immutable. Format: properties/{propertyId}
    #[prost(string, tag = "2")]
    pub parent: std::string::String,
    /// Immutable. Google Ads customer ID.
    #[prost(string, tag = "3")]
    pub customer_id: std::string::String,
    /// Output only. If true, this link is for a Google Ads manager account.
    #[prost(bool, tag = "4")]
    pub can_manage_clients: bool,
    /// Enable personalized advertising features with this integration.
    /// Automatically publish my Google Analytics audience lists and Google
    /// Analytics remarketing events/parameters to the linked Google Ads account.
    #[prost(bool, tag = "5")]
    pub ads_personalization_enabled: bool,
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
/// Request message for GetAccount RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountRequest {
    /// Required. The name of the account to lookup.
    /// Format: accounts/{account_id}
    /// Example: "accounts/100"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
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
    pub page_token: std::string::String,
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
    pub accounts: ::std::vec::Vec<Account>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for DeleteAccount RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAccountRequest {
    /// Required. The name of the Account to soft-delete.
    /// Format: accounts/{account_id}
    /// Example: "accounts/100"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for UpdateAccount RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccountRequest {
    /// Required. The account to update.
    /// The account's `name` field is used to identify the account.
    #[prost(message, optional, tag = "1")]
    pub account: ::std::option::Option<Account>,
    /// The list of fields to be updated. Omitted fields will not be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for ProvisionAccountTicket RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisionAccountTicketRequest {
    /// The account to create.
    #[prost(message, optional, tag = "1")]
    pub account: ::std::option::Option<Account>,
    /// Redirect URI where the user will be sent after accepting Terms of Service.
    /// Must be configured in Developers Console as a Redirect URI
    #[prost(string, tag = "2")]
    pub redirect_uri: std::string::String,
}
/// Response message for ProvisionAccountTicket RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisionAccountTicketResponse {
    /// The param to be passed in the ToS link.
    #[prost(string, tag = "1")]
    pub account_ticket_id: std::string::String,
}
/// Request message for GetProperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertyRequest {
    /// Required. The name of the property to lookup.
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
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
    /// | Filter                      | Description                               |
    /// |-----------------------------|-------------------------------------------|
    /// | parent:accounts/123         | The account with account id: 123.         |
    /// | firebase_project:project-id | The firebase project with id: project-id. |
    /// | firebase_project:123        | The firebase project with number: 123.    |
    #[prost(string, tag = "1")]
    pub filter: std::string::String,
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
    pub page_token: std::string::String,
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
    pub properties: ::std::vec::Vec<Property>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for UpdateProperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePropertyRequest {
    /// Required. The property to update.
    /// The property's `name` field is used to identify the property to be
    /// updated.
    #[prost(message, optional, tag = "1")]
    pub property: ::std::option::Option<Property>,
    /// The list of fields to be updated. Omitted fields will not be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateProperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePropertyRequest {
    /// Required. The property to create.
    /// Note: the supplied property must specify its parent.
    #[prost(message, optional, tag = "1")]
    pub property: ::std::option::Option<Property>,
}
/// Request message for DeleteProperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePropertyRequest {
    /// Required. The name of the Property to soft-delete.
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for GetUserLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserLinkRequest {
    /// Required. Example format: accounts/1234/userLinks/5678
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for BatchGetUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetUserLinksRequest {
    /// Required. The account or property that all user links in the request are
    /// for. The parent of all provided values for the 'names' field must match
    /// this field.
    /// Example format: accounts/1234
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The names of the user links to retrieve.
    /// A maximum of 1000 user links can be retrieved in a batch.
    /// Format: accounts/{accountId}/userLinks/{userLinkId}
    #[prost(string, repeated, tag = "2")]
    pub names: ::std::vec::Vec<std::string::String>,
}
/// Response message for BatchGetUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetUserLinksResponse {
    /// The requested user links.
    #[prost(message, repeated, tag = "1")]
    pub user_links: ::std::vec::Vec<UserLink>,
}
/// Request message for ListUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserLinksRequest {
    /// Required. Example format: accounts/1234
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
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
    pub page_token: std::string::String,
}
/// Response message for ListUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserLinksResponse {
    /// List of UserLinks. These will be ordered stably, but in an arbitrary order.
    #[prost(message, repeated, tag = "1")]
    pub user_links: ::std::vec::Vec<UserLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for AuditUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditUserLinksRequest {
    /// Required. Example format: accounts/1234
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
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
    pub page_token: std::string::String,
}
/// Response message for AuditUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditUserLinksResponse {
    /// List of AuditUserLinks. These will be ordered stably, but in an arbitrary
    /// order.
    #[prost(message, repeated, tag = "1")]
    pub user_links: ::std::vec::Vec<AuditUserLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
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
    pub parent: std::string::String,
    /// Required. The user link to create.
    #[prost(message, optional, tag = "2")]
    pub user_link: ::std::option::Option<UserLink>,
}
/// Request message for BatchCreateUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateUserLinksRequest {
    /// Required. The account or property that all user links in the request are for.
    /// This field is required. The parent field in the CreateUserLinkRequest
    /// messages must either be empty or match this field.
    /// Example format: accounts/1234
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The requests specifying the user links to create.
    /// A maximum of 1000 user links can be created in a batch.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::std::vec::Vec<CreateUserLinkRequest>,
}
/// Response message for BatchCreateUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateUserLinksResponse {
    /// The user links created.
    #[prost(message, repeated, tag = "1")]
    pub user_links: ::std::vec::Vec<UserLink>,
}
/// Request message for UpdateUserLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserLinkRequest {
    /// Required. The user link to update.
    #[prost(message, optional, tag = "1")]
    pub user_link: ::std::option::Option<UserLink>,
}
/// Request message for BatchUpdateUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateUserLinksRequest {
    /// Required. The account or property that all user links in the request are
    /// for. The parent field in the UpdateUserLinkRequest messages must either be
    /// empty or match this field.
    /// Example format: accounts/1234
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The requests specifying the user links to update.
    /// A maximum of 1000 user links can be updated in a batch.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::std::vec::Vec<UpdateUserLinkRequest>,
}
/// Response message for BatchUpdateUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateUserLinksResponse {
    /// The user links updated.
    #[prost(message, repeated, tag = "1")]
    pub user_links: ::std::vec::Vec<UserLink>,
}
/// Request message for DeleteUserLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserLinkRequest {
    /// Required. Example format: accounts/1234/userLinks/5678
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for BatchDeleteUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteUserLinksRequest {
    /// Required. The account or property that all user links in the request are
    /// for. The parent of all values for user link names to delete must match this
    /// field.
    /// Example format: accounts/1234
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The requests specifying the user links to update.
    /// A maximum of 1000 user links can be updated in a batch.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::std::vec::Vec<DeleteUserLinkRequest>,
}
/// Request message for GetWebDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWebDataStreamRequest {
    /// Required. The name of the web data stream to lookup.
    /// Format: properties/{property_id}/webDataStreams/{stream_id}
    /// Example: "properties/123/webDataStreams/456"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for DeleteWebDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWebDataStreamRequest {
    /// Required. The name of the web data stream to delete.
    /// Format: properties/{property_id}/webDataStreams/{stream_id}
    /// Example: "properties/123/webDataStreams/456"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for UpdateWebDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWebDataStreamRequest {
    /// Required. The web stream to update.
    /// The `name` field is used to identify the web stream to be updated.
    #[prost(message, optional, tag = "1")]
    pub web_data_stream: ::std::option::Option<WebDataStream>,
    /// The list of fields to be updated. Omitted fields will not be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateWebDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWebDataStreamRequest {
    /// Required. The web stream to create.
    #[prost(message, optional, tag = "1")]
    pub web_data_stream: ::std::option::Option<WebDataStream>,
    /// Required. The parent resource where this web data stream will be created.
    /// Format: properties/123
    #[prost(string, tag = "2")]
    pub parent: std::string::String,
}
/// Request message for ListWebDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWebDataStreamsRequest {
    /// Required. The name of the parent property.
    /// For example, to list results of web streams under the property with Id
    /// 123: "properties/123"
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
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
    pub page_token: std::string::String,
}
/// Request message for ListWebDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWebDataStreamsResponse {
    /// Results that matched the filter criteria and were accessible to the caller.
    #[prost(message, repeated, tag = "1")]
    pub web_data_streams: ::std::vec::Vec<WebDataStream>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for GetIosAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIosAppDataStreamRequest {
    /// Required. The name of the iOS app data stream to lookup.
    /// Format: properties/{property_id}/iosAppDataStreams/{stream_id}
    /// Example: "properties/123/iosAppDataStreams/456"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for DeleteIosAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteIosAppDataStreamRequest {
    /// Required. The name of the iOS app data stream to delete.
    /// Format: properties/{property_id}/iosAppDataStreams/{stream_id}
    /// Example: "properties/123/iosAppDataStreams/456"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for UpdateIosAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIosAppDataStreamRequest {
    /// Required. The iOS app stream to update.
    /// The `name` field is used to identify the iOS app stream to be updated.
    #[prost(message, optional, tag = "1")]
    pub ios_app_data_stream: ::std::option::Option<IosAppDataStream>,
    /// The list of fields to be updated. Omitted fields will not be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateIosAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIosAppDataStreamRequest {
    /// Required. The iOS app data stream to create.
    #[prost(message, optional, tag = "1")]
    pub ios_app_data_stream: ::std::option::Option<IosAppDataStream>,
    /// Required. The parent resource where this ios app data stream will be created.
    /// Format: properties/123
    #[prost(string, tag = "2")]
    pub parent: std::string::String,
}
/// Request message for ListIosAppDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIosAppDataStreamsRequest {
    /// Required. The name of the parent property.
    /// For example, to list results of app streams under the property with Id
    /// 123: "properties/123"
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
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
    pub page_token: std::string::String,
}
/// Request message for ListIosAppDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIosAppDataStreamsResponse {
    /// Results that matched the filter criteria and were accessible to the caller.
    #[prost(message, repeated, tag = "1")]
    pub ios_app_data_streams: ::std::vec::Vec<IosAppDataStream>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for GetAndroidAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAndroidAppDataStreamRequest {
    /// Required. The name of the android app data stream to lookup.
    /// Format: properties/{property_id}/androidAppDataStreams/{stream_id}
    /// Example: "properties/123/androidAppDataStreams/456"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for DeleteAndroidAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAndroidAppDataStreamRequest {
    /// Required. The name of the android app data stream to delete.
    /// Format: properties/{property_id}/androidAppDataStreams/{stream_id}
    /// Example: "properties/123/androidAppDataStreams/456"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for UpdateAndroidAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAndroidAppDataStreamRequest {
    /// Required. The android app stream to update.
    /// The `name` field is used to identify the android app stream to be updated.
    #[prost(message, optional, tag = "1")]
    pub android_app_data_stream: ::std::option::Option<AndroidAppDataStream>,
    /// The list of fields to be updated. Omitted fields will not be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateAndroidAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAndroidAppDataStreamRequest {
    /// Required. The android app stream to create.
    #[prost(message, optional, tag = "1")]
    pub android_app_data_stream: ::std::option::Option<AndroidAppDataStream>,
    /// Required. The parent resource where this android app data stream will be created.
    /// Format: properties/123
    #[prost(string, tag = "2")]
    pub parent: std::string::String,
}
/// Request message for ListAndroidAppDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAndroidAppDataStreamsRequest {
    /// Required. The name of the parent property.
    /// For example, to limit results to app streams under the property with Id
    /// 123: "properties/123"
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
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
    pub page_token: std::string::String,
}
/// Request message for ListAndroidDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAndroidAppDataStreamsResponse {
    /// Results that matched the filter criteria and were accessible to the caller.
    #[prost(message, repeated, tag = "1")]
    pub android_app_data_streams: ::std::vec::Vec<AndroidAppDataStream>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for GetEnhancedMeasurementSettings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEnhancedMeasurementSettingsRequest {
    /// Required. The name of the settings to lookup.
    /// Format:
    /// properties/{property_id}/webDataStreams/{stream_id}/enhancedMeasurementSettings
    /// Example: "properties/1000/webDataStreams/2000/enhancedMeasurementSettings"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for UpdateEnhancedMeasurementSettings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEnhancedMeasurementSettingsRequest {
    /// Required. The settings to update.
    /// The `name` field is used to identify the settings to be updated.
    #[prost(message, optional, tag = "1")]
    pub enhanced_measurement_settings: ::std::option::Option<EnhancedMeasurementSettings>,
    /// The list of fields to be updated. Omitted fields will not be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateFirebaseLink RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFirebaseLinkRequest {
    /// Required. Format: properties/{property_id}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The Firebase link to create.
    #[prost(message, optional, tag = "2")]
    pub firebase_link: ::std::option::Option<FirebaseLink>,
}
/// Request message for UpdateFirebaseLink RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFirebaseLinkRequest {
    /// Required. The Firebase link to update.
    #[prost(message, optional, tag = "1")]
    pub firebase_link: ::std::option::Option<FirebaseLink>,
    /// The list of fields to be updated. Omitted fields will not be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteFirebaseLink RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFirebaseLinkRequest {
    /// Required. Format: properties/{property_id}/firebaseLinks/{firebase_link_id}
    /// Example: properties/1234/firebaseLinks/5678
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for ListFirebaseLinks RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFirebaseLinksRequest {
    /// Required. Format: properties/{property_id}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
}
/// Response message for ListFirebaseLinks RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFirebaseLinksResponse {
    /// List of FirebaseLinks. This will have at most one value.
    #[prost(message, repeated, tag = "1")]
    pub firebase_links: ::std::vec::Vec<FirebaseLink>,
}
/// Request message for GetGlobalSiteTag RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGlobalSiteTagRequest {
    /// Required. The name of the site tag to lookup.
    /// Note that site tags are singletons and do not have unique IDs.
    /// Format: properties/{property_id}/webDataStreams/{stream_id}/globalSiteTag
    /// Example: "properties/123/webDataStreams/456/globalSiteTag"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for CreateGoogleAdsLink RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGoogleAdsLinkRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The GoogleAdsLink to create.
    #[prost(message, optional, tag = "2")]
    pub google_ads_link: ::std::option::Option<GoogleAdsLink>,
}
/// Request message for UpdateGoogleAdsLink RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGoogleAdsLinkRequest {
    /// The GoogleAdsLink to update
    #[prost(message, optional, tag = "1")]
    pub google_ads_link: ::std::option::Option<GoogleAdsLink>,
    /// The list of fields to be updated. Omitted fields will not be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteGoogleAdsLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGoogleAdsLinkRequest {
    /// Required. Example format: properties/1234/googleAdsLinks/5678
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for ListGoogleAdsLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGoogleAdsLinksRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
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
    pub page_token: std::string::String,
}
/// Response message for ListGoogleAdsLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGoogleAdsLinksResponse {
    /// List of GoogleAdsLinks.
    #[prost(message, repeated, tag = "1")]
    pub google_ads_links: ::std::vec::Vec<GoogleAdsLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod management_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service Interface for the GA Management API (App+Web)."]
    pub struct ManagementClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ManagementClient<T>
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
        #[doc = " Throws \"Target not found\" if no such account found, or if caller does not"]
        #[doc = " have permissions to access it."]
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
                "/google.analytics.management.v1alpha.Management/GetAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns all accounts accessible by the caller."]
        #[doc = ""]
        #[doc = " Note that these accounts might not currently have App+Web properties."]
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
                "/google.analytics.management.v1alpha.Management/ListAccounts",
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
                "/google.analytics.management.v1alpha.Management/DeleteAccount",
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
                "/google.analytics.management.v1alpha.Management/UpdateAccount",
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
                "/google.analytics.management.v1alpha.Management/ProvisionAccountTicket",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single \"App+Web\" Property."]
        #[doc = ""]
        #[doc = " Throws \"Target not found\" if no such property found, if property is not"]
        #[doc = " of the type \"App+Web\", or if caller does not have permissions to access it."]
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
                "/google.analytics.management.v1alpha.Management/GetProperty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns child Properties under the specified parent Account."]
        #[doc = ""]
        #[doc = " Only \"App+Web\" properties will be returned."]
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
                "/google.analytics.management.v1alpha.Management/ListProperties",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an \"App+Web\" property with the specified location and attributes."]
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
                "/google.analytics.management.v1alpha.Management/CreateProperty",
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
        #[doc = " Returns an error if the target is not found, or is not an App+Web Property."]
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
                "/google.analytics.management.v1alpha.Management/DeleteProperty",
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
                "/google.analytics.management.v1alpha.Management/UpdateProperty",
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
                "/google.analytics.management.v1alpha.Management/GetUserLink",
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
                "/google.analytics.management.v1alpha.Management/BatchGetUserLinks",
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
                "/google.analytics.management.v1alpha.Management/ListUserLinks",
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
                "/google.analytics.management.v1alpha.Management/AuditUserLinks",
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
                "/google.analytics.management.v1alpha.Management/CreateUserLink",
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
                "/google.analytics.management.v1alpha.Management/BatchCreateUserLinks",
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
                "/google.analytics.management.v1alpha.Management/UpdateUserLink",
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
                "/google.analytics.management.v1alpha.Management/BatchUpdateUserLinks",
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
                "/google.analytics.management.v1alpha.Management/DeleteUserLink",
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
                "/google.analytics.management.v1alpha.Management/BatchDeleteUserLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single WebDataStream"]
        #[doc = ""]
        #[doc = " Throws \"Target not found\" if no such web data stream found, or if the"]
        #[doc = " caller does not have permissions to access it."]
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
                "/google.analytics.management.v1alpha.Management/GetWebDataStream",
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
                "/google.analytics.management.v1alpha.Management/DeleteWebDataStream",
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
                "/google.analytics.management.v1alpha.Management/UpdateWebDataStream",
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
                "/google.analytics.management.v1alpha.Management/CreateWebDataStream",
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
                "/google.analytics.management.v1alpha.Management/ListWebDataStreams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single IosAppDataStream"]
        #[doc = ""]
        #[doc = " Throws \"Target not found\" if no such iOS app data stream found, or if the"]
        #[doc = " caller does not have permissions to access it."]
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
                "/google.analytics.management.v1alpha.Management/GetIosAppDataStream",
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
                "/google.analytics.management.v1alpha.Management/DeleteIosAppDataStream",
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
                "/google.analytics.management.v1alpha.Management/UpdateIosAppDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an iOS app data stream with the specified location and attributes."]
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
                "/google.analytics.management.v1alpha.Management/CreateIosAppDataStream",
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
                "/google.analytics.management.v1alpha.Management/ListIosAppDataStreams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single AndroidAppDataStream"]
        #[doc = ""]
        #[doc = " Throws \"Target not found\" if no such android app data stream found, or if"]
        #[doc = " the caller does not have permissions to access it."]
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
                "/google.analytics.management.v1alpha.Management/GetAndroidAppDataStream",
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
                "/google.analytics.management.v1alpha.Management/DeleteAndroidAppDataStream",
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
                "/google.analytics.management.v1alpha.Management/UpdateAndroidAppDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an android app stream with the specified location and attributes."]
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
                "/google.analytics.management.v1alpha.Management/CreateAndroidAppDataStream",
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
                "/google.analytics.management.v1alpha.Management/ListAndroidAppDataStreams",
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
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.management.v1alpha.Management/GetEnhancedMeasurementSettings",
            );
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
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.management.v1alpha.Management/UpdateEnhancedMeasurementSettings",
            );
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
                "/google.analytics.management.v1alpha.Management/CreateFirebaseLink",
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
                "/google.analytics.management.v1alpha.Management/UpdateFirebaseLink",
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
                "/google.analytics.management.v1alpha.Management/DeleteFirebaseLink",
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
                "/google.analytics.management.v1alpha.Management/ListFirebaseLinks",
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
                "/google.analytics.management.v1alpha.Management/GetGlobalSiteTag",
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
                "/google.analytics.management.v1alpha.Management/CreateGoogleAdsLink",
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
                "/google.analytics.management.v1alpha.Management/UpdateGoogleAdsLink",
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
                "/google.analytics.management.v1alpha.Management/DeleteGoogleAdsLink",
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
                "/google.analytics.management.v1alpha.Management/ListGoogleAdsLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ManagementClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ManagementClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ManagementClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod management_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ManagementServer."]
    #[async_trait]
    pub trait Management: Send + Sync + 'static {
        #[doc = " Lookup for a single Account."]
        #[doc = " Throws \"Target not found\" if no such account found, or if caller does not"]
        #[doc = " have permissions to access it."]
        async fn get_account(
            &self,
            request: tonic::Request<super::GetAccountRequest>,
        ) -> Result<tonic::Response<super::Account>, tonic::Status>;
        #[doc = " Returns all accounts accessible by the caller."]
        #[doc = ""]
        #[doc = " Note that these accounts might not currently have App+Web properties."]
        #[doc = " Soft-deleted (ie: \"trashed\") accounts are excluded by default."]
        #[doc = " Returns an empty list if no relevant accounts are found."]
        async fn list_accounts(
            &self,
            request: tonic::Request<super::ListAccountsRequest>,
        ) -> Result<tonic::Response<super::ListAccountsResponse>, tonic::Status>;
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
        async fn delete_account(
            &self,
            request: tonic::Request<super::DeleteAccountRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Updates an account."]
        async fn update_account(
            &self,
            request: tonic::Request<super::UpdateAccountRequest>,
        ) -> Result<tonic::Response<super::Account>, tonic::Status>;
        #[doc = " Requests a ticket for creating an account."]
        async fn provision_account_ticket(
            &self,
            request: tonic::Request<super::ProvisionAccountTicketRequest>,
        ) -> Result<tonic::Response<super::ProvisionAccountTicketResponse>, tonic::Status>;
        #[doc = " Lookup for a single \"App+Web\" Property."]
        #[doc = ""]
        #[doc = " Throws \"Target not found\" if no such property found, if property is not"]
        #[doc = " of the type \"App+Web\", or if caller does not have permissions to access it."]
        async fn get_property(
            &self,
            request: tonic::Request<super::GetPropertyRequest>,
        ) -> Result<tonic::Response<super::Property>, tonic::Status>;
        #[doc = " Returns child Properties under the specified parent Account."]
        #[doc = ""]
        #[doc = " Only \"App+Web\" properties will be returned."]
        #[doc = " Properties will be excluded if the caller does not have access."]
        #[doc = " Soft-deleted (ie: \"trashed\") properties are excluded by default."]
        #[doc = " Returns an empty list if no relevant properties are found."]
        async fn list_properties(
            &self,
            request: tonic::Request<super::ListPropertiesRequest>,
        ) -> Result<tonic::Response<super::ListPropertiesResponse>, tonic::Status>;
        #[doc = " Creates an \"App+Web\" property with the specified location and attributes."]
        async fn create_property(
            &self,
            request: tonic::Request<super::CreatePropertyRequest>,
        ) -> Result<tonic::Response<super::Property>, tonic::Status>;
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
        #[doc = " Returns an error if the target is not found, or is not an App+Web Property."]
        async fn delete_property(
            &self,
            request: tonic::Request<super::DeletePropertyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Updates a property."]
        async fn update_property(
            &self,
            request: tonic::Request<super::UpdatePropertyRequest>,
        ) -> Result<tonic::Response<super::Property>, tonic::Status>;
        #[doc = " Gets information about a user's link to an account or property."]
        async fn get_user_link(
            &self,
            request: tonic::Request<super::GetUserLinkRequest>,
        ) -> Result<tonic::Response<super::UserLink>, tonic::Status>;
        #[doc = " Gets information about multiple users' links to an account or property."]
        async fn batch_get_user_links(
            &self,
            request: tonic::Request<super::BatchGetUserLinksRequest>,
        ) -> Result<tonic::Response<super::BatchGetUserLinksResponse>, tonic::Status>;
        #[doc = " Lists all user links on an account or property."]
        async fn list_user_links(
            &self,
            request: tonic::Request<super::ListUserLinksRequest>,
        ) -> Result<tonic::Response<super::ListUserLinksResponse>, tonic::Status>;
        #[doc = " Lists all user links on an account or property, including implicit ones"]
        #[doc = " that come from effective permissions granted by groups or organization"]
        #[doc = " admin roles."]
        #[doc = ""]
        #[doc = " If a returned user link does not have direct permissions, they cannot"]
        #[doc = " be removed from the account or property directly with the DeleteUserLink"]
        #[doc = " command. They have to be removed from the group/etc that gives them"]
        #[doc = " permissions, which is currently only usable/discoverable in the GA or GMP"]
        #[doc = " UIs."]
        async fn audit_user_links(
            &self,
            request: tonic::Request<super::AuditUserLinksRequest>,
        ) -> Result<tonic::Response<super::AuditUserLinksResponse>, tonic::Status>;
        #[doc = " Creates a user link on an account or property."]
        #[doc = ""]
        #[doc = " If the user with the specified email already has permissions on the"]
        #[doc = " account or property, then the user's existing permissions will be unioned"]
        #[doc = " with the permissions specified in the new UserLink."]
        async fn create_user_link(
            &self,
            request: tonic::Request<super::CreateUserLinkRequest>,
        ) -> Result<tonic::Response<super::UserLink>, tonic::Status>;
        #[doc = " Creates information about multiple users' links to an account or property."]
        #[doc = ""]
        #[doc = " This method is transactional. If any UserLink cannot be created, none of"]
        #[doc = " the UserLinks will be created."]
        async fn batch_create_user_links(
            &self,
            request: tonic::Request<super::BatchCreateUserLinksRequest>,
        ) -> Result<tonic::Response<super::BatchCreateUserLinksResponse>, tonic::Status>;
        #[doc = " Updates a user link on an account or property."]
        async fn update_user_link(
            &self,
            request: tonic::Request<super::UpdateUserLinkRequest>,
        ) -> Result<tonic::Response<super::UserLink>, tonic::Status>;
        #[doc = " Updates information about multiple users' links to an account or property."]
        async fn batch_update_user_links(
            &self,
            request: tonic::Request<super::BatchUpdateUserLinksRequest>,
        ) -> Result<tonic::Response<super::BatchUpdateUserLinksResponse>, tonic::Status>;
        #[doc = " Deletes a user link on an account or property."]
        async fn delete_user_link(
            &self,
            request: tonic::Request<super::DeleteUserLinkRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Deletes information about multiple users' links to an account or property."]
        async fn batch_delete_user_links(
            &self,
            request: tonic::Request<super::BatchDeleteUserLinksRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Lookup for a single WebDataStream"]
        #[doc = ""]
        #[doc = " Throws \"Target not found\" if no such web data stream found, or if the"]
        #[doc = " caller does not have permissions to access it."]
        async fn get_web_data_stream(
            &self,
            request: tonic::Request<super::GetWebDataStreamRequest>,
        ) -> Result<tonic::Response<super::WebDataStream>, tonic::Status>;
        #[doc = " Deletes a web stream on a property."]
        async fn delete_web_data_stream(
            &self,
            request: tonic::Request<super::DeleteWebDataStreamRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Updates a web stream on a property."]
        async fn update_web_data_stream(
            &self,
            request: tonic::Request<super::UpdateWebDataStreamRequest>,
        ) -> Result<tonic::Response<super::WebDataStream>, tonic::Status>;
        #[doc = " Creates a web stream with the specified location and attributes."]
        async fn create_web_data_stream(
            &self,
            request: tonic::Request<super::CreateWebDataStreamRequest>,
        ) -> Result<tonic::Response<super::WebDataStream>, tonic::Status>;
        #[doc = " Returns child web data streams under the specified parent property."]
        #[doc = ""]
        #[doc = " Web data streams will be excluded if the caller does not have access."]
        #[doc = " Returns an empty list if no relevant web data streams are found."]
        async fn list_web_data_streams(
            &self,
            request: tonic::Request<super::ListWebDataStreamsRequest>,
        ) -> Result<tonic::Response<super::ListWebDataStreamsResponse>, tonic::Status>;
        #[doc = " Lookup for a single IosAppDataStream"]
        #[doc = ""]
        #[doc = " Throws \"Target not found\" if no such iOS app data stream found, or if the"]
        #[doc = " caller does not have permissions to access it."]
        async fn get_ios_app_data_stream(
            &self,
            request: tonic::Request<super::GetIosAppDataStreamRequest>,
        ) -> Result<tonic::Response<super::IosAppDataStream>, tonic::Status>;
        #[doc = " Deletes an iOS app stream on a property."]
        async fn delete_ios_app_data_stream(
            &self,
            request: tonic::Request<super::DeleteIosAppDataStreamRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Updates an iOS app stream on a property."]
        async fn update_ios_app_data_stream(
            &self,
            request: tonic::Request<super::UpdateIosAppDataStreamRequest>,
        ) -> Result<tonic::Response<super::IosAppDataStream>, tonic::Status>;
        #[doc = " Creates an iOS app data stream with the specified location and attributes."]
        async fn create_ios_app_data_stream(
            &self,
            request: tonic::Request<super::CreateIosAppDataStreamRequest>,
        ) -> Result<tonic::Response<super::IosAppDataStream>, tonic::Status>;
        #[doc = " Returns child iOS app data streams under the specified parent property."]
        #[doc = ""]
        #[doc = " iOS app data streams will be excluded if the caller does not have access."]
        #[doc = " Returns an empty list if no relevant iOS app data streams are found."]
        async fn list_ios_app_data_streams(
            &self,
            request: tonic::Request<super::ListIosAppDataStreamsRequest>,
        ) -> Result<tonic::Response<super::ListIosAppDataStreamsResponse>, tonic::Status>;
        #[doc = " Lookup for a single AndroidAppDataStream"]
        #[doc = ""]
        #[doc = " Throws \"Target not found\" if no such android app data stream found, or if"]
        #[doc = " the caller does not have permissions to access it."]
        async fn get_android_app_data_stream(
            &self,
            request: tonic::Request<super::GetAndroidAppDataStreamRequest>,
        ) -> Result<tonic::Response<super::AndroidAppDataStream>, tonic::Status>;
        #[doc = " Deletes an android app stream on a property."]
        async fn delete_android_app_data_stream(
            &self,
            request: tonic::Request<super::DeleteAndroidAppDataStreamRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Updates an android app stream on a property."]
        async fn update_android_app_data_stream(
            &self,
            request: tonic::Request<super::UpdateAndroidAppDataStreamRequest>,
        ) -> Result<tonic::Response<super::AndroidAppDataStream>, tonic::Status>;
        #[doc = " Creates an android app stream with the specified location and attributes."]
        async fn create_android_app_data_stream(
            &self,
            request: tonic::Request<super::CreateAndroidAppDataStreamRequest>,
        ) -> Result<tonic::Response<super::AndroidAppDataStream>, tonic::Status>;
        #[doc = " Returns child android app streams under the specified parent property."]
        #[doc = ""]
        #[doc = " Android app streams will be excluded if the caller does not have access."]
        #[doc = " Returns an empty list if no relevant android app streams are found."]
        async fn list_android_app_data_streams(
            &self,
            request: tonic::Request<super::ListAndroidAppDataStreamsRequest>,
        ) -> Result<tonic::Response<super::ListAndroidAppDataStreamsResponse>, tonic::Status>;
        #[doc = " Returns the singleton enhanced measurement settings for this web stream."]
        #[doc = " Note that the stream must enable enhanced measurement for these settings to"]
        #[doc = " take effect."]
        async fn get_enhanced_measurement_settings(
            &self,
            request: tonic::Request<super::GetEnhancedMeasurementSettingsRequest>,
        ) -> Result<tonic::Response<super::EnhancedMeasurementSettings>, tonic::Status>;
        #[doc = " Updates the singleton enhanced measurement settings for this web stream."]
        #[doc = " Note that the stream must enable enhanced measurement for these settings to"]
        #[doc = " take effect."]
        async fn update_enhanced_measurement_settings(
            &self,
            request: tonic::Request<super::UpdateEnhancedMeasurementSettingsRequest>,
        ) -> Result<tonic::Response<super::EnhancedMeasurementSettings>, tonic::Status>;
        #[doc = " Creates a FirebaseLink."]
        #[doc = ""]
        #[doc = " Properties can have at most one FirebaseLink."]
        async fn create_firebase_link(
            &self,
            request: tonic::Request<super::CreateFirebaseLinkRequest>,
        ) -> Result<tonic::Response<super::FirebaseLink>, tonic::Status>;
        #[doc = " Updates a FirebaseLink on a property"]
        async fn update_firebase_link(
            &self,
            request: tonic::Request<super::UpdateFirebaseLinkRequest>,
        ) -> Result<tonic::Response<super::FirebaseLink>, tonic::Status>;
        #[doc = " Deletes a FirebaseLink on a property"]
        async fn delete_firebase_link(
            &self,
            request: tonic::Request<super::DeleteFirebaseLinkRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Lists FirebaseLinks on a property."]
        #[doc = " Properties can have at most one FirebaseLink."]
        async fn list_firebase_links(
            &self,
            request: tonic::Request<super::ListFirebaseLinksRequest>,
        ) -> Result<tonic::Response<super::ListFirebaseLinksResponse>, tonic::Status>;
        #[doc = " Returns the Site Tag for the specified web stream."]
        #[doc = " Site Tags are immutable singletons."]
        async fn get_global_site_tag(
            &self,
            request: tonic::Request<super::GetGlobalSiteTagRequest>,
        ) -> Result<tonic::Response<super::GlobalSiteTag>, tonic::Status>;
        #[doc = " Creates a GoogleAdsLink."]
        async fn create_google_ads_link(
            &self,
            request: tonic::Request<super::CreateGoogleAdsLinkRequest>,
        ) -> Result<tonic::Response<super::GoogleAdsLink>, tonic::Status>;
        #[doc = " Updates a GoogleAdsLink on a property"]
        async fn update_google_ads_link(
            &self,
            request: tonic::Request<super::UpdateGoogleAdsLinkRequest>,
        ) -> Result<tonic::Response<super::GoogleAdsLink>, tonic::Status>;
        #[doc = " Deletes a GoogleAdsLink on a property"]
        async fn delete_google_ads_link(
            &self,
            request: tonic::Request<super::DeleteGoogleAdsLinkRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Lists GoogleAdsLinks on a property."]
        async fn list_google_ads_links(
            &self,
            request: tonic::Request<super::ListGoogleAdsLinksRequest>,
        ) -> Result<tonic::Response<super::ListGoogleAdsLinksResponse>, tonic::Status>;
    }
    #[doc = " Service Interface for the GA Management API (App+Web)."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ManagementServer<T: Management> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Management> ManagementServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ManagementServer<T>
    where
        T: Management,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req . uri ( ) . path ( ) { "/google.analytics.management.v1alpha.Management/GetAccount" => { # [ allow ( non_camel_case_types ) ] struct GetAccountSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: GetAccountRequest > for GetAccountSvc < T > { type Response = super :: Account ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetAccountRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_account ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetAccountSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/ListAccounts" => { # [ allow ( non_camel_case_types ) ] struct ListAccountsSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: ListAccountsRequest > for ListAccountsSvc < T > { type Response = super :: ListAccountsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListAccountsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_accounts ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListAccountsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/DeleteAccount" => { # [ allow ( non_camel_case_types ) ] struct DeleteAccountSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: DeleteAccountRequest > for DeleteAccountSvc < T > { type Response = ( ) ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeleteAccountRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . delete_account ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeleteAccountSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/UpdateAccount" => { # [ allow ( non_camel_case_types ) ] struct UpdateAccountSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: UpdateAccountRequest > for UpdateAccountSvc < T > { type Response = super :: Account ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateAccountRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_account ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateAccountSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/ProvisionAccountTicket" => { # [ allow ( non_camel_case_types ) ] struct ProvisionAccountTicketSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: ProvisionAccountTicketRequest > for ProvisionAccountTicketSvc < T > { type Response = super :: ProvisionAccountTicketResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ProvisionAccountTicketRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . provision_account_ticket ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ProvisionAccountTicketSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/GetProperty" => { # [ allow ( non_camel_case_types ) ] struct GetPropertySvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: GetPropertyRequest > for GetPropertySvc < T > { type Response = super :: Property ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetPropertyRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_property ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetPropertySvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/ListProperties" => { # [ allow ( non_camel_case_types ) ] struct ListPropertiesSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: ListPropertiesRequest > for ListPropertiesSvc < T > { type Response = super :: ListPropertiesResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListPropertiesRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_properties ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListPropertiesSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/CreateProperty" => { # [ allow ( non_camel_case_types ) ] struct CreatePropertySvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: CreatePropertyRequest > for CreatePropertySvc < T > { type Response = super :: Property ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreatePropertyRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . create_property ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreatePropertySvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/DeleteProperty" => { # [ allow ( non_camel_case_types ) ] struct DeletePropertySvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: DeletePropertyRequest > for DeletePropertySvc < T > { type Response = ( ) ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeletePropertyRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . delete_property ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeletePropertySvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/UpdateProperty" => { # [ allow ( non_camel_case_types ) ] struct UpdatePropertySvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: UpdatePropertyRequest > for UpdatePropertySvc < T > { type Response = super :: Property ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdatePropertyRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_property ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdatePropertySvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/GetUserLink" => { # [ allow ( non_camel_case_types ) ] struct GetUserLinkSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: GetUserLinkRequest > for GetUserLinkSvc < T > { type Response = super :: UserLink ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetUserLinkRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_user_link ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetUserLinkSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/BatchGetUserLinks" => { # [ allow ( non_camel_case_types ) ] struct BatchGetUserLinksSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: BatchGetUserLinksRequest > for BatchGetUserLinksSvc < T > { type Response = super :: BatchGetUserLinksResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: BatchGetUserLinksRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . batch_get_user_links ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = BatchGetUserLinksSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/ListUserLinks" => { # [ allow ( non_camel_case_types ) ] struct ListUserLinksSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: ListUserLinksRequest > for ListUserLinksSvc < T > { type Response = super :: ListUserLinksResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListUserLinksRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_user_links ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListUserLinksSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/AuditUserLinks" => { # [ allow ( non_camel_case_types ) ] struct AuditUserLinksSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: AuditUserLinksRequest > for AuditUserLinksSvc < T > { type Response = super :: AuditUserLinksResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: AuditUserLinksRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . audit_user_links ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = AuditUserLinksSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/CreateUserLink" => { # [ allow ( non_camel_case_types ) ] struct CreateUserLinkSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: CreateUserLinkRequest > for CreateUserLinkSvc < T > { type Response = super :: UserLink ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateUserLinkRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . create_user_link ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateUserLinkSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/BatchCreateUserLinks" => { # [ allow ( non_camel_case_types ) ] struct BatchCreateUserLinksSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: BatchCreateUserLinksRequest > for BatchCreateUserLinksSvc < T > { type Response = super :: BatchCreateUserLinksResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: BatchCreateUserLinksRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . batch_create_user_links ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = BatchCreateUserLinksSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/UpdateUserLink" => { # [ allow ( non_camel_case_types ) ] struct UpdateUserLinkSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: UpdateUserLinkRequest > for UpdateUserLinkSvc < T > { type Response = super :: UserLink ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateUserLinkRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_user_link ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateUserLinkSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/BatchUpdateUserLinks" => { # [ allow ( non_camel_case_types ) ] struct BatchUpdateUserLinksSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: BatchUpdateUserLinksRequest > for BatchUpdateUserLinksSvc < T > { type Response = super :: BatchUpdateUserLinksResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: BatchUpdateUserLinksRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . batch_update_user_links ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = BatchUpdateUserLinksSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/DeleteUserLink" => { # [ allow ( non_camel_case_types ) ] struct DeleteUserLinkSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: DeleteUserLinkRequest > for DeleteUserLinkSvc < T > { type Response = ( ) ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeleteUserLinkRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . delete_user_link ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeleteUserLinkSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/BatchDeleteUserLinks" => { # [ allow ( non_camel_case_types ) ] struct BatchDeleteUserLinksSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: BatchDeleteUserLinksRequest > for BatchDeleteUserLinksSvc < T > { type Response = ( ) ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: BatchDeleteUserLinksRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . batch_delete_user_links ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = BatchDeleteUserLinksSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/GetWebDataStream" => { # [ allow ( non_camel_case_types ) ] struct GetWebDataStreamSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: GetWebDataStreamRequest > for GetWebDataStreamSvc < T > { type Response = super :: WebDataStream ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetWebDataStreamRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_web_data_stream ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetWebDataStreamSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/DeleteWebDataStream" => { # [ allow ( non_camel_case_types ) ] struct DeleteWebDataStreamSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: DeleteWebDataStreamRequest > for DeleteWebDataStreamSvc < T > { type Response = ( ) ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeleteWebDataStreamRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . delete_web_data_stream ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeleteWebDataStreamSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/UpdateWebDataStream" => { # [ allow ( non_camel_case_types ) ] struct UpdateWebDataStreamSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: UpdateWebDataStreamRequest > for UpdateWebDataStreamSvc < T > { type Response = super :: WebDataStream ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateWebDataStreamRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_web_data_stream ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateWebDataStreamSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/CreateWebDataStream" => { # [ allow ( non_camel_case_types ) ] struct CreateWebDataStreamSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: CreateWebDataStreamRequest > for CreateWebDataStreamSvc < T > { type Response = super :: WebDataStream ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateWebDataStreamRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . create_web_data_stream ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateWebDataStreamSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/ListWebDataStreams" => { # [ allow ( non_camel_case_types ) ] struct ListWebDataStreamsSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: ListWebDataStreamsRequest > for ListWebDataStreamsSvc < T > { type Response = super :: ListWebDataStreamsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListWebDataStreamsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_web_data_streams ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListWebDataStreamsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/GetIosAppDataStream" => { # [ allow ( non_camel_case_types ) ] struct GetIosAppDataStreamSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: GetIosAppDataStreamRequest > for GetIosAppDataStreamSvc < T > { type Response = super :: IosAppDataStream ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetIosAppDataStreamRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_ios_app_data_stream ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetIosAppDataStreamSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/DeleteIosAppDataStream" => { # [ allow ( non_camel_case_types ) ] struct DeleteIosAppDataStreamSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: DeleteIosAppDataStreamRequest > for DeleteIosAppDataStreamSvc < T > { type Response = ( ) ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeleteIosAppDataStreamRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . delete_ios_app_data_stream ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeleteIosAppDataStreamSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/UpdateIosAppDataStream" => { # [ allow ( non_camel_case_types ) ] struct UpdateIosAppDataStreamSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: UpdateIosAppDataStreamRequest > for UpdateIosAppDataStreamSvc < T > { type Response = super :: IosAppDataStream ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateIosAppDataStreamRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_ios_app_data_stream ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateIosAppDataStreamSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/CreateIosAppDataStream" => { # [ allow ( non_camel_case_types ) ] struct CreateIosAppDataStreamSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: CreateIosAppDataStreamRequest > for CreateIosAppDataStreamSvc < T > { type Response = super :: IosAppDataStream ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateIosAppDataStreamRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . create_ios_app_data_stream ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateIosAppDataStreamSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/ListIosAppDataStreams" => { # [ allow ( non_camel_case_types ) ] struct ListIosAppDataStreamsSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: ListIosAppDataStreamsRequest > for ListIosAppDataStreamsSvc < T > { type Response = super :: ListIosAppDataStreamsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListIosAppDataStreamsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_ios_app_data_streams ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListIosAppDataStreamsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/GetAndroidAppDataStream" => { # [ allow ( non_camel_case_types ) ] struct GetAndroidAppDataStreamSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: GetAndroidAppDataStreamRequest > for GetAndroidAppDataStreamSvc < T > { type Response = super :: AndroidAppDataStream ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetAndroidAppDataStreamRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_android_app_data_stream ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetAndroidAppDataStreamSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/DeleteAndroidAppDataStream" => { # [ allow ( non_camel_case_types ) ] struct DeleteAndroidAppDataStreamSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: DeleteAndroidAppDataStreamRequest > for DeleteAndroidAppDataStreamSvc < T > { type Response = ( ) ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeleteAndroidAppDataStreamRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . delete_android_app_data_stream ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeleteAndroidAppDataStreamSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/UpdateAndroidAppDataStream" => { # [ allow ( non_camel_case_types ) ] struct UpdateAndroidAppDataStreamSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: UpdateAndroidAppDataStreamRequest > for UpdateAndroidAppDataStreamSvc < T > { type Response = super :: AndroidAppDataStream ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateAndroidAppDataStreamRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_android_app_data_stream ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateAndroidAppDataStreamSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/CreateAndroidAppDataStream" => { # [ allow ( non_camel_case_types ) ] struct CreateAndroidAppDataStreamSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: CreateAndroidAppDataStreamRequest > for CreateAndroidAppDataStreamSvc < T > { type Response = super :: AndroidAppDataStream ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateAndroidAppDataStreamRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . create_android_app_data_stream ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateAndroidAppDataStreamSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/ListAndroidAppDataStreams" => { # [ allow ( non_camel_case_types ) ] struct ListAndroidAppDataStreamsSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: ListAndroidAppDataStreamsRequest > for ListAndroidAppDataStreamsSvc < T > { type Response = super :: ListAndroidAppDataStreamsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListAndroidAppDataStreamsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_android_app_data_streams ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListAndroidAppDataStreamsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/GetEnhancedMeasurementSettings" => { # [ allow ( non_camel_case_types ) ] struct GetEnhancedMeasurementSettingsSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: GetEnhancedMeasurementSettingsRequest > for GetEnhancedMeasurementSettingsSvc < T > { type Response = super :: EnhancedMeasurementSettings ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetEnhancedMeasurementSettingsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_enhanced_measurement_settings ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetEnhancedMeasurementSettingsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/UpdateEnhancedMeasurementSettings" => { # [ allow ( non_camel_case_types ) ] struct UpdateEnhancedMeasurementSettingsSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: UpdateEnhancedMeasurementSettingsRequest > for UpdateEnhancedMeasurementSettingsSvc < T > { type Response = super :: EnhancedMeasurementSettings ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateEnhancedMeasurementSettingsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_enhanced_measurement_settings ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateEnhancedMeasurementSettingsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/CreateFirebaseLink" => { # [ allow ( non_camel_case_types ) ] struct CreateFirebaseLinkSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: CreateFirebaseLinkRequest > for CreateFirebaseLinkSvc < T > { type Response = super :: FirebaseLink ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateFirebaseLinkRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . create_firebase_link ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateFirebaseLinkSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/UpdateFirebaseLink" => { # [ allow ( non_camel_case_types ) ] struct UpdateFirebaseLinkSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: UpdateFirebaseLinkRequest > for UpdateFirebaseLinkSvc < T > { type Response = super :: FirebaseLink ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateFirebaseLinkRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_firebase_link ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateFirebaseLinkSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/DeleteFirebaseLink" => { # [ allow ( non_camel_case_types ) ] struct DeleteFirebaseLinkSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: DeleteFirebaseLinkRequest > for DeleteFirebaseLinkSvc < T > { type Response = ( ) ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeleteFirebaseLinkRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . delete_firebase_link ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeleteFirebaseLinkSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/ListFirebaseLinks" => { # [ allow ( non_camel_case_types ) ] struct ListFirebaseLinksSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: ListFirebaseLinksRequest > for ListFirebaseLinksSvc < T > { type Response = super :: ListFirebaseLinksResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListFirebaseLinksRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_firebase_links ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListFirebaseLinksSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/GetGlobalSiteTag" => { # [ allow ( non_camel_case_types ) ] struct GetGlobalSiteTagSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: GetGlobalSiteTagRequest > for GetGlobalSiteTagSvc < T > { type Response = super :: GlobalSiteTag ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetGlobalSiteTagRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_global_site_tag ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetGlobalSiteTagSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/CreateGoogleAdsLink" => { # [ allow ( non_camel_case_types ) ] struct CreateGoogleAdsLinkSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: CreateGoogleAdsLinkRequest > for CreateGoogleAdsLinkSvc < T > { type Response = super :: GoogleAdsLink ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateGoogleAdsLinkRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . create_google_ads_link ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateGoogleAdsLinkSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/UpdateGoogleAdsLink" => { # [ allow ( non_camel_case_types ) ] struct UpdateGoogleAdsLinkSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: UpdateGoogleAdsLinkRequest > for UpdateGoogleAdsLinkSvc < T > { type Response = super :: GoogleAdsLink ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateGoogleAdsLinkRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_google_ads_link ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateGoogleAdsLinkSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/DeleteGoogleAdsLink" => { # [ allow ( non_camel_case_types ) ] struct DeleteGoogleAdsLinkSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: DeleteGoogleAdsLinkRequest > for DeleteGoogleAdsLinkSvc < T > { type Response = ( ) ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeleteGoogleAdsLinkRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . delete_google_ads_link ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeleteGoogleAdsLinkSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.analytics.management.v1alpha.Management/ListGoogleAdsLinks" => { # [ allow ( non_camel_case_types ) ] struct ListGoogleAdsLinksSvc < T : Management > ( pub Arc < T > ) ; impl < T : Management > tonic :: server :: UnaryService < super :: ListGoogleAdsLinksRequest > for ListGoogleAdsLinksSvc < T > { type Response = super :: ListGoogleAdsLinksResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListGoogleAdsLinksRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_google_ads_links ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListGoogleAdsLinksSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } _ => Box :: pin ( async move { Ok ( http :: Response :: builder ( ) . status ( 200 ) . header ( "grpc-status" , "12" ) . body ( tonic :: body :: BoxBody :: empty ( ) ) . unwrap ( ) ) } ) , }
        }
    }
    impl<T: Management> Clone for ManagementServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Management> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
}
