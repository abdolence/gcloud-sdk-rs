/// A custom attribute that is not explicitly modeled in a resource, e.g.
/// \[UserEvent][google.cloud.discoveryengine.v1beta.UserEvent\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAttribute {
    /// The textual values of this custom attribute. For example, `["yellow",
    /// "green"]` when the key is "color".
    ///
    /// Empty string is not allowed. Otherwise, an INVALID_ARGUMENT error is
    /// returned.
    ///
    /// Exactly one of
    /// \[text][google.cloud.discoveryengine.v1beta.CustomAttribute.text\] or
    /// \[numbers][google.cloud.discoveryengine.v1beta.CustomAttribute.numbers\]
    /// should be set. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, repeated, tag="1")]
    pub text: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The numerical values of this custom attribute. For example, `[2.3, 15.4]`
    /// when the key is "lengths_cm".
    ///
    /// Exactly one of
    /// \[text][google.cloud.discoveryengine.v1beta.CustomAttribute.text\] or
    /// \[numbers][google.cloud.discoveryengine.v1beta.CustomAttribute.numbers\]
    /// should be set. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(double, repeated, tag="2")]
    pub numbers: ::prost::alloc::vec::Vec<f64>,
}
/// Information of an end user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    /// Highly recommended for logged-in users. Unique identifier for logged-in
    /// user, such as a user name. Don't set for anonymous users.
    ///
    /// Always use a hashed value for this ID.
    ///
    /// Don't set the field to the same fixed ID for different users. This mixes
    /// the event history of those users together, which results in degraded
    /// model quality.
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// User agent as included in the HTTP header. Required for getting
    /// \[SearchResponse.sponsored_results][\].
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 1,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// This should not be set when using the client side event reporting with
    /// GTM or JavaScript tag in
    /// \[UserEventService.CollectUserEvent][google.cloud.discoveryengine.v1beta.UserEventService.CollectUserEvent\]
    /// or if \[direct_user_request][\] is set.
    #[prost(string, tag="2")]
    pub user_agent: ::prost::alloc::string::String,
}
/// Document captures all raw metadata information of items to be recommended or
/// searched.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    /// Immutable. The full resource name of the document.
    /// Format:
    /// `projects/{project}/locations/{location}/dataStores/{data_store}/branches/{branch}/documents/{document_id}`.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 1024
    /// characters.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The identifier of the document.
    ///
    /// Id should conform to \[RFC-1034\](<https://tools.ietf.org/html/rfc1034>)
    /// standard with a length limit of 63 characters.
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    /// Required. The identifier of the schema located in the same data store.
    #[prost(string, tag="3")]
    pub schema_id: ::prost::alloc::string::String,
    /// The identifier of the parent document. Currently supports at most two level
    /// document hierarchy.
    ///
    /// Id should conform to \[RFC-1034\](<https://tools.ietf.org/html/rfc1034>)
    /// standard with a length limit of 63 characters.
    #[prost(string, tag="7")]
    pub parent_document_id: ::prost::alloc::string::String,
    /// Data representation. One of
    /// \[struct_data][google.cloud.discoveryengine.v1beta.Document.struct_data\] or
    /// \[json_data][google.cloud.discoveryengine.v1beta.Document.json_data\] should
    /// be provided otherwise an INVALID_ARGUMENT error is thrown.
    #[prost(oneof="document::Data", tags="4, 5")]
    pub data: ::core::option::Option<document::Data>,
}
/// Nested message and enum types in `Document`.
pub mod document {
    /// Data representation. One of
    /// \[struct_data][google.cloud.discoveryengine.v1beta.Document.struct_data\] or
    /// \[json_data][google.cloud.discoveryengine.v1beta.Document.json_data\] should
    /// be provided otherwise an INVALID_ARGUMENT error is thrown.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// The structured JSON data for the document. It should conform to the
        /// registered \[schema][\] or an INVALID_ARGUMENT error is thrown.
        #[prost(message, tag="4")]
        StructData(::prost_types::Struct),
        /// The JSON string representation of the document. It should conform to the
        /// registered \[schema][\] or an INVALID_ARGUMENT error is thrown.
        #[prost(string, tag="5")]
        JsonData(::prost::alloc::string::String),
    }
}
/// UserEvent captures all metadata information DiscoveryEngine API needs to know
/// about how end users interact with customers' website.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEvent {
    /// Required. User event type. Allowed values are:
    ///
    /// Generic values:
    /// * `search`: Search for Documents.
    /// * `view-item`: Detailed page view of a Document.
    /// * `view-item-list`: View of a panel or ordered list of Documents.
    /// * `view-home-page`: View of the home page.
    /// * `view-category-page`: View of a category page, e.g. Home > Men > Jeans
    ///
    /// Retail-related values:
    /// * `add-to-cart`: Add an item(s) to cart, e.g. in Retail online shopping
    /// * `purchase`: Purchase an item(s)
    ///
    /// Media-related values:
    /// * `media-play`: Start/resume watching a video, playing a song, etc.
    /// * `media-complete`: Finished or stopped midway through a video, song, etc.
    #[prost(string, tag="1")]
    pub event_type: ::prost::alloc::string::String,
    /// Required. A unique identifier for tracking visitors.
    ///
    /// For example, this could be implemented with an HTTP cookie, which should be
    /// able to uniquely identify a visitor on a single device. This unique
    /// identifier should not change if the visitor log in/out of the website.
    ///
    /// Do not set the field to the same fixed ID for different users. This mixes
    /// the event history of those users together, which results in degraded model
    /// quality.
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// The field should not contain PII or user-data. We recommend to use Google
    /// Analytics [Client
    /// ID](<https://developers.google.com/analytics/devguides/collection/analyticsjs/field-reference#clientId>)
    /// for this field.
    #[prost(string, tag="2")]
    pub user_pseudo_id: ::prost::alloc::string::String,
    /// Only required for
    /// \[UserEventService.ImportUserEvents][google.cloud.discoveryengine.v1beta.UserEventService.ImportUserEvents\]
    /// method. Timestamp of when the user event happened.
    #[prost(message, optional, tag="3")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Information about the end user.
    #[prost(message, optional, tag="4")]
    pub user_info: ::core::option::Option<UserInfo>,
    /// Should set to true if the request is made directly from the end user, in
    /// which case the
    /// \[UserEvent.user_info.user_agent][google.cloud.discoveryengine.v1beta.UserInfo.user_agent\]
    /// can be populated from the HTTP request.
    ///
    /// This flag should be set only if the API request is made directly from the
    /// end user such as a mobile app (and not if a gateway or a server is
    /// processing and pushing the user events).
    ///
    /// This should not be set when using the JavaScript tag in
    /// \[UserEventService.CollectUserEvent][google.cloud.discoveryengine.v1beta.UserEventService.CollectUserEvent\].
    #[prost(bool, tag="5")]
    pub direct_user_request: bool,
    /// A unique identifier for tracking a visitor session with a length limit of
    /// 128 bytes. A session is an aggregation of an end user behavior in a time
    /// span.
    ///
    /// A general guideline to populate the sesion_id:
    /// 1. If user has no activity for 30 min, a new session_id should be assigned.
    /// 2. The session_id should be unique across users, suggest use uuid or add
    /// \[UserEvent.user_pseudo_id][google.cloud.discoveryengine.v1beta.UserEvent.user_pseudo_id\]
    /// as prefix.
    #[prost(string, tag="6")]
    pub session_id: ::prost::alloc::string::String,
    /// Page metadata such as categories and other critical information for certain
    /// event types such as `view-category-page`.
    #[prost(message, optional, tag="7")]
    pub page_info: ::core::option::Option<PageInfo>,
    /// Token to attribute an API response to user action(s) to trigger the event.
    ///
    /// Highly recommended for user events that are the result of
    /// \[PredictionService.Predict][\]. This field enables accurate attribution of
    /// recommendation model performance.
    ///
    /// The value must be one of:
    ///
    /// * \[PredictResponse.attribution_token][\] for events that are the result of
    /// \[PredictionService.Predict][\].
    /// * \[SearchResponse.attribution_token][google.cloud.discoveryengine.v1beta.SearchResponse.attribution_token\] for events that are the result of
    /// \[SearchService.Search][\].
    /// * \[CompleteQueryResponse.attribution_token][\] for events that are the
    /// result of \[SearchService.CompleteQuery][\].
    ///
    /// This token enables us to accurately attribute page view or conversion
    /// completion back to the event and the particular predict response containing
    /// this clicked/purchased product. If user clicks on product K in the
    /// recommendation results, pass \[PredictResponse.attribution_token][\] as a URL
    /// parameter to product K's page. When recording events on product K's page,
    /// log the \[PredictResponse.attribution_token][\] to this field.
    #[prost(string, tag="8")]
    pub attribution_token: ::prost::alloc::string::String,
    /// The filter syntax consists of an expression language for constructing a
    /// predicate from one or more fields of the documents being filtered.
    ///
    /// One example is for `search` events, the associated
    /// \[SearchService.SearchRequest][\] may contain a filter expression in
    /// \[SearchService.SearchRequest.filter][\] conforming to
    /// <https://google.aip.dev/160#filtering.>
    ///
    /// Similarly, for `view-item-list` events that are generated from a
    /// \[PredictionService.PredictRequest][\], this field may be populated directly
    /// from \[PredictionService.PredictRequest.filter][\] conforming to
    /// <https://google.aip.dev/160#filtering.>
    ///
    /// The value must be a UTF-8 encoded string with a length limit of 1,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag="9")]
    pub filter: ::prost::alloc::string::String,
    /// List of Documents associated with this user event.
    ///
    /// This field is optional except for the following event types:
    ///
    /// * `view-item`
    /// * `add-to-cart`
    /// * `purchase`
    /// * `media-play`
    /// * `media-complete`
    ///
    /// In a `search` event, this field represents the documents returned to the
    /// end user on the current page (the end user may have not finished browsing
    /// the whole page yet). When a new page is returned to the end user, after
    /// pagination/filtering/ordering even for the same query, a new `search` event
    /// with different
    /// \[UserEvent.documents][google.cloud.discoveryengine.v1beta.UserEvent.documents\]
    /// is desired.
    #[prost(message, repeated, tag="10")]
    pub documents: ::prost::alloc::vec::Vec<DocumentInfo>,
    /// Panel metadata associated with this user event.
    #[prost(message, optional, tag="11")]
    pub panel: ::core::option::Option<PanelInfo>,
    /// Search API details related to the event.
    ///
    /// This field should be set for `search` event.
    #[prost(message, optional, tag="12")]
    pub search_info: ::core::option::Option<SearchInfo>,
    /// CompleteQuery API details related to the event.
    ///
    /// This field should be set for `search` event when autocomplete function is
    /// enabled and the user clicks a suggestion for search.
    #[prost(message, optional, tag="13")]
    pub completion_info: ::core::option::Option<CompletionInfo>,
    /// The transaction metadata (if any) associated with this user event.
    #[prost(message, optional, tag="14")]
    pub transaction_info: ::core::option::Option<TransactionInfo>,
    /// A list of identifiers for the independent experiment groups this user event
    /// belongs to. This is used to distinguish between user events associated with
    /// different experiment setups on the customer end.
    #[prost(string, repeated, tag="15")]
    pub tag_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The promotion IDs if this is an event associated with promotions.
    /// Currently, this field is restricted to at most one ID.
    #[prost(string, repeated, tag="16")]
    pub promotion_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Extra user event features to include in the recommendation model.
    /// These attributes must NOT contain data that needs to be parsed or processed
    /// further, e.g. JSON or other encodings.
    ///
    /// If you provide custom attributes for ingested user events, also include
    /// them in the user events that you associate with prediction requests. Custom
    /// attribute formatting must be consistent between imported events and events
    /// provided with prediction requests. This lets the DiscoveryEngine API use
    /// those custom attributes when training models and serving predictions, which
    /// helps improve recommendation quality.
    ///
    /// This field needs to pass all below criteria, otherwise an INVALID_ARGUMENT
    /// error is returned:
    ///
    /// * The key must be a UTF-8 encoded string with a length limit of 5,000
    ///    characters.
    /// * For text attributes, at most 400 values are allowed. Empty values are not
    ///    allowed. Each value must be a UTF-8 encoded string with a length limit of
    ///    256 characters.
    /// * For number attributes, at most 400 values are allowed.
    ///
    /// For product recommendations, an example of extra user information is
    /// traffic_channel, which is how a user arrives at the site. Users can arrive
    /// at the site by coming to the site directly, coming through Google
    /// search, or in other ways.
    #[prost(map="string, message", tag="17")]
    pub attributes: ::std::collections::HashMap<::prost::alloc::string::String, CustomAttribute>,
    /// Media-specific info.
    #[prost(message, optional, tag="18")]
    pub media_info: ::core::option::Option<MediaInfo>,
}
/// Detailed page information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageInfo {
    /// A unique ID of a web page view.
    ///
    /// This should be kept the same for all user events triggered from the same
    /// pageview. For example, an item detail page view could trigger multiple
    /// events as the user is browsing the page. The `pageViewId` property should
    /// be kept the same for all these events so that they can be grouped together
    /// properly.
    ///
    /// When using the client side event reporting with JavaScript pixel and Google
    /// Tag Manager, this value is filled in automatically.
    #[prost(string, tag="1")]
    pub pageview_id: ::prost::alloc::string::String,
    /// The most specific category associated with a category page.
    ///
    /// To represent full path of category, use '>' sign to separate different
    /// hierarchies. If '>' is part of the category name, please replace it with
    /// other character(s).
    ///
    /// Category pages include special pages such as sales or promotions. For
    /// instance, a special sale page may have the category hierarchy:
    /// "pageCategory" : "Sales > 2017 Black Friday Deals".
    ///
    /// Required for `view-category-page` events. Other event types should not set
    /// this field. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag="2")]
    pub page_category: ::prost::alloc::string::String,
    /// Complete URL (window.location.href) of the user's current page.
    ///
    /// When using the client side event reporting with JavaScript pixel and Google
    /// Tag Manager, this value is filled in automatically. Maximum length 5,000
    /// characters.
    #[prost(string, tag="3")]
    pub uri: ::prost::alloc::string::String,
    /// The referrer URL of the current page.
    ///
    /// When using the client side event reporting with JavaScript pixel and Google
    /// Tag Manager, this value is filled in automatically. However, some browser
    /// privacy restrictions may cause this field to be empty.
    #[prost(string, tag="4")]
    pub referrer_uri: ::prost::alloc::string::String,
}
/// Detailed search information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchInfo {
    /// The user's search query.
    ///
    /// See
    /// \[SearchRequest.query][google.cloud.discoveryengine.v1beta.SearchRequest.query\]
    /// for definition.
    ///
    /// The value must be a UTF-8 encoded string with a length limit of 5,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// At least one of
    /// \[search_query][google.cloud.discoveryengine.v1beta.SearchInfo.search_query\]
    /// or \[page_categories][\] is required for `search` events. Other event types
    /// should not set this field. Otherwise, an INVALID_ARGUMENT error is
    /// returned.
    #[prost(string, tag="1")]
    pub search_query: ::prost::alloc::string::String,
    /// The order in which products are returned, if applicable.
    ///
    /// See
    /// \[SearchRequest.order_by][google.cloud.discoveryengine.v1beta.SearchRequest.order_by\]
    /// for definition and syntax.
    ///
    /// The value must be a UTF-8 encoded string with a length limit of 1,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// This can only be set for `search` events. Other event types should not set
    /// this field. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag="2")]
    pub order_by: ::prost::alloc::string::String,
    /// An integer that specifies the current offset for pagination (the 0-indexed
    /// starting location, amongst the products deemed by the API as relevant).
    ///
    /// See
    /// \[SearchRequest.offset][google.cloud.discoveryengine.v1beta.SearchRequest.offset\]
    /// for definition.
    ///
    /// If this field is negative, an INVALID_ARGUMENT is returned.
    ///
    /// This can only be set for `search` events. Other event types should not set
    /// this field. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(int32, optional, tag="3")]
    pub offset: ::core::option::Option<i32>,
}
/// Detailed completion information including completion attribution token and
/// clicked completion info.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletionInfo {
    /// End user selected \[CompleteQueryResponse.CompletionResult.suggestion][\].
    #[prost(string, tag="1")]
    pub selected_suggestion: ::prost::alloc::string::String,
    /// End user selected \[CompleteQueryResponse.CompletionResult.suggestion][\]
    /// position, starting from 0.
    #[prost(int32, tag="2")]
    pub selected_position: i32,
}
/// A transaction represents the entire purchase transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfo {
    /// Required. Total non-zero value associated with the transaction. This value
    /// may include shipping, tax, or other adjustments to the total value that you
    /// want to include.
    #[prost(float, optional, tag="1")]
    pub value: ::core::option::Option<f32>,
    /// Required. Currency code. Use three-character ISO-4217 code.
    #[prost(string, tag="2")]
    pub currency: ::prost::alloc::string::String,
    /// The transaction ID with a length limit of 128 characters.
    #[prost(string, tag="3")]
    pub transaction_id: ::prost::alloc::string::String,
    /// All the taxes associated with the transaction.
    #[prost(float, optional, tag="4")]
    pub tax: ::core::option::Option<f32>,
    /// All the costs associated with the products. These can be manufacturing
    /// costs, shipping expenses not borne by the end user, or any other costs,
    /// such that:
    ///
    /// * Profit =
    /// \[value][google.cloud.discoveryengine.v1beta.TransactionInfo.value\] -
    /// \[tax][google.cloud.discoveryengine.v1beta.TransactionInfo.tax\] -
    /// \[cost][google.cloud.discoveryengine.v1beta.TransactionInfo.cost\]
    #[prost(float, optional, tag="5")]
    pub cost: ::core::option::Option<f32>,
    /// The total discount(s) value applied to this transaction.
    /// This figure should be excluded from
    /// \[TransactionInfo.value][google.cloud.discoveryengine.v1beta.TransactionInfo.value\]
    ///
    /// For example, if a user paid
    /// \[TransactionInfo.value][google.cloud.discoveryengine.v1beta.TransactionInfo.value\]
    /// amount, then nominal (pre-discount) value of the transaction is the sum of
    /// \[TransactionInfo.value][google.cloud.discoveryengine.v1beta.TransactionInfo.value\]
    /// and
    /// \[TransactionInfo.discount_value][google.cloud.discoveryengine.v1beta.TransactionInfo.discount_value\]
    ///
    /// This means that profit is calculated the same way, regardless of the
    /// discount value, and that
    /// \[TransactionInfo.discount_value][google.cloud.discoveryengine.v1beta.TransactionInfo.discount_value\]
    /// can be larger than
    /// \[TransactionInfo.value][google.cloud.discoveryengine.v1beta.TransactionInfo.value\]:
    ///
    /// * Profit =
    /// \[value][google.cloud.discoveryengine.v1beta.TransactionInfo.value\] -
    /// \[tax][google.cloud.discoveryengine.v1beta.TransactionInfo.tax\] -
    /// \[cost][google.cloud.discoveryengine.v1beta.TransactionInfo.cost\]
    #[prost(float, optional, tag="6")]
    pub discount_value: ::core::option::Option<f32>,
}
/// Detailed document information associated with a user event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentInfo {
    /// Quantity of the Document associated with the user event. Defaults to 1.
    ///
    /// For example, this field will be 2 if two quantities of the same Document
    /// are involved in a `add-to-cart` event.
    ///
    /// Required for events of the following event types:
    /// * `add-to-cart`
    /// * `purchase`
    #[prost(int32, optional, tag="3")]
    pub quantity: ::core::option::Option<i32>,
    /// The promotion IDs associated with this Document.
    /// Currently, this field is restricted to at most one ID.
    #[prost(string, repeated, tag="4")]
    pub promotion_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A required descriptor of the associated Document.
    ///
    /// * If \[id][google.cloud.discoveryengine.v1beta.DocumentInfo.id\] is
    /// specified, then the default values for <location>, <data_store_id>, and
    /// <branch_id> are used when annotating with the stored Document.
    ///
    /// * If \[name][google.cloud.discoveryengine.v1beta.DocumentInfo.name\] is
    /// specified, then the provided values (default values allowed) for
    /// <location>, <data_store_id>, and <branch_id> are used when annotating with
    /// the stored Document.
    #[prost(oneof="document_info::DocumentDescriptor", tags="1, 2")]
    pub document_descriptor: ::core::option::Option<document_info::DocumentDescriptor>,
}
/// Nested message and enum types in `DocumentInfo`.
pub mod document_info {
    /// A required descriptor of the associated Document.
    ///
    /// * If \[id][google.cloud.discoveryengine.v1beta.DocumentInfo.id\] is
    /// specified, then the default values for <location>, <data_store_id>, and
    /// <branch_id> are used when annotating with the stored Document.
    ///
    /// * If \[name][google.cloud.discoveryengine.v1beta.DocumentInfo.name\] is
    /// specified, then the provided values (default values allowed) for
    /// <location>, <data_store_id>, and <branch_id> are used when annotating with
    /// the stored Document.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DocumentDescriptor {
        /// Required. The Document resource ID.
        #[prost(string, tag="1")]
        Id(::prost::alloc::string::String),
        /// Required. The Document resource full name, of the form:
        /// projects/<project_id>/locations/<location>/dataStores/<data_store_id>/branches/<branch_id>/documents/<document_id>
        #[prost(string, tag="2")]
        Name(::prost::alloc::string::String),
    }
}
/// Detailed panel information associated with a user event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PanelInfo {
    /// The panel ID.
    #[prost(string, tag="2")]
    pub panel_id: ::prost::alloc::string::String,
    /// The display name of the panel.
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
    /// The ordered position of the panel, if shown to the user with other panels.
    /// If set, then
    /// \[total_panels][google.cloud.discoveryengine.v1beta.PanelInfo.total_panels\]
    /// must also be set.
    #[prost(int32, optional, tag="4")]
    pub panel_position: ::core::option::Option<i32>,
    /// The total number of panels, including this one, shown to the user.
    /// Must be set if
    /// \[panel_position][google.cloud.discoveryengine.v1beta.PanelInfo.panel_position\]
    /// is set.
    #[prost(int32, optional, tag="5")]
    pub total_panels: ::core::option::Option<i32>,
}
/// Media-specific user event information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaInfo {
    /// The media progress time in seconds, if applicable.
    /// For example, if the end user has finished 90 seconds of a playback video,
    /// then \[MediaInfo.media_progress_duration.seconds][Duration.seconds\] should
    /// be set to 90.
    #[prost(message, optional, tag="1")]
    pub media_progress_duration: ::core::option::Option<::prost_types::Duration>,
    /// Media progress should be computed using only the media_progress_duration
    /// relative to the media total length.
    ///
    /// This value must be between [0, 1.0] inclusive.
    ///
    /// If this is not a playback or the progress cannot be computed (e.g. ongoing
    /// livestream), this field should be unset.
    #[prost(float, optional, tag="2")]
    pub media_progress_percentage: ::core::option::Option<f32>,
}
/// Google Cloud Storage location for input content.
/// format.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    /// Required. Google Cloud Storage URIs to input files. URI can be up to
    /// 2000 characters long. URIs can match the full object path (for example,
    /// `gs://bucket/directory/object.json`) or a pattern matching one or more
    /// files, such as `gs://bucket/directory/*.json`. A request can
    /// contain at most 100 files, and each file can be up to 2 GB.
    #[prost(string, repeated, tag="1")]
    pub input_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The schema to use when parsing the data from the source.
    ///
    /// Supported values for imports:
    ///
    /// * `user_event` (default): One JSON
    /// \[UserEvent][google.cloud.discoveryengine.v1beta.UserEvent\] per line.
    ///
    /// * `document` (default): One JSON
    /// \[Document][google.cloud.discoveryengine.v1beta.Document\] per line. Each
    /// document must
    ///    have a valid
    ///    \[Document.id][google.cloud.discoveryengine.v1beta.Document.id\].
    #[prost(string, tag="2")]
    pub data_schema: ::prost::alloc::string::String,
}
/// BigQuery source import data from.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQuerySource {
    /// The project ID (can be project # or ID) that the BigQuery source is in with
    /// a length limit of 128 characters. If not specified, inherits the project
    /// ID from the parent request.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The BigQuery data set to copy the data from with a length limit
    /// of 1,024 characters.
    #[prost(string, tag="2")]
    pub dataset_id: ::prost::alloc::string::String,
    /// Required. The BigQuery table to copy the data from with a length limit of
    /// 1,024 characters.
    #[prost(string, tag="3")]
    pub table_id: ::prost::alloc::string::String,
    /// Intermediate Cloud Storage directory used for the import with a length
    /// limit of 2,000 characters. Can be specified if one wants to have the
    /// BigQuery export to a specific Cloud Storage directory.
    #[prost(string, tag="4")]
    pub gcs_staging_dir: ::prost::alloc::string::String,
    /// The schema to use when parsing the data from the source.
    ///
    /// Supported values for imports:
    ///
    /// * `user_event` (default): One JSON
    /// \[UserEvent][google.cloud.discoveryengine.v1beta.UserEvent\] per line.
    ///
    /// * `document` (default): One JSON
    /// \[Document][google.cloud.discoveryengine.v1beta.Document\] per line. Each
    /// document must have a valid \[document.id][\].
    #[prost(string, tag="6")]
    pub data_schema: ::prost::alloc::string::String,
    /// BigQuery table partition info. Leave this empty if the BigQuery table
    /// is not partitioned.
    #[prost(oneof="big_query_source::Partition", tags="5")]
    pub partition: ::core::option::Option<big_query_source::Partition>,
}
/// Nested message and enum types in `BigQuerySource`.
pub mod big_query_source {
    /// BigQuery table partition info. Leave this empty if the BigQuery table
    /// is not partitioned.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Partition {
        /// BigQuery time partitioned table's _PARTITIONDATE in YYYY-MM-DD format.
        #[prost(message, tag="5")]
        PartitionDate(super::super::super::super::r#type::Date),
    }
}
/// Configuration of destination for Import related errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportErrorConfig {
    /// Required. Errors destination.
    #[prost(oneof="import_error_config::Destination", tags="1")]
    pub destination: ::core::option::Option<import_error_config::Destination>,
}
/// Nested message and enum types in `ImportErrorConfig`.
pub mod import_error_config {
    /// Required. Errors destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Google Cloud Storage prefix for import errors. This must be an empty,
        /// existing Cloud Storage directory. Import errors will be written to
        /// sharded files in this directory, one per line, as a JSON-encoded
        /// `google.rpc.Status` message.
        #[prost(string, tag="1")]
        GcsPrefix(::prost::alloc::string::String),
    }
}
/// Request message for the ImportUserEvents request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportUserEventsRequest {
    /// Required. Parent DataStore resource name, of the form
    /// `projects/{project}/locations/{location}/dataStores/{data_store}`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The desired location of errors incurred during the Import. Cannot be set
    /// for inline user event imports.
    #[prost(message, optional, tag="5")]
    pub errors_config: ::core::option::Option<ImportErrorConfig>,
    /// The desired input source of the user event data.
    #[prost(oneof="import_user_events_request::Source", tags="2, 3, 4")]
    pub source: ::core::option::Option<import_user_events_request::Source>,
}
/// Nested message and enum types in `ImportUserEventsRequest`.
pub mod import_user_events_request {
    /// The inline source for the input config for ImportUserEvents method.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InlineSource {
        /// Required. A list of user events to import. Recommended max of 10k items.
        #[prost(message, repeated, tag="1")]
        pub user_events: ::prost::alloc::vec::Vec<super::UserEvent>,
    }
    /// The desired input source of the user event data.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Required. The Inline source for the input content for UserEvents.
        #[prost(message, tag="2")]
        InlineSource(InlineSource),
        /// Required. Google Cloud Storage location for the input content.
        #[prost(message, tag="3")]
        GcsSource(super::GcsSource),
        /// Required. BigQuery input source.
        #[prost(message, tag="4")]
        BigquerySource(super::BigQuerySource),
    }
}
/// Response of the ImportUserEventsRequest. If the long running
/// operation was successful, then this message is returned by the
/// google.longrunning.Operations.response field if the operation was successful.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportUserEventsResponse {
    /// A sample of errors encountered while processing the request.
    #[prost(message, repeated, tag="1")]
    pub error_samples: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Echoes the destination for the complete errors if this field was set in
    /// the request.
    #[prost(message, optional, tag="2")]
    pub errors_config: ::core::option::Option<ImportErrorConfig>,
    /// Count of user events imported with complete existing Documents.
    #[prost(int64, tag="3")]
    pub joined_events_count: i64,
    /// Count of user events imported, but with Document information not found
    /// in the existing Branch.
    #[prost(int64, tag="4")]
    pub unjoined_events_count: i64,
}
/// Metadata related to the progress of the Import operation. This will be
/// returned by the google.longrunning.Operation.metadata field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportUserEventsMetadata {
    /// Operation create time.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag="2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Count of entries that were processed successfully.
    #[prost(int64, tag="3")]
    pub success_count: i64,
    /// Count of entries that encountered errors while processing.
    #[prost(int64, tag="4")]
    pub failure_count: i64,
}
/// Metadata related to the progress of the ImportDocuments operation. This will
/// be returned by the google.longrunning.Operation.metadata field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDocumentsMetadata {
    /// Operation create time.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag="2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Count of entries that were processed successfully.
    #[prost(int64, tag="3")]
    pub success_count: i64,
    /// Count of entries that encountered errors while processing.
    #[prost(int64, tag="4")]
    pub failure_count: i64,
}
/// Request message for Import methods.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDocumentsRequest {
    /// Required. The parent branch resource name, such as
    /// `projects/{project}/locations/{location}/dataStores/{data_store}/branches/{branch}`.
    /// Requires create/update permission.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The desired location of errors incurred during the Import.
    #[prost(message, optional, tag="5")]
    pub errors_config: ::core::option::Option<ImportErrorConfig>,
    /// The mode of reconciliation between existing documents and the documents to
    /// be imported. Defaults to
    /// \[ReconciliationMode.INCREMENTAL][google.cloud.discoveryengine.v1beta.ImportDocumentsRequest.ReconciliationMode.INCREMENTAL\].
    #[prost(enumeration="import_documents_request::ReconciliationMode", tag="6")]
    pub reconciliation_mode: i32,
    /// Required. The source of the input.
    #[prost(oneof="import_documents_request::Source", tags="2, 3, 4")]
    pub source: ::core::option::Option<import_documents_request::Source>,
}
/// Nested message and enum types in `ImportDocumentsRequest`.
pub mod import_documents_request {
    /// The inline source for the input config for ImportDocuments method.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InlineSource {
        /// Required. A list of documents to update/create. Each document must have a
        /// valid \[Document.id][google.cloud.discoveryengine.v1beta.Document.id\].
        /// Recommended max of 100 items.
        #[prost(message, repeated, tag="1")]
        pub documents: ::prost::alloc::vec::Vec<super::Document>,
    }
    /// Indicates how imported documents are reconciled with the existing documents
    /// created or imported before.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReconciliationMode {
        /// Defaults to INCREMENTAL.
        Unspecified = 0,
        /// Inserts new documents or updates existing documents.
        Incremental = 1,
        /// Calculates diff and replaces the entire document dataset. Existing
        /// documents may be deleted if they are not present in the source location.
        Full = 2,
    }
    impl ReconciliationMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReconciliationMode::Unspecified => "RECONCILIATION_MODE_UNSPECIFIED",
                ReconciliationMode::Incremental => "INCREMENTAL",
                ReconciliationMode::Full => "FULL",
            }
        }
    }
    /// Required. The source of the input.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The Inline source for the input content for documents.
        #[prost(message, tag="2")]
        InlineSource(InlineSource),
        /// Google Cloud Storage location for the input content.
        #[prost(message, tag="3")]
        GcsSource(super::GcsSource),
        /// BigQuery input source.
        #[prost(message, tag="4")]
        BigquerySource(super::BigQuerySource),
    }
}
/// Response of the
/// \[ImportDocumentsRequest][google.cloud.discoveryengine.v1beta.ImportDocumentsRequest\].
/// If the long running operation is done, then this message is returned by the
/// google.longrunning.Operations.response field if the operation was successful.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDocumentsResponse {
    /// A sample of errors encountered while processing the request.
    #[prost(message, repeated, tag="1")]
    pub error_samples: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Echoes the destination for the complete errors in the request if set.
    #[prost(message, optional, tag="2")]
    pub errors_config: ::core::option::Option<ImportErrorConfig>,
}
/// Request message for
/// \[DocumentService.GetDocument][google.cloud.discoveryengine.v1beta.DocumentService.GetDocument\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentRequest {
    /// Required. Full resource name of
    /// \[Document][google.cloud.discoveryengine.v1beta.Document\], such as
    /// `projects/{project}/locations/{location}/dataStores/{data_store}/branches/{branch}/documents/{document}`.
    ///
    /// If the caller does not have permission to access the
    /// \[Document][google.cloud.discoveryengine.v1beta.Document\], regardless of
    /// whether or not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the requested \[Document][google.cloud.discoveryengine.v1beta.Document\]
    /// does not exist, a NOT_FOUND error is returned.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[DocumentService.ListDocuments][google.cloud.discoveryengine.v1beta.DocumentService.ListDocuments\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDocumentsRequest {
    /// Required. The parent branch resource name, such as
    /// `projects/{project}/locations/{location}/dataStores/{data_store}/branches/{branch}`.
    /// Use `default_branch` as the branch ID, to list documents under the default
    /// branch.
    ///
    /// If the caller does not have permission to list \[Documents][\]s under this
    /// branch, regardless of whether or not this branch exists, a
    /// PERMISSION_DENIED error is returned.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of \[Document][google.cloud.discoveryengine.v1beta.Document\]s
    /// to return. If unspecified, defaults to 100. The maximum allowed value is
    /// 1000. Values above 1000 will be coerced to 1000.
    ///
    /// If this field is negative, an INVALID_ARGUMENT error is returned.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token
    /// \[ListDocumentsResponse.next_page_token][google.cloud.discoveryengine.v1beta.ListDocumentsResponse.next_page_token\],
    /// received from a previous
    /// \[DocumentService.ListDocuments][google.cloud.discoveryengine.v1beta.DocumentService.ListDocuments\]
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// \[DocumentService.ListDocuments][google.cloud.discoveryengine.v1beta.DocumentService.ListDocuments\]
    /// must match the call that provided the page token. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// \[DocumentService.ListDocuments][google.cloud.discoveryengine.v1beta.DocumentService.ListDocuments\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDocumentsResponse {
    /// The \[Document][google.cloud.discoveryengine.v1beta.Document\]s.
    #[prost(message, repeated, tag="1")]
    pub documents: ::prost::alloc::vec::Vec<Document>,
    /// A token that can be sent as
    /// \[ListDocumentsRequest.page_token][google.cloud.discoveryengine.v1beta.ListDocumentsRequest.page_token\]
    /// to retrieve the next page. If this field is omitted, there are no
    /// subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// \[DocumentService.CreateDocument][google.cloud.discoveryengine.v1beta.DocumentService.CreateDocument\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDocumentRequest {
    /// Required. The parent resource name, such as
    /// `projects/{project}/locations/{location}/dataStores/{data_store}/branches/{branch}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The \[Document][google.cloud.discoveryengine.v1beta.Document\] to
    /// create.
    #[prost(message, optional, tag="2")]
    pub document: ::core::option::Option<Document>,
    /// Required. The ID to use for the
    /// \[Document][google.cloud.discoveryengine.v1beta.Document\], which will become
    /// the final component of the
    /// \[Document.name][google.cloud.discoveryengine.v1beta.Document.name\].
    ///
    /// If the caller does not have permission to create the
    /// \[Document][google.cloud.discoveryengine.v1beta.Document\], regardless of
    /// whether or not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// This field must be unique among all
    /// \[Document][google.cloud.discoveryengine.v1beta.Document\]s with the same
    /// \[parent][google.cloud.discoveryengine.v1beta.CreateDocumentRequest.parent\].
    /// Otherwise, an ALREADY_EXISTS error is returned.
    ///
    /// This field must conform to \[RFC-1034\](<https://tools.ietf.org/html/rfc1034>)
    /// standard with a length limit of 63 characters. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    #[prost(string, tag="3")]
    pub document_id: ::prost::alloc::string::String,
}
/// Request message for
/// \[DocumentService.UpdateDocument][google.cloud.discoveryengine.v1beta.DocumentService.UpdateDocument\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDocumentRequest {
    /// Required. The document to update/create.
    ///
    /// If the caller does not have permission to update the
    /// \[Document][google.cloud.discoveryengine.v1beta.Document\], regardless of
    /// whether or not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the \[Document][google.cloud.discoveryengine.v1beta.Document\] to update
    /// does not exist and
    /// \[allow_missing][google.cloud.discoveryengine.v1beta.UpdateDocumentRequest.allow_missing\]
    /// is not set, a NOT_FOUND error is returned.
    #[prost(message, optional, tag="1")]
    pub document: ::core::option::Option<Document>,
    /// If set to true, and the
    /// \[Document][google.cloud.discoveryengine.v1beta.Document\] is not found, a
    /// new \[Document][google.cloud.discoveryengine.v1beta.Document\] will be
    /// created.
    #[prost(bool, tag="2")]
    pub allow_missing: bool,
}
/// Request message for
/// \[DocumentService.DeleteDocument][google.cloud.discoveryengine.v1beta.DocumentService.DeleteDocument\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDocumentRequest {
    /// Required. Full resource name of
    /// \[Document][google.cloud.discoveryengine.v1beta.Document\], such as
    /// `projects/{project}/locations/{location}/dataStores/{data_store}/branches/{branch}/documents/{document}`.
    ///
    /// If the caller does not have permission to delete the
    /// \[Document][google.cloud.discoveryengine.v1beta.Document\], regardless of
    /// whether or not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the \[Document][google.cloud.discoveryengine.v1beta.Document\] to delete
    /// does not exist, a NOT_FOUND error is returned.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod document_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for ingesting
    /// [Document][google.cloud.discoveryengine.v1beta.Document] information of the
    /// customer's website.
    #[derive(Debug, Clone)]
    pub struct DocumentServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DocumentServiceClient<tonic::transport::Channel> {
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
    impl<T> DocumentServiceClient<T>
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
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DocumentServiceClient<InterceptedService<T, F>>
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
            DocumentServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Gets a [Document][google.cloud.discoveryengine.v1beta.Document].
        pub async fn get_document(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDocumentRequest>,
        ) -> Result<tonic::Response<super::Document>, tonic::Status> {
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
                "/google.cloud.discoveryengine.v1beta.DocumentService/GetDocument",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a list of [Document][google.cloud.discoveryengine.v1beta.Document]s.
        pub async fn list_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDocumentsRequest>,
        ) -> Result<tonic::Response<super::ListDocumentsResponse>, tonic::Status> {
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
                "/google.cloud.discoveryengine.v1beta.DocumentService/ListDocuments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a [Document][google.cloud.discoveryengine.v1beta.Document].
        pub async fn create_document(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDocumentRequest>,
        ) -> Result<tonic::Response<super::Document>, tonic::Status> {
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
                "/google.cloud.discoveryengine.v1beta.DocumentService/CreateDocument",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a [Document][google.cloud.discoveryengine.v1beta.Document].
        pub async fn update_document(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDocumentRequest>,
        ) -> Result<tonic::Response<super::Document>, tonic::Status> {
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
                "/google.cloud.discoveryengine.v1beta.DocumentService/UpdateDocument",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a [Document][google.cloud.discoveryengine.v1beta.Document].
        pub async fn delete_document(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDocumentRequest>,
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
                "/google.cloud.discoveryengine.v1beta.DocumentService/DeleteDocument",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Bulk import of multiple
        /// [Document][google.cloud.discoveryengine.v1beta.Document]s. Request
        /// processing may be synchronous. Non-existing items will be created.
        ///
        /// Note: It is possible for a subset of the
        /// [Document][google.cloud.discoveryengine.v1beta.Document]s to be
        /// successfully updated.
        pub async fn import_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportDocumentsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.discoveryengine.v1beta.DocumentService/ImportDocuments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for Recommend method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendRequest {
    /// Required. Full resource name of the format:
    /// projects/*/locations/global/dataStores/*/servingConfigs/*
    ///
    /// Before you can request recommendations from your model, you must create at
    /// least one serving config  for it.
    #[prost(string, tag="1")]
    pub serving_config: ::prost::alloc::string::String,
    /// Required. Context about the user, what they are looking at and what action
    /// they took to trigger the Recommend request. Note that this user event
    /// detail won't be ingested to userEvent logs. Thus, a separate userEvent
    /// write request is required for event logging.
    ///
    /// Don't set
    /// \[UserEvent.user_pseudo_id][google.cloud.discoveryengine.v1beta.UserEvent.user_pseudo_id\]
    /// or
    /// \[UserEvent.user_info.user_id][google.cloud.discoveryengine.v1beta.UserInfo.user_id\]
    /// to the same fixed ID for different users. If you are trying to receive
    /// non-personalized recommendations (not recommended; this can negatively
    /// impact model performance), instead set
    /// \[UserEvent.user_pseudo_id][google.cloud.discoveryengine.v1beta.UserEvent.user_pseudo_id\]
    /// to a random unique ID and leave
    /// \[UserEvent.user_info.user_id][google.cloud.discoveryengine.v1beta.UserInfo.user_id\]
    /// unset.
    #[prost(message, optional, tag="2")]
    pub user_event: ::core::option::Option<UserEvent>,
    /// Maximum number of results to return. Set this property
    /// to the number of recommendation results needed. If zero, the service will
    /// choose a reasonable default. The maximum allowed value is 100. Values
    /// above 100 will be coerced to 100.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Filter for restricting recommendation results with a length limit of 5,000
    /// characters. Currently, only filter expressions on the `filter_tags`
    /// attribute is supported.
    ///
    ///
    /// Examples:
    ///
    ///   * (filter_tags: ANY("Red", "Blue") OR filter_tags: ANY("Hot", "Cold"))
    ///   * (filter_tags: ANY("Red", "Blue")) AND NOT (filter_tags: ANY("Green"))
    ///
    /// If your filter blocks all results, the API will return generic
    /// (unfiltered) popular Documents. If you only want results strictly matching
    /// the filters, set `strictFiltering` to True in
    /// \[RecommendRequest.params][google.cloud.discoveryengine.v1beta.RecommendRequest.params\]
    /// to receive empty results instead.
    ///
    /// Note that the API will never return Documents with storageStatus of
    /// "EXPIRED" or "DELETED" regardless of filter choices.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Use validate only mode for this recommendation query. If set to true, a
    /// fake model will be used that returns arbitrary Document IDs.
    /// Note that the validate only mode should only be used for testing the API,
    /// or if the model is not ready.
    #[prost(bool, tag="5")]
    pub validate_only: bool,
    /// Additional domain specific parameters for the recommendations.
    ///
    /// Allowed values:
    ///
    /// * `returnDocument`: Boolean. If set to true, the associated Document
    ///     object will be returned in
    ///     \[RecommendResponse.results.document][RecommendationResult.document\].
    /// * `returnScore`: Boolean. If set to true, the recommendation 'score'
    ///     corresponding to each returned Document will be set in
    ///     \[RecommendResponse.results.metadata][RecommendationResult.metadata\]. The
    ///     given 'score' indicates the probability of a Document conversion given
    ///     the user's context and history.
    /// * `strictFiltering`: Boolean. True by default. If set to false, the service
    ///     will return generic (unfiltered) popular Documents instead of empty if
    ///     your filter blocks all recommendation results.
    /// * `diversityLevel`: String. Default empty. If set to be non-empty, then
    ///     it needs to be one of:
    ///     *  'no-diversity'
    ///     *  'low-diversity'
    ///     *  'medium-diversity'
    ///     *  'high-diversity'
    ///     *  'auto-diversity'
    ///     This gives request-level control and adjusts recommendation results
    ///     based on Document category.
    #[prost(map="string, message", tag="6")]
    pub params: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
}
/// Response message for Recommend method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendResponse {
    /// A list of recommended Documents. The order represents the ranking (from the
    /// most relevant Document to the least).
    #[prost(message, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<recommend_response::RecommendationResult>,
    /// A unique attribution token. This should be included in the
    /// \[UserEvent][google.cloud.discoveryengine.v1beta.UserEvent\] logs resulting
    /// from this recommendation, which enables accurate attribution of
    /// recommendation model performance.
    #[prost(string, tag="2")]
    pub attribution_token: ::prost::alloc::string::String,
    /// IDs of documents in the request that were missing from the default Branch
    /// associated with the requested ServingConfig.
    #[prost(string, repeated, tag="3")]
    pub missing_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// True if
    /// \[RecommendRequest.validate_only][google.cloud.discoveryengine.v1beta.RecommendRequest.validate_only\]
    /// was set.
    #[prost(bool, tag="4")]
    pub validate_only: bool,
}
/// Nested message and enum types in `RecommendResponse`.
pub mod recommend_response {
    /// RecommendationResult represents a generic recommendation result with
    /// associated metadata.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RecommendationResult {
        /// Resource ID of the recommended Document.
        #[prost(string, tag="1")]
        pub id: ::prost::alloc::string::String,
        /// Set if `returnDocument` is set to true in
        /// \[RecommendRequest.params][google.cloud.discoveryengine.v1beta.RecommendRequest.params\].
        #[prost(message, optional, tag="2")]
        pub document: ::core::option::Option<super::Document>,
        /// Additional Document metadata / annotations.
        ///
        /// Possible values:
        ///
        /// * `score`: Recommendation score in double value. Is set if
        ///    `returnScore` is set to true in
        ///    \[RecommendRequest.params][google.cloud.discoveryengine.v1beta.RecommendRequest.params\].
        #[prost(map="string, message", tag="3")]
        pub metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
    }
}
/// Generated client implementations.
pub mod recommendation_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for making recommendations.
    #[derive(Debug, Clone)]
    pub struct RecommendationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RecommendationServiceClient<tonic::transport::Channel> {
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
    impl<T> RecommendationServiceClient<T>
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
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RecommendationServiceClient<InterceptedService<T, F>>
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
            RecommendationServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Makes a recommendation, which requires a contextual user event.
        pub async fn recommend(
            &mut self,
            request: impl tonic::IntoRequest<super::RecommendRequest>,
        ) -> Result<tonic::Response<super::RecommendResponse>, tonic::Status> {
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
                "/google.cloud.discoveryengine.v1beta.RecommendationService/Recommend",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for WriteUserEvent method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteUserEventRequest {
    /// Required. The parent DataStore resource name, such as
    /// `projects/{project}/locations/{location}/dataStores/{data_store}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User event to write.
    #[prost(message, optional, tag="2")]
    pub user_event: ::core::option::Option<UserEvent>,
}
/// Request message for CollectUserEvent method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectUserEventRequest {
    /// Required. The parent DataStore resource name, such as
    /// `projects/{project}/locations/{location}/dataStores/{data_store}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. URL encoded UserEvent proto with a length limit of 2,000,000
    /// characters.
    #[prost(string, tag="2")]
    pub user_event: ::prost::alloc::string::String,
    /// The URL including cgi-parameters but excluding the hash fragment with a
    /// length limit of 5,000 characters. This is often more useful than the
    /// referer URL, because many browsers only send the domain for 3rd party
    /// requests.
    #[prost(string, optional, tag="3")]
    pub uri: ::core::option::Option<::prost::alloc::string::String>,
    /// The event timestamp in milliseconds. This prevents browser caching of
    /// otherwise identical get requests. The name is abbreviated to reduce the
    /// payload bytes.
    #[prost(int64, optional, tag="4")]
    pub ets: ::core::option::Option<i64>,
}
/// Generated client implementations.
pub mod user_event_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for ingesting end user actions on a website to Discovery Engine API.
    #[derive(Debug, Clone)]
    pub struct UserEventServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UserEventServiceClient<tonic::transport::Channel> {
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
    impl<T> UserEventServiceClient<T>
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
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UserEventServiceClient<InterceptedService<T, F>>
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
            UserEventServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Writes a single user event.
        pub async fn write_user_event(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteUserEventRequest>,
        ) -> Result<tonic::Response<super::UserEvent>, tonic::Status> {
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
                "/google.cloud.discoveryengine.v1beta.UserEventService/WriteUserEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Writes a single user event from the browser. This uses a GET request to
        /// due to browser restriction of POST-ing to a 3rd party domain.
        ///
        /// This method is used only by the Discovery Engine API JavaScript pixel and
        /// Google Tag Manager. Users should not call this method directly.
        pub async fn collect_user_event(
            &mut self,
            request: impl tonic::IntoRequest<super::CollectUserEventRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::api::HttpBody>,
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
                "/google.cloud.discoveryengine.v1beta.UserEventService/CollectUserEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Bulk import of User events. Request processing might be
        /// synchronous. Events that already exist are skipped.
        /// Use this method for backfilling historical user events.
        ///
        /// Operation.response is of type ImportResponse. Note that it is
        /// possible for a subset of the items to be successfully inserted.
        /// Operation.metadata is of type ImportMetadata.
        pub async fn import_user_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportUserEventsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.discoveryengine.v1beta.UserEventService/ImportUserEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
