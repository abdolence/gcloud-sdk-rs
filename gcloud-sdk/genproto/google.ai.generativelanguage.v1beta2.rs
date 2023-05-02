/// A collection of source attributions for a piece of content.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CitationMetadata {
    /// Citations to sources for a specific response.
    #[prost(message, repeated, tag = "1")]
    pub citation_sources: ::prost::alloc::vec::Vec<CitationSource>,
}
/// A citation to a source for a portion of a specific response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CitationSource {
    /// Optional. Start of segment of the response that is attributed to this
    /// source.
    ///
    /// Index indicates the start of the segment, measured in bytes.
    #[prost(int32, optional, tag = "1")]
    pub start_index: ::core::option::Option<i32>,
    /// Optional. End of the attributed segment, exclusive.
    #[prost(int32, optional, tag = "2")]
    pub end_index: ::core::option::Option<i32>,
    /// Optional. URI that is attributed as a source for a portion of the text.
    #[prost(string, optional, tag = "3")]
    pub uri: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. License for the GitHub project that is attributed as a source for
    /// segment.
    ///
    /// License info is required for code citations.
    #[prost(string, optional, tag = "4")]
    pub license: ::core::option::Option<::prost::alloc::string::String>,
}
/// Request to generate a message response from the model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateMessageRequest {
    /// Required. The name of the model to use.
    ///
    /// Format: `name=models/{model}`.
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. The structured textual input given to the model as a prompt.
    ///
    /// Given a
    /// prompt, the model will return what it predicts is the next message in the
    /// discussion.
    #[prost(message, optional, tag = "2")]
    pub prompt: ::core::option::Option<MessagePrompt>,
    /// Optional. Controls the randomness of the output.
    ///
    /// Values can range over `\[0.0,1.0\]`,
    /// inclusive. A value closer to `1.0` will produce responses that are more
    /// varied, while a value closer to `0.0` will typically result in
    /// less surprising responses from the model.
    #[prost(float, optional, tag = "3")]
    pub temperature: ::core::option::Option<f32>,
    /// Optional. The number of generated response messages to return.
    ///
    /// This value must be between
    /// `[1, 8]`, inclusive. If unset, this will default to `1`.
    #[prost(int32, optional, tag = "4")]
    pub candidate_count: ::core::option::Option<i32>,
    /// Optional. The maximum cumulative probability of tokens to consider when
    /// sampling.
    ///
    /// The model uses combined Top-k and nucleus sampling.
    ///
    /// Nucleus sampling considers the smallest set of tokens whose probability
    /// sum is at least `top_p`.
    #[prost(float, optional, tag = "5")]
    pub top_p: ::core::option::Option<f32>,
    /// Optional. The maximum number of tokens to consider when sampling.
    ///
    /// The model uses combined Top-k and nucleus sampling.
    ///
    /// Top-k sampling considers the set of `top_k` most probable tokens.
    #[prost(int32, optional, tag = "6")]
    pub top_k: ::core::option::Option<i32>,
}
/// The response from the model.
///
/// This includes candidate messages and
/// conversation history in the form of chronologically-ordered messages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateMessageResponse {
    /// Candidate response messages from the model.
    #[prost(message, repeated, tag = "1")]
    pub candidates: ::prost::alloc::vec::Vec<Message>,
    /// The conversation history used by the model.
    #[prost(message, repeated, tag = "2")]
    pub messages: ::prost::alloc::vec::Vec<Message>,
}
/// The base unit of structured text.
///
/// A `Message` includes an `author` and the `content` of
/// the `Message`.
///
/// The `author` is used to tag messages when they are fed to the
/// model as text.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    /// Optional. The author of this Message.
    ///
    /// This serves as a key for tagging
    /// the content of this Message when it is fed to the model as text.
    ///
    /// The author can be any alphanumeric string.
    #[prost(string, tag = "1")]
    pub author: ::prost::alloc::string::String,
    /// Required. The text content of the structured `Message`.
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
    /// Output only. Citation information for model-generated `content` in this
    /// `Message`.
    ///
    /// If this `Message` was generated as output from the model, this field may be
    /// populated with attribution information for any text included in the
    /// `content`. This field is used only on output.
    #[prost(message, optional, tag = "3")]
    pub citation_metadata: ::core::option::Option<CitationMetadata>,
}
/// All of the structured input text passed to the model as a prompt.
///
/// A `MessagePrompt` contains a structured set of fields that provide context
/// for the conversation, examples of user input/model output message pairs that
/// prime the model to respond in different ways, and the conversation history
/// or list of messages representing the alternating turns of the conversation
/// between the user and the model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagePrompt {
    /// Optional. Text that should be provided to the model first to ground the
    /// response.
    ///
    /// If not empty, this `context` will be given to the model first before the
    /// `examples` and `messages`. When using a `context` be sure to provide it
    /// with every request to maintain continuity.
    ///
    /// This field can be a description of your prompt to the model to help provide
    /// context and guide the responses. Examples: "Translate the phrase from
    /// English to French." or "Given a statement, classify the sentiment as happy,
    /// sad or neutral."
    ///
    /// Anything included in this field will take precedence over message history
    /// if the total input size exceeds the model's `input_token_limit` and the
    /// input request is truncated.
    #[prost(string, tag = "1")]
    pub context: ::prost::alloc::string::String,
    /// Optional. Examples of what the model should generate.
    ///
    /// This includes both user input and the response that the model should
    /// emulate.
    ///
    /// These `examples` are treated identically to conversation messages except
    /// that they take precedence over the history in `messages`:
    /// If the total input size exceeds the model's `input_token_limit` the input
    /// will be truncated. Items will be dropped from `messages` before `examples`.
    #[prost(message, repeated, tag = "2")]
    pub examples: ::prost::alloc::vec::Vec<Example>,
    /// Required. A snapshot of the recent conversation history sorted
    /// chronologically.
    ///
    /// Turns alternate between two authors.
    ///
    /// If the total input size exceeds the model's `input_token_limit` the input
    /// will be truncated: The oldest items will be dropped from `messages`.
    #[prost(message, repeated, tag = "3")]
    pub messages: ::prost::alloc::vec::Vec<Message>,
}
/// An input/output example used to instruct the Model.
///
/// It demonstrates how the model should respond or format its response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Example {
    /// An example of an input `Message` from the user.
    #[prost(message, optional, tag = "1")]
    pub input: ::core::option::Option<Message>,
    /// An example of what the model should output given the input.
    #[prost(message, optional, tag = "2")]
    pub output: ::core::option::Option<Message>,
}
/// Counts the number of tokens in the `prompt` sent to a model.
///
/// Models may tokenize text differently, so each model may return a different
/// `token_count`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountMessageTokensRequest {
    /// Required. The model's resource name. This serves as an ID for the Model to
    /// use.
    ///
    /// This name should match a model name returned by the `ListModels` method.
    ///
    /// Format: `models/{model}`
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. The prompt, whose token count is to be returned.
    #[prost(message, optional, tag = "2")]
    pub prompt: ::core::option::Option<MessagePrompt>,
}
/// A response from `CountMessageTokens`.
///
/// It returns the model's `token_count` for the `prompt`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountMessageTokensResponse {
    /// The number of tokens that the `model` tokenizes the `prompt` into.
    ///
    /// Always non-negative.
    #[prost(int32, tag = "1")]
    pub token_count: i32,
}
/// Generated client implementations.
pub mod discuss_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// An API for using Generative Language Models (GLMs) in dialog applications.
    ///
    /// Also known as large language models (LLMs), this API provides models that
    /// are trained for multi-turn dialog.
    #[derive(Debug, Clone)]
    pub struct DiscussServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DiscussServiceClient<tonic::transport::Channel> {
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
    impl<T> DiscussServiceClient<T>
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
        ) -> DiscussServiceClient<InterceptedService<T, F>>
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
            DiscussServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Generates a response from the model given an input `MessagePrompt`.
        pub async fn generate_message(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateMessageResponse>,
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
                "/google.ai.generativelanguage.v1beta2.DiscussService/GenerateMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta2.DiscussService",
                        "GenerateMessage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Runs a model's tokenizer on a string and returns the token count.
        pub async fn count_message_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::CountMessageTokensRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CountMessageTokensResponse>,
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
                "/google.ai.generativelanguage.v1beta2.DiscussService/CountMessageTokens",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta2.DiscussService",
                        "CountMessageTokens",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Information about a Generative Language Model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// Required. The resource name of the `Model`.
    ///
    /// Format: `models/{model}` with a `{model}` naming convention of:
    ///
    /// * "{base_model_id}-{version}"
    ///
    /// Examples:
    ///
    /// * `models/chat-pison-001`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The name of the base model, pass this to the generation request.
    ///
    /// Examples:
    ///
    /// * `chat-bison`
    #[prost(string, tag = "2")]
    pub base_model_id: ::prost::alloc::string::String,
    /// Required. The version number of the model.
    ///
    /// This represents the major version
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    /// The human-readable name of the model. E.g. "Chat Bison".
    ///
    /// The name can be up to 128 characters long and can consist of any UTF-8
    /// characters.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// A short description of the model.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Maximum number of input tokens allowed for this model.
    #[prost(int32, tag = "6")]
    pub input_token_limit: i32,
    /// Maximum number of output tokens available for this model.
    #[prost(int32, tag = "7")]
    pub output_token_limit: i32,
    /// The model's supported generation methods.
    ///
    /// The method names are defined as Pascal case
    /// strings, such as `generateMessage` which correspond to API methods.
    #[prost(string, repeated, tag = "8")]
    pub supported_generation_methods: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Controls the randomness of the output.
    ///
    /// Values can range over `\[0.0,1.0\]`, inclusive. A value closer to `1.0` will
    /// produce responses that are more varied, while a value closer to `0.0` will
    /// typically result in less surprising responses from the model.
    /// This value specifies default to be used by the backend while making the
    /// call to the model.
    #[prost(float, optional, tag = "9")]
    pub temperature: ::core::option::Option<f32>,
    /// For Nucleus sampling.
    ///
    /// Nucleus sampling considers the smallest set of tokens whose probability
    /// sum is at least `top_p`.
    /// This value specifies default to be used by the backend while making the
    /// call to the model.
    #[prost(float, optional, tag = "10")]
    pub top_p: ::core::option::Option<f32>,
    /// For Top-k sampling.
    ///
    /// Top-k sampling considers the set of `top_k` most probable tokens.
    /// This value specifies default to be used by the backend while making the
    /// call to the model.
    #[prost(int32, optional, tag = "11")]
    pub top_k: ::core::option::Option<i32>,
}
/// Request for getting information about a specific Model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelRequest {
    /// Required. The resource name of the model.
    ///
    /// This name should match a model name returned by the `ListModels` method.
    ///
    /// Format: `models/{model}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for listing all Models.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsRequest {
    /// The maximum number of `Models` to return (per page).
    ///
    /// The service may return fewer models.
    /// If unspecified, at most 50 models will be returned per page.
    /// This method returns at most 1000 models per page, even if you pass a larger
    /// page_size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListModels` call.
    ///
    /// Provide the `page_token` returned by one request as an argument to the next
    /// request to retrieve the next page.
    ///
    /// When paginating, all other parameters provided to `ListModels` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response from `ListModel` containing a paginated list of Models.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsResponse {
    /// The returned Models.
    #[prost(message, repeated, tag = "1")]
    pub models: ::prost::alloc::vec::Vec<Model>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    ///
    /// If this field is omitted, there are no more pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod model_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for getting metadata information about Generative Models.
    #[derive(Debug, Clone)]
    pub struct ModelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ModelServiceClient<tonic::transport::Channel> {
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
    impl<T> ModelServiceClient<T>
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
        ) -> ModelServiceClient<InterceptedService<T, F>>
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
            ModelServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets information about a specific Model.
        pub async fn get_model(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModelRequest>,
        ) -> std::result::Result<tonic::Response<super::Model>, tonic::Status> {
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
                "/google.ai.generativelanguage.v1beta2.ModelService/GetModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta2.ModelService",
                        "GetModel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists models available through the API.
        pub async fn list_models(
            &mut self,
            request: impl tonic::IntoRequest<super::ListModelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListModelsResponse>,
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
                "/google.ai.generativelanguage.v1beta2.ModelService/ListModels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta2.ModelService",
                        "ListModels",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Request to generate a text completion response from the model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateTextRequest {
    /// Required. The model name to use with the format name=models/{model}.
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. The free-form input text given to the model as a prompt.
    ///
    /// Given a prompt, the model will generate a TextCompletion response it
    /// predicts as the completion of the input text.
    #[prost(message, optional, tag = "2")]
    pub prompt: ::core::option::Option<TextPrompt>,
    /// Controls the randomness of the output.
    /// Note: The default value varies by model, see the `Model.temperature`
    /// attribute of the `Model` returned the `getModel` function.
    ///
    /// Values can range from \[0.0,1.0\],
    /// inclusive. A value closer to 1.0 will produce responses that are more
    /// varied and creative, while a value closer to 0.0 will typically result in
    /// more straightforward responses from the model.
    #[prost(float, optional, tag = "3")]
    pub temperature: ::core::option::Option<f32>,
    /// Number of generated responses to return.
    ///
    /// This value must be between [1, 8], inclusive. If unset, this will default
    /// to 1.
    #[prost(int32, optional, tag = "4")]
    pub candidate_count: ::core::option::Option<i32>,
    /// The maximum number of tokens to include in a candidate.
    ///
    /// If unset, this will default to 64.
    #[prost(int32, optional, tag = "5")]
    pub max_output_tokens: ::core::option::Option<i32>,
    /// The maximum cumulative probability of tokens to consider when sampling.
    ///
    /// The model uses combined Top-k and nucleus sampling.
    ///
    /// Tokens are sorted based on their assigned probabilities so that only the
    /// most liekly tokens are considered. Top-k sampling directly limits the
    /// maximum number of tokens to consider, while Nucleus sampling limits number
    /// of tokens based on the cumulative probability.
    ///
    /// Note: The default value varies by model, see the `Model.top_p`
    /// attribute of the `Model` returned the `getModel` function.
    #[prost(float, optional, tag = "6")]
    pub top_p: ::core::option::Option<f32>,
    /// The maximum number of tokens to consider when sampling.
    ///
    /// The model uses combined Top-k and nucleus sampling.
    ///
    /// Top-k sampling considers the set of `top_k` most probable tokens.
    /// Defaults to 40.
    ///
    /// Note: The default value varies by model, see the `Model.top_k`
    /// attribute of the `Model` returned the `getModel` function.
    #[prost(int32, optional, tag = "7")]
    pub top_k: ::core::option::Option<i32>,
    /// The set of character sequences (up to 5) that will stop output generation.
    /// If specified, the API will stop at the first appearance of a stop
    /// sequence. The stop sequence will not be included as part of the response.
    #[prost(string, repeated, tag = "9")]
    pub stop_sequences: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The response from the model, including candidate completions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateTextResponse {
    /// Candidate responses from the model.
    #[prost(message, repeated, tag = "1")]
    pub candidates: ::prost::alloc::vec::Vec<TextCompletion>,
}
/// Text given to the model as a prompt.
///
/// The Model will use this TextPrompt to Generate a text completion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextPrompt {
    /// Required. The prompt text.
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
}
/// Output text returned from a model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextCompletion {
    /// Output only. The generated text returned from the model.
    #[prost(string, tag = "1")]
    pub output: ::prost::alloc::string::String,
}
/// Request to get a text embedding from the model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbedTextRequest {
    /// Required. The model name to use with the format model=models/{model}.
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. The free-form input text that the model will turn into an
    /// embedding.
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
/// The response to a EmbedTextRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbedTextResponse {
    /// Output only. The embedding generated from the input text.
    #[prost(message, optional, tag = "1")]
    pub embedding: ::core::option::Option<Embedding>,
}
/// A list of floats representing the embedding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Embedding {
    /// The embedding values.
    #[prost(float, repeated, tag = "1")]
    pub value: ::prost::alloc::vec::Vec<f32>,
}
/// Generated client implementations.
pub mod text_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// API for using Generative Language Models (GLMs) trained to generate text.
    ///
    /// Also known as Large Language Models (LLM)s, these generate text given an
    /// input prompt from the user.
    #[derive(Debug, Clone)]
    pub struct TextServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TextServiceClient<tonic::transport::Channel> {
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
    impl<T> TextServiceClient<T>
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
        ) -> TextServiceClient<InterceptedService<T, F>>
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
            TextServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Generates a response from the model given an input message.
        pub async fn generate_text(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateTextRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateTextResponse>,
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
                "/google.ai.generativelanguage.v1beta2.TextService/GenerateText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta2.TextService",
                        "GenerateText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generates an embedding from the model given an input message.
        pub async fn embed_text(
            &mut self,
            request: impl tonic::IntoRequest<super::EmbedTextRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EmbedTextResponse>,
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
                "/google.ai.generativelanguage.v1beta2.TextService/EmbedText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta2.TextService",
                        "EmbedText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
