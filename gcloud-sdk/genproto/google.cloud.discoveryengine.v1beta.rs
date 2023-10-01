/// A floating point interval.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Interval {
    /// The lower bound of the interval. If neither of the min fields are
    /// set, then the lower bound is negative infinity.
    ///
    /// This field must be not larger than max.
    /// Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(oneof = "interval::Min", tags = "1, 2")]
    pub min: ::core::option::Option<interval::Min>,
    /// The upper bound of the interval. If neither of the max fields are
    /// set, then the upper bound is positive infinity.
    ///
    /// This field must be not smaller than min.
    /// Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(oneof = "interval::Max", tags = "3, 4")]
    pub max: ::core::option::Option<interval::Max>,
}
/// Nested message and enum types in `Interval`.
pub mod interval {
    /// The lower bound of the interval. If neither of the min fields are
    /// set, then the lower bound is negative infinity.
    ///
    /// This field must be not larger than max.
    /// Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Min {
        /// Inclusive lower bound.
        #[prost(double, tag = "1")]
        Minimum(f64),
        /// Exclusive lower bound.
        #[prost(double, tag = "2")]
        ExclusiveMinimum(f64),
    }
    /// The upper bound of the interval. If neither of the max fields are
    /// set, then the upper bound is positive infinity.
    ///
    /// This field must be not smaller than min.
    /// Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Max {
        /// Inclusive upper bound.
        #[prost(double, tag = "3")]
        Maximum(f64),
        /// Exclusive upper bound.
        #[prost(double, tag = "4")]
        ExclusiveMaximum(f64),
    }
}
/// A custom attribute that is not explicitly modeled in a resource, e.g.
/// [UserEvent][google.cloud.discoveryengine.v1beta.UserEvent].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAttribute {
    /// The textual values of this custom attribute. For example, `["yellow",
    /// "green"]` when the key is "color".
    ///
    /// Empty string is not allowed. Otherwise, an `INVALID_ARGUMENT` error is
    /// returned.
    ///
    /// Exactly one of
    /// [CustomAttribute.text][google.cloud.discoveryengine.v1beta.CustomAttribute.text]
    /// or
    /// [CustomAttribute.numbers][google.cloud.discoveryengine.v1beta.CustomAttribute.numbers]
    /// should be set. Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(string, repeated, tag = "1")]
    pub text: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The numerical values of this custom attribute. For example, `\[2.3, 15.4\]`
    /// when the key is "lengths_cm".
    ///
    /// Exactly one of
    /// [CustomAttribute.text][google.cloud.discoveryengine.v1beta.CustomAttribute.text]
    /// or
    /// [CustomAttribute.numbers][google.cloud.discoveryengine.v1beta.CustomAttribute.numbers]
    /// should be set. Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(double, repeated, tag = "2")]
    pub numbers: ::prost::alloc::vec::Vec<f64>,
}
/// Information of an end user.
#[allow(clippy::derive_partial_eq_without_eq)]
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
    /// characters. Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    /// User agent as included in the HTTP header.
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 1,000
    /// characters. Otherwise, an `INVALID_ARGUMENT` error is returned.
    ///
    /// This should not be set when using the client side event reporting with
    /// GTM or JavaScript tag in
    /// [UserEventService.CollectUserEvent][google.cloud.discoveryengine.v1beta.UserEventService.CollectUserEvent]
    /// or if
    /// [UserEvent.direct_user_request][google.cloud.discoveryengine.v1beta.UserEvent.direct_user_request]
    /// is set.
    #[prost(string, tag = "2")]
    pub user_agent: ::prost::alloc::string::String,
}
/// Double list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleList {
    /// Double values.
    #[prost(double, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<f64>,
}
/// Request message for
/// [CompletionService.CompleteQuery][google.cloud.discoveryengine.v1beta.CompletionService.CompleteQuery]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteQueryRequest {
    /// Required. The parent data store resource name for which the completion is
    /// performed, such as
    /// `projects/*/locations/global/collections/default_collection/dataStores/default_data_store`.
    #[prost(string, tag = "1")]
    pub data_store: ::prost::alloc::string::String,
    /// Required. The typeahead input used to fetch suggestions. Maximum length is
    /// 128 characters.
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// Selects data model of query suggestions for serving. Currently supported
    /// values:
    ///
    /// * `document` - Using suggestions generated from user-imported documents.
    /// * `search-history` - Using suggestions generated from the past history of
    /// [SearchService.Search][google.cloud.discoveryengine.v1beta.SearchService.Search]
    /// API calls. Do not use it when there is no traffic for Search API.
    /// * `user-event` - Using suggestions generated from user-imported search
    /// events.
    /// * `document-completable` - Using suggestions taken directly from
    /// user-imported document fields marked as completable.
    ///
    /// Default values:
    ///
    /// * `document` is the default model for regular dataStores.
    /// * `search-history` is the default model for
    /// [IndustryVertical.SITE_SEARCH][] dataStores.
    #[prost(string, tag = "3")]
    pub query_model: ::prost::alloc::string::String,
    /// A unique identifier for tracking visitors. For example, this could be
    /// implemented with an HTTP cookie, which should be able to uniquely identify
    /// a visitor on a single device. This unique identifier should not change if
    /// the visitor logs in or out of the website.
    ///
    /// This field should NOT have a fixed value such as `unknown_visitor`.
    ///
    /// This should be the same identifier as
    /// [UserEvent.user_pseudo_id][google.cloud.discoveryengine.v1beta.UserEvent.user_pseudo_id]
    /// and
    /// [SearchRequest.user_pseudo_id][google.cloud.discoveryengine.v1beta.SearchRequest.user_pseudo_id].
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "4")]
    pub user_pseudo_id: ::prost::alloc::string::String,
    /// Indicates if tail suggestions should be returned if there are no
    /// suggestions that match the full query. Even if set to true, if there are
    /// suggestions that match the full query, those are returned and no
    /// tail suggestions are returned.
    #[prost(bool, tag = "5")]
    pub include_tail_suggestions: bool,
}
/// Response message for
/// [CompletionService.CompleteQuery][google.cloud.discoveryengine.v1beta.CompletionService.CompleteQuery]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteQueryResponse {
    /// Results of the matched query suggestions. The result list is ordered and
    /// the first result is a top suggestion.
    #[prost(message, repeated, tag = "1")]
    pub query_suggestions: ::prost::alloc::vec::Vec<
        complete_query_response::QuerySuggestion,
    >,
    /// True if the returned suggestions are all tail suggestions.
    ///
    /// For tail matching to be triggered, include_tail_suggestions in the request
    /// must be true and there must be no suggestions that match the full query.
    #[prost(bool, tag = "2")]
    pub tail_match_triggered: bool,
}
/// Nested message and enum types in `CompleteQueryResponse`.
pub mod complete_query_response {
    /// Suggestions as search queries.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QuerySuggestion {
        /// The suggestion for the query.
        #[prost(string, tag = "1")]
        pub suggestion: ::prost::alloc::string::String,
    }
}
/// Generated client implementations.
pub mod completion_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for Auto-Completion.
    #[derive(Debug, Clone)]
    pub struct CompletionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CompletionServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CompletionServiceClient<T>
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
        ) -> CompletionServiceClient<InterceptedService<T, F>>
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
            CompletionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Completes the specified user input with keyword suggestions.
        pub async fn complete_query(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteQueryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CompleteQueryResponse>,
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
                "/google.cloud.discoveryengine.v1beta.CompletionService/CompleteQuery",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.CompletionService",
                        "CompleteQuery",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Document captures all raw metadata information of items to be recommended or
/// searched.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    /// Immutable. The full resource name of the document.
    /// Format:
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document_id}`.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 1024
    /// characters.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The identifier of the document.
    ///
    /// Id should conform to [RFC-1034](<https://tools.ietf.org/html/rfc1034>)
    /// standard with a length limit of 63 characters.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// The identifier of the schema located in the same data store.
    #[prost(string, tag = "3")]
    pub schema_id: ::prost::alloc::string::String,
    /// The unstructured data linked to this document. Content must be set if this
    /// document is under a
    /// `CONTENT_REQUIRED` data store.
    #[prost(message, optional, tag = "10")]
    pub content: ::core::option::Option<document::Content>,
    /// The identifier of the parent document. Currently supports at most two level
    /// document hierarchy.
    ///
    /// Id should conform to [RFC-1034](<https://tools.ietf.org/html/rfc1034>)
    /// standard with a length limit of 63 characters.
    #[prost(string, tag = "7")]
    pub parent_document_id: ::prost::alloc::string::String,
    /// Output only. This field is OUTPUT_ONLY.
    /// It contains derived data that are not in the original input document.
    #[prost(message, optional, tag = "6")]
    pub derived_struct_data: ::core::option::Option<::prost_types::Struct>,
    /// Data representation. One of
    /// [struct_data][google.cloud.discoveryengine.v1beta.Document.struct_data] or
    /// [json_data][google.cloud.discoveryengine.v1beta.Document.json_data] should
    /// be provided otherwise an `INVALID_ARGUMENT` error is thrown.
    #[prost(oneof = "document::Data", tags = "4, 5")]
    pub data: ::core::option::Option<document::Data>,
}
/// Nested message and enum types in `Document`.
pub mod document {
    /// Unstructured data linked to this document.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Content {
        /// The MIME type of the content. Supported types:
        ///
        /// * `application/pdf` (PDF, only native PDFs are supported for now)
        /// * `text/html` (HTML)
        /// * `application/vnd.openxmlformats-officedocument.wordprocessingml.document` (DOCX)
        /// * `application/vnd.openxmlformats-officedocument.presentationml.presentation` (PPTX)
        /// * `text/plain` (TXT)
        ///
        /// See <https://www.iana.org/assignments/media-types/media-types.xhtml.>
        #[prost(string, tag = "1")]
        pub mime_type: ::prost::alloc::string::String,
        #[prost(oneof = "content::Content", tags = "2, 3")]
        pub content: ::core::option::Option<content::Content>,
    }
    /// Nested message and enum types in `Content`.
    pub mod content {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Content {
            /// The content represented as a stream of bytes. The maximum length is
            /// 1,000,000 bytes (1 MB / ~0.95 MiB).
            ///
            /// Note: As with all `bytes` fields, this field is represented as pure
            /// binary in Protocol Buffers and base64-encoded string in JSON. For
            /// example, `abc123!?$*&()'-=@~` should be represented as
            /// `YWJjMTIzIT8kKiYoKSctPUB+` in JSON. See
            /// <https://developers.google.com/protocol-buffers/docs/proto3#json.>
            #[prost(bytes, tag = "2")]
            RawBytes(::prost::alloc::vec::Vec<u8>),
            /// The URI of the content. Only Cloud Storage URIs (e.g.
            /// `gs://bucket-name/path/to/file`) are supported. The maximum file size
            /// is 100 MB.
            #[prost(string, tag = "3")]
            Uri(::prost::alloc::string::String),
        }
    }
    /// Data representation. One of
    /// [struct_data][google.cloud.discoveryengine.v1beta.Document.struct_data] or
    /// [json_data][google.cloud.discoveryengine.v1beta.Document.json_data] should
    /// be provided otherwise an `INVALID_ARGUMENT` error is thrown.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// The structured JSON data for the document. It should conform to the
        /// registered [Schema][google.cloud.discoveryengine.v1beta.Schema] or an
        /// `INVALID_ARGUMENT` error is thrown.
        #[prost(message, tag = "4")]
        StructData(::prost_types::Struct),
        /// The JSON string representation of the document. It should conform to the
        /// registered [Schema][google.cloud.discoveryengine.v1beta.Schema] or an
        /// `INVALID_ARGUMENT` error is thrown.
        #[prost(string, tag = "5")]
        JsonData(::prost::alloc::string::String),
    }
}
/// Request message for
/// [SearchService.Search][google.cloud.discoveryengine.v1beta.SearchService.Search]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRequest {
    /// Required. The resource name of the Search serving config, such as
    /// `projects/*/locations/global/collections/default_collection/dataStores/default_data_store/servingConfigs/default_serving_config`.
    /// This field is used to identify the serving configuration name, set
    /// of models used to make the search.
    #[prost(string, tag = "1")]
    pub serving_config: ::prost::alloc::string::String,
    /// The branch resource name, such as
    /// `projects/*/locations/global/collections/default_collection/dataStores/default_data_store/branches/0`.
    ///
    /// Use `default_branch` as the branch ID or leave this field empty, to search
    /// documents under the default branch.
    #[prost(string, tag = "2")]
    pub branch: ::prost::alloc::string::String,
    /// Raw search query.
    #[prost(string, tag = "3")]
    pub query: ::prost::alloc::string::String,
    /// Raw image query.
    #[prost(message, optional, tag = "19")]
    pub image_query: ::core::option::Option<search_request::ImageQuery>,
    /// Maximum number of [Document][google.cloud.discoveryengine.v1beta.Document]s
    /// to return. If unspecified, defaults to a reasonable value. The maximum
    /// allowed value is 100. Values above 100 are coerced to 100.
    ///
    /// If this field is negative, an  `INVALID_ARGUMENT`  is returned.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// A page token received from a previous
    /// [SearchService.Search][google.cloud.discoveryengine.v1beta.SearchService.Search]
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// [SearchService.Search][google.cloud.discoveryengine.v1beta.SearchService.Search]
    /// must match the call that provided the page token. Otherwise, an
    ///   `INVALID_ARGUMENT`  error is returned.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
    /// A 0-indexed integer that specifies the current offset (that is, starting
    /// result location, amongst the
    /// [Document][google.cloud.discoveryengine.v1beta.Document]s deemed by the API
    /// as relevant) in search results. This field is only considered if
    /// [page_token][google.cloud.discoveryengine.v1beta.SearchRequest.page_token]
    /// is unset.
    ///
    /// If this field is negative, an  `INVALID_ARGUMENT`  is returned.
    #[prost(int32, tag = "6")]
    pub offset: i32,
    /// The filter syntax consists of an expression language for constructing a
    /// predicate from one or more fields of the documents being filtered. Filter
    /// expression is case-sensitive.
    ///
    /// If this field is unrecognizable, an  `INVALID_ARGUMENT`  is returned.
    #[prost(string, tag = "7")]
    pub filter: ::prost::alloc::string::String,
    /// The order in which documents are returned. Documents can be ordered by
    /// a field in an [Document][google.cloud.discoveryengine.v1beta.Document]
    /// object. Leave it unset if ordered by relevance. `order_by` expression is
    /// case-sensitive.
    ///
    /// If this field is unrecognizable, an `INVALID_ARGUMENT` is returned.
    #[prost(string, tag = "8")]
    pub order_by: ::prost::alloc::string::String,
    /// Information about the end user.
    /// Highly recommended for analytics.
    /// [UserInfo.user_agent][google.cloud.discoveryengine.v1beta.UserInfo.user_agent]
    /// is used to deduce `device_type` for analytics.
    #[prost(message, optional, tag = "21")]
    pub user_info: ::core::option::Option<UserInfo>,
    /// Facet specifications for faceted search. If empty, no facets are returned.
    ///
    /// A maximum of 100 values are allowed. Otherwise, an  `INVALID_ARGUMENT`
    /// error is returned.
    #[prost(message, repeated, tag = "9")]
    pub facet_specs: ::prost::alloc::vec::Vec<search_request::FacetSpec>,
    /// Boost specification to boost certain documents.
    #[prost(message, optional, tag = "10")]
    pub boost_spec: ::core::option::Option<search_request::BoostSpec>,
    /// Additional search parameters.
    ///
    /// For public website search only, supported values are:
    ///
    /// * `user_country_code`: string. Default empty. If set to non-empty, results
    ///     are restricted or boosted based on the location provided.
    /// * `search_type`: double. Default empty. Enables non-webpage searching
    ///    depending on the value. The only valid non-default value is 1,
    ///    which enables image searching.
    #[prost(map = "string, message", tag = "11")]
    pub params: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost_types::Value,
    >,
    /// The query expansion specification that specifies the conditions under which
    /// query expansion occurs.
    #[prost(message, optional, tag = "13")]
    pub query_expansion_spec: ::core::option::Option<search_request::QueryExpansionSpec>,
    /// The spell correction specification that specifies the mode under
    /// which spell correction takes effect.
    #[prost(message, optional, tag = "14")]
    pub spell_correction_spec: ::core::option::Option<
        search_request::SpellCorrectionSpec,
    >,
    /// A unique identifier for tracking visitors. For example, this could be
    /// implemented with an HTTP cookie, which should be able to uniquely identify
    /// a visitor on a single device. This unique identifier should not change if
    /// the visitor logs in or out of the website.
    ///
    /// This field should NOT have a fixed value such as `unknown_visitor`.
    ///
    /// This should be the same identifier as
    /// [UserEvent.user_pseudo_id][google.cloud.discoveryengine.v1beta.UserEvent.user_pseudo_id]
    /// and
    /// [CompleteQueryRequest.user_pseudo_id][google.cloud.discoveryengine.v1beta.CompleteQueryRequest.user_pseudo_id]
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an  `INVALID_ARGUMENT`  error is returned.
    #[prost(string, tag = "15")]
    pub user_pseudo_id: ::prost::alloc::string::String,
    /// A specification for configuring the behavior of content search.
    #[prost(message, optional, tag = "24")]
    pub content_search_spec: ::core::option::Option<search_request::ContentSearchSpec>,
    /// Uses the provided embedding to do additional semantic document retrieval.
    /// The retrieval is based on the dot product of
    /// [SearchRequest.embedding_spec.embedding_vectors.vector][] and the document
    /// embedding that is provided in
    /// [SearchRequest.embedding_spec.embedding_vectors.field_path][].
    ///
    /// If [SearchRequest.embedding_spec.embedding_vectors.field_path][] is not
    /// provided, it will use [ServingConfig.embedding_config.field_paths][].
    #[prost(message, optional, tag = "23")]
    pub embedding_spec: ::core::option::Option<search_request::EmbeddingSpec>,
    /// The ranking expression controls the customized ranking on retrieval
    /// documents. This overrides [ServingConfig.ranking_expression][].
    /// The ranking expression is a single function or multiple functions that are
    /// joint by "+".
    ///    * ranking_expression = function, { " + ", function };
    /// Supported functions:
    ///    * double * relevance_score
    ///    * double * dotProduct(embedding_field_path)
    /// Function variables:
    ///    `relevance_score`: pre-defined keywords, used for measure relevance
    ///    between query and document.
    ///    `embedding_field_path`: the document embedding field
    ///    used with query embedding vector.
    ///    `dotProduct`: embedding function between embedding_field_path and query
    ///    embedding vector.
    ///
    ///   Example ranking expression:
    ///     If document has an embedding field doc_embedding, the ranking expression
    ///     could be `0.5 * relevance_score + 0.3 * dotProduct(doc_embedding)`.
    #[prost(string, tag = "26")]
    pub ranking_expression: ::prost::alloc::string::String,
    /// Whether to turn on safe search. This is only supported for
    /// website search.
    #[prost(bool, tag = "20")]
    pub safe_search: bool,
    /// The user labels applied to a resource must meet the following requirements:
    ///
    /// * Each resource can have multiple labels, up to a maximum of 64.
    /// * Each label must be a key-value pair.
    /// * Keys have a minimum length of 1 character and a maximum length of 63
    ///    characters and cannot be empty. Values can be empty and have a maximum
    ///    length of 63 characters.
    /// * Keys and values can contain only lowercase letters, numeric characters,
    ///    underscores, and dashes. All characters must use UTF-8 encoding, and
    ///    international characters are allowed.
    /// * The key portion of a label must be unique. However, you can use the same
    ///    key with multiple resources.
    /// * Keys must start with a lowercase letter or international character.
    ///
    /// See [Google Cloud
    /// Document](<https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements>)
    /// for more details.
    #[prost(map = "string, string", tag = "22")]
    pub user_labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `SearchRequest`.
