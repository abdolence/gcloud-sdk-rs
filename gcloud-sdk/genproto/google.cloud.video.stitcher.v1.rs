/// Container for a live session's ad tag detail.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveAdTagDetail {
    /// The resource name in the form of
    /// `projects/{project}/locations/{location}/liveSessions/{live_session}/liveAdTagDetails/{id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// A list of ad requests.
    #[prost(message, repeated, tag="2")]
    pub ad_requests: ::prost::alloc::vec::Vec<AdRequest>,
}
/// Information related to the details for one ad tag.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VodAdTagDetail {
    /// The name of the ad tag detail for the specified VOD session, in the form of
    /// `projects/{project}/locations/{location}/vodSessions/{vod_session_id}/vodAdTagDetails/{id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// A list of ad requests for one ad tag.
    #[prost(message, repeated, tag="2")]
    pub ad_requests: ::prost::alloc::vec::Vec<AdRequest>,
}
/// Details of an ad request to an ad server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdRequest {
    /// The ad tag URI processed with integrated macros.
    #[prost(string, tag="1")]
    pub uri: ::prost::alloc::string::String,
    /// The request metadata used to make the ad request.
    #[prost(message, optional, tag="2")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
    /// The response metadata received from the ad request.
    #[prost(message, optional, tag="3")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
}
/// Metadata for an ad request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMetadata {
    /// The HTTP headers of the ad request.
    #[prost(message, optional, tag="1")]
    pub headers: ::core::option::Option<::prost_types::Struct>,
}
/// Metadata for the response of an ad request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMetadata {
    /// Error message received when making the ad request.
    #[prost(string, tag="1")]
    pub error: ::prost::alloc::string::String,
    /// Headers from the response.
    #[prost(message, optional, tag="2")]
    pub headers: ::core::option::Option<::prost_types::Struct>,
    /// Status code for the response.
    #[prost(string, tag="3")]
    pub status_code: ::prost::alloc::string::String,
    /// Size in bytes of the response.
    #[prost(int32, tag="4")]
    pub size_bytes: i32,
    /// Total time elapsed for the response.
    #[prost(message, optional, tag="5")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// The body of the response.
    #[prost(string, tag="6")]
    pub body: ::prost::alloc::string::String,
}
/// Configuration for a CDN key. Used by the Video Stitcher
/// to sign URIs for fetching video manifests and signing
/// media segments for playback.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdnKey {
    /// The resource name of the CDN key, in the form of
    /// `projects/{project}/locations/{location}/cdnKeys/{id}`.
    /// The name is ignored when creating a CDN key.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The hostname this key applies to.
    #[prost(string, tag="4")]
    pub hostname: ::prost::alloc::string::String,
    /// Configuration associated with the CDN key.
    #[prost(oneof="cdn_key::CdnKeyConfig", tags="5, 6")]
    pub cdn_key_config: ::core::option::Option<cdn_key::CdnKeyConfig>,
}
/// Nested message and enum types in `CdnKey`.
pub mod cdn_key {
    /// Configuration associated with the CDN key.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CdnKeyConfig {
        /// The configuration for a Google Cloud CDN key.
        #[prost(message, tag="5")]
        GoogleCdnKey(super::GoogleCdnKey),
        /// The configuration for an Akamai CDN key.
        #[prost(message, tag="6")]
        AkamaiCdnKey(super::AkamaiCdnKey),
    }
}
/// Configuration for a Google Cloud CDN key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleCdnKey {
    /// Input only. Secret for this Google Cloud CDN key.
    #[prost(bytes="vec", tag="1")]
    pub private_key: ::prost::alloc::vec::Vec<u8>,
    /// The public name of the Google Cloud CDN key.
    #[prost(string, tag="2")]
    pub key_name: ::prost::alloc::string::String,
}
/// Configuration for an Akamai CDN key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AkamaiCdnKey {
    /// Input only. Token key for the Akamai CDN edge configuration.
    #[prost(bytes="vec", tag="1")]
    pub token_key: ::prost::alloc::vec::Vec<u8>,
}
/// Describes an event and a trigger URI.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Describes the event that occurred.
    #[prost(enumeration="event::EventType", tag="1")]
    pub r#type: i32,
    /// The URI to trigger for this event.
    #[prost(string, tag="2")]
    pub uri: ::prost::alloc::string::String,
    /// The ID of the event.
    #[prost(string, tag="3")]
    pub id: ::prost::alloc::string::String,
    /// The offset in seconds if the event type is `PROGRESS`.
    #[prost(message, optional, tag="4")]
    pub offset: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    /// Describes the event that occurred.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventType {
        /// The event type is unspecified.
        Unspecified = 0,
        /// First frame of creative ad viewed.
        CreativeView = 1,
        /// Creative ad started.
        Start = 2,
        /// Start of an ad break.
        BreakStart = 3,
        /// End of an ad break.
        BreakEnd = 4,
        /// Impression.
        Impression = 5,
        /// First quartile progress.
        FirstQuartile = 6,
        /// Midpoint progress.
        Midpoint = 7,
        /// Third quartile progress.
        ThirdQuartile = 8,
        /// Ad progress completed.
        Complete = 9,
        /// Specific progress event with an offset.
        Progress = 10,
        /// Player muted.
        Mute = 11,
        /// Player unmuted.
        Unmute = 12,
        /// Player paused.
        Pause = 13,
        /// Click event.
        Click = 14,
        /// Click-through event.
        ClickThrough = 15,
        /// Player rewinding.
        Rewind = 16,
        /// Player resumed.
        Resume = 17,
        /// Error event.
        Error = 18,
        /// Ad expanded to a larger size.
        Expand = 21,
        /// Ad collapsed to a smaller size.
        Collapse = 22,
        /// Non-linear ad closed.
        Close = 24,
        /// Linear ad closed.
        CloseLinear = 25,
        /// Ad skipped.
        Skip = 26,
        /// Accept invitation event.
        AcceptInvitation = 27,
    }
}
/// Indicates a time in which a list of events should be triggered
/// during media playback.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressEvent {
    /// The time when the following tracking events occurs. The time is in
    /// seconds relative to the start of the VOD asset.
    #[prost(message, optional, tag="1")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
    /// The list of progress tracking events for the ad break. These can be of
    /// the following IAB types: `BREAK_START`, `BREAK_END`, `IMPRESSION`,
    /// `CREATIVE_VIEW`, `START`, `FIRST_QUARTILE`, `MIDPOINT`, `THIRD_QUARTILE`,
    /// `COMPLETE`, `PROGRESS`.
    #[prost(message, repeated, tag="2")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
/// Metadata for companion ads.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompanionAds {
    /// Indicates how many of the companions should be displayed with the ad.
    #[prost(enumeration="companion_ads::DisplayRequirement", tag="1")]
    pub display_requirement: i32,
    /// List of companion ads.
    #[prost(message, repeated, tag="2")]
    pub companions: ::prost::alloc::vec::Vec<Companion>,
}
/// Nested message and enum types in `CompanionAds`.
pub mod companion_ads {
    /// Indicates how many of the companions should be displayed with the ad.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DisplayRequirement {
        /// Required companions are not specified. The default is ALL.
        Unspecified = 0,
        /// All companions are required to be displayed.
        All = 1,
        /// At least one of companions needs to be displayed.
        Any = 2,
        /// All companions are optional for display.
        None = 3,
    }
}
/// Metadata for a companion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Companion {
    /// The API necessary to communicate with the creative if available.
    #[prost(string, tag="1")]
    pub api_framework: ::prost::alloc::string::String,
    /// The pixel height of the placement slot for the intended creative.
    #[prost(int32, tag="2")]
    pub height_px: i32,
    /// The pixel width of the placement slot for the intended creative.
    #[prost(int32, tag="3")]
    pub width_px: i32,
    /// The pixel height of the creative.
    #[prost(int32, tag="4")]
    pub asset_height_px: i32,
    /// The maximum pixel height of the creative in its expanded state.
    #[prost(int32, tag="5")]
    pub expanded_height_px: i32,
    /// The pixel width of the creative.
    #[prost(int32, tag="6")]
    pub asset_width_px: i32,
    /// The maximum pixel width of the creative in its expanded state.
    #[prost(int32, tag="7")]
    pub expanded_width_px: i32,
    /// The ID used to identify the desired placement on a publisher's page.
    /// Values to be used should be discussed between publishers and
    /// advertisers.
    #[prost(string, tag="8")]
    pub ad_slot_id: ::prost::alloc::string::String,
    /// The list of tracking events for the companion.
    #[prost(message, repeated, tag="9")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    /// Ad resource associated with the companion ad.
    #[prost(oneof="companion::AdResource", tags="10, 11, 12")]
    pub ad_resource: ::core::option::Option<companion::AdResource>,
}
/// Nested message and enum types in `Companion`.
pub mod companion {
    /// Ad resource associated with the companion ad.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AdResource {
        /// The IFrame ad resource associated with the companion ad.
        #[prost(message, tag="10")]
        IframeAdResource(super::IframeAdResource),
        /// The static ad resource associated with the companion ad.
        #[prost(message, tag="11")]
        StaticAdResource(super::StaticAdResource),
        /// The HTML ad resource associated with the companion ad.
        #[prost(message, tag="12")]
        HtmlAdResource(super::HtmlAdResource),
    }
}
/// Metadata for an HTML ad resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HtmlAdResource {
    /// The HTML to display for the ad resource.
    #[prost(string, tag="1")]
    pub html_source: ::prost::alloc::string::String,
}
/// Metadata for an IFrame ad resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IframeAdResource {
    /// URI source for an IFrame to display for the ad resource.
    #[prost(string, tag="1")]
    pub uri: ::prost::alloc::string::String,
}
/// Metadata for a static ad resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticAdResource {
    /// URI to the static file for the ad resource.
    #[prost(string, tag="1")]
    pub uri: ::prost::alloc::string::String,
    /// Describes the MIME type of the ad resource.
    #[prost(string, tag="2")]
    pub creative_type: ::prost::alloc::string::String,
}
/// Metadata for a VOD session.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VodSession {
    /// Output only. The name of the VOD session, in the form of
    /// `projects/{project_number}/locations/{location}/vodSessions/{id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Metadata of what was stitched into the content.
    #[prost(message, optional, tag="2")]
    pub interstitials: ::core::option::Option<Interstitials>,
    /// Output only. The playback URI of the stitched content.
    #[prost(string, tag="4")]
    pub play_uri: ::prost::alloc::string::String,
    /// Required. URI of the media to stitch.
    #[prost(string, tag="5")]
    pub source_uri: ::prost::alloc::string::String,
    /// Required. Ad tag URI.
    #[prost(string, tag="6")]
    pub ad_tag_uri: ::prost::alloc::string::String,
    /// Key value pairs for ad tag macro replacement. If the
    /// specified ad tag URI has macros, this field provides the mapping
    /// to the value that will replace the macro in the ad tag URI.
    /// Macros are designated by square brackets.
    /// For example:
    ///
    ///   Ad tag URI: `"<https://doubleclick.google.com/ad/1?geo_id=\[geoId\]"`>
    ///
    ///   Ad tag macro map: `{"geoId": "123"}`
    ///
    ///   Fully qualified ad tag:
    ///   `"`<https://doubleclick.google.com/ad/1?geo_id=123"`>
    #[prost(map="string, string", tag="7")]
    pub ad_tag_macro_map: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Indicates whether client side ad tracking is enabled. If client
    /// side ad tracking is enabled, then the client player is expected
    /// to trigger playback and activity events itself.
    /// If this is set to false, server side ad tracking is enabled,
    /// causing the Video Stitcher service will trigger playback events
    /// on behalf of the client player.
    #[prost(bool, tag="8")]
    pub client_ad_tracking: bool,
    /// Additional options that affect the output of the manifest.
    #[prost(message, optional, tag="9")]
    pub manifest_options: ::core::option::Option<ManifestOptions>,
    /// Output only. The generated ID of the VodSession's source media.
    #[prost(string, tag="10")]
    pub asset_id: ::prost::alloc::string::String,
}
/// Describes what was stitched into a VOD session's manifest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Interstitials {
    /// List of ad breaks ordered by time.
    #[prost(message, repeated, tag="1")]
    pub ad_breaks: ::prost::alloc::vec::Vec<VodSessionAdBreak>,
    /// Information related to the content of the VOD session.
    #[prost(message, optional, tag="2")]
    pub session_content: ::core::option::Option<VodSessionContent>,
}
/// Metadata for an inserted ad in a VOD session.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VodSessionAd {
    /// Duration in seconds of the ad.
    #[prost(message, optional, tag="1")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// Metadata of companion ads associated with the ad.
    #[prost(message, optional, tag="2")]
    pub companion_ads: ::core::option::Option<CompanionAds>,
    /// The list of progress tracking events for the ad break. These can be of
    /// the following IAB types: `MUTE`, `UNMUTE`, `PAUSE`, `CLICK`,
    /// `CLICK_THROUGH`, `REWIND`, `RESUME`, `ERROR`, `FULLSCREEN`,
    /// `EXIT_FULLSCREEN`, `EXPAND`, `COLLAPSE`, `ACCEPT_INVITATION_LINEAR`,
    /// `CLOSE_LINEAR`, `SKIP`.
    #[prost(message, repeated, tag="3")]
    pub activity_events: ::prost::alloc::vec::Vec<Event>,
}
/// Metadata for the entire stitched content in a VOD session.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VodSessionContent {
    /// The total duration in seconds of the content including the ads stitched
    /// in.
    #[prost(message, optional, tag="1")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