pub mod search_request {
    /// Specifies the image query input.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ImageQuery {
        #[prost(oneof = "image_query::Image", tags = "1")]
        pub image: ::core::option::Option<image_query::Image>,
    }
    /// Nested message and enum types in `ImageQuery`.
    pub mod image_query {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Image {
            /// Base64 encoded image bytes. Supported image formats: JPEG, PNG, and
            /// BMP.
            #[prost(string, tag = "1")]
            ImageBytes(::prost::alloc::string::String),
        }
    }
    /// A facet specification to perform faceted search.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FacetSpec {
        /// Required. The facet key specification.
        #[prost(message, optional, tag = "1")]
        pub facet_key: ::core::option::Option<facet_spec::FacetKey>,
        /// Maximum of facet values that should be returned for this facet. If
        /// unspecified, defaults to 20. The maximum allowed value is 300. Values
        /// above 300 are coerced to 300.
        ///
        /// If this field is negative, an  `INVALID_ARGUMENT`  is returned.
        #[prost(int32, tag = "2")]
        pub limit: i32,
        /// List of keys to exclude when faceting.
        ///
        ///
        /// By default,
        /// [FacetKey.key][google.cloud.discoveryengine.v1beta.SearchRequest.FacetSpec.FacetKey.key]
        /// is not excluded from the filter unless it is listed in this field.
        ///
        /// Listing a facet key in this field allows its values to appear as facet
        /// results, even when they are filtered out of search results. Using this
        /// field does not affect what search results are returned.
        ///
        /// For example, suppose there are 100 documents with the color facet "Red"
        /// and 200 documents with the color facet "Blue". A query containing the
        /// filter "color:ANY("Red")" and having "color" as
        /// [FacetKey.key][google.cloud.discoveryengine.v1beta.SearchRequest.FacetSpec.FacetKey.key]
        /// would by default return only "Red" documents in the search results, and
        /// also return "Red" with count 100 as the only color facet. Although there
        /// are also blue documents available, "Blue" would not be shown as an
        /// available facet value.
        ///
        /// If "color" is listed in "excludedFilterKeys", then the query returns the
        /// facet values "Red" with count 100 and "Blue" with count 200, because the
        /// "color" key is now excluded from the filter. Because this field doesn't
        /// affect search results, the search results are still correctly filtered to
        /// return only "Red" documents.
        ///
        /// A maximum of 100 values are allowed. Otherwise, an  `INVALID_ARGUMENT`
        /// error is returned.
        #[prost(string, repeated, tag = "3")]
        pub excluded_filter_keys: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// Enables dynamic position for this facet. If set to true, the position of
        /// this facet among all facets in the response is determined automatically.
        /// If dynamic facets are enabled, it is ordered together.
        /// If set to false, the position of this facet in the
        /// response is the same as in the request, and it is ranked before
        /// the facets with dynamic position enable and all dynamic facets.
        ///
        /// For example, you may always want to have rating facet returned in
        /// the response, but it's not necessarily to always display the rating facet
        /// at the top. In that case, you can set enable_dynamic_position to true so
        /// that the position of rating facet in response is determined
        /// automatically.
        ///
        /// Another example, assuming you have the following facets in the request:
        ///
        /// * "rating", enable_dynamic_position = true
        ///
        /// * "price", enable_dynamic_position = false
        ///
        /// * "brands", enable_dynamic_position = false
        ///
        /// And also you have a dynamic facets enabled, which generates a facet
        /// `gender`. Then the final order of the facets in the response can be
        /// ("price", "brands", "rating", "gender") or ("price", "brands", "gender",
        /// "rating") depends on how API orders "gender" and "rating" facets.
        /// However, notice that "price" and "brands" are always
        /// ranked at first and second position because their enable_dynamic_position
        /// is false.
        #[prost(bool, tag = "4")]
        pub enable_dynamic_position: bool,
    }
    /// Nested message and enum types in `FacetSpec`.
    pub mod facet_spec {
        /// Specifies how a facet is computed.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FacetKey {
            /// Required. Supported textual and numerical facet keys in
            /// [Document][google.cloud.discoveryengine.v1beta.Document] object, over
            /// which the facet values are computed. Facet key is case-sensitive.
            #[prost(string, tag = "1")]
            pub key: ::prost::alloc::string::String,
            /// Set only if values should be bucketed into intervals. Must be set
            /// for facets with numerical values. Must not be set for facet with text
            /// values. Maximum number of intervals is 30.
            #[prost(message, repeated, tag = "2")]
            pub intervals: ::prost::alloc::vec::Vec<super::super::Interval>,
            /// Only get facet for the given restricted values. Only supported on
            /// textual fields. For example, suppose "category" has three values
            /// "Action > 2022", "Action > 2021" and "Sci-Fi > 2022". If set
            /// "restricted_values" to "Action > 2022", the "category" facet only
            /// contains "Action > 2022". Only supported on textual fields. Maximum
            /// is 10.
            #[prost(string, repeated, tag = "3")]
            pub restricted_values: ::prost::alloc::vec::Vec<
                ::prost::alloc::string::String,
            >,
            /// Only get facet values that start with the given string prefix. For
            /// example, suppose "category" has three values "Action > 2022",
            /// "Action > 2021" and "Sci-Fi > 2022". If set "prefixes" to "Action", the
            /// "category" facet only contains "Action > 2022" and "Action > 2021".
            /// Only supported on textual fields. Maximum is 10.
            #[prost(string, repeated, tag = "4")]
            pub prefixes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Only get facet values that contains the given strings. For example,
            /// suppose "category" has three values "Action > 2022",
            /// "Action > 2021" and "Sci-Fi > 2022". If set "contains" to "2022", the
            /// "category" facet only contains "Action > 2022" and "Sci-Fi > 2022".
            /// Only supported on textual fields. Maximum is 10.
            #[prost(string, repeated, tag = "5")]
            pub contains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// True to make facet keys case insensitive when getting faceting
            /// values with prefixes or contains; false otherwise.
            #[prost(bool, tag = "6")]
            pub case_insensitive: bool,
            /// The order in which documents are returned.
            ///
            /// Allowed values are:
            ///
            /// * "count desc", which means order by
            /// [SearchResponse.Facet.values.count][google.cloud.discoveryengine.v1beta.SearchResponse.Facet.FacetValue.count]
            /// descending.
            ///
            /// * "value desc", which means order by
            /// [SearchResponse.Facet.values.value][google.cloud.discoveryengine.v1beta.SearchResponse.Facet.FacetValue.value]
            /// descending.
            ///    Only applies to textual facets.
            ///
            /// If not set, textual values are sorted in [natural
            /// order](<https://en.wikipedia.org/wiki/Natural_sort_order>); numerical
            /// intervals are sorted in the order given by
            /// [FacetSpec.FacetKey.intervals][google.cloud.discoveryengine.v1beta.SearchRequest.FacetSpec.FacetKey.intervals].
            #[prost(string, tag = "7")]
            pub order_by: ::prost::alloc::string::String,
        }
    }
    /// Boost specification to boost certain documents.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BoostSpec {
        /// Condition boost specifications. If a document matches multiple conditions
        /// in the specifictions, boost scores from these specifications are all
        /// applied and combined in a non-linear way. Maximum number of
        /// specifications is 20.
        #[prost(message, repeated, tag = "1")]
        pub condition_boost_specs: ::prost::alloc::vec::Vec<
            boost_spec::ConditionBoostSpec,
        >,
    }
    /// Nested message and enum types in `BoostSpec`.
    pub mod boost_spec {
        /// Boost applies to documents which match a condition.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ConditionBoostSpec {
            /// An expression which specifies a boost condition. The syntax and
            /// supported fields are the same as a filter expression. See
            /// [SearchRequest.filter][google.cloud.discoveryengine.v1beta.SearchRequest.filter]
            /// for detail syntax and limitations.
            ///
            /// Examples:
            ///
            /// * To boost documents with document ID "doc_1" or "doc_2", and
            /// color
            ///    "Red" or "Blue":
            ///      * (id: ANY("doc_1", "doc_2")) AND (color: ANY("Red","Blue"))
            #[prost(string, tag = "1")]
            pub condition: ::prost::alloc::string::String,
            /// Strength of the condition boost, which should be in \[-1, 1\]. Negative
            /// boost means demotion. Default is 0.0.
            ///
            /// Setting to 1.0 gives the document a big promotion. However, it does not
            /// necessarily mean that the boosted document will be the top result at
            /// all times, nor that other documents will be excluded. Results could
            /// still be shown even when none of them matches the condition. And
            /// results that are significantly more relevant to the search query can
            /// still trump your heavily favored but irrelevant documents.
            ///
            /// Setting to -1.0 gives the document a big demotion. However, results
            /// that are deeply relevant might still be shown. The document will have
            /// an upstream battle to get a fairly high ranking, but it is not blocked
            /// out completely.
            ///
            /// Setting to 0.0 means no boost applied. The boosting condition is
            /// ignored.
            #[prost(float, tag = "2")]
            pub boost: f32,
        }
    }
    /// Specification to determine under which conditions query expansion should
    /// occur.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QueryExpansionSpec {
        /// The condition under which query expansion should occur. Default to
        /// [Condition.DISABLED][google.cloud.discoveryengine.v1beta.SearchRequest.QueryExpansionSpec.Condition.DISABLED].
        #[prost(enumeration = "query_expansion_spec::Condition", tag = "1")]
        pub condition: i32,
        /// Whether to pin unexpanded results. If this field is set to true,
        /// unexpanded products are always at the top of the search results, followed
        /// by the expanded results.
        #[prost(bool, tag = "2")]
        pub pin_unexpanded_results: bool,
    }
    /// Nested message and enum types in `QueryExpansionSpec`.
    pub mod query_expansion_spec {
        /// Enum describing under which condition query expansion should occur.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum Condition {
            /// Unspecified query expansion condition. In this case, server behavior
            /// defaults to
            /// [Condition.DISABLED][google.cloud.discoveryengine.v1beta.SearchRequest.QueryExpansionSpec.Condition.DISABLED].
            Unspecified = 0,
            /// Disabled query expansion. Only the exact search query is used, even if
            /// [SearchResponse.total_size][google.cloud.discoveryengine.v1beta.SearchResponse.total_size]
            /// is zero.
            Disabled = 1,
            /// Automatic query expansion built by the Search API.
            Auto = 2,
        }
        impl Condition {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Condition::Unspecified => "CONDITION_UNSPECIFIED",
                    Condition::Disabled => "DISABLED",
                    Condition::Auto => "AUTO",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "CONDITION_UNSPECIFIED" => Some(Self::Unspecified),
                    "DISABLED" => Some(Self::Disabled),
                    "AUTO" => Some(Self::Auto),
                    _ => None,
                }
            }
        }
    }
    /// The specification for query spell correction.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SpellCorrectionSpec {
        /// The mode under which spell correction should take effect to
        /// replace the original search query. Default to
        /// [Mode.AUTO][google.cloud.discoveryengine.v1beta.SearchRequest.SpellCorrectionSpec.Mode.AUTO].
        #[prost(enumeration = "spell_correction_spec::Mode", tag = "1")]
        pub mode: i32,
    }
    /// Nested message and enum types in `SpellCorrectionSpec`.
    pub mod spell_correction_spec {
        /// Enum describing under which mode spell correction should occur.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum Mode {
            /// Unspecified spell correction mode. In this case, server behavior
            /// defaults to
            /// [Mode.AUTO][google.cloud.discoveryengine.v1beta.SearchRequest.SpellCorrectionSpec.Mode.AUTO].
            Unspecified = 0,
            /// Search API will try to find a spell suggestion if there
            /// is any and put in the
            /// [SearchResponse.corrected_query][google.cloud.discoveryengine.v1beta.SearchResponse.corrected_query].
            /// The spell suggestion will not be used as the search query.
            SuggestionOnly = 1,
            /// Automatic spell correction built by the Search API. Search will
            /// be based on the corrected query if found.
            Auto = 2,
        }
        impl Mode {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Mode::Unspecified => "MODE_UNSPECIFIED",
                    Mode::SuggestionOnly => "SUGGESTION_ONLY",
                    Mode::Auto => "AUTO",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "MODE_UNSPECIFIED" => Some(Self::Unspecified),
                    "SUGGESTION_ONLY" => Some(Self::SuggestionOnly),
                    "AUTO" => Some(Self::Auto),
                    _ => None,
                }
            }
        }
    }
    /// A specification for configuring the behavior of content search.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ContentSearchSpec {
        /// If `snippetSpec` is not specified, snippets are not included in the
        /// search response.
        #[prost(message, optional, tag = "1")]
        pub snippet_spec: ::core::option::Option<content_search_spec::SnippetSpec>,
        /// If `summarySpec` is not specified, summaries are not included in the
        /// search response.
        #[prost(message, optional, tag = "2")]
        pub summary_spec: ::core::option::Option<content_search_spec::SummarySpec>,
        /// If there is no extractive_content_spec provided, there will be no
        /// extractive answer in the search response.
        #[prost(message, optional, tag = "3")]
        pub extractive_content_spec: ::core::option::Option<
            content_search_spec::ExtractiveContentSpec,
        >,
    }
    /// Nested message and enum types in `ContentSearchSpec`.
    pub mod content_search_spec {
        /// A specification for configuring snippets in a search response.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SnippetSpec {
            /// \[DEPRECATED\] This field is deprecated. To control snippet return, use
            /// `return_snippet` field. For backwards compatibility, we will return
            /// snippet if max_snippet_count > 0.
            #[deprecated]
            #[prost(int32, tag = "1")]
            pub max_snippet_count: i32,
            /// \[DEPRECATED\] This field is deprecated and will have no affect on the
            /// snippet.
            #[deprecated]
            #[prost(bool, tag = "2")]
            pub reference_only: bool,
            /// If `true`, then return snippet. If no snippet can be generated, we
            /// return "No snippet is available for this page." A `snippet_status` with
            /// `SUCCESS` or `NO_SNIPPET_AVAILABLE` will also be returned.
            #[prost(bool, tag = "3")]
            pub return_snippet: bool,
        }
        /// A specification for configuring a summary returned in a search
        /// response.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SummarySpec {
            /// The number of top results to generate the summary from. If the number
            /// of results returned is less than `summaryResultCount`, the summary is
            /// generated from all of the results.
            ///
            /// At most five results can be used to generate a summary.
            #[prost(int32, tag = "1")]
            pub summary_result_count: i32,
            /// Specifies whether to include citations in the summary. The default
            /// value is `false`.
            ///
            /// When this field is set to `true`, summaries include in-line citation
            /// numbers.
            ///
            /// Example summary including citations:
            ///
            /// BigQuery is Google Cloud's fully managed and completely serverless
            /// enterprise data warehouse \[1\]. BigQuery supports all data types, works
            /// across clouds, and has built-in machine learning and business
            /// intelligence, all within a unified platform \[2, 3\].
            ///
            /// The citation numbers refer to the returned search results and are
            /// 1-indexed. For example, \[1\] means that the sentence is attributed to
            /// the first search result. \[2, 3\] means that the sentence is attributed
            /// to both the second and third search results.
            #[prost(bool, tag = "2")]
            pub include_citations: bool,
            /// Specifies whether to filter out adversarial queries. The default value
            /// is `false`.
            ///
            /// Google employs search-query classification to detect adversarial
            /// queries. No summary is returned if the search query is classified as an
            /// adversarial query. For example, a user might ask a question regarding
            /// negative comments about the company or submit a query designed to
            /// generate unsafe, policy-violating output. If this field is set to
            /// `true`, we skip generating summaries for adversarial queries and return
            /// fallback messages instead.
            #[prost(bool, tag = "3")]
            pub ignore_adversarial_query: bool,
            /// Specifies whether to filter out queries that are not summary-seeking.
            /// The default value is `false`.
            ///
            /// Google employs search-query classification to detect summary-seeking
            /// queries. No summary is returned if the search query is classified as a
            /// non-summary seeking query. For example, `why is the sky blue` and `Who
            /// is the best soccer player in the world?` are summary-seeking queries,
            /// but `SFO airport` and `world cup 2026` are not. They are most likely
            /// navigational queries. If this field is set to `true`, we skip
            /// generating summaries for non-summary seeking queries and return
            /// fallback messages instead.
            #[prost(bool, tag = "4")]
            pub ignore_non_summary_seeking_query: bool,
            /// Language code for Summary. Use language tags defined by
            /// [BCP47][<https://www.rfc-editor.org/rfc/bcp/bcp47.txt].>
            #[prost(string, tag = "6")]
            pub language_code: ::prost::alloc::string::String,
        }
        /// A specification for configuring the extractive content in a search
        /// response.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ExtractiveContentSpec {
            /// The maximum number of extractive answers returned in each search
            /// result.
            ///
            /// An extractive answer is a verbatim answer extracted from the original
            /// document, which provides a precise and contextually relevant answer to
            /// the search query.
            ///
            /// If the number of matching answers is less than the
            /// `max_extractive_answer_count`, return all of the answers. Otherwise,
            /// return the `max_extractive_answer_count`.
            ///
            /// At most one answer is returned for each
            /// [SearchResult][google.cloud.discoveryengine.v1beta.SearchResponse.SearchResult].
            #[prost(int32, tag = "1")]
            pub max_extractive_answer_count: i32,
            /// The max number of extractive segments returned in each search result.
            /// Only applied if the
            /// [DataStore][google.cloud.discoveryengine.v1beta.DataStore] is set to
            /// [DataStore.ContentConfig.CONTENT_REQUIRED][google.cloud.discoveryengine.v1beta.DataStore.ContentConfig.CONTENT_REQUIRED]
            /// or
            /// [DataStore.solution_types][google.cloud.discoveryengine.v1beta.DataStore.solution_types]
            /// is
            /// [SOLUTION_TYPE_CHAT][google.cloud.discoveryengine.v1beta.SolutionType.SOLUTION_TYPE_CHAT].
            ///
            /// An extractive segment is a text segment extracted from the original
            /// document that is relevant to the search query, and, in general, more
            /// verbose than an extractive answer. The segment could then be used as
            /// input for LLMs to generate summaries and answers.
            ///
            /// If the number of matching segments is less than
            /// `max_extractive_segment_count`, return all of the segments. Otherwise,
            /// return the `max_extractive_segment_count`.
            #[prost(int32, tag = "2")]
            pub max_extractive_segment_count: i32,
            /// Specifies whether to return the confidence score from the extractive
            /// segments in each search result. The default value is `false`.
            #[prost(bool, tag = "3")]
            pub return_extractive_segment_score: bool,
            /// Specifies whether to also include the adjacent from each selected
            /// segments.
            /// Return at most `num_previous_segments` segments before each selected
            /// segments.
            #[prost(int32, tag = "4")]
            pub num_previous_segments: i32,
            /// Return at most `num_next_segments` segments after each selected
            /// segments.
            #[prost(int32, tag = "5")]
            pub num_next_segments: i32,
        }
    }
    /// The specification that uses customized query embedding vector to do
    /// semantic document retrieval.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EmbeddingSpec {
        /// The embedding vector used for retrieval. Limit to 1.
        #[prost(message, repeated, tag = "1")]
        pub embedding_vectors: ::prost::alloc::vec::Vec<embedding_spec::EmbeddingVector>,
    }
    /// Nested message and enum types in `EmbeddingSpec`.
    pub mod embedding_spec {
        /// Embedding vector.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct EmbeddingVector {
            /// Embedding field path in schema.
            #[prost(string, tag = "1")]
            pub field_path: ::prost::alloc::string::String,
            /// Query embedding vector.
            #[prost(float, repeated, tag = "2")]
            pub vector: ::prost::alloc::vec::Vec<f32>,
        }
    }
}
/// Response message for
/// [SearchService.Search][google.cloud.discoveryengine.v1beta.SearchService.Search]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResponse {
    /// A list of matched documents. The order represents the ranking.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<search_response::SearchResult>,
    /// Results of facets requested by user.
    #[prost(message, repeated, tag = "2")]
    pub facets: ::prost::alloc::vec::Vec<search_response::Facet>,
    /// Guided search result.
    #[prost(message, optional, tag = "8")]
    pub guided_search_result: ::core::option::Option<
        search_response::GuidedSearchResult,
    >,
    /// The estimated total count of matched items irrespective of pagination. The
    /// count of
    /// [results][google.cloud.discoveryengine.v1beta.SearchResponse.results]
    /// returned by pagination may be less than the
    /// [total_size][google.cloud.discoveryengine.v1beta.SearchResponse.total_size]
    /// that matches.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
    /// A unique search token. This should be included in the
    /// [UserEvent][google.cloud.discoveryengine.v1beta.UserEvent] logs resulting
    /// from this search, which enables accurate attribution of search model
    /// performance.
    #[prost(string, tag = "4")]
    pub attribution_token: ::prost::alloc::string::String,
    /// The URI of a customer-defined redirect page. If redirect action is
    /// triggered, no search is performed, and only
    /// [redirect_uri][google.cloud.discoveryengine.v1beta.SearchResponse.redirect_uri]
    /// and
    /// [attribution_token][google.cloud.discoveryengine.v1beta.SearchResponse.attribution_token]
    /// are set in the response.
    #[prost(string, tag = "12")]
    pub redirect_uri: ::prost::alloc::string::String,
    /// A token that can be sent as
    /// [SearchRequest.page_token][google.cloud.discoveryengine.v1beta.SearchRequest.page_token]
    /// to retrieve the next page. If this field is omitted, there are no
    /// subsequent pages.
    #[prost(string, tag = "5")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Contains the spell corrected query, if found. If the spell correction type
    /// is AUTOMATIC, then the search results are based on corrected_query.
    /// Otherwise the original query is used for search.
    #[prost(string, tag = "7")]
    pub corrected_query: ::prost::alloc::string::String,
    /// A summary as part of the search results.
    /// This field is only returned if
    /// [SearchRequest.ContentSearchSpec.summary_spec][google.cloud.discoveryengine.v1beta.SearchRequest.ContentSearchSpec.summary_spec]
    /// is set.
    #[prost(message, optional, tag = "9")]
    pub summary: ::core::option::Option<search_response::Summary>,
    /// Controls applied as part of the Control service.
    #[prost(string, repeated, tag = "10")]
    pub applied_controls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Query expansion information for the returned results.
    #[prost(message, optional, tag = "14")]
    pub query_expansion_info: ::core::option::Option<
        search_response::QueryExpansionInfo,
    >,
}
/// Nested message and enum types in `SearchResponse`.
pub mod search_response {
    /// Represents the search results.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SearchResult {
        /// [Document.id][google.cloud.discoveryengine.v1beta.Document.id] of the
        /// searched [Document][google.cloud.discoveryengine.v1beta.Document].
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// The document data snippet in the search response. Only fields that are
        /// marked as retrievable are populated.
        #[prost(message, optional, tag = "2")]
        pub document: ::core::option::Option<super::Document>,
        /// Google provided available scores.
        #[prost(map = "string, message", tag = "4")]
        pub model_scores: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            super::DoubleList,
        >,
    }
    /// A facet result.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Facet {
        /// The key for this facet. E.g., "colors" or "price". It matches
        /// [SearchRequest.FacetSpec.FacetKey.key][google.cloud.discoveryengine.v1beta.SearchRequest.FacetSpec.FacetKey.key].
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        /// The facet values for this field.
        #[prost(message, repeated, tag = "2")]
        pub values: ::prost::alloc::vec::Vec<facet::FacetValue>,
        /// Whether the facet is dynamically generated.
        #[prost(bool, tag = "3")]
        pub dynamic_facet: bool,
    }
    /// Nested message and enum types in `Facet`.
    pub mod facet {
        /// A facet value which contains value names and their count.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FacetValue {
            /// Number of items that have this facet value.
            #[prost(int64, tag = "3")]
            pub count: i64,
            /// A facet value which contains values.
            #[prost(oneof = "facet_value::FacetValue", tags = "1, 2")]
            pub facet_value: ::core::option::Option<facet_value::FacetValue>,
        }
        /// Nested message and enum types in `FacetValue`.
        pub mod facet_value {
            /// A facet value which contains values.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum FacetValue {
                /// Text value of a facet, such as "Black" for facet "colors".
                #[prost(string, tag = "1")]
                Value(::prost::alloc::string::String),
                /// Interval value for a facet, such as [10, 20) for facet "price". It
                /// matches
                /// [SearchRequest.FacetSpec.FacetKey.intervals][google.cloud.discoveryengine.v1beta.SearchRequest.FacetSpec.FacetKey.intervals].
                #[prost(message, tag = "2")]
                Interval(super::super::super::Interval),
            }
        }
    }
    /// Guided search result. The guided search helps user to refine the search
    /// results and narrow down to the real needs from a broaded search results.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GuidedSearchResult {
        /// A list of ranked refinement attributes.
        #[prost(message, repeated, tag = "1")]
        pub refinement_attributes: ::prost::alloc::vec::Vec<
            guided_search_result::RefinementAttribute,
        >,
        /// Suggested follow-up questions.
        #[prost(string, repeated, tag = "2")]
        pub follow_up_questions: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
    /// Nested message and enum types in `GuidedSearchResult`.
    pub mod guided_search_result {
        /// Useful attribute for search result refinements.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RefinementAttribute {
            /// Attribute key used to refine the results e.g. 'movie_type'.
            #[prost(string, tag = "1")]
            pub attribute_key: ::prost::alloc::string::String,
            /// Attribute value used to refine the results e.g. 'drama'.
            #[prost(string, tag = "2")]
            pub attribute_value: ::prost::alloc::string::String,
        }
    }
    /// Summary of the top N search result specified by the summary spec.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Summary {
        /// The summary content.
        #[prost(string, tag = "1")]
        pub summary_text: ::prost::alloc::string::String,
        /// Additional summary-skipped reasons. This provides the reason for ignored
        /// cases. If nothing is skipped, this field is not set.
        #[prost(enumeration = "summary::SummarySkippedReason", repeated, tag = "2")]
        pub summary_skipped_reasons: ::prost::alloc::vec::Vec<i32>,
        /// A collection of Safety Attribute categories and their associated
        /// confidence scores.
        #[prost(message, optional, tag = "3")]
        pub safety_attributes: ::core::option::Option<summary::SafetyAttributes>,
    }
    /// Nested message and enum types in `Summary`.
    pub mod summary {
        /// Safety Attribute categories and their associated confidence scores.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SafetyAttributes {
            /// The display names of Safety Attribute categories associated with the
            /// generated content. Order matches the Scores.
            #[prost(string, repeated, tag = "1")]
            pub categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// The confidence scores of the each category, higher
            /// value means higher confidence. Order matches the Categories.
            #[prost(float, repeated, tag = "2")]
            pub scores: ::prost::alloc::vec::Vec<f32>,
        }
        /// An Enum for summary-skipped reasons.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum SummarySkippedReason {
            /// Default value. The summary skipped reason is not specified.
            Unspecified = 0,
            /// The adversarial query ignored case.
            ///
            /// Only populated when
            /// [SummarySpec.ignore_adversarial_query][google.cloud.discoveryengine.v1beta.SearchRequest.ContentSearchSpec.SummarySpec.ignore_adversarial_query]
            /// is set to `true`.
            AdversarialQueryIgnored = 1,
            /// The non-summary seeking query ignored case.
            ///
            /// Only populated when
            /// [SummarySpec.ignore_non_summary_seeking_query][google.cloud.discoveryengine.v1beta.SearchRequest.ContentSearchSpec.SummarySpec.ignore_non_summary_seeking_query]
            /// is set to `true`.
            NonSummarySeekingQueryIgnored = 2,
            /// The out-of-domain query ignored case.
            ///
            /// Google skips the summary if there are no high-relevance search results.
            /// For example, the data store contains facts about company A but the
            /// user query is asking questions about company B.
            OutOfDomainQueryIgnored = 3,
            /// The potential policy violation case.
            ///
            /// Google skips the summary if there is a potential policy violation
            /// detected. This includes content that may be violent or toxic.
            PotentialPolicyViolation = 4,
            /// The LLM addon not enabled case.
            ///
            /// Google skips the summary if the LLM addon is not enabled.
            LlmAddonNotEnabled = 5,
        }
        impl SummarySkippedReason {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    SummarySkippedReason::Unspecified => {
                        "SUMMARY_SKIPPED_REASON_UNSPECIFIED"
                    }
                    SummarySkippedReason::AdversarialQueryIgnored => {
                        "ADVERSARIAL_QUERY_IGNORED"
                    }
                    SummarySkippedReason::NonSummarySeekingQueryIgnored => {
                        "NON_SUMMARY_SEEKING_QUERY_IGNORED"
                    }
                    SummarySkippedReason::OutOfDomainQueryIgnored => {
                        "OUT_OF_DOMAIN_QUERY_IGNORED"
                    }
                    SummarySkippedReason::PotentialPolicyViolation => {
                        "POTENTIAL_POLICY_VIOLATION"
                    }
                    SummarySkippedReason::LlmAddonNotEnabled => "LLM_ADDON_NOT_ENABLED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "SUMMARY_SKIPPED_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                    "ADVERSARIAL_QUERY_IGNORED" => Some(Self::AdversarialQueryIgnored),
                    "NON_SUMMARY_SEEKING_QUERY_IGNORED" => {
                        Some(Self::NonSummarySeekingQueryIgnored)
                    }
                    "OUT_OF_DOMAIN_QUERY_IGNORED" => Some(Self::OutOfDomainQueryIgnored),
                    "POTENTIAL_POLICY_VIOLATION" => Some(Self::PotentialPolicyViolation),
                    "LLM_ADDON_NOT_ENABLED" => Some(Self::LlmAddonNotEnabled),
                    _ => None,
                }
            }
        }
    }
    /// Information describing query expansion including whether expansion has
    /// occurred.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QueryExpansionInfo {
        /// Bool describing whether query expansion has occurred.
        #[prost(bool, tag = "1")]
        pub expanded_query: bool,
        /// Number of pinned results. This field will only be set when expansion
        /// happens and
        /// [SearchRequest.QueryExpansionSpec.pin_unexpanded_results][google.cloud.discoveryengine.v1beta.SearchRequest.QueryExpansionSpec.pin_unexpanded_results]
        /// is set to true.
        #[prost(int64, tag = "2")]
        pub pinned_result_count: i64,
    }
}
/// Generated client implementations.
pub mod search_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for search.
    #[derive(Debug, Clone)]
    pub struct SearchServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SearchServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SearchServiceClient<T>
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
        ) -> SearchServiceClient<InterceptedService<T, F>>
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
            SearchServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Performs a search.
        pub async fn search(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchRequest>,
        ) -> std::result::Result<tonic::Response<super::SearchResponse>, tonic::Status> {
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
                "/google.cloud.discoveryengine.v1beta.SearchService/Search",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.SearchService",
                        "Search",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// External conversation proto definition.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Conversation {
    /// Immutable. Fully qualified name
    /// `project/*/locations/global/collections/{collection}/dataStore/*/conversations/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The state of the Conversation.
    #[prost(enumeration = "conversation::State", tag = "2")]
    pub state: i32,
    /// A unique identifier for tracking users.
    #[prost(string, tag = "3")]
    pub user_pseudo_id: ::prost::alloc::string::String,
    /// Conversation messages.
    #[prost(message, repeated, tag = "4")]
    pub messages: ::prost::alloc::vec::Vec<ConversationMessage>,
    /// Output only. The time the conversation started.
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the conversation finished.
    #[prost(message, optional, tag = "6")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Conversation`.
pub mod conversation {
    /// Enumeration of the state of the conversation.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// Unknown.
        Unspecified = 0,
        /// Conversation is currently open.
        InProgress = 1,
        /// Conversation has been completed.
        Completed = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::InProgress => "IN_PROGRESS",
                State::Completed => "COMPLETED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "IN_PROGRESS" => Some(Self::InProgress),
                "COMPLETED" => Some(Self::Completed),
                _ => None,
            }
        }
    }
}
/// Defines a reply message to user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reply {
    /// DEPRECATED: use `summary` instead.
    /// Text reply.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub reply: ::prost::alloc::string::String,
    /// References in the reply.
    #[deprecated]
    #[prost(message, repeated, tag = "2")]
    pub references: ::prost::alloc::vec::Vec<reply::Reference>,
    /// Summary based on search results.
    #[prost(message, optional, tag = "3")]
    pub summary: ::core::option::Option<search_response::Summary>,
}
/// Nested message and enum types in `Reply`.
pub mod reply {
    /// Defines reference in reply.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Reference {
        /// URI link reference.
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
        /// Anchor text.
        #[prost(string, tag = "2")]
        pub anchor_text: ::prost::alloc::string::String,
        /// Anchor text start index.
        #[prost(int32, tag = "3")]
        pub start: i32,
        /// Anchor text end index.
        #[prost(int32, tag = "4")]
        pub end: i32,
    }
}
/// Defines context of the conversation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversationContext {
    /// The current list of documents the user is seeing.
    /// It contains the document resource references.
    #[prost(string, repeated, tag = "1")]
    pub context_documents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The current active document the user opened.
    /// It contains the document resource reference.
    #[prost(string, tag = "2")]
    pub active_document: ::prost::alloc::string::String,
}
/// Defines text input.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextInput {
    /// Text input.
    #[prost(string, tag = "1")]
    pub input: ::prost::alloc::string::String,
    /// Conversation context of the input.
    #[prost(message, optional, tag = "2")]
    pub context: ::core::option::Option<ConversationContext>,
}
/// Defines a conversation message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversationMessage {
    /// Output only. Message creation timestamp.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof = "conversation_message::Message", tags = "1, 2")]
    pub message: ::core::option::Option<conversation_message::Message>,
}
/// Nested message and enum types in `ConversationMessage`.
pub mod conversation_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// User text input.
        #[prost(message, tag = "1")]
        UserInput(super::TextInput),
        /// Search reply.
        #[prost(message, tag = "2")]
        Reply(super::Reply),
    }
}
/// Request message for
/// [ConversationalSearchService.ConverseConversation][google.cloud.discoveryengine.v1beta.ConversationalSearchService.ConverseConversation]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConverseConversationRequest {
    /// Required. The resource name of the Conversation to get. Format:
    /// `projects/{project_number}/locations/{location_id}/collections/{collection}/dataStores/{data_store_id}/conversations/{conversation_id}`.
    /// Use
    /// `projects/{project_number}/locations/{location_id}/collections/{collection}/dataStores/{data_store_id}/conversations/-`
    /// to activate auto session mode, which automatically creates a new
    /// conversation inside a ConverseConversation session.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Current user input.
    #[prost(message, optional, tag = "2")]
    pub query: ::core::option::Option<TextInput>,
    /// The resource name of the Serving Config to use. Format:
    /// `projects/{project_number}/locations/{location_id}/collections/{collection}/dataStores/{data_store_id}/servingConfigs/{serving_config_id}`
    /// If this is not set, the default serving config will be used.
    #[prost(string, tag = "3")]
    pub serving_config: ::prost::alloc::string::String,
    /// The conversation to be used by auto session only. The name field will be
    /// ignored as we automatically assign new name for the conversation in auto
    /// session.
    #[prost(message, optional, tag = "5")]
    pub conversation: ::core::option::Option<Conversation>,
    /// Whether to turn on safe search.
    #[prost(bool, tag = "6")]
    pub safe_search: bool,
    /// The user labels applied to a resource must meet the following requirements:
    ///
    /// * Each resource can have multiple labels, up to a maximum of 64.
    /// * Each label must be a key-value pair.
    /// * Keys have a minimum length of 1 character and a maximum length of 63
    ///    characters and cannot be empty. Values can be empty and have a maximum
    ///    length of 63 characters.
    /// * Keys and values can contain only lowercase letters, numeric characters,
    ///    underscores, and dashes. All characters must use UTF-8 encoding, and
    ///    international characters are allowed.
    /// * The key portion of a label must be unique. However, you can use the same
    ///    key with multiple resources.
    /// * Keys must start with a lowercase letter or international character.
    ///
    /// See [Google Cloud
    /// Document](<https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements>)
    /// for more details.
    #[prost(map = "string, string", tag = "7")]
    pub user_labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// A specification for configuring the summary returned in the response.
    #[prost(message, optional, tag = "8")]
    pub summary_spec: ::core::option::Option<
        search_request::content_search_spec::SummarySpec,
    >,
}
/// Response message for
/// [ConversationalSearchService.ConverseConversation][google.cloud.discoveryengine.v1beta.ConversationalSearchService.ConverseConversation]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConverseConversationResponse {
    /// Answer to the current query.
    #[prost(message, optional, tag = "1")]
    pub reply: ::core::option::Option<Reply>,
    /// Updated conversation including the answer.
    #[prost(message, optional, tag = "2")]
    pub conversation: ::core::option::Option<Conversation>,
    /// Suggested related questions.
    #[prost(string, repeated, tag = "6")]
    pub related_questions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Search Results.
    #[prost(message, repeated, tag = "3")]
    pub search_results: ::prost::alloc::vec::Vec<search_response::SearchResult>,
}
/// Request for CreateConversation method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConversationRequest {
    /// Required. Full resource name of parent data store. Format:
    /// `projects/{project_number}/locations/{location_id}/collections/{collection}/dataStores/{data_store_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The conversation to create.
    #[prost(message, optional, tag = "2")]
    pub conversation: ::core::option::Option<Conversation>,
}
/// Request for UpdateConversation method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConversationRequest {
    /// Required. The Conversation to update.
    #[prost(message, optional, tag = "1")]
    pub conversation: ::core::option::Option<Conversation>,
    /// Indicates which fields in the provided
    /// [Conversation][google.cloud.discoveryengine.v1beta.Conversation] to update.
    /// The following are NOT supported:
    ///
    /// * [conversation.name][]
    ///
    /// If not set or empty, all supported fields are updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for DeleteConversation method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConversationRequest {
    /// Required. The resource name of the Conversation to delete. Format:
    /// `projects/{project_number}/locations/{location_id}/collections/{collection}/dataStores/{data_store_id}/conversations/{conversation_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for GetConversation method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConversationRequest {
    /// Required. The resource name of the Conversation to get. Format:
    /// `projects/{project_number}/locations/{location_id}/collections/{collection}/dataStores/{data_store_id}/conversations/{conversation_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for ListConversations method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConversationsRequest {
    /// Required. The data store resource name. Format:
    /// `projects/{project_number}/locations/{location_id}/collections/{collection}/dataStores/{data_store_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of results to return. If unspecified, defaults
    /// to 50. Max allowed value is 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListConversations` call.
    /// Provide this to retrieve the subsequent page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter to apply on the list results. The supported features are:
    /// user_pseudo_id, state.
    ///
    /// Example:
    /// "user_pseudo_id = some_id"
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// A comma-separated list of fields to order by, sorted in ascending order.
    /// Use "desc" after a field name for descending.
    /// Supported fields:
    ///    * `update_time`
    ///    * `create_time`
    ///    * `conversation_name`
    ///
    /// Example:
    /// "update_time desc"
    /// "create_time"
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for ListConversations method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConversationsResponse {
    /// All the Conversations for a given data store.
    #[prost(message, repeated, tag = "1")]
    pub conversations: ::prost::alloc::vec::Vec<Conversation>,
    /// Pagination token, if not returned indicates the last page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod conversational_search_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for conversational search.
    #[derive(Debug, Clone)]
    pub struct ConversationalSearchServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ConversationalSearchServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ConversationalSearchServiceClient<T>
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
        ) -> ConversationalSearchServiceClient<InterceptedService<T, F>>
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
            ConversationalSearchServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Converses a conversation.
        pub async fn converse_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::ConverseConversationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConverseConversationResponse>,
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
                "/google.cloud.discoveryengine.v1beta.ConversationalSearchService/ConverseConversation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.ConversationalSearchService",
                        "ConverseConversation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a Conversation.
        ///
        /// If the [Conversation][google.cloud.discoveryengine.v1beta.Conversation] to
        /// create already exists, an ALREADY_EXISTS error is returned.
        pub async fn create_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConversationRequest>,
        ) -> std::result::Result<tonic::Response<super::Conversation>, tonic::Status> {
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
                "/google.cloud.discoveryengine.v1beta.ConversationalSearchService/CreateConversation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.ConversationalSearchService",
                        "CreateConversation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a Conversation.
        ///
        /// If the [Conversation][google.cloud.discoveryengine.v1beta.Conversation] to
        /// delete does not exist, a NOT_FOUND error is returned.
        pub async fn delete_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConversationRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.discoveryengine.v1beta.ConversationalSearchService/DeleteConversation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.ConversationalSearchService",
                        "DeleteConversation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a Conversation.
        ///
        /// [Conversation][google.cloud.discoveryengine.v1beta.Conversation] action
        /// type cannot be changed. If the
        /// [Conversation][google.cloud.discoveryengine.v1beta.Conversation] to update
        /// does not exist, a NOT_FOUND error is returned.
        pub async fn update_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConversationRequest>,
        ) -> std::result::Result<tonic::Response<super::Conversation>, tonic::Status> {
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
                "/google.cloud.discoveryengine.v1beta.ConversationalSearchService/UpdateConversation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.ConversationalSearchService",
                        "UpdateConversation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a Conversation.
        pub async fn get_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConversationRequest>,
        ) -> std::result::Result<tonic::Response<super::Conversation>, tonic::Status> {
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
                "/google.cloud.discoveryengine.v1beta.ConversationalSearchService/GetConversation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.ConversationalSearchService",
                        "GetConversation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all Conversations by their parent
        /// [DataStore][google.cloud.discoveryengine.v1beta.DataStore].
        pub async fn list_conversations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConversationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConversationsResponse>,
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
                "/google.cloud.discoveryengine.v1beta.ConversationalSearchService/ListConversations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.ConversationalSearchService",
                        "ListConversations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// UserEvent captures all metadata information Discovery Engine API needs to
/// know about how end users interact with customers' website.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEvent {
    /// Required. User event type. Allowed values are:
    ///
    /// Generic values:
    ///
    /// * `search`: Search for Documents.
    /// * `view-item`: Detailed page view of a Document.
    /// * `view-item-list`: View of a panel or ordered list of Documents.
    /// * `view-home-page`: View of the home page.
    /// * `view-category-page`: View of a category page, e.g. Home > Men > Jeans
    ///
    /// Retail-related values:
    ///
    /// * `add-to-cart`: Add an item(s) to cart, e.g. in Retail online shopping
    /// * `purchase`: Purchase an item(s)
    ///
    /// Media-related values:
    ///
    /// * `media-play`: Start/resume watching a video, playing a song, etc.
    /// * `media-complete`: Finished or stopped midway through a video, song, etc.
    #[prost(string, tag = "1")]
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
    /// characters. Otherwise, an `INVALID_ARGUMENT` error is returned.
    ///
    /// The field should not contain PII or user-data. We recommend to use Google
    /// Analytics [Client
    /// ID](<https://developers.google.com/analytics/devguides/collection/analyticsjs/field-reference#clientId>)
    /// for this field.
    #[prost(string, tag = "2")]
    pub user_pseudo_id: ::prost::alloc::string::String,
    /// Only required for
    /// [UserEventService.ImportUserEvents][google.cloud.discoveryengine.v1beta.UserEventService.ImportUserEvents]
    /// method. Timestamp of when the user event happened.
    #[prost(message, optional, tag = "3")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Information about the end user.
    #[prost(message, optional, tag = "4")]
    pub user_info: ::core::option::Option<UserInfo>,
    /// Should set to true if the request is made directly from the end user, in
    /// which case the
    /// [UserEvent.user_info.user_agent][google.cloud.discoveryengine.v1beta.UserInfo.user_agent]
    /// can be populated from the HTTP request.
    ///
    /// This flag should be set only if the API request is made directly from the
    /// end user such as a mobile app (and not if a gateway or a server is
    /// processing and pushing the user events).
    ///
    /// This should not be set when using the JavaScript tag in
    /// [UserEventService.CollectUserEvent][google.cloud.discoveryengine.v1beta.UserEventService.CollectUserEvent].
    #[prost(bool, tag = "5")]
    pub direct_user_request: bool,
    /// A unique identifier for tracking a visitor session with a length limit of
    /// 128 bytes. A session is an aggregation of an end user behavior in a time
    /// span.
    ///
    /// A general guideline to populate the session_id:
    ///
    /// 1. If user has no activity for 30 min, a new session_id should be assigned.
    /// 2. The session_id should be unique across users, suggest use uuid or add
    /// [UserEvent.user_pseudo_id][google.cloud.discoveryengine.v1beta.UserEvent.user_pseudo_id]
    /// as prefix.
    #[prost(string, tag = "6")]
    pub session_id: ::prost::alloc::string::String,
    /// Page metadata such as categories and other critical information for certain
    /// event types such as `view-category-page`.
    #[prost(message, optional, tag = "7")]
    pub page_info: ::core::option::Option<PageInfo>,
    /// Token to attribute an API response to user action(s) to trigger the event.
    ///
    /// Highly recommended for user events that are the result of
    /// [RecommendationService.Recommend][google.cloud.discoveryengine.v1beta.RecommendationService.Recommend].
    /// This field enables accurate attribution of recommendation model
    /// performance.
    ///
    /// The value must be one of:
    ///
    /// * [RecommendResponse.attribution_token][google.cloud.discoveryengine.v1beta.RecommendResponse.attribution_token] for events that are the result of
    /// [RecommendationService.Recommend][google.cloud.discoveryengine.v1beta.RecommendationService.Recommend].
    /// * [SearchResponse.attribution_token][google.cloud.discoveryengine.v1beta.SearchResponse.attribution_token] for events that are the result of
    /// [SearchService.Search][google.cloud.discoveryengine.v1beta.SearchService.Search].
    ///
    /// This token enables us to accurately attribute page view or conversion
    /// completion back to the event and the particular predict response containing
    /// this clicked/purchased product. If user clicks on product K in the
    /// recommendation results, pass
    /// [RecommendResponse.attribution_token][google.cloud.discoveryengine.v1beta.RecommendResponse.attribution_token]
    /// as a URL parameter to product K's page. When recording events on product
    /// K's page, log the
    /// [RecommendResponse.attribution_token][google.cloud.discoveryengine.v1beta.RecommendResponse.attribution_token]
    /// to this field.
    #[prost(string, tag = "8")]
    pub attribution_token: ::prost::alloc::string::String,
    /// The filter syntax consists of an expression language for constructing a
    /// predicate from one or more fields of the documents being filtered.
    ///
    /// One example is for `search` events, the associated
    /// [SearchRequest][google.cloud.discoveryengine.v1beta.SearchRequest] may
    /// contain a filter expression in
    /// [SearchRequest.filter][google.cloud.discoveryengine.v1beta.SearchRequest.filter]
    /// conforming to <https://google.aip.dev/160#filtering.>
    ///
    /// Similarly, for `view-item-list` events that are generated from a
    /// [RecommendationService.RecommendRequest][], this field may be populated
    /// directly from [RecommendationService.RecommendRequest.filter][] conforming
    /// to <https://google.aip.dev/160#filtering.>
    ///
    /// The value must be a UTF-8 encoded string with a length limit of 1,000
    /// characters. Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "9")]
    pub filter: ::prost::alloc::string::String,
    /// List of [Document][google.cloud.discoveryengine.v1beta.Document]s
    /// associated with this user event.
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
    /// [UserEvent.documents][google.cloud.discoveryengine.v1beta.UserEvent.documents]
    /// is desired.
    #[prost(message, repeated, tag = "10")]
    pub documents: ::prost::alloc::vec::Vec<DocumentInfo>,
    /// Panel metadata associated with this user event.
    #[prost(message, optional, tag = "11")]
    pub panel: ::core::option::Option<PanelInfo>,
    /// [SearchService.Search][google.cloud.discoveryengine.v1beta.SearchService.Search]
    /// details related to the event.
    ///
    /// This field should be set for `search` event.
    #[prost(message, optional, tag = "12")]
    pub search_info: ::core::option::Option<SearchInfo>,
    /// [CompletionService.CompleteQuery][google.cloud.discoveryengine.v1beta.CompletionService.CompleteQuery]
    /// details related to the event.
    ///
    /// This field should be set for `search` event when autocomplete function is
    /// enabled and the user clicks a suggestion for search.
    #[prost(message, optional, tag = "13")]
    pub completion_info: ::core::option::Option<CompletionInfo>,
    /// The transaction metadata (if any) associated with this user event.
    #[prost(message, optional, tag = "14")]
    pub transaction_info: ::core::option::Option<TransactionInfo>,
    /// A list of identifiers for the independent experiment groups this user event
    /// belongs to. This is used to distinguish between user events associated with
    /// different experiment setups on the customer end.
    #[prost(string, repeated, tag = "15")]
    pub tag_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The promotion IDs if this is an event associated with promotions.
    /// Currently, this field is restricted to at most one ID.
    #[prost(string, repeated, tag = "16")]
    pub promotion_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Extra user event features to include in the recommendation model.
    /// These attributes must NOT contain data that needs to be parsed or processed
    /// further, e.g. JSON or other encodings.
    ///
    /// If you provide custom attributes for ingested user events, also include
    /// them in the user events that you associate with prediction requests. Custom
    /// attribute formatting must be consistent between imported events and events
    /// provided with prediction requests. This lets the Discovery Engine API use
    /// those custom attributes when training models and serving predictions, which
    /// helps improve recommendation quality.
    ///
    /// This field needs to pass all below criteria, otherwise an
    /// `INVALID_ARGUMENT` error is returned:
    ///
    /// * The key must be a UTF-8 encoded string with a length limit of 5,000
    ///    characters.
    /// * For text attributes, at most 400 values are allowed. Empty values are not
    ///    allowed. Each value must be a UTF-8 encoded string with a length limit of
    ///    256 characters.
    /// * For number attributes, at most 400 values are allowed.
    ///
    /// For product recommendations, an example of extra user information is
    /// `traffic_channel`, which is how a user arrives at the site. Users can
    /// arrive
    /// at the site by coming to the site directly, coming through Google
    /// search, or in other ways.
    #[prost(map = "string, message", tag = "17")]
    pub attributes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        CustomAttribute,
    >,
    /// Media-specific info.
    #[prost(message, optional, tag = "18")]
    pub media_info: ::core::option::Option<MediaInfo>,
}
/// Detailed page information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageInfo {
    /// A unique ID of a web page view.
    ///
    /// This should be kept the same for all user events triggered from the same
    /// pageview. For example, an item detail page view could trigger multiple
    /// events as the user is browsing the page. The `pageview_id` property should
    /// be kept the same for all these events so that they can be grouped together
    /// properly.
    ///
    /// When using the client side event reporting with JavaScript pixel and Google
    /// Tag Manager, this value is filled in automatically.
    #[prost(string, tag = "1")]
    pub pageview_id: ::prost::alloc::string::String,
    /// The most specific category associated with a category page.
    ///
    /// To represent full path of category, use '>' sign to separate different
    /// hierarchies. If '>' is part of the category name, please replace it with
    /// other character(s).
    ///
    /// Category pages include special pages such as sales or promotions. For
    /// instance, a special sale page may have the category hierarchy:
    /// `"pageCategory" : "Sales > 2017 Black Friday Deals"`.
    ///
    /// Required for `view-category-page` events. Other event types should not set
    /// this field. Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "2")]
    pub page_category: ::prost::alloc::string::String,
    /// Complete URL (window.location.href) of the user's current page.
    ///
    /// When using the client side event reporting with JavaScript pixel and Google
    /// Tag Manager, this value is filled in automatically. Maximum length 5,000
    /// characters.
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// The referrer URL of the current page.
    ///
    /// When using the client side event reporting with JavaScript pixel and Google
    /// Tag Manager, this value is filled in automatically. However, some browser
    /// privacy restrictions may cause this field to be empty.
    #[prost(string, tag = "4")]
    pub referrer_uri: ::prost::alloc::string::String,
}
/// Detailed search information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchInfo {
    /// The user's search query.
    ///
    /// See
    /// [SearchRequest.query][google.cloud.discoveryengine.v1beta.SearchRequest.query]
    /// for definition.
    ///
    /// The value must be a UTF-8 encoded string with a length limit of 5,000
    /// characters. Otherwise, an `INVALID_ARGUMENT` error is returned.
    ///
    /// At least one of
    /// [search_query][google.cloud.discoveryengine.v1beta.SearchInfo.search_query]
    /// or
    /// [PageInfo.page_category][google.cloud.discoveryengine.v1beta.PageInfo.page_category]
    /// is required for `search` events. Other event types should not set this
    /// field. Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "1")]
    pub search_query: ::prost::alloc::string::String,
    /// The order in which products are returned, if applicable.
    ///
    /// See
    /// [SearchRequest.order_by][google.cloud.discoveryengine.v1beta.SearchRequest.order_by]
    /// for definition and syntax.
    ///
    /// The value must be a UTF-8 encoded string with a length limit of 1,000
    /// characters. Otherwise, an `INVALID_ARGUMENT` error is returned.
    ///
    /// This can only be set for `search` events. Other event types should not set
    /// this field. Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "2")]
    pub order_by: ::prost::alloc::string::String,
    /// An integer that specifies the current offset for pagination (the 0-indexed
    /// starting location, amongst the products deemed by the API as relevant).
    ///
    /// See
    /// [SearchRequest.offset][google.cloud.discoveryengine.v1beta.SearchRequest.offset]
    /// for definition.
    ///
    /// If this field is negative, an `INVALID_ARGUMENT` is returned.
    ///
    /// This can only be set for `search` events. Other event types should not set
    /// this field. Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(int32, optional, tag = "3")]
    pub offset: ::core::option::Option<i32>,
}
/// Detailed completion information including completion attribution token and
/// clicked completion info.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletionInfo {
    /// End user selected
    /// [CompleteQueryResponse.QuerySuggestion.suggestion][google.cloud.discoveryengine.v1beta.CompleteQueryResponse.QuerySuggestion.suggestion].
    #[prost(string, tag = "1")]
    pub selected_suggestion: ::prost::alloc::string::String,
    /// End user selected
    /// [CompleteQueryResponse.QuerySuggestion.suggestion][google.cloud.discoveryengine.v1beta.CompleteQueryResponse.QuerySuggestion.suggestion]
    /// position, starting from 0.
    #[prost(int32, tag = "2")]
    pub selected_position: i32,
}
/// A transaction represents the entire purchase transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfo {
    /// Required. Total non-zero value associated with the transaction. This value
    /// may include shipping, tax, or other adjustments to the total value that you
    /// want to include.
    #[prost(float, optional, tag = "1")]
    pub value: ::core::option::Option<f32>,
    /// Required. Currency code. Use three-character ISO-4217 code.
    #[prost(string, tag = "2")]
    pub currency: ::prost::alloc::string::String,
    /// The transaction ID with a length limit of 128 characters.
    #[prost(string, tag = "3")]
    pub transaction_id: ::prost::alloc::string::String,
    /// All the taxes associated with the transaction.
    #[prost(float, optional, tag = "4")]
    pub tax: ::core::option::Option<f32>,
    /// All the costs associated with the products. These can be manufacturing
    /// costs, shipping expenses not borne by the end user, or any other costs,
    /// such that:
    ///
    /// * Profit =
    /// [value][google.cloud.discoveryengine.v1beta.TransactionInfo.value] -
    /// [tax][google.cloud.discoveryengine.v1beta.TransactionInfo.tax] -
    /// [cost][google.cloud.discoveryengine.v1beta.TransactionInfo.cost]
    #[prost(float, optional, tag = "5")]
    pub cost: ::core::option::Option<f32>,
    /// The total discount(s) value applied to this transaction.
    /// This figure should be excluded from
    /// [TransactionInfo.value][google.cloud.discoveryengine.v1beta.TransactionInfo.value]
    ///
    /// For example, if a user paid
    /// [TransactionInfo.value][google.cloud.discoveryengine.v1beta.TransactionInfo.value]
    /// amount, then nominal (pre-discount) value of the transaction is the sum of
    /// [TransactionInfo.value][google.cloud.discoveryengine.v1beta.TransactionInfo.value]
    /// and
    /// [TransactionInfo.discount_value][google.cloud.discoveryengine.v1beta.TransactionInfo.discount_value]
    ///
    /// This means that profit is calculated the same way, regardless of the
    /// discount value, and that
    /// [TransactionInfo.discount_value][google.cloud.discoveryengine.v1beta.TransactionInfo.discount_value]
    /// can be larger than
    /// [TransactionInfo.value][google.cloud.discoveryengine.v1beta.TransactionInfo.value]:
    ///
    /// * Profit =
    /// [value][google.cloud.discoveryengine.v1beta.TransactionInfo.value] -
    /// [tax][google.cloud.discoveryengine.v1beta.TransactionInfo.tax] -
    /// [cost][google.cloud.discoveryengine.v1beta.TransactionInfo.cost]
    #[prost(float, optional, tag = "6")]
    pub discount_value: ::core::option::Option<f32>,
}
/// Detailed document information associated with a user event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentInfo {
    /// Quantity of the Document associated with the user event. Defaults to 1.
    ///
    /// For example, this field will be 2 if two quantities of the same Document
    /// are involved in a `add-to-cart` event.
    ///
    /// Required for events of the following event types:
    ///
    /// * `add-to-cart`
    /// * `purchase`
    #[prost(int32, optional, tag = "3")]
    pub quantity: ::core::option::Option<i32>,
    /// The promotion IDs associated with this Document.
    /// Currently, this field is restricted to at most one ID.
    #[prost(string, repeated, tag = "4")]
    pub promotion_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A required descriptor of the associated
    /// [Document][google.cloud.discoveryengine.v1beta.Document].
    ///
    /// * If [id][google.cloud.discoveryengine.v1beta.DocumentInfo.id] is
    /// specified, then the default values for
    /// `{location}`, `{collection_id}`, `{data_store_id}`, and `{branch_id}` are
    /// used when annotating with the stored Document.
    ///
    /// * If [name][google.cloud.discoveryengine.v1beta.DocumentInfo.name] is
    /// specified, then the provided values (default values allowed) for
    /// `{location}`, `{collection_id}`, `{data_store_id}`, and
    /// `{branch_id}` are used when annotating with the stored Document.
    #[prost(oneof = "document_info::DocumentDescriptor", tags = "1, 2, 6")]
    pub document_descriptor: ::core::option::Option<document_info::DocumentDescriptor>,
}
/// Nested message and enum types in `DocumentInfo`.
pub mod document_info {
    /// A required descriptor of the associated
    /// [Document][google.cloud.discoveryengine.v1beta.Document].
    ///
    /// * If [id][google.cloud.discoveryengine.v1beta.DocumentInfo.id] is
    /// specified, then the default values for
    /// `{location}`, `{collection_id}`, `{data_store_id}`, and `{branch_id}` are
    /// used when annotating with the stored Document.
    ///
    /// * If [name][google.cloud.discoveryengine.v1beta.DocumentInfo.name] is
    /// specified, then the provided values (default values allowed) for
    /// `{location}`, `{collection_id}`, `{data_store_id}`, and
    /// `{branch_id}` are used when annotating with the stored Document.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DocumentDescriptor {
        /// The [Document][google.cloud.discoveryengine.v1beta.Document] resource ID.
        #[prost(string, tag = "1")]
        Id(::prost::alloc::string::String),
        /// The [Document][google.cloud.discoveryengine.v1beta.Document] resource
        /// full name, of the form:
        /// `projects/{project_id}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/branches/{branch_id}/documents/{document_id}`
        #[prost(string, tag = "2")]
        Name(::prost::alloc::string::String),
        /// The [Document][google.cloud.discoveryengine.v1beta.Document] URI - only
        /// allowed for website data stores.
        #[prost(string, tag = "6")]
        Uri(::prost::alloc::string::String),
    }
}
/// Detailed panel information associated with a user event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PanelInfo {
    /// Required. The panel ID.
    #[prost(string, tag = "2")]
    pub panel_id: ::prost::alloc::string::String,
    /// The display name of the panel.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// The ordered position of the panel, if shown to the user with other panels.
    /// If set, then
    /// [total_panels][google.cloud.discoveryengine.v1beta.PanelInfo.total_panels]
    /// must also be set.
    #[prost(int32, optional, tag = "4")]
    pub panel_position: ::core::option::Option<i32>,
    /// The total number of panels, including this one, shown to the user.
    /// Must be set if
    /// [panel_position][google.cloud.discoveryengine.v1beta.PanelInfo.panel_position]
    /// is set.
    #[prost(int32, optional, tag = "5")]
    pub total_panels: ::core::option::Option<i32>,
}
/// Media-specific user event information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaInfo {
    /// The media progress time in seconds, if applicable.
    /// For example, if the end user has finished 90 seconds of a playback video,
    /// then
    /// [MediaInfo.media_progress_duration.seconds][google.protobuf.Duration.seconds]
    /// should be set to 90.
    #[prost(message, optional, tag = "1")]
    pub media_progress_duration: ::core::option::Option<::prost_types::Duration>,
    /// Media progress should be computed using only the
    /// [media_progress_duration][google.cloud.discoveryengine.v1beta.MediaInfo.media_progress_duration]
    /// relative to the media total length.
    ///
    /// This value must be between `\[0, 1.0\]` inclusive.
    ///
    /// If this is not a playback or the progress cannot be computed (e.g. ongoing
    /// livestream), this field should be unset.
    #[prost(float, optional, tag = "2")]
    pub media_progress_percentage: ::core::option::Option<f32>,
}
/// Cloud Storage location for input content.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    /// Required. Cloud Storage URIs to input files. URI can be up to
    /// 2000 characters long. URIs can match the full object path (for example,
    /// `gs://bucket/directory/object.json`) or a pattern matching one or more
    /// files, such as `gs://bucket/directory/*.json`.
    ///
    /// A request can contain at most 100 files (or 100,000 files if `data_schema`
    /// is `content`). Each file can be up to 2 GB (or 100 MB if `data_schema` is
    /// `content`).
    #[prost(string, repeated, tag = "1")]
    pub input_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The schema to use when parsing the data from the source.
    ///
    /// Supported values for document imports:
    ///
    /// * `document` (default): One JSON
    /// [Document][google.cloud.discoveryengine.v1beta.Document] per line. Each
    /// document must
    ///    have a valid
    ///    [Document.id][google.cloud.discoveryengine.v1beta.Document.id].
    /// * `content`: Unstructured data (e.g. PDF, HTML). Each file matched by
    ///    `input_uris` becomes a document, with the ID set to the first 128
    ///    bits of SHA256(URI) encoded as a hex string.
    /// * `custom`: One custom data JSON per row in arbitrary format that conforms
    ///    to the defined [Schema][google.cloud.discoveryengine.v1beta.Schema] of
    ///    the data store. This can only be used by Gen App Builder.
    /// * `csv`: A CSV file with header conforming to the defined
    /// [Schema][google.cloud.discoveryengine.v1beta.Schema] of the
    ///    data store. Each entry after the header is imported as a Document.
    ///    This can only be used by Gen App Builder.
    ///
    /// Supported values for user even imports:
    ///
    /// * `user_event` (default): One JSON
    /// [UserEvent][google.cloud.discoveryengine.v1beta.UserEvent] per line.
    #[prost(string, tag = "2")]
    pub data_schema: ::prost::alloc::string::String,
}
/// BigQuery source import data from.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQuerySource {
    /// The project ID (can be project # or ID) that the BigQuery source is in with
    /// a length limit of 128 characters. If not specified, inherits the project
    /// ID from the parent request.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The BigQuery data set to copy the data from with a length limit
    /// of 1,024 characters.
    #[prost(string, tag = "2")]
    pub dataset_id: ::prost::alloc::string::String,
    /// Required. The BigQuery table to copy the data from with a length limit of
    /// 1,024 characters.
    #[prost(string, tag = "3")]
    pub table_id: ::prost::alloc::string::String,
    /// Intermediate Cloud Storage directory used for the import with a length
    /// limit of 2,000 characters. Can be specified if one wants to have the
    /// BigQuery export to a specific Cloud Storage directory.
    #[prost(string, tag = "4")]
    pub gcs_staging_dir: ::prost::alloc::string::String,
    /// The schema to use when parsing the data from the source.
    ///
    /// Supported values for user event imports:
    ///
    /// * `user_event` (default): One
    /// [UserEvent][google.cloud.discoveryengine.v1beta.UserEvent] per row.
    ///
    /// Supported values for document imports:
    ///
    /// * `document` (default): One
    /// [Document][google.cloud.discoveryengine.v1beta.Document] format per
    ///    row. Each document must have a valid
    ///    [Document.id][google.cloud.discoveryengine.v1beta.Document.id] and one of
    ///    [Document.json_data][google.cloud.discoveryengine.v1beta.Document.json_data]
    ///    or
    ///    [Document.struct_data][google.cloud.discoveryengine.v1beta.Document.struct_data].
    /// * `custom`: One custom data per row in arbitrary format that conforms to
    ///    the defined [Schema][google.cloud.discoveryengine.v1beta.Schema] of the
    ///    data store. This can only be used by Gen App Builder.
    #[prost(string, tag = "6")]
    pub data_schema: ::prost::alloc::string::String,
    /// BigQuery table partition info. Leave this empty if the BigQuery table
    /// is not partitioned.
    #[prost(oneof = "big_query_source::Partition", tags = "5")]
    pub partition: ::core::option::Option<big_query_source::Partition>,
}
/// Nested message and enum types in `BigQuerySource`.
pub mod big_query_source {
    /// BigQuery table partition info. Leave this empty if the BigQuery table
    /// is not partitioned.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Partition {
        /// BigQuery time partitioned table's _PARTITIONDATE in YYYY-MM-DD format.
        #[prost(message, tag = "5")]
        PartitionDate(super::super::super::super::r#type::Date),
    }
}
/// Configuration of destination for Import related errors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportErrorConfig {
    /// Required. Errors destination.
    #[prost(oneof = "import_error_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<import_error_config::Destination>,
}
/// Nested message and enum types in `ImportErrorConfig`.
pub mod import_error_config {
    /// Required. Errors destination.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Cloud Storage prefix for import errors. This must be an empty,
        /// existing Cloud Storage directory. Import errors are written to
        /// sharded files in this directory, one per line, as a JSON-encoded
        /// `google.rpc.Status` message.
        #[prost(string, tag = "1")]
        GcsPrefix(::prost::alloc::string::String),
    }
}
/// Request message for the ImportUserEvents request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportUserEventsRequest {
    /// Required. Parent DataStore resource name, of the form
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The desired location of errors incurred during the Import. Cannot be set
    /// for inline user event imports.
    #[prost(message, optional, tag = "5")]
    pub error_config: ::core::option::Option<ImportErrorConfig>,
    /// Required - The desired input source of the user event data.
    #[prost(oneof = "import_user_events_request::Source", tags = "2, 3, 4")]
    pub source: ::core::option::Option<import_user_events_request::Source>,
}
/// Nested message and enum types in `ImportUserEventsRequest`.
pub mod import_user_events_request {
    /// The inline source for the input config for ImportUserEvents method.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InlineSource {
        /// Required. A list of user events to import. Recommended max of 10k items.
        #[prost(message, repeated, tag = "1")]
        pub user_events: ::prost::alloc::vec::Vec<super::UserEvent>,
    }
    /// Required - The desired input source of the user event data.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The Inline source for the input content for UserEvents.
        #[prost(message, tag = "2")]
        InlineSource(InlineSource),
        /// Cloud Storage location for the input content.
        #[prost(message, tag = "3")]
        GcsSource(super::GcsSource),
        /// BigQuery input source.
        #[prost(message, tag = "4")]
        BigquerySource(super::BigQuerySource),
    }
}
/// Response of the ImportUserEventsRequest. If the long running
/// operation was successful, then this message is returned by the
/// google.longrunning.Operations.response field if the operation was successful.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportUserEventsResponse {
    /// A sample of errors encountered while processing the request.
    #[prost(message, repeated, tag = "1")]
    pub error_samples: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Echoes the destination for the complete errors if this field was set in
    /// the request.
    #[prost(message, optional, tag = "2")]
    pub error_config: ::core::option::Option<ImportErrorConfig>,
    /// Count of user events imported with complete existing Documents.
    #[prost(int64, tag = "3")]
    pub joined_events_count: i64,
    /// Count of user events imported, but with Document information not found
    /// in the existing Branch.
    #[prost(int64, tag = "4")]
    pub unjoined_events_count: i64,
}
/// Metadata related to the progress of the Import operation. This is
/// returned by the google.longrunning.Operation.metadata field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportUserEventsMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Count of entries that were processed successfully.
    #[prost(int64, tag = "3")]
    pub success_count: i64,
    /// Count of entries that encountered errors while processing.
    #[prost(int64, tag = "4")]
    pub failure_count: i64,
}
/// Metadata related to the progress of the ImportDocuments operation. This is
/// returned by the google.longrunning.Operation.metadata field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDocumentsMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Count of entries that were processed successfully.
    #[prost(int64, tag = "3")]
    pub success_count: i64,
    /// Count of entries that encountered errors while processing.
    #[prost(int64, tag = "4")]
    pub failure_count: i64,
}
/// Request message for Import methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDocumentsRequest {
    /// Required. The parent branch resource name, such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}`.
    /// Requires create/update permission.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The desired location of errors incurred during the Import.
    #[prost(message, optional, tag = "5")]
    pub error_config: ::core::option::Option<ImportErrorConfig>,
    /// The mode of reconciliation between existing documents and the documents to
    /// be imported. Defaults to
    /// [ReconciliationMode.INCREMENTAL][google.cloud.discoveryengine.v1beta.ImportDocumentsRequest.ReconciliationMode.INCREMENTAL].
    #[prost(enumeration = "import_documents_request::ReconciliationMode", tag = "6")]
    pub reconciliation_mode: i32,
    /// Whether to automatically generate IDs for the documents if absent.
    ///
    /// If set to `true`,
    /// [Document.id][google.cloud.discoveryengine.v1beta.Document.id]s are
    /// automatically generated based on the hash of the payload, where IDs may not
    /// be consistent during multiple imports. In which case
    /// [ReconciliationMode.FULL][google.cloud.discoveryengine.v1beta.ImportDocumentsRequest.ReconciliationMode.FULL]
    /// is highly recommended to avoid duplicate contents. If unset or set to
    /// `false`, [Document.id][google.cloud.discoveryengine.v1beta.Document.id]s
    /// have to be specified using
    /// [id_field][google.cloud.discoveryengine.v1beta.ImportDocumentsRequest.id_field],
    /// otherwise, documents without IDs fail to be imported.
    ///
    /// Only set this field when using
    /// [GcsSource][google.cloud.discoveryengine.v1beta.GcsSource] or
    /// [BigQuerySource][google.cloud.discoveryengine.v1beta.BigQuerySource], and
    /// when
    /// [GcsSource.data_schema][google.cloud.discoveryengine.v1beta.GcsSource.data_schema]
    /// or
    /// [BigQuerySource.data_schema][google.cloud.discoveryengine.v1beta.BigQuerySource.data_schema]
    /// is `custom` or `csv`. Otherwise, an INVALID_ARGUMENT error is thrown.
    #[prost(bool, tag = "8")]
    pub auto_generate_ids: bool,
    /// The field in the Cloud Storage and BigQuery sources that indicates the
    /// unique IDs of the documents.
    ///
    /// For [GcsSource][google.cloud.discoveryengine.v1beta.GcsSource] it is the
    /// key of the JSON field. For instance, `my_id` for JSON `{"my_id":
    /// "some_uuid"}`. For
    /// [BigQuerySource][google.cloud.discoveryengine.v1beta.BigQuerySource] it is
    /// the column name of the BigQuery table where the unique ids are stored.
    ///
    /// The values of the JSON field or the BigQuery column are used as the
    /// [Document.id][google.cloud.discoveryengine.v1beta.Document.id]s. The JSON
    /// field or the BigQuery column must be of string type, and the values must be
    /// set as valid strings conform to
    /// [RFC-1034](<https://tools.ietf.org/html/rfc1034>) with 1-63 characters.
    /// Otherwise, documents without valid IDs fail to be imported.
    ///
    /// Only set this field when using
    /// [GcsSource][google.cloud.discoveryengine.v1beta.GcsSource] or
    /// [BigQuerySource][google.cloud.discoveryengine.v1beta.BigQuerySource], and
    /// when
    /// [GcsSource.data_schema][google.cloud.discoveryengine.v1beta.GcsSource.data_schema]
    /// or
    /// [BigQuerySource.data_schema][google.cloud.discoveryengine.v1beta.BigQuerySource.data_schema]
    /// is `custom`. And only set this field when
    /// [auto_generate_ids][google.cloud.discoveryengine.v1beta.ImportDocumentsRequest.auto_generate_ids]
    /// is unset or set as `false`. Otherwise, an INVALID_ARGUMENT error is thrown.
    ///
    /// If it is unset, a default value `_id` is used when importing from the
    /// allowed data sources.
    #[prost(string, tag = "9")]
    pub id_field: ::prost::alloc::string::String,
    /// Required. The source of the input.
    #[prost(oneof = "import_documents_request::Source", tags = "2, 3, 4")]
    pub source: ::core::option::Option<import_documents_request::Source>,
}
/// Nested message and enum types in `ImportDocumentsRequest`.
pub mod import_documents_request {
    /// The inline source for the input config for ImportDocuments method.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InlineSource {
        /// Required. A list of documents to update/create. Each document must have a
        /// valid [Document.id][google.cloud.discoveryengine.v1beta.Document.id].
        /// Recommended max of 100 items.
        #[prost(message, repeated, tag = "1")]
        pub documents: ::prost::alloc::vec::Vec<super::Document>,
    }
    /// Indicates how imported documents are reconciled with the existing documents
    /// created or imported before.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ReconciliationMode {
        /// Defaults to `INCREMENTAL`.
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
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RECONCILIATION_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "INCREMENTAL" => Some(Self::Incremental),
                "FULL" => Some(Self::Full),
                _ => None,
            }
        }
    }
    /// Required. The source of the input.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The Inline source for the input content for documents.
        #[prost(message, tag = "2")]
        InlineSource(InlineSource),
        /// Cloud Storage location for the input content.
        #[prost(message, tag = "3")]
        GcsSource(super::GcsSource),
        /// BigQuery input source.
        #[prost(message, tag = "4")]
        BigquerySource(super::BigQuerySource),
    }
}
/// Response of the
/// [ImportDocumentsRequest][google.cloud.discoveryengine.v1beta.ImportDocumentsRequest].
/// If the long running operation is done, then this message is returned by the
/// google.longrunning.Operations.response field if the operation was successful.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDocumentsResponse {
    /// A sample of errors encountered while processing the request.
    #[prost(message, repeated, tag = "1")]
    pub error_samples: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Echoes the destination for the complete errors in the request if set.
    #[prost(message, optional, tag = "2")]
    pub error_config: ::core::option::Option<ImportErrorConfig>,
}
/// Request message for
/// [DocumentService.PurgeDocuments][google.cloud.discoveryengine.v1beta.DocumentService.PurgeDocuments]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeDocumentsRequest {
    /// Required. The parent resource name, such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Filter matching documents to purge. Only currently supported
    /// value is
    /// `*` (all items).
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Actually performs the purge. If `force` is set to false, return the
    /// expected purge count without deleting any documents.
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// Response message for
/// [DocumentService.PurgeDocuments][google.cloud.discoveryengine.v1beta.DocumentService.PurgeDocuments]
/// method. If the long running operation is successfully done, then this message
/// is returned by the google.longrunning.Operations.response field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeDocumentsResponse {
    /// The total count of documents purged as a result of the operation.
    #[prost(int64, tag = "1")]
    pub purge_count: i64,
    /// A sample of document names that will be deleted. Only populated if `force`
    /// is set to false. A max of 100 names will be returned and the names are
    /// chosen at random.
    #[prost(string, repeated, tag = "2")]
    pub purge_sample: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Metadata related to the progress of the PurgeDocuments operation.