/// Metadata for an inserted ad break.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VodSessionAdBreak {
    /// List of events that are expected to be triggered, ordered by time.
    #[prost(message, repeated, tag="1")]
    pub progress_events: ::prost::alloc::vec::Vec<ProgressEvent>,
    /// Ordered list of ads stitched into the ad break.
    #[prost(message, repeated, tag="2")]
    pub ads: ::prost::alloc::vec::Vec<VodSessionAd>,
    /// Ad break end time in seconds relative to the start of the VOD asset.
    #[prost(message, optional, tag="3")]
    pub end_time_offset: ::core::option::Option<::prost_types::Duration>,
    /// Ad break start time in seconds relative to the start of the VOD asset.
    #[prost(message, optional, tag="4")]
    pub start_time_offset: ::core::option::Option<::prost_types::Duration>,
}
/// Metadata for a live session.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveSession {
    /// Output only. The name of the live session, in the form of
    /// `projects/{project}/locations/{location}/liveSessions/{id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The URI to play the live session's ad-stitched stream.
    #[prost(string, tag="2")]
    pub play_uri: ::prost::alloc::string::String,
    /// The URI of the live session's source stream.
    #[prost(string, tag="3")]
    pub source_uri: ::prost::alloc::string::String,
    /// The default ad tag to use when no ad tag ids are specified in an ad break's
    /// SCTE-35 message.
    ///
    /// default_ad_tag_id is necessary when `adTagMap` has more than one key. Its
    /// value must be present in the `adTagMap`.
    #[prost(string, tag="4")]
    pub default_ad_tag_id: ::prost::alloc::string::String,
    /// Key value pairs for ad tags. Ads parsed from ad tags must be MP4 videos
    /// each with at least one audio track.
    #[prost(map="string, message", tag="5")]
    pub ad_tag_map: ::std::collections::HashMap<::prost::alloc::string::String, AdTag>,
    /// Key value pairs for ad tag macro replacement. If the
    /// specified ad tag URI has macros, this field provides the mapping
    /// to the value that will replace the macro in the ad tag URI.
    /// Macros are designated by square brackets.
    ///
    /// For example:
    ///
    ///   Ad tag URI: "<https://doubleclick.google.com/ad/1?geo_id=\[geoId\]">
    ///
    ///   Ad tag macros: `{"geoId": "123"}`
    ///
    ///   Fully qualified ad tag:
    ///   `"<https://doubleclick.google.com/ad/1?geo_id=123"`>
    #[prost(map="string, string", tag="6")]
    pub ad_tag_macros: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Whether client side ad tracking is enabled. If enabled, the client player
    /// is expected to trigger playback and activity events itself. Otherwise,
    /// server side ad tracking is enabled and the Video Stitcher API will trigger
    /// playback events on behalf of the client player.
    #[prost(bool, tag="7")]
    pub client_ad_tracking: bool,
    /// The default slate to use when no slates are specified in an ad break's
    /// SCTE-35 message. When specified, this value must match the ID for a slate
    /// that has already been created via the
    /// \[CreateSlate\](projects.locations.slates/create) method.
    #[prost(string, tag="8")]
    pub default_slate_id: ::prost::alloc::string::String,
    /// Defines the stitcher behavior in case an ad does not align exactly with
    /// the ad break boundaries. If not specified, the default is `COMPLETE_AD`.
    #[prost(enumeration="live_session::StitchingPolicy", tag="9")]
    pub stitching_policy: i32,
    /// Additional options that affect the output of the manifest.
    #[prost(message, optional, tag="10")]
    pub manifest_options: ::core::option::Option<ManifestOptions>,
    /// Output only. The generated ID of the LiveSession's source stream.
    #[prost(string, tag="11")]
    pub stream_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `LiveSession`.
pub mod live_session {
    /// Defines the stitcher behavior in case an ad does not align exactly with
    /// the ad break boundaries. If not specified, the default is COMPLETE_AD.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StitchingPolicy {
        /// Stitching policy is not specified.
        Unspecified = 0,
        /// Finishes stitching the current ad before returning to content.
        CompleteAd = 1,
        /// Cuts an ad short and returns to content in the middle of the ad.
        CutCurrent = 3,
    }
}
/// Metadata of an ad tag.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdTag {
    /// Ad tag URI template.
    #[prost(string, tag="1")]
    pub uri: ::prost::alloc::string::String,
}
/// Options for manifest generation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManifestOptions {
    /// If specified, the output manifest will only return renditions matching the
    /// specified filters.
    #[prost(message, repeated, tag="1")]
    pub include_renditions: ::prost::alloc::vec::Vec<RenditionFilter>,
    /// If specified, the output manifest will orders the video and muxed
    /// renditions by bitrate according to the ordering policy.
    #[prost(enumeration="manifest_options::OrderPolicy", tag="2")]
    pub bitrate_order: i32,
}
/// Nested message and enum types in `ManifestOptions`.
pub mod manifest_options {
    /// Defines the ordering policy during manifest generation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OrderPolicy {
        /// Ordering policy is not specified.
        Unspecified = 0,
        /// Order by ascending.
        Ascending = 1,
        /// Order by descending.
        Descending = 2,
    }
}
/// Filters for a video or muxed redition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenditionFilter {
    /// Bitrate in bits per second for the rendition. If set, only renditions with
    /// the exact bitrate will match.
    #[prost(int32, tag="1")]
    pub bitrate_bps: i32,
    /// Codecs for the rendition. If set, only renditions with the exact value
    /// will match.
    #[prost(string, tag="2")]
    pub codecs: ::prost::alloc::string::String,
}
/// Slate object
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Slate {
    /// Output only. The name of the slate, in the form of
    /// `projects/{project_number}/locations/{location}/slates/{id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The URI to fetch the source content for the slate. This URI must return an
    /// MP4 video with at least one audio track.
    #[prost(string, tag="2")]
    pub uri: ::prost::alloc::string::String,
}
/// Detailed information related to the interstitial of a VOD session.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VodStitchDetail {
    /// The name of the stitch detail in the specified VOD session, in the form of
    /// `projects/{project}/locations/{location}/vodSessions/{vod_session_id}/vodStitchDetails/{id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// A list of ad processing details for the fetched ad playlist.
    #[prost(message, repeated, tag="3")]
    pub ad_stitch_details: ::prost::alloc::vec::Vec<AdStitchDetail>,
}
/// Metadata for a stitched ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdStitchDetail {
    /// Required. The ad break ID of the processed ad.
    #[prost(string, tag="1")]
    pub ad_break_id: ::prost::alloc::string::String,
    /// Required. The ad ID of the processed ad.
    #[prost(string, tag="2")]
    pub ad_id: ::prost::alloc::string::String,
    /// Required. The time offset of the processed ad.
    #[prost(message, optional, tag="3")]
    pub ad_time_offset: ::core::option::Option<::prost_types::Duration>,
    /// Optional. Indicates the reason why the ad has been skipped.
    #[prost(string, tag="4")]
    pub skip_reason: ::prost::alloc::string::String,
    /// Optional. The metadata of the chosen media file for the ad.
    #[prost(map="string, message", tag="5")]
    pub media: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
}
/// Request message for VideoStitcherService.createCdnKey.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCdnKeyRequest {
    /// Required. The project in which the CDN key should be created, in the form of
    /// `projects/{project_number}/locations/{location}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The CDN key resource to create.
    #[prost(message, optional, tag="2")]
    pub cdn_key: ::core::option::Option<CdnKey>,
    /// Required. The ID to use for the CDN key, which will become the final component of
    /// the CDN key's resource name.
    ///
    /// This value should conform to RFC-1034, which restricts to
    /// lower-case letters, numbers, and hyphen, with the first character a
    /// letter, the last a letter or a number, and a 63 character maximum.
    #[prost(string, tag="3")]
    pub cdn_key_id: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.listCdnKeys.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCdnKeysRequest {
    /// Required. The project that contains the list of CDN keys, in the form of
    /// `projects/{project_number}/locations/{location}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for VideoStitcher.ListCdnKeys.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCdnKeysResponse {
    /// List of CDN keys.
    #[prost(message, repeated, tag="1")]
    pub cdn_keys: ::prost::alloc::vec::Vec<CdnKey>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for VideoStitcherService.getCdnKey.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCdnKeyRequest {
    /// Required. The name of the CDN key to be retrieved, in the form of
    /// `projects/{project}/locations/{location}/cdnKeys/{id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.deleteCdnKey.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCdnKeyRequest {
    /// Required. The name of the CDN key to be deleted, in the form of
    /// `projects/{project_number}/locations/{location}/cdnKeys/{id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.updateCdnKey.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCdnKeyRequest {
    /// Required. The CDN key resource which replaces the resource on the server.
    #[prost(message, optional, tag="1")]
    pub cdn_key: ::core::option::Option<CdnKey>,
    /// Required. The update mask applies to the resource.
    /// For the `FieldMask` definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for VideoStitcherService.createVodSession
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVodSessionRequest {
    /// Required. The project and location in which the VOD session should be created, in the
    /// form of `projects/{project_number}/locations/{location}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Parameters for creating a session.
    #[prost(message, optional, tag="2")]
    pub vod_session: ::core::option::Option<VodSession>,
}
/// Request message for VideoStitcherService.getVodSession
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVodSessionRequest {
    /// Required. The name of the VOD session to be retrieved, in the form of
    /// `projects/{project_number}/locations/{location}/vodSessions/{id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.listVodStitchDetails.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVodStitchDetailsRequest {
    /// Required. The VOD session where the stitch details belong to, in the form of
    /// `projects/{project}/locations/{location}/vodSessions/{id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for VideoStitcherService.listVodStitchDetails.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVodStitchDetailsResponse {
    /// A List of stitch Details.
    #[prost(message, repeated, tag="1")]
    pub vod_stitch_details: ::prost::alloc::vec::Vec<VodStitchDetail>,
    /// The pagination token.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.getVodStitchDetail.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVodStitchDetailRequest {
    /// Required. The name of the stitch detail in the specified VOD session, in the form of
    /// `projects/{project}/locations/{location}/vodSessions/{vod_session_id}/vodStitchDetails/{id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.listVodAdTagDetails.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVodAdTagDetailsRequest {
    /// Required. The VOD session which the ad tag details belong to, in the form of
    /// `projects/{project}/locations/{location}/vodSessions/{vod_session_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for VideoStitcherService.listVodAdTagDetails.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVodAdTagDetailsResponse {
    /// A List of ad tag details.
    #[prost(message, repeated, tag="1")]
    pub vod_ad_tag_details: ::prost::alloc::vec::Vec<VodAdTagDetail>,
    /// The pagination token.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.getVodAdTagDetail
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVodAdTagDetailRequest {
    /// Required. The name of the ad tag detail for the specified VOD session, in the form of
    /// `projects/{project}/locations/{location}/vodSessions/{vod_session_id}/vodAdTagDetails/{vod_ad_tag_detail}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.listLiveAdTagDetails.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLiveAdTagDetailsRequest {
    /// Required. The resource parent in the form of
    /// `projects/{project}/locations/{location}/liveSessions/{live_session}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The pagination token returned from a previous List request.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for VideoStitcherService.listLiveAdTagDetails.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLiveAdTagDetailsResponse {
    /// A list of live session ad tag details.
    #[prost(message, repeated, tag="1")]
    pub live_ad_tag_details: ::prost::alloc::vec::Vec<LiveAdTagDetail>,
    /// The pagination token.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.getLiveAdTagDetail
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLiveAdTagDetailRequest {
    /// Required. The resource name in the form of
    /// `projects/{project}/locations/{location}/liveSessions/{live_session}/liveAdTagDetails/{live_ad_tag_detail}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.createSlate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSlateRequest {
    /// Required. The project in which the slate should be created, in the form of
    /// `projects/{project_number}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The unique identifier for the slate.
    /// This value should conform to RFC-1034, which restricts to
    /// lower-case letters, numbers, and hyphen, with the first character a
    /// letter, the last a letter or a number, and a 63 character maximum.
    #[prost(string, tag="2")]
    pub slate_id: ::prost::alloc::string::String,
    /// Required. The slate to create.
    #[prost(message, optional, tag="3")]
    pub slate: ::core::option::Option<Slate>,
}
/// Request message for VideoStitcherService.getSlate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSlateRequest {
    /// Required. The name of the slate to be retrieved, of the slate, in the form of
    /// `projects/{project_number}/locations/{location}/slates/{id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.listSlates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSlatesRequest {
    /// Required. The project to list slates, in the form of `projects/{project_number}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for VideoStitcherService.listSlates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSlatesResponse {
    /// The list of slates
    #[prost(message, repeated, tag="1")]
    pub slates: ::prost::alloc::vec::Vec<Slate>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for VideoStitcherService.updateSlate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSlateRequest {
    /// Required. The resource with updated fields.
    #[prost(message, optional, tag="1")]
    pub slate: ::core::option::Option<Slate>,
    /// Required. The update mask which specifies fields which should be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for VideoStitcherService.deleteSlate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSlateRequest {
    /// Required. The name of the slate to be deleted, in the form of
    /// `projects/{project_number}/locations/{location}/slates/{id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.createLiveSession.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLiveSessionRequest {
    /// Required. The project and location in which the live session should be created,
    /// in the form of `projects/{project_number}/locations/{location}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Parameters for creating a live session.
    #[prost(message, optional, tag="2")]
    pub live_session: ::core::option::Option<LiveSession>,
}
/// Request message for VideoStitcherService.getSession.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLiveSessionRequest {
    /// Required. The name of the live session, in the form of
    /// `projects/{project_number}/locations/{location}/liveSessions/{id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod video_stitcher_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Video-On-Demand content stitching API allows you to insert ads
    /// into (VoD) video on demand files. You will be able to render custom
    /// scrubber bars with highlighted ads, enforce ad policies, allow
    /// seamless playback and tracking on native players and monetize
    /// content with any standard VMAP compliant ad server.
    #[derive(Debug, Clone)]
    pub struct VideoStitcherServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VideoStitcherServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> VideoStitcherServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> VideoStitcherServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            VideoStitcherServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Creates a new CDN key.
        pub async fn create_cdn_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCdnKeyRequest>,
        ) -> Result<tonic::Response<super::CdnKey>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/CreateCdnKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all CDN keys in the specified project and location.
        pub async fn list_cdn_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCdnKeysRequest>,
        ) -> Result<tonic::Response<super::ListCdnKeysResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/ListCdnKeys",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the specified CDN key.
        pub async fn get_cdn_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCdnKeyRequest>,
        ) -> Result<tonic::Response<super::CdnKey>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/GetCdnKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the specified CDN key.
        pub async fn delete_cdn_key(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCdnKeyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/DeleteCdnKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the specified CDN key. Only update fields specified
        /// in the call method body.
        pub async fn update_cdn_key(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCdnKeyRequest>,
        ) -> Result<tonic::Response<super::CdnKey>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/UpdateCdnKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a client side playback VOD session and returns the full
        /// tracking and playback metadata of the session.
        pub async fn create_vod_session(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVodSessionRequest>,
        ) -> Result<tonic::Response<super::VodSession>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/CreateVodSession",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the full tracking, playback metadata, and relevant ad-ops
        /// logs for the specified VOD session.
        pub async fn get_vod_session(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVodSessionRequest>,
        ) -> Result<tonic::Response<super::VodSession>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/GetVodSession",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a list of detailed stitching information of the specified VOD
        /// session.
        pub async fn list_vod_stitch_details(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVodStitchDetailsRequest>,
        ) -> Result<
            tonic::Response<super::ListVodStitchDetailsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/ListVodStitchDetails",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the specified stitching information for the specified VOD session.
        pub async fn get_vod_stitch_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVodStitchDetailRequest>,
        ) -> Result<tonic::Response<super::VodStitchDetail>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/GetVodStitchDetail",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Return the list of ad tag details for the specified VOD session.
        pub async fn list_vod_ad_tag_details(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVodAdTagDetailsRequest>,
        ) -> Result<tonic::Response<super::ListVodAdTagDetailsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/ListVodAdTagDetails",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the specified ad tag detail for the specified VOD session.
        pub async fn get_vod_ad_tag_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVodAdTagDetailRequest>,
        ) -> Result<tonic::Response<super::VodAdTagDetail>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/GetVodAdTagDetail",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Return the list of ad tag details for the specified live session.
        pub async fn list_live_ad_tag_details(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLiveAdTagDetailsRequest>,
        ) -> Result<
            tonic::Response<super::ListLiveAdTagDetailsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/ListLiveAdTagDetails",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the specified ad tag detail for the specified live session.
        pub async fn get_live_ad_tag_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLiveAdTagDetailRequest>,
        ) -> Result<tonic::Response<super::LiveAdTagDetail>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/GetLiveAdTagDetail",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a slate.
        pub async fn create_slate(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSlateRequest>,
        ) -> Result<tonic::Response<super::Slate>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/CreateSlate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all slates in the specified project and location.
        pub async fn list_slates(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSlatesRequest>,
        ) -> Result<tonic::Response<super::ListSlatesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/ListSlates",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the specified slate.
        pub async fn get_slate(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSlateRequest>,
        ) -> Result<tonic::Response<super::Slate>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/GetSlate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the specified slate.
        pub async fn update_slate(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSlateRequest>,
        ) -> Result<tonic::Response<super::Slate>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/UpdateSlate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the specified slate.
        pub async fn delete_slate(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSlateRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/DeleteSlate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new live session.
        pub async fn create_live_session(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateLiveSessionRequest>,
        ) -> Result<tonic::Response<super::LiveSession>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/CreateLiveSession",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the details for the specified live session.
        pub async fn get_live_session(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLiveSessionRequest>,
        ) -> Result<tonic::Response<super::LiveSession>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.stitcher.v1.VideoStitcherService/GetLiveSession",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