/// This will be returned by the google.longrunning.Operation.metadata field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeDocumentsMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Count of entries that were deleted successfully.
    #[prost(int64, tag = "3")]
    pub success_count: i64,
    /// Count of entries that encountered errors while processing.
    #[prost(int64, tag = "4")]
    pub failure_count: i64,
}
/// Request message for
/// [DocumentService.GetDocument][google.cloud.discoveryengine.v1beta.DocumentService.GetDocument]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentRequest {
    /// Required. Full resource name of
    /// [Document][google.cloud.discoveryengine.v1beta.Document], such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document}`.
    ///
    /// If the caller does not have permission to access the
    /// [Document][google.cloud.discoveryengine.v1beta.Document], regardless of
    /// whether or not it exists, a `PERMISSION_DENIED` error is returned.
    ///
    /// If the requested [Document][google.cloud.discoveryengine.v1beta.Document]
    /// does not exist, a `NOT_FOUND` error is returned.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [DocumentService.ListDocuments][google.cloud.discoveryengine.v1beta.DocumentService.ListDocuments]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDocumentsRequest {
    /// Required. The parent branch resource name, such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}`.
    /// Use `default_branch` as the branch ID, to list documents under the default
    /// branch.
    ///
    /// If the caller does not have permission to list
    /// [Document][google.cloud.discoveryengine.v1beta.Document]s under this
    /// branch, regardless of whether or not this branch exists, a
    /// `PERMISSION_DENIED` error is returned.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of [Document][google.cloud.discoveryengine.v1beta.Document]s
    /// to return. If unspecified, defaults to 100. The maximum allowed value is
    /// 1000. Values above 1000 will be coerced to 1000.
    ///
    /// If this field is negative, an `INVALID_ARGUMENT` error is returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token
    /// [ListDocumentsResponse.next_page_token][google.cloud.discoveryengine.v1beta.ListDocumentsResponse.next_page_token],
    /// received from a previous
    /// [DocumentService.ListDocuments][google.cloud.discoveryengine.v1beta.DocumentService.ListDocuments]
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// [DocumentService.ListDocuments][google.cloud.discoveryengine.v1beta.DocumentService.ListDocuments]
    /// must match the call that provided the page token. Otherwise, an
    /// `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// [DocumentService.ListDocuments][google.cloud.discoveryengine.v1beta.DocumentService.ListDocuments]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDocumentsResponse {
    /// The [Document][google.cloud.discoveryengine.v1beta.Document]s.
    #[prost(message, repeated, tag = "1")]
    pub documents: ::prost::alloc::vec::Vec<Document>,
    /// A token that can be sent as
    /// [ListDocumentsRequest.page_token][google.cloud.discoveryengine.v1beta.ListDocumentsRequest.page_token]
    /// to retrieve the next page. If this field is omitted, there are no
    /// subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [DocumentService.CreateDocument][google.cloud.discoveryengine.v1beta.DocumentService.CreateDocument]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDocumentRequest {
    /// Required. The parent resource name, such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The [Document][google.cloud.discoveryengine.v1beta.Document] to
    /// create.
    #[prost(message, optional, tag = "2")]
    pub document: ::core::option::Option<Document>,
    /// Required. The ID to use for the
    /// [Document][google.cloud.discoveryengine.v1beta.Document], which will become
    /// the final component of the
    /// [Document.name][google.cloud.discoveryengine.v1beta.Document.name].
    ///
    /// If the caller does not have permission to create the
    /// [Document][google.cloud.discoveryengine.v1beta.Document], regardless of
    /// whether or not it exists, a `PERMISSION_DENIED` error is returned.
    ///
    /// This field must be unique among all
    /// [Document][google.cloud.discoveryengine.v1beta.Document]s with the same
    /// [parent][google.cloud.discoveryengine.v1beta.CreateDocumentRequest.parent].
    /// Otherwise, an `ALREADY_EXISTS` error is returned.
    ///
    /// This field must conform to [RFC-1034](<https://tools.ietf.org/html/rfc1034>)
    /// standard with a length limit of 63 characters. Otherwise, an
    /// `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "3")]
    pub document_id: ::prost::alloc::string::String,
}
/// Request message for
/// [DocumentService.UpdateDocument][google.cloud.discoveryengine.v1beta.DocumentService.UpdateDocument]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDocumentRequest {
    /// Required. The document to update/create.
    ///
    /// If the caller does not have permission to update the
    /// [Document][google.cloud.discoveryengine.v1beta.Document], regardless of
    /// whether or not it exists, a `PERMISSION_DENIED` error is returned.
    ///
    /// If the [Document][google.cloud.discoveryengine.v1beta.Document] to update
    /// does not exist and
    /// [allow_missing][google.cloud.discoveryengine.v1beta.UpdateDocumentRequest.allow_missing]
    /// is not set, a `NOT_FOUND` error is returned.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// If set to true, and the
    /// [Document][google.cloud.discoveryengine.v1beta.Document] is not found, a
    /// new [Document][google.cloud.discoveryengine.v1beta.Document] will be
    /// created.
    #[prost(bool, tag = "2")]
    pub allow_missing: bool,
}
/// Request message for
/// [DocumentService.DeleteDocument][google.cloud.discoveryengine.v1beta.DocumentService.DeleteDocument]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDocumentRequest {
    /// Required. Full resource name of
    /// [Document][google.cloud.discoveryengine.v1beta.Document], such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document}`.
    ///
    /// If the caller does not have permission to delete the
    /// [Document][google.cloud.discoveryengine.v1beta.Document], regardless of
    /// whether or not it exists, a `PERMISSION_DENIED` error is returned.
    ///
    /// If the [Document][google.cloud.discoveryengine.v1beta.Document] to delete
    /// does not exist, a `NOT_FOUND` error is returned.
    #[prost(string, tag = "1")]
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
            D: TryInto<tonic::transport::Endpoint>,
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Gets a [Document][google.cloud.discoveryengine.v1beta.Document].
        pub async fn get_document(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDocumentRequest>,
        ) -> std::result::Result<tonic::Response<super::Document>, tonic::Status> {
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.DocumentService",
                        "GetDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a list of [Document][google.cloud.discoveryengine.v1beta.Document]s.
        pub async fn list_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDocumentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDocumentsResponse>,
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
                "/google.cloud.discoveryengine.v1beta.DocumentService/ListDocuments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.DocumentService",
                        "ListDocuments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a [Document][google.cloud.discoveryengine.v1beta.Document].
        pub async fn create_document(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDocumentRequest>,
        ) -> std::result::Result<tonic::Response<super::Document>, tonic::Status> {
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.DocumentService",
                        "CreateDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a [Document][google.cloud.discoveryengine.v1beta.Document].
        pub async fn update_document(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDocumentRequest>,
        ) -> std::result::Result<tonic::Response<super::Document>, tonic::Status> {
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.DocumentService",
                        "UpdateDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a [Document][google.cloud.discoveryengine.v1beta.Document].
        pub async fn delete_document(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDocumentRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.DocumentService",
                        "DeleteDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
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
        ) -> std::result::Result<
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.DocumentService",
                        "ImportDocuments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Permanently deletes all selected
        /// [Document][google.cloud.discoveryengine.v1beta.Document]s in a branch.
        ///
        /// This process is asynchronous. Depending on the number of
        /// [Document][google.cloud.discoveryengine.v1beta.Document]s to be deleted,
        /// this operation can take hours to complete. Before the delete operation
        /// completes, some [Document][google.cloud.discoveryengine.v1beta.Document]s
        /// might still be returned by
        /// [DocumentService.GetDocument][google.cloud.discoveryengine.v1beta.DocumentService.GetDocument]
        /// or
        /// [DocumentService.ListDocuments][google.cloud.discoveryengine.v1beta.DocumentService.ListDocuments].
        ///
        /// To get a list of the
        /// [Document][google.cloud.discoveryengine.v1beta.Document]s to be deleted,
        /// set
        /// [PurgeDocumentsRequest.force][google.cloud.discoveryengine.v1beta.PurgeDocumentsRequest.force]
        /// to false.
        pub async fn purge_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::PurgeDocumentsRequest>,
        ) -> std::result::Result<
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
                "/google.cloud.discoveryengine.v1beta.DocumentService/PurgeDocuments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.DocumentService",
                        "PurgeDocuments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Request message for Recommend method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendRequest {
    /// Required. Full resource name of the format:
    /// `projects/*/locations/global/collections/*/dataStores/*/servingConfigs/*`
    ///
    /// Before you can request recommendations from your model, you must create at
    /// least one serving config  for it.
    #[prost(string, tag = "1")]
    pub serving_config: ::prost::alloc::string::String,
    /// Required. Context about the user, what they are looking at and what action
    /// they took to trigger the Recommend request. Note that this user event
    /// detail won't be ingested to userEvent logs. Thus, a separate userEvent
    /// write request is required for event logging.
    ///
    /// Don't set
    /// [UserEvent.user_pseudo_id][google.cloud.discoveryengine.v1beta.UserEvent.user_pseudo_id]
    /// or
    /// [UserEvent.user_info.user_id][google.cloud.discoveryengine.v1beta.UserInfo.user_id]
    /// to the same fixed ID for different users. If you are trying to receive
    /// non-personalized recommendations (not recommended; this can negatively
    /// impact model performance), instead set
    /// [UserEvent.user_pseudo_id][google.cloud.discoveryengine.v1beta.UserEvent.user_pseudo_id]
    /// to a random unique ID and leave
    /// [UserEvent.user_info.user_id][google.cloud.discoveryengine.v1beta.UserInfo.user_id]
    /// unset.
    #[prost(message, optional, tag = "2")]
    pub user_event: ::core::option::Option<UserEvent>,
    /// Maximum number of results to return. Set this property
    /// to the number of recommendation results needed. If zero, the service will
    /// choose a reasonable default. The maximum allowed value is 100. Values
    /// above 100 will be coerced to 100.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Filter for restricting recommendation results with a length limit of 5,000
    /// characters. Currently, only filter expressions on the `filter_tags`
    /// attribute is supported.
    ///
    ///
    /// Examples:
    ///
    ///   * `(filter_tags: ANY("Red", "Blue") OR filter_tags: ANY("Hot", "Cold"))`
    ///   * `(filter_tags: ANY("Red", "Blue")) AND NOT (filter_tags: ANY("Green"))`
    ///
    /// If your filter blocks all results, the API will return generic
    /// (unfiltered) popular Documents. If you only want results strictly matching
    /// the filters, set `strictFiltering` to True in
    /// [RecommendRequest.params][google.cloud.discoveryengine.v1beta.RecommendRequest.params]
    /// to receive empty results instead.
    ///
    /// Note that the API will never return
    /// [Document][google.cloud.discoveryengine.v1beta.Document]s with
    /// `storageStatus` of `EXPIRED` or `DELETED` regardless of filter choices.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Use validate only mode for this recommendation query. If set to true, a
    /// fake model will be used that returns arbitrary Document IDs.
    /// Note that the validate only mode should only be used for testing the API,
    /// or if the model is not ready.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
    /// Additional domain specific parameters for the recommendations.
    ///
    /// Allowed values:
    ///
    /// * `returnDocument`: Boolean. If set to true, the associated Document
    ///     object will be returned in
    ///     [RecommendResponse.RecommendationResult.document][google.cloud.discoveryengine.v1beta.RecommendResponse.RecommendationResult.document].
    /// * `returnScore`: Boolean. If set to true, the recommendation 'score'
    ///     corresponding to each returned Document will be set in
    ///     [RecommendResponse.RecommendationResult.metadata][google.cloud.discoveryengine.v1beta.RecommendResponse.RecommendationResult.metadata].
    ///     The given 'score' indicates the probability of a Document conversion
    ///     given the user's context and history.
    /// * `strictFiltering`: Boolean. True by default. If set to false, the service
    ///     will return generic (unfiltered) popular Documents instead of empty if
    ///     your filter blocks all recommendation results.
    /// * `diversityLevel`: String. Default empty. If set to be non-empty, then
    ///     it needs to be one of:
    ///      *  `no-diversity`
    ///      *  `low-diversity`
    ///      *  `medium-diversity`
    ///      *  `high-diversity`
    ///      *  `auto-diversity`
    ///     This gives request-level control and adjusts recommendation results
    ///     based on Document category.
    #[prost(map = "string, message", tag = "6")]
    pub params: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost_types::Value,
    >,
    /// The user labels applied to a resource must meet the following requirements:
    ///
    /// * Each resource can have multiple labels, up to a maximum of 64.
    /// * Each label must be a key-value pair.
    /// * Keys have a minimum length of 1 character and a maximum length of 63
    ///    characters and cannot be empty. Values can be empty and have a maximum
    ///    length of 63 characters.
    /// * Keys and values can contain only lowercase letters, numeric characters,
    ///    underscores, and dashes. All characters must use UTF-8 encoding, and
    ///    international characters are allowed.
    /// * The key portion of a label must be unique. However, you can use the same
    ///    key with multiple resources.
    /// * Keys must start with a lowercase letter or international character.
    ///
    /// See [Requirements for
    /// labels](<https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements>)
    /// for more details.
    #[prost(map = "string, string", tag = "8")]
    pub user_labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Response message for Recommend method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendResponse {
    /// A list of recommended Documents. The order represents the ranking (from the
    /// most relevant Document to the least).
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<recommend_response::RecommendationResult>,
    /// A unique attribution token. This should be included in the
    /// [UserEvent][google.cloud.discoveryengine.v1beta.UserEvent] logs resulting
    /// from this recommendation, which enables accurate attribution of
    /// recommendation model performance.
    #[prost(string, tag = "2")]
    pub attribution_token: ::prost::alloc::string::String,
    /// IDs of documents in the request that were missing from the default Branch
    /// associated with the requested ServingConfig.
    #[prost(string, repeated, tag = "3")]
    pub missing_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// True if
    /// [RecommendRequest.validate_only][google.cloud.discoveryengine.v1beta.RecommendRequest.validate_only]
    /// was set.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Nested message and enum types in `RecommendResponse`.
pub mod recommend_response {
    /// RecommendationResult represents a generic recommendation result with
    /// associated metadata.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RecommendationResult {
        /// Resource ID of the recommended Document.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// Set if `returnDocument` is set to true in
        /// [RecommendRequest.params][google.cloud.discoveryengine.v1beta.RecommendRequest.params].
        #[prost(message, optional, tag = "2")]
        pub document: ::core::option::Option<super::Document>,
        /// Additional Document metadata / annotations.
        ///
        /// Possible values:
        ///
        /// * `score`: Recommendation score in double value. Is set if
        ///    `returnScore` is set to true in
        ///    [RecommendRequest.params][google.cloud.discoveryengine.v1beta.RecommendRequest.params].
        #[prost(map = "string, message", tag = "3")]
        pub metadata: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost_types::Value,
        >,
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
            D: TryInto<tonic::transport::Endpoint>,
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Makes a recommendation, which requires a contextual user event.
        pub async fn recommend(
            &mut self,
            request: impl tonic::IntoRequest<super::RecommendRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RecommendResponse>,
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
                "/google.cloud.discoveryengine.v1beta.RecommendationService/Recommend",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.RecommendationService",
                        "Recommend",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Defines the structure and layout of a type of document data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    /// Immutable. The full resource name of the schema, in the format of
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/schemas/{schema}`.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 1024
    /// characters.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Schema representation. One of
    /// [struct_schema][google.cloud.discoveryengine.v1beta.Schema.struct_schema]
    /// or [json_schema][google.cloud.discoveryengine.v1beta.Schema.json_schema]
    /// should be provided otherwise an `INVALID_ARGUMENT` error is thrown.
    #[prost(oneof = "schema::Schema", tags = "2, 3")]
    pub schema: ::core::option::Option<schema::Schema>,
}
/// Nested message and enum types in `Schema`.
pub mod schema {
    /// Schema representation. One of
    /// [struct_schema][google.cloud.discoveryengine.v1beta.Schema.struct_schema]
    /// or [json_schema][google.cloud.discoveryengine.v1beta.Schema.json_schema]
    /// should be provided otherwise an `INVALID_ARGUMENT` error is thrown.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Schema {
        /// The structured representation of the schema.
        #[prost(message, tag = "2")]
        StructSchema(::prost_types::Struct),
        /// The JSON representation of the schema.
        #[prost(string, tag = "3")]
        JsonSchema(::prost::alloc::string::String),
    }
}
/// Request message for
/// [SchemaService.GetSchema][google.cloud.discoveryengine.v1beta.SchemaService.GetSchema]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSchemaRequest {
    /// Required. The full resource name of the schema, in the format of
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/schemas/{schema}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [SchemaService.ListSchemas][google.cloud.discoveryengine.v1beta.SchemaService.ListSchemas]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSchemasRequest {
    /// Required. The parent data store resource name, in the format of
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of [Schema][google.cloud.discoveryengine.v1beta.Schema]s
    /// to return. The service may return fewer than this value.
    ///
    /// If unspecified, at most 100
    /// [Schema][google.cloud.discoveryengine.v1beta.Schema]s will be returned.
    ///
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous
    /// [SchemaService.ListSchemas][google.cloud.discoveryengine.v1beta.SchemaService.ListSchemas]
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// [SchemaService.ListSchemas][google.cloud.discoveryengine.v1beta.SchemaService.ListSchemas]
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// [SchemaService.ListSchemas][google.cloud.discoveryengine.v1beta.SchemaService.ListSchemas]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSchemasResponse {
    /// The [Schema][google.cloud.discoveryengine.v1beta.Schema]s.
    #[prost(message, repeated, tag = "1")]
    pub schemas: ::prost::alloc::vec::Vec<Schema>,
    /// A token that can be sent as
    /// [ListSchemasRequest.page_token][google.cloud.discoveryengine.v1beta.ListSchemasRequest.page_token]
    /// to retrieve the next page. If this field is omitted, there are no
    /// subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [SchemaService.CreateSchema][google.cloud.discoveryengine.v1beta.SchemaService.CreateSchema]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSchemaRequest {
    /// Required. The parent data store resource name, in the format of
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The [Schema][google.cloud.discoveryengine.v1beta.Schema] to
    /// create.
    #[prost(message, optional, tag = "2")]
    pub schema: ::core::option::Option<Schema>,
    /// Required. The ID to use for the
    /// [Schema][google.cloud.discoveryengine.v1beta.Schema], which will become the
    /// final component of the
    /// [Schema.name][google.cloud.discoveryengine.v1beta.Schema.name].
    ///
    /// This field should conform to
    /// [RFC-1034](<https://tools.ietf.org/html/rfc1034>) standard with a length
    /// limit of 63 characters.
    #[prost(string, tag = "3")]
    pub schema_id: ::prost::alloc::string::String,
}
/// Request message for
/// [SchemaService.UpdateSchema][google.cloud.discoveryengine.v1beta.SchemaService.UpdateSchema]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSchemaRequest {
    /// Required. The [Schema][google.cloud.discoveryengine.v1beta.Schema] to
    /// update.
    #[prost(message, optional, tag = "1")]
    pub schema: ::core::option::Option<Schema>,
    /// If set to true, and the
    /// [Schema][google.cloud.discoveryengine.v1beta.Schema] is not found, a new
    /// [Schema][google.cloud.discoveryengine.v1beta.Schema] will be created. In
    /// this situation, `update_mask` is ignored.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
}
/// Request message for
/// [SchemaService.DeleteSchema][google.cloud.discoveryengine.v1beta.SchemaService.DeleteSchema]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSchemaRequest {
    /// Required. The full resource name of the schema, in the format of
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/schemas/{schema}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Metadata for Create Schema LRO.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSchemaMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Metadata for UpdateSchema LRO.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSchemaMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Metadata for DeleteSchema LRO.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSchemaMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Generated client implementations.
pub mod schema_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing [Schema][google.cloud.discoveryengine.v1beta.Schema]s.
    #[derive(Debug, Clone)]
    pub struct SchemaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SchemaServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SchemaServiceClient<T>
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
        ) -> SchemaServiceClient<InterceptedService<T, F>>
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
            SchemaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Gets a [Schema][google.cloud.discoveryengine.v1beta.Schema].
        pub async fn get_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::Schema>, tonic::Status> {
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
                "/google.cloud.discoveryengine.v1beta.SchemaService/GetSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.SchemaService",
                        "GetSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a list of [Schema][google.cloud.discoveryengine.v1beta.Schema]s.
        pub async fn list_schemas(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSchemasRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSchemasResponse>,
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
                "/google.cloud.discoveryengine.v1beta.SchemaService/ListSchemas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.SchemaService",
                        "ListSchemas",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a [Schema][google.cloud.discoveryengine.v1beta.Schema].
        pub async fn create_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSchemaRequest>,
        ) -> std::result::Result<
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
                "/google.cloud.discoveryengine.v1beta.SchemaService/CreateSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.SchemaService",
                        "CreateSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a [Schema][google.cloud.discoveryengine.v1beta.Schema].
        pub async fn update_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSchemaRequest>,
        ) -> std::result::Result<
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
                "/google.cloud.discoveryengine.v1beta.SchemaService/UpdateSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.SchemaService",
                        "UpdateSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a [Schema][google.cloud.discoveryengine.v1beta.Schema].
        pub async fn delete_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSchemaRequest>,
        ) -> std::result::Result<
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
                "/google.cloud.discoveryengine.v1beta.SchemaService/DeleteSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.SchemaService",
                        "DeleteSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Request message for WriteUserEvent method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteUserEventRequest {
    /// Required. The parent DataStore resource name, such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User event to write.
    #[prost(message, optional, tag = "2")]
    pub user_event: ::core::option::Option<UserEvent>,
}
/// Request message for CollectUserEvent method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectUserEventRequest {
    /// Required. The parent DataStore resource name, such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. URL encoded UserEvent proto with a length limit of 2,000,000
    /// characters.
    #[prost(string, tag = "2")]
    pub user_event: ::prost::alloc::string::String,
    /// The URL including cgi-parameters but excluding the hash fragment with a
    /// length limit of 5,000 characters. This is often more useful than the
    /// referer URL, because many browsers only send the domain for third-party
    /// requests.
    #[prost(string, optional, tag = "3")]
    pub uri: ::core::option::Option<::prost::alloc::string::String>,
    /// The event timestamp in milliseconds. This prevents browser caching of
    /// otherwise identical get requests. The name is abbreviated to reduce the
    /// payload bytes.
    #[prost(int64, optional, tag = "4")]
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
            D: TryInto<tonic::transport::Endpoint>,
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Writes a single user event.
        pub async fn write_user_event(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteUserEventRequest>,
        ) -> std::result::Result<tonic::Response<super::UserEvent>, tonic::Status> {
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.UserEventService",
                        "WriteUserEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Writes a single user event from the browser. This uses a GET request to
        /// due to browser restriction of POST-ing to a third-party domain.
        ///
        /// This method is used only by the Discovery Engine API JavaScript pixel and
        /// Google Tag Manager. Users should not call this method directly.
        pub async fn collect_user_event(
            &mut self,
            request: impl tonic::IntoRequest<super::CollectUserEventRequest>,
        ) -> std::result::Result<
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.UserEventService",
                        "CollectUserEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
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
        ) -> std::result::Result<
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1beta.UserEventService",
                        "ImportUserEvents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
