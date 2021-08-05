/// Represents a response message that can be returned by a conversational agent.
///
/// Response messages are also used for output audio synthesis. The approach is
/// as follows:
///
/// * If at least one OutputAudioText response is present, then all
///   OutputAudioText responses are linearly concatenated, and the result is used
///   for output audio synthesis.
/// * If the OutputAudioText responses are a mixture of text and SSML, then the
///   concatenated result is treated as SSML; otherwise, the result is treated as
///   either text or SSML as appropriate. The agent designer should ideally use
///   either text or SSML consistently throughout the bot design.
/// * Otherwise, all Text responses are linearly concatenated, and the result is
///   used for output audio synthesis.
///
/// This approach allows for more sophisticated user experience scenarios, where
/// the text displayed to the user may differ from what is heard.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMessage {
    /// Required. The rich response message.
    #[prost(
        oneof = "response_message::Message",
        tags = "1, 2, 9, 8, 10, 11, 12, 13"
    )]
    pub message: ::core::option::Option<response_message::Message>,
}
/// Nested message and enum types in `ResponseMessage`.
pub mod response_message {
    /// The text response message.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Text {
        /// Required. A collection of text responses.
        #[prost(string, repeated, tag = "1")]
        pub text: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Output only. Whether the playback of this message can be interrupted by the end
        /// user's speech and the client can then starts the next Dialogflow
        /// request.
        #[prost(bool, tag = "2")]
        pub allow_playback_interruption: bool,
    }
    /// Indicates that the conversation should be handed off to a live agent.
    ///
    /// Dialogflow only uses this to determine which conversations were handed off
    /// to a human agent for measurement purposes. What else to do with this signal
    /// is up to you and your handoff procedures.
    ///
    /// You may set this, for example:
    /// * In the [entry_fulfillment][google.cloud.dialogflow.cx.v3.Page.entry_fulfillment] of a [Page][google.cloud.dialogflow.cx.v3.Page] if
    ///   entering the page indicates something went extremely wrong in the
    ///   conversation.
    /// * In a webhook response when you determine that the customer issue can only
    ///   be handled by a human.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LiveAgentHandoff {
        /// Custom metadata for your handoff procedure. Dialogflow doesn't impose
        /// any structure on this.
        #[prost(message, optional, tag = "1")]
        pub metadata: ::core::option::Option<::prost_types::Struct>,
    }
    /// Indicates that the conversation succeeded, i.e., the bot handled the issue
    /// that the customer talked to it about.
    ///
    /// Dialogflow only uses this to determine which conversations should be
    /// counted as successful and doesn't process the metadata in this message in
    /// any way. Note that Dialogflow also considers conversations that get to the
    /// conversation end page as successful even if they don't return
    /// [ConversationSuccess][google.cloud.dialogflow.cx.v3.ResponseMessage.ConversationSuccess].
    ///
    /// You may set this, for example:
    /// * In the [entry_fulfillment][google.cloud.dialogflow.cx.v3.Page.entry_fulfillment] of a [Page][google.cloud.dialogflow.cx.v3.Page] if
    ///   entering the page indicates that the conversation succeeded.
    /// * In a webhook response when you determine that you handled the customer
    ///   issue.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConversationSuccess {
        /// Custom metadata. Dialogflow doesn't impose any structure on this.
        #[prost(message, optional, tag = "1")]
        pub metadata: ::core::option::Option<::prost_types::Struct>,
    }
    /// A text or ssml response that is preferentially used for TTS output audio
    /// synthesis, as described in the comment on the ResponseMessage message.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OutputAudioText {
        /// Output only. Whether the playback of this message can be interrupted by the end
        /// user's speech and the client can then starts the next Dialogflow
        /// request.
        #[prost(bool, tag = "3")]
        pub allow_playback_interruption: bool,
        /// The source, which is either plain text or SSML.
        #[prost(oneof = "output_audio_text::Source", tags = "1, 2")]
        pub source: ::core::option::Option<output_audio_text::Source>,
    }
    /// Nested message and enum types in `OutputAudioText`.
    pub mod output_audio_text {
        /// The source, which is either plain text or SSML.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Source {
            /// The raw text to be synthesized.
            #[prost(string, tag = "1")]
            Text(::prost::alloc::string::String),
            /// The SSML text to be synthesized. For more information, see
            /// [SSML](/speech/text-to-speech/docs/ssml).
            #[prost(string, tag = "2")]
            Ssml(::prost::alloc::string::String),
        }
    }
    /// Indicates that interaction with the Dialogflow agent has ended.
    /// This message is generated by Dialogflow only and not supposed to be
    /// defined by the user.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EndInteraction {}
    /// Specifies an audio clip to be played by the client as part of the response.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlayAudio {
        /// Required. URI of the audio clip. Dialogflow does not impose any validation on this
        /// value. It is specific to the client that reads it.
        #[prost(string, tag = "1")]
        pub audio_uri: ::prost::alloc::string::String,
        /// Output only. Whether the playback of this message can be interrupted by the end
        /// user's speech and the client can then starts the next Dialogflow
        /// request.
        #[prost(bool, tag = "2")]
        pub allow_playback_interruption: bool,
    }
    /// Represents an audio message that is composed of both segments
    /// synthesized from the Dialogflow agent prompts and ones hosted externally
    /// at the specified URIs.
    /// The external URIs are specified via
    /// [play_audio][google.cloud.dialogflow.cx.v3.ResponseMessage.play_audio].
    /// This message is generated by Dialogflow only and not supposed to be
    /// defined by the user.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MixedAudio {
        /// Segments this audio response is composed of.
        #[prost(message, repeated, tag = "1")]
        pub segments: ::prost::alloc::vec::Vec<mixed_audio::Segment>,
    }
    /// Nested message and enum types in `MixedAudio`.
    pub mod mixed_audio {
        /// Represents one segment of audio.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Segment {
            /// Output only. Whether the playback of this segment can be interrupted by the end
            /// user's speech and the client should then start the next Dialogflow
            /// request.
            #[prost(bool, tag = "3")]
            pub allow_playback_interruption: bool,
            /// Content of the segment.
            #[prost(oneof = "segment::Content", tags = "1, 2")]
            pub content: ::core::option::Option<segment::Content>,
        }
        /// Nested message and enum types in `Segment`.
        pub mod segment {
            /// Content of the segment.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Content {
                /// Raw audio synthesized from the Dialogflow agent's response using
                /// the output config specified in the request.
                #[prost(bytes, tag = "1")]
                Audio(::prost::alloc::vec::Vec<u8>),
                /// Client-specific URI that points to an audio clip accessible to the
                /// client. Dialogflow does not impose any validation on it.
                #[prost(string, tag = "2")]
                Uri(::prost::alloc::string::String),
            }
        }
    }
    /// Required. The rich response message.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// Returns a text response.
        #[prost(message, tag = "1")]
        Text(Text),
        /// Returns a response containing a custom, platform-specific payload.
        #[prost(message, tag = "2")]
        Payload(::prost_types::Struct),
        /// Indicates that the conversation succeeded.
        #[prost(message, tag = "9")]
        ConversationSuccess(ConversationSuccess),
        /// A text or ssml response that is preferentially used for TTS output audio
        /// synthesis, as described in the comment on the ResponseMessage message.
        #[prost(message, tag = "8")]
        OutputAudioText(OutputAudioText),
        /// Hands off conversation to a human agent.
        #[prost(message, tag = "10")]
        LiveAgentHandoff(LiveAgentHandoff),
        /// Output only. A signal that indicates the interaction with the Dialogflow agent has
        /// ended.
        /// This message is generated by Dialogflow only when the conversation
        /// reaches `END_SESSION` page. It is not supposed to be defined by the user.
        ///
        /// It's guaranteed that there is at most one such message in each response.
        #[prost(message, tag = "11")]
        EndInteraction(EndInteraction),
        /// Signal that the client should play an audio clip hosted at a
        /// client-specific URI. Dialogflow uses this to construct
        /// [mixed_audio][google.cloud.dialogflow.cx.v3.ResponseMessage.mixed_audio]. However, Dialogflow itself
        /// does not try to read or process the URI in any way.
        #[prost(message, tag = "12")]
        PlayAudio(PlayAudio),
        /// Output only. An audio response message composed of both the synthesized Dialogflow
        /// agent responses and responses defined via
        /// [play_audio][google.cloud.dialogflow.cx.v3.ResponseMessage.play_audio].
        /// This message is generated by Dialogflow only and not supposed to be
        /// defined by the user.
        #[prost(message, tag = "13")]
        MixedAudio(MixedAudio),
    }
}
/// A fulfillment can do one or more of the following actions at the same time:
///
///   * Generate rich message responses.
///   * Set parameter values.
///   * Call the webhook.
///
/// Fulfillments can be called at various stages in the [Page][google.cloud.dialogflow.cx.v3.Page] or
/// [Form][google.cloud.dialogflow.cx.v3.Form] lifecycle. For example, when a [DetectIntentRequest][google.cloud.dialogflow.cx.v3.DetectIntentRequest] drives a
/// session to enter a new page, the page's entry fulfillment can add a static
/// response to the [QueryResult][google.cloud.dialogflow.cx.v3.QueryResult] in the returning [DetectIntentResponse][google.cloud.dialogflow.cx.v3.DetectIntentResponse],
/// call the webhook (for example, to load user data from a database), or both.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fulfillment {
    /// The list of rich message responses to present to the user.
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<ResponseMessage>,
    /// The webhook to call.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/webhooks/<Webhook ID>`.
    #[prost(string, tag = "2")]
    pub webhook: ::prost::alloc::string::String,
    /// Whether Dialogflow should return currently queued fulfillment response
    /// messages in streaming APIs. If a webhook is specified, it happens before
    /// Dialogflow invokes webhook.
    /// Warning:
    /// 1) This flag only affects streaming API. Responses are still queued
    /// and returned once in non-streaming API.
    /// 2) The flag can be enabled in any fulfillment but only the first 3 partial
    /// responses will be returned. You may only want to apply it to fulfillments
    /// that have slow webhooks.
    #[prost(bool, tag = "8")]
    pub return_partial_responses: bool,
    /// The tag used by the webhook to identify which fulfillment is being called.
    /// This field is required if `webhook` is specified.
    #[prost(string, tag = "3")]
    pub tag: ::prost::alloc::string::String,
    /// Set parameter values before executing the webhook.
    #[prost(message, repeated, tag = "4")]
    pub set_parameter_actions: ::prost::alloc::vec::Vec<fulfillment::SetParameterAction>,
    /// Conditional cases for this fulfillment.
    #[prost(message, repeated, tag = "5")]
    pub conditional_cases: ::prost::alloc::vec::Vec<fulfillment::ConditionalCases>,
}
/// Nested message and enum types in `Fulfillment`.
pub mod fulfillment {
    /// Setting a parameter value.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetParameterAction {
        /// Display name of the parameter.
        #[prost(string, tag = "1")]
        pub parameter: ::prost::alloc::string::String,
        /// The new value of the parameter. A null value clears the parameter.
        #[prost(message, optional, tag = "2")]
        pub value: ::core::option::Option<::prost_types::Value>,
    }
    /// A list of cascading if-else conditions. Cases are mutually exclusive.
    /// The first one with a matching condition is selected, all the rest ignored.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConditionalCases {
        /// A list of cascading if-else conditions.
        #[prost(message, repeated, tag = "1")]
        pub cases: ::prost::alloc::vec::Vec<conditional_cases::Case>,
    }
    /// Nested message and enum types in `ConditionalCases`.
    pub mod conditional_cases {
        /// Each case has a Boolean condition. When it is evaluated to be True, the
        /// corresponding messages will be selected and evaluated recursively.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Case {
            /// The condition to activate and select this case. Empty means the
            /// condition is always true. The condition is evaluated against [form
            /// parameters][Form.parameters] or [session
            /// parameters][SessionInfo.parameters].
            ///
            /// See the [conditions
            /// reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition).
            #[prost(string, tag = "1")]
            pub condition: ::prost::alloc::string::String,
            /// A list of case content.
            #[prost(message, repeated, tag = "2")]
            pub case_content: ::prost::alloc::vec::Vec<case::CaseContent>,
        }
        /// Nested message and enum types in `Case`.
        pub mod case {
            /// The list of messages or conditional cases to activate for this case.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct CaseContent {
                /// Either a message is returned or additional cases to be evaluated.
                #[prost(oneof = "case_content::CasesOrMessage", tags = "1, 2")]
                pub cases_or_message: ::core::option::Option<case_content::CasesOrMessage>,
            }
            /// Nested message and enum types in `CaseContent`.
            pub mod case_content {
                /// Either a message is returned or additional cases to be evaluated.
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum CasesOrMessage {
                    /// Returned message.
                    #[prost(message, tag = "1")]
                    Message(super::super::super::super::ResponseMessage),
                    /// Additional cases to be evaluated.
                    #[prost(message, tag = "2")]
                    AdditionalCases(super::super::super::ConditionalCases),
                }
            }
        }
    }
}
/// A Dialogflow CX conversation (session) can be described and visualized as a
/// state machine. The states of a CX session are represented by pages.
///
/// For each flow, you define many pages, where your combined pages can handle a
/// complete conversation on the topics the flow is designed for. At any given
/// moment, exactly one page is the current page, the current page is considered
/// active, and the flow associated with that page is considered active. Every
/// flow has a special start page. When a flow initially becomes active, the
/// start page page becomes the current page. For each conversational turn, the
/// current page will either stay the same or transition to another page.
///
/// You configure each page to collect information from the end-user that is
/// relevant for the conversational state represented by the page.
///
/// For more information, see the
/// [Page guide](https://cloud.google.com/dialogflow/cx/docs/concept/page).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Page {
    /// The unique identifier of the page.
    /// Required for the [Pages.UpdatePage][google.cloud.dialogflow.cx.v3.Pages.UpdatePage] method. [Pages.CreatePage][google.cloud.dialogflow.cx.v3.Pages.CreatePage]
    /// populates the name automatically.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/pages/<Page ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The human-readable name of the page, unique within the agent.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The fulfillment to call when the session is entering the page.
    #[prost(message, optional, tag = "7")]
    pub entry_fulfillment: ::core::option::Option<Fulfillment>,
    /// The form associated with the page, used for collecting parameters
    /// relevant to the page.
    #[prost(message, optional, tag = "4")]
    pub form: ::core::option::Option<Form>,
    /// Ordered list of [`TransitionRouteGroups`][google.cloud.dialogflow.cx.v3.TransitionRouteGroup] associated
    /// with the page. Transition route groups must be unique within a page.
    ///
    /// *   If multiple transition routes within a page scope refer to the same
    ///     intent, then the precedence order is: page's transition route -> page's
    ///     transition route group -> flow's transition routes.
    ///
    /// *   If multiple transition route groups within a page contain the same
    ///     intent, then the first group in the ordered list takes precedence.
    ///
    /// Format:`projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>`.
    #[prost(string, repeated, tag = "11")]
    pub transition_route_groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of transitions for the transition rules of this page.
    /// They route the conversation to another page in the same flow, or another
    /// flow.
    ///
    /// When we are in a certain page, the TransitionRoutes are evalauted in the
    /// following order:
    ///
    /// *   TransitionRoutes defined in the page with intent specified.
    /// *   TransitionRoutes defined in the
    ///     [transition route groups][google.cloud.dialogflow.cx.v3.Page.transition_route_groups] with intent
    ///     specified.
    /// *   TransitionRoutes defined in flow with intent specified.
    /// *   TransitionRoutes defined in the
    ///     [transition route groups][google.cloud.dialogflow.cx.v3.Flow.transition_route_groups] with intent
    ///     specified.
    /// *   TransitionRoutes defined in the page with only condition specified.
    /// *   TransitionRoutes defined in the
    ///     [transition route groups][google.cloud.dialogflow.cx.v3.Page.transition_route_groups] with only
    ///     condition specified.
    #[prost(message, repeated, tag = "9")]
    pub transition_routes: ::prost::alloc::vec::Vec<TransitionRoute>,
    /// Handlers associated with the page to handle events such as webhook errors,
    /// no match or no input.
    #[prost(message, repeated, tag = "10")]
    pub event_handlers: ::prost::alloc::vec::Vec<EventHandler>,
}
/// A form is a data model that groups related parameters that can be collected
/// from the user. The process in which the agent prompts the user and collects
/// parameter values from the user is called form filling. A form can be added to
/// a [page][google.cloud.dialogflow.cx.v3.Page]. When form filling is done, the filled parameters will be
/// written to the [session][google.cloud.dialogflow.cx.v3.SessionInfo.parameters].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Form {
    /// Parameters to collect from the user.
    #[prost(message, repeated, tag = "1")]
    pub parameters: ::prost::alloc::vec::Vec<form::Parameter>,
}
/// Nested message and enum types in `Form`.
pub mod form {
    /// Represents a form parameter.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Parameter {
        /// Required. The human-readable name of the parameter, unique within the
        /// form.
        #[prost(string, tag = "1")]
        pub display_name: ::prost::alloc::string::String,
        /// Indicates whether the parameter is required. Optional parameters will not
        /// trigger prompts; however, they are filled if the user specifies them.
        /// Required parameters must be filled before form filling concludes.
        #[prost(bool, tag = "2")]
        pub required: bool,
        /// Required. The entity type of the parameter.
        /// Format: `projects/-/locations/-/agents/-/entityTypes/<System Entity Type
        /// ID>` for system entity types (for example,
        /// `projects/-/locations/-/agents/-/entityTypes/sys.date`), or
        /// `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/entityTypes/<Entity Type ID>` for developer entity types.
        #[prost(string, tag = "3")]
        pub entity_type: ::prost::alloc::string::String,
        /// Indicates whether the parameter represents a list of values.
        #[prost(bool, tag = "4")]
        pub is_list: bool,
        /// Required. Defines fill behavior for the parameter.
        #[prost(message, optional, tag = "7")]
        pub fill_behavior: ::core::option::Option<parameter::FillBehavior>,
        /// The default value of an optional parameter. If the parameter is required,
        /// the default value will be ignored.
        #[prost(message, optional, tag = "9")]
        pub default_value: ::core::option::Option<::prost_types::Value>,
        /// Indicates whether the parameter content should be redacted in log.  If
        /// redaction is enabled, the parameter content will be replaced by parameter
        /// name during logging.
        /// Note: the parameter content is subject to redaction if either parameter
        /// level redaction or [entity type level redaction][google.cloud.dialogflow.cx.v3.EntityType.redact] is
        /// enabled.
        #[prost(bool, tag = "11")]
        pub redact: bool,
    }
    /// Nested message and enum types in `Parameter`.
    pub mod parameter {
        /// Configuration for how the filling of a parameter should be handled.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FillBehavior {
            /// Required. The fulfillment to provide the initial prompt that the agent
            /// can present to the user in order to fill the parameter.
            #[prost(message, optional, tag = "3")]
            pub initial_prompt_fulfillment: ::core::option::Option<super::super::Fulfillment>,
            /// The handlers for parameter-level events, used to provide reprompt for
            /// the parameter or transition to a different page/flow. The supported
            /// events are:
            /// *   `sys.no-match-<N>`, where N can be from 1 to 6
            /// *   `sys.no-match-default`
            /// *   `sys.no-input-<N>`, where N can be from 1 to 6
            /// *   `sys.no-input-default`
            /// *   `sys.invalid-parameter`
            ///
            /// `initial_prompt_fulfillment` provides the first prompt for the
            /// parameter.
            ///
            /// If the user's response does not fill the parameter, a
            /// no-match/no-input event will be triggered, and the fulfillment
            /// associated with the `sys.no-match-1`/`sys.no-input-1` handler (if
            /// defined) will be called to provide a prompt. The
            /// `sys.no-match-2`/`sys.no-input-2` handler (if defined) will respond to
            /// the next no-match/no-input event, and so on.
            ///
            /// A `sys.no-match-default` or `sys.no-input-default` handler will be used
            /// to handle all following no-match/no-input events after all numbered
            /// no-match/no-input handlers for the parameter are consumed.
            ///
            /// A `sys.invalid-parameter` handler can be defined to handle the case
            /// where the parameter values have been `invalidated` by webhook. For
            /// example, if the user's response fill the parameter, however the
            /// parameter was invalidated by webhook, the fulfillment associated with
            /// the `sys.invalid-parameter` handler (if defined) will be called to
            /// provide a prompt.
            ///
            /// If the event handler for the corresponding event can't be found on the
            /// parameter, `initial_prompt_fulfillment` will be re-prompted.
            #[prost(message, repeated, tag = "5")]
            pub reprompt_event_handlers: ::prost::alloc::vec::Vec<super::super::EventHandler>,
        }
    }
}
/// An event handler specifies an [event][google.cloud.dialogflow.cx.v3.EventHandler.event] that can be handled
/// during a session. When the specified event happens, the following actions are
/// taken in order:
///
/// *   If there is a
/// [`trigger_fulfillment`][google.cloud.dialogflow.cx.v3.EventHandler.trigger_fulfillment] associated with
/// the event, it will be called.
/// *   If there is a [`target_page`][google.cloud.dialogflow.cx.v3.EventHandler.target_page] associated
/// with the event, the session will transition into the specified page.
/// *   If there is a [`target_flow`][google.cloud.dialogflow.cx.v3.EventHandler.target_flow] associated
/// with the event, the session will transition into the specified flow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventHandler {
    /// Output only. The unique identifier of this event handler.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    /// Required. The name of the event to handle.
    #[prost(string, tag = "4")]
    pub event: ::prost::alloc::string::String,
    /// The fulfillment to call when the event occurs.
    /// Handling webhook errors with a fulfillment enabled with webhook could
    /// cause infinite loop. It is invalid to specify such fulfillment for a
    /// handler handling webhooks.
    #[prost(message, optional, tag = "5")]
    pub trigger_fulfillment: ::core::option::Option<Fulfillment>,
    /// The target to transition to, either a page in the same host flow (the flow
    /// that owns this [TransitionRoute][google.cloud.dialogflow.cx.v3.TransitionRoute]), or another flow in the same agent.
    #[prost(oneof = "event_handler::Target", tags = "2, 3")]
    pub target: ::core::option::Option<event_handler::Target>,
}
/// Nested message and enum types in `EventHandler`.
pub mod event_handler {
    /// The target to transition to, either a page in the same host flow (the flow
    /// that owns this [TransitionRoute][google.cloud.dialogflow.cx.v3.TransitionRoute]), or another flow in the same agent.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// The target page to transition to.
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/flows/<Flow ID>/pages/<Page ID>`.
        #[prost(string, tag = "2")]
        TargetPage(::prost::alloc::string::String),
        /// The target flow to transition to.
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/flows/<Flow ID>`.
        #[prost(string, tag = "3")]
        TargetFlow(::prost::alloc::string::String),
    }
}
/// A transition route specifies a [intent][google.cloud.dialogflow.cx.v3.Intent] that can be matched and/or a
/// data condition that can be evaluated during a session. When a specified
/// transition is matched, the following actions are taken in order:
///
/// *   If there is a
/// [`trigger_fulfillment`][google.cloud.dialogflow.cx.v3.TransitionRoute.trigger_fulfillment] associated with
/// the transition, it will be called.
/// *   If there is a [`target_page`][google.cloud.dialogflow.cx.v3.TransitionRoute.target_page] associated
/// with the transition, the session will transition into the specified page.
/// *   If there is a [`target_flow`][google.cloud.dialogflow.cx.v3.TransitionRoute.target_flow] associated
/// with the transition, the session will transition into the specified flow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitionRoute {
    /// Output only. The unique identifier of this transition route.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    /// The unique identifier of an [Intent][google.cloud.dialogflow.cx.v3.Intent].
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/intents/<Intent ID>`.
    /// Indicates that the transition can only happen when the given intent is
    /// matched.
    /// At least one of `intent` or `condition` must be specified. When both
    /// `intent` and `condition` are specified, the transition can only happen
    /// when both are fulfilled.
    #[prost(string, tag = "1")]
    pub intent: ::prost::alloc::string::String,
    /// The condition to evaluate against [form parameters][google.cloud.dialogflow.cx.v3.Form.parameters] or
    /// [session parameters][google.cloud.dialogflow.cx.v3.SessionInfo.parameters].
    ///
    /// See the [conditions
    /// reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition).
    /// At least one of `intent` or `condition` must be specified. When both
    /// `intent` and `condition` are specified, the transition can only happen
    /// when both are fulfilled.
    #[prost(string, tag = "2")]
    pub condition: ::prost::alloc::string::String,
    /// The fulfillment to call when the condition is satisfied. At least one of
    /// `trigger_fulfillment` and `target` must be specified. When both are
    /// defined, `trigger_fulfillment` is executed first.
    #[prost(message, optional, tag = "3")]
    pub trigger_fulfillment: ::core::option::Option<Fulfillment>,
    /// The target to transition to, either a page in the same host flow (the flow
    /// that owns this [TransitionRoute][google.cloud.dialogflow.cx.v3.TransitionRoute]), or another flow in the same agent.
    #[prost(oneof = "transition_route::Target", tags = "4, 5")]
    pub target: ::core::option::Option<transition_route::Target>,
}
/// Nested message and enum types in `TransitionRoute`.
pub mod transition_route {
    /// The target to transition to, either a page in the same host flow (the flow
    /// that owns this [TransitionRoute][google.cloud.dialogflow.cx.v3.TransitionRoute]), or another flow in the same agent.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// The target page to transition to.
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/flows/<Flow ID>/pages/<Page ID>`.
        #[prost(string, tag = "4")]
        TargetPage(::prost::alloc::string::String),
        /// The target flow to transition to.
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/flows/<Flow ID>`.
        #[prost(string, tag = "5")]
        TargetFlow(::prost::alloc::string::String),
    }
}
/// The request message for [Pages.ListPages][google.cloud.dialogflow.cx.v3.Pages.ListPages].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPagesRequest {
    /// Required. The flow to list all pages for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The language to list pages for. The following fields are language
    /// dependent:
    ///
    /// *  `Page.entry_fulfillment.messages`
    /// *  `Page.entry_fulfillment.conditional_cases`
    /// *  `Page.event_handlers.trigger_fulfillment.messages`
    /// *  `Page.event_handlers.trigger_fulfillment.conditional_cases`
    /// *  `Page.form.parameters.fill_behavior.initial_prompt_fulfillment.messages`
    /// *
    /// `Page.form.parameters.fill_behavior.initial_prompt_fulfillment.conditional_cases`
    /// *  `Page.form.parameters.fill_behavior.reprompt_event_handlers.messages`
    /// *
    /// `Page.form.parameters.fill_behavior.reprompt_event_handlers.conditional_cases`
    /// *  `Page.transition_routes.trigger_fulfillment.messages`
    /// *  `Page.transition_routes.trigger_fulfillment.conditional_cases`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for [Pages.ListPages][google.cloud.dialogflow.cx.v3.Pages.ListPages].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPagesResponse {
    /// The list of pages. There will be a maximum number of items returned based
    /// on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub pages: ::prost::alloc::vec::Vec<Page>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for [Pages.GetPage][google.cloud.dialogflow.cx.v3.Pages.GetPage].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPageRequest {
    /// Required. The name of the page.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/pages/<Page ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The language to retrieve the page for. The following fields are language
    /// dependent:
    ///
    /// *  `Page.entry_fulfillment.messages`
    /// *  `Page.entry_fulfillment.conditional_cases`
    /// *  `Page.event_handlers.trigger_fulfillment.messages`
    /// *  `Page.event_handlers.trigger_fulfillment.conditional_cases`
    /// *  `Page.form.parameters.fill_behavior.initial_prompt_fulfillment.messages`
    /// *
    /// `Page.form.parameters.fill_behavior.initial_prompt_fulfillment.conditional_cases`
    /// *  `Page.form.parameters.fill_behavior.reprompt_event_handlers.messages`
    /// *
    /// `Page.form.parameters.fill_behavior.reprompt_event_handlers.conditional_cases`
    /// *  `Page.transition_routes.trigger_fulfillment.messages`
    /// *  `Page.transition_routes.trigger_fulfillment.conditional_cases`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
}
/// The request message for [Pages.CreatePage][google.cloud.dialogflow.cx.v3.Pages.CreatePage].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePageRequest {
    /// Required. The flow to create a page for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The page to create.
    #[prost(message, optional, tag = "2")]
    pub page: ::core::option::Option<Page>,
    /// The language of the following fields in `page`:
    ///
    /// *  `Page.entry_fulfillment.messages`
    /// *  `Page.entry_fulfillment.conditional_cases`
    /// *  `Page.event_handlers.trigger_fulfillment.messages`
    /// *  `Page.event_handlers.trigger_fulfillment.conditional_cases`
    /// *  `Page.form.parameters.fill_behavior.initial_prompt_fulfillment.messages`
    /// *
    /// `Page.form.parameters.fill_behavior.initial_prompt_fulfillment.conditional_cases`
    /// *  `Page.form.parameters.fill_behavior.reprompt_event_handlers.messages`
    /// *
    /// `Page.form.parameters.fill_behavior.reprompt_event_handlers.conditional_cases`
    /// *  `Page.transition_routes.trigger_fulfillment.messages`
    /// *  `Page.transition_routes.trigger_fulfillment.conditional_cases`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// The request message for [Pages.UpdatePage][google.cloud.dialogflow.cx.v3.Pages.UpdatePage].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePageRequest {
    /// Required. The page to update.
    #[prost(message, optional, tag = "1")]
    pub page: ::core::option::Option<Page>,
    /// The language of the following fields in `page`:
    ///
    /// *  `Page.entry_fulfillment.messages`
    /// *  `Page.entry_fulfillment.conditional_cases`
    /// *  `Page.event_handlers.trigger_fulfillment.messages`
    /// *  `Page.event_handlers.trigger_fulfillment.conditional_cases`
    /// *  `Page.form.parameters.fill_behavior.initial_prompt_fulfillment.messages`
    /// *
    /// `Page.form.parameters.fill_behavior.initial_prompt_fulfillment.conditional_cases`
    /// *  `Page.form.parameters.fill_behavior.reprompt_event_handlers.messages`
    /// *
    /// `Page.form.parameters.fill_behavior.reprompt_event_handlers.conditional_cases`
    /// *  `Page.transition_routes.trigger_fulfillment.messages`
    /// *  `Page.transition_routes.trigger_fulfillment.conditional_cases`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// The mask to control which fields get updated. If the mask is not present,
    /// all fields will be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for [Pages.DeletePage][google.cloud.dialogflow.cx.v3.Pages.DeletePage].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePageRequest {
    /// Required. The name of the page to delete.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/Flows/<flow ID>/pages/<Page ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// This field has no effect for pages with no incoming transitions.
    /// For pages with incoming transitions:
    ///
    /// *  If `force` is set to false, an error will be returned with message
    ///    indicating the incoming transitions.
    /// *  If `force` is set to true, Dialogflow will remove the page, as well as
    ///    any transitions to the page (i.e. [Target
    ///    page][EventHandler.target_page] in event handlers or [Target
    ///    page][TransitionRoute.target_page] in transition routes that point to
    ///    this page will be cleared).
    #[prost(bool, tag = "2")]
    pub force: bool,
}
#[doc = r" Generated client implementations."]
pub mod pages_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for managing [Pages][google.cloud.dialogflow.cx.v3.Page]."]
    #[derive(Debug, Clone)]
    pub struct PagesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PagesClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
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
        ) -> PagesClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            PagesClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns the list of all pages in the specified flow."]
        pub async fn list_pages(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPagesRequest>,
        ) -> Result<tonic::Response<super::ListPagesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Pages/ListPages",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified page."]
        pub async fn get_page(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPageRequest>,
        ) -> Result<tonic::Response<super::Page>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Pages/GetPage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a page in the specified flow."]
        #[doc = ""]
        #[doc = " Note: You should always train a flow prior to sending it queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn create_page(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePageRequest>,
        ) -> Result<tonic::Response<super::Page>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Pages/CreatePage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified page."]
        #[doc = ""]
        #[doc = " Note: You should always train a flow prior to sending it queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn update_page(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePageRequest>,
        ) -> Result<tonic::Response<super::Page>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Pages/UpdatePage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified page."]
        #[doc = ""]
        #[doc = " Note: You should always train a flow prior to sending it queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn delete_page(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePageRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Pages/DeletePage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Agent/flow validation message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationMessage {
    /// The type of the resources where the message is found.
    #[prost(enumeration = "validation_message::ResourceType", tag = "1")]
    pub resource_type: i32,
    /// The names of the resources where the message is found.
    #[deprecated]
    #[prost(string, repeated, tag = "2")]
    pub resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The resource names of the resources where the message is found.
    #[prost(message, repeated, tag = "6")]
    pub resource_names: ::prost::alloc::vec::Vec<ResourceName>,
    /// Indicates the severity of the message.
    #[prost(enumeration = "validation_message::Severity", tag = "3")]
    pub severity: i32,
    /// The message detail.
    #[prost(string, tag = "4")]
    pub detail: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ValidationMessage`.
pub mod validation_message {
    /// Resource types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResourceType {
        /// Unspecified.
        Unspecified = 0,
        /// Agent.
        Agent = 1,
        /// Intent.
        Intent = 2,
        /// Intent training phrase.
        IntentTrainingPhrase = 8,
        /// Intent parameter.
        IntentParameter = 9,
        /// Multiple intents.
        Intents = 10,
        /// Multiple training phrases.
        IntentTrainingPhrases = 11,
        /// Entity type.
        EntityType = 3,
        /// Multiple entity types.
        EntityTypes = 12,
        /// Webhook.
        Webhook = 4,
        /// Flow.
        Flow = 5,
        /// Page.
        Page = 6,
        /// Multiple pages.
        Pages = 13,
        /// Transition route group.
        TransitionRouteGroup = 7,
    }
    /// Severity level.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        /// Unspecified.
        Unspecified = 0,
        /// The agent doesn't follow Dialogflow best practices.
        Info = 1,
        /// The agent may not behave as expected.
        Warning = 2,
        /// The agent may experience failures.
        Error = 3,
    }
}
/// Resource name and display name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceName {
    /// Name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Display name.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
}
/// Settings related to NLU.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NluSettings {
    /// Indicates the type of NLU model.
    #[prost(enumeration = "nlu_settings::ModelType", tag = "1")]
    pub model_type: i32,
    /// To filter out false positive results and still get variety in matched
    /// natural language inputs for your agent, you can tune the machine learning
    /// classification threshold. If the returned score value is less than the
    /// threshold value, then a no-match event will be triggered. The score values
    /// range from 0.0 (completely uncertain) to 1.0 (completely certain). If set
    /// to 0.0, the default of 0.3 is used.
    #[prost(float, tag = "3")]
    pub classification_threshold: f32,
    /// Indicates NLU model training mode.
    #[prost(enumeration = "nlu_settings::ModelTrainingMode", tag = "4")]
    pub model_training_mode: i32,
}
/// Nested message and enum types in `NluSettings`.
pub mod nlu_settings {
    /// NLU model type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ModelType {
        /// Not specified. `MODEL_TYPE_STANDARD` will be used.
        Unspecified = 0,
        /// Use standard NLU model.
        Standard = 1,
        /// Use advanced NLU model.
        Advanced = 3,
    }
    /// NLU model training mode.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ModelTrainingMode {
        /// Not specified. `MODEL_TRAINING_MODE_AUTOMATIC` will be used.
        Unspecified = 0,
        /// NLU model training is automatically triggered when a flow gets modified.
        /// User can also manually trigger model training in this mode.
        Automatic = 1,
        /// User needs to manually trigger NLU model training. Best for large flows
        /// whose models take long time to train.
        Manual = 2,
    }
}
/// Flows represents the conversation flows when you build your chatbot agent.
///
/// A flow consists of many pages connected by the transition routes.
/// Conversations always start with the built-in Start Flow (with an all-0 ID).
/// Transition routes can direct the conversation session from the current flow
/// (parent flow) to another flow (sub flow). When the sub flow is finished,
/// Dialogflow will bring the session back to the parent flow, where the sub flow
/// is started.
///
/// Usually, when a transition route is followed by a matched intent, the intent
/// will be "consumed". This means the intent won't activate more transition
/// routes. However, when the followed transition route moves the conversation
/// session into a different flow, the matched intent can be carried over and to
/// be consumed in the target flow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flow {
    /// The unique identifier of the flow.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The human-readable name of the flow.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The description of the flow. The maximum length is 500 characters. If
    /// exceeded, the request is rejected.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// A flow's transition routes serve two purposes:
    ///
    /// *   They are responsible for matching the user's first utterances in the
    /// flow.
    /// *   They are inherited by every page's [transition
    /// routes][Page.transition_routes] and can support use cases such as the user
    /// saying "help" or "can I talk to a human?", which can be handled in a common
    /// way regardless of the current page. Transition routes defined in the page
    /// have higher priority than those defined in the flow.
    ///
    /// TransitionRoutes are evalauted in the following order:
    ///
    /// *   TransitionRoutes with intent specified..
    /// *   TransitionRoutes with only condition specified.
    ///
    /// TransitionRoutes with intent specified are inherited by pages in the flow.
    #[prost(message, repeated, tag = "4")]
    pub transition_routes: ::prost::alloc::vec::Vec<TransitionRoute>,
    /// A flow's event handlers serve two purposes:
    ///
    /// *   They are responsible for handling events (e.g. no match,
    /// webhook errors) in the flow.
    /// *   They are inherited by every page's [event
    /// handlers][Page.event_handlers], which can be used to handle common events
    /// regardless of the current page. Event handlers defined in the page
    /// have higher priority than those defined in the flow.
    ///
    /// Unlike [transition_routes][google.cloud.dialogflow.cx.v3.Flow.transition_routes], these handlers are
    /// evaluated on a first-match basis. The first one that matches the event
    /// get executed, with the rest being ignored.
    #[prost(message, repeated, tag = "10")]
    pub event_handlers: ::prost::alloc::vec::Vec<EventHandler>,
    /// A flow's transition route group serve two purposes:
    ///
    /// *   They are responsible for matching the user's first utterances in the
    /// flow.
    /// *   They are inherited by every page's [transition
    /// route groups][Page.transition_route_groups]. Transition route groups
    /// defined in the page have higher priority than those defined in the flow.
    ///
    /// Format:`projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>`.
    #[prost(string, repeated, tag = "15")]
    pub transition_route_groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// NLU related settings of the flow.
    #[prost(message, optional, tag = "11")]
    pub nlu_settings: ::core::option::Option<NluSettings>,
}
/// The request message for [Flows.CreateFlow][google.cloud.dialogflow.cx.v3.Flows.CreateFlow].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFlowRequest {
    /// Required. The agent to create a flow for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The flow to create.
    #[prost(message, optional, tag = "2")]
    pub flow: ::core::option::Option<Flow>,
    /// The language of the following fields in `flow`:
    ///
    /// *  `Flow.event_handlers.trigger_fulfillment.messages`
    /// *  `Flow.event_handlers.trigger_fulfillment.conditional_cases`
    /// *  `Flow.transition_routes.trigger_fulfillment.messages`
    /// *  `Flow.transition_routes.trigger_fulfillment.conditional_cases`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// The request message for [Flows.DeleteFlow][google.cloud.dialogflow.cx.v3.Flows.DeleteFlow].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFlowRequest {
    /// Required. The name of the flow to delete.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// This field has no effect for flows with no incoming transitions.
    /// For flows with incoming transitions:
    ///
    /// *  If `force` is set to false, an error will be returned with message
    ///    indicating the incoming transitions.
    /// *  If `force` is set to true, Dialogflow will remove the flow, as well as
    ///    any transitions to the flow (i.e. [Target
    ///    flow][EventHandler.target_flow] in event handlers or [Target
    ///    flow][TransitionRoute.target_flow] in transition routes that point to
    ///    this flow will be cleared).
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// The request message for [Flows.ListFlows][google.cloud.dialogflow.cx.v3.Flows.ListFlows].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFlowsRequest {
    /// Required. The agent containing the flows.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The language to list flows for. The following fields are language
    /// dependent:
    ///
    /// *  `Flow.event_handlers.trigger_fulfillment.messages`
    /// *  `Flow.event_handlers.trigger_fulfillment.conditional_cases`
    /// *  `Flow.transition_routes.trigger_fulfillment.messages`
    /// *  `Flow.transition_routes.trigger_fulfillment.conditional_cases`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "4")]
    pub language_code: ::prost::alloc::string::String,
}
/// The response message for [Flows.ListFlows][google.cloud.dialogflow.cx.v3.Flows.ListFlows].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFlowsResponse {
    /// The list of flows. There will be a maximum number of items returned based
    /// on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub flows: ::prost::alloc::vec::Vec<Flow>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The response message for [Flows.GetFlow][google.cloud.dialogflow.cx.v3.Flows.GetFlow].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFlowRequest {
    /// Required. The name of the flow to get.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The language to retrieve the flow for. The following fields are language
    /// dependent:
    ///
    /// *  `Flow.event_handlers.trigger_fulfillment.messages`
    /// *  `Flow.event_handlers.trigger_fulfillment.conditional_cases`
    /// *  `Flow.transition_routes.trigger_fulfillment.messages`
    /// *  `Flow.transition_routes.trigger_fulfillment.conditional_cases`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
}
/// The request message for [Flows.UpdateFlow][google.cloud.dialogflow.cx.v3.Flows.UpdateFlow].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFlowRequest {
    /// Required. The flow to update.
    #[prost(message, optional, tag = "1")]
    pub flow: ::core::option::Option<Flow>,
    /// Required. The mask to control which fields get updated. If `update_mask` is not
    /// specified, an error will be returned.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The language of the following fields in `flow`:
    ///
    /// *  `Flow.event_handlers.trigger_fulfillment.messages`
    /// *  `Flow.event_handlers.trigger_fulfillment.conditional_cases`
    /// *  `Flow.transition_routes.trigger_fulfillment.messages`
    /// *  `Flow.transition_routes.trigger_fulfillment.conditional_cases`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// The request message for [Flows.TrainFlow][google.cloud.dialogflow.cx.v3.Flows.TrainFlow].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrainFlowRequest {
    /// Required. The flow to train.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for [Flows.ValidateFlow][google.cloud.dialogflow.cx.v3.Flows.ValidateFlow].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateFlowRequest {
    /// Required. The flow to validate.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If not specified, the agent's default language is used.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
}
/// The request message for [Flows.GetFlowValidationResult][google.cloud.dialogflow.cx.v3.Flows.GetFlowValidationResult].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFlowValidationResultRequest {
    /// Required. The flow name.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/validationResult`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If not specified, the agent's default language is used.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
}
/// The response message for [Flows.GetFlowValidationResult][google.cloud.dialogflow.cx.v3.Flows.GetFlowValidationResult].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlowValidationResult {
    /// The unique identifier of the flow validation result.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/validationResult`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Contains all validation messages.
    #[prost(message, repeated, tag = "2")]
    pub validation_messages: ::prost::alloc::vec::Vec<ValidationMessage>,
    /// Last time the flow was validated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The request message for [Flows.ImportFlow][google.cloud.dialogflow.cx.v3.Flows.ImportFlow].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportFlowRequest {
    /// Required. The agent to import the flow into.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Flow import mode. If not specified, `KEEP` is assumed.
    #[prost(enumeration = "import_flow_request::ImportOption", tag = "4")]
    pub import_option: i32,
    /// Required. The flow to import.
    #[prost(oneof = "import_flow_request::Flow", tags = "2, 3")]
    pub flow: ::core::option::Option<import_flow_request::Flow>,
}
/// Nested message and enum types in `ImportFlowRequest`.
pub mod import_flow_request {
    /// Import option.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ImportOption {
        /// Unspecified. Treated as `KEEP`.
        Unspecified = 0,
        /// Always respect settings in exported flow content. It may cause a
        /// import failure if some settings (e.g. custom NLU) are not supported in
        /// the agent to import into.
        Keep = 1,
        /// Fallback to default settings if some settings are not supported in the
        /// agent to import into. E.g. Standard NLU will be used if custom NLU is
        /// not available.
        Fallback = 2,
    }
    /// Required. The flow to import.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Flow {
        /// The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI
        /// to import flow from. The format of this URI must be
        /// `gs://<bucket-name>/<object-name>`.
        #[prost(string, tag = "2")]
        FlowUri(::prost::alloc::string::String),
        /// Uncompressed raw byte content for flow.
        #[prost(bytes, tag = "3")]
        FlowContent(::prost::alloc::vec::Vec<u8>),
    }
}
/// The response message for [Flows.ImportFlow][google.cloud.dialogflow.cx.v3.Flows.ImportFlow].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportFlowResponse {
    /// The unique identifier of the new flow.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub flow: ::prost::alloc::string::String,
}
/// The request message for [Flows.ExportFlow][google.cloud.dialogflow.cx.v3.Flows.ExportFlow].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportFlowRequest {
    /// Required. The name of the flow to export.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI to
    /// export the flow to. The format of this URI must be
    /// `gs://<bucket-name>/<object-name>`.
    /// If left unspecified, the serialized flow is returned inline.
    #[prost(string, tag = "2")]
    pub flow_uri: ::prost::alloc::string::String,
    /// Optional. Whether to export flows referenced by the specified flow.
    #[prost(bool, tag = "4")]
    pub include_referenced_flows: bool,
}
/// The response message for [Flows.ExportFlow][google.cloud.dialogflow.cx.v3.Flows.ExportFlow].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportFlowResponse {
    /// The exported flow.
    #[prost(oneof = "export_flow_response::Flow", tags = "1, 2")]
    pub flow: ::core::option::Option<export_flow_response::Flow>,
}
/// Nested message and enum types in `ExportFlowResponse`.
pub mod export_flow_response {
    /// The exported flow.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Flow {
        /// The URI to a file containing the exported flow. This field is populated
        /// only if `flow_uri` is specified in [ExportFlowRequest][google.cloud.dialogflow.cx.v3.ExportFlowRequest].
        #[prost(string, tag = "1")]
        FlowUri(::prost::alloc::string::String),
        /// Uncompressed raw byte content for flow.
        #[prost(bytes, tag = "2")]
        FlowContent(::prost::alloc::vec::Vec<u8>),
    }
}
#[doc = r" Generated client implementations."]
pub mod flows_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for managing [Flows][google.cloud.dialogflow.cx.v3.Flow]."]
    #[derive(Debug, Clone)]
    pub struct FlowsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FlowsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
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
        ) -> FlowsClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            FlowsClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates a flow in the specified agent."]
        #[doc = ""]
        #[doc = " Note: You should always train a flow prior to sending it queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn create_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFlowRequest>,
        ) -> Result<tonic::Response<super::Flow>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Flows/CreateFlow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a specified flow."]
        pub async fn delete_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFlowRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Flows/DeleteFlow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the list of all flows in the specified agent."]
        pub async fn list_flows(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFlowsRequest>,
        ) -> Result<tonic::Response<super::ListFlowsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Flows/ListFlows",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified flow."]
        pub async fn get_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFlowRequest>,
        ) -> Result<tonic::Response<super::Flow>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Flows/GetFlow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified flow."]
        #[doc = ""]
        #[doc = " Note: You should always train a flow prior to sending it queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn update_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFlowRequest>,
        ) -> Result<tonic::Response<super::Flow>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Flows/UpdateFlow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Trains the specified flow. Note that only the flow in 'draft' environment"]
        #[doc = " is trained."]
        #[doc = ""]
        #[doc = " Note: You should always train a flow prior to sending it queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn train_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::TrainFlowRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.cx.v3.Flows/TrainFlow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Validates the specified flow and creates or updates validation results."]
        #[doc = " Please call this API after the training is completed to get the complete"]
        #[doc = " validation results."]
        pub async fn validate_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateFlowRequest>,
        ) -> Result<tonic::Response<super::FlowValidationResult>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Flows/ValidateFlow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the latest flow validation result. Flow validation is performed"]
        #[doc = " when ValidateFlow is called."]
        pub async fn get_flow_validation_result(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFlowValidationResultRequest>,
        ) -> Result<tonic::Response<super::FlowValidationResult>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Flows/GetFlowValidationResult",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Imports the specified flow to the specified agent from a binary file."]
        #[doc = ""]
        #[doc = " Note: You should always train a flow prior to sending it queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn import_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportFlowRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.cx.v3.Flows/ImportFlow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Exports the specified flow to a binary file."]
        #[doc = ""]
        #[doc = " Note that resources (e.g. intents, entities, webhooks) that the flow"]
        #[doc = " references will also be exported."]
        pub async fn export_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportFlowRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.cx.v3.Flows/ExportFlow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The request message for [SecuritySettingsService.GetSecuritySettings][google.cloud.dialogflow.cx.v3.SecuritySettingsService.GetSecuritySettings].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecuritySettingsRequest {
    /// Required. Resource name of the settings.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/securitySettings/<security settings ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for [SecuritySettingsService.UpdateSecuritySettings][google.cloud.dialogflow.cx.v3.SecuritySettingsService.UpdateSecuritySettings].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSecuritySettingsRequest {
    /// Required. [SecuritySettings] object that contains values for each of the
    /// fields to update.
    #[prost(message, optional, tag = "1")]
    pub security_settings: ::core::option::Option<SecuritySettings>,
    /// Required. The mask to control which fields get updated. If the mask is not present,
    /// all fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for [SecuritySettings.ListSecuritySettings][].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecuritySettingsRequest {
    /// Required. The location to list all security settings for.
    /// Format: `projects/<Project ID>/locations/<Location ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return in a single page. By default 20 and
    /// at most 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for [SecuritySettings.ListSecuritySettings][].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecuritySettingsResponse {
    /// The list of security settings.
    #[prost(message, repeated, tag = "1")]
    pub security_settings: ::prost::alloc::vec::Vec<SecuritySettings>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for [SecuritySettings.CreateSecuritySettings][].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSecuritySettingsRequest {
    /// Required. The location to create an [SecuritySettings][google.cloud.dialogflow.cx.v3.SecuritySettings] for.
    /// Format: `projects/<Project ID>/locations/<Location ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The security settings to create.
    #[prost(message, optional, tag = "2")]
    pub security_settings: ::core::option::Option<SecuritySettings>,
}
/// The request message for [SecuritySettings.DeleteSecuritySettings][].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSecuritySettingsRequest {
    /// Required. The name of the [SecuritySettings][google.cloud.dialogflow.cx.v3.SecuritySettings] to delete.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/securitySettings/<Security Settings ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the settings related to security issues, such as data redaction
/// and data retention. It may take hours for updates on the settings to
/// propagate to all the related components and take effect.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecuritySettings {
    /// Required. Resource name of the settings.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/securitySettings/<Security Settings ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The human-readable name of the security settings, unique within the
    /// location.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Strategy that defines how we do redaction.
    #[prost(enumeration = "security_settings::RedactionStrategy", tag = "3")]
    pub redaction_strategy: i32,
    /// Defines the data for which Dialogflow applies redaction. Dialogflow does
    /// not redact data that it does not have access to – for example, Cloud
    /// logging.
    #[prost(enumeration = "security_settings::RedactionScope", tag = "4")]
    pub redaction_scope: i32,
    /// DLP inspect template name. Use this template to define inspect base
    /// settings.
    ///
    /// If empty, we use the default DLP inspect config.
    ///
    /// The template name will have one of the following formats:
    /// `projects/<Project ID>/inspectTemplates/<Template ID>` OR
    /// `projects/<Project ID>/locations/<Location ID>/inspectTemplates/<Template
    /// ID>` OR
    /// `organizations/<Organization ID>/inspectTemplates/<Template ID>`
    #[prost(string, tag = "9")]
    pub inspect_template: ::prost::alloc::string::String,
    /// List of types of data to remove when retention settings triggers purge.
    #[prost(enumeration = "security_settings::PurgeDataType", repeated, tag = "8")]
    pub purge_data_types: ::prost::alloc::vec::Vec<i32>,
    /// Specifies how data is retained. Note that even if the data is
    /// purged due to retention policy, we may still hold it in backup storage for
    /// a few days without allowing direct readings.
    #[prost(oneof = "security_settings::DataRetention", tags = "6")]
    pub data_retention: ::core::option::Option<security_settings::DataRetention>,
}
/// Nested message and enum types in `SecuritySettings`.
pub mod security_settings {
    /// Defines how we redact data.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RedactionStrategy {
        /// Do not redact.
        Unspecified = 0,
        /// Call redaction service to clean up the data to be persisted.
        RedactWithService = 1,
    }
    /// Defines what types of data to redact.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RedactionScope {
        /// Don't redact any kind of data.
        Unspecified = 0,
        /// On data to be written to disk or similar devices that are capable of
        /// holding data even if power is disconnected. This includes data that are
        /// temporarily saved on disk.
        RedactDiskStorage = 2,
    }
    /// Type of data we purge after retention settings triggers purge.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PurgeDataType {
        /// Unspecified. Do not use.
        Unspecified = 0,
        /// Dialogflow history. This does not include Cloud logging, which is
        /// owned by the user - not Dialogflow.
        DialogflowHistory = 1,
    }
    /// Specifies how data is retained. Note that even if the data is
    /// purged due to retention policy, we may still hold it in backup storage for
    /// a few days without allowing direct readings.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataRetention {
        /// Retains data in interaction logging for the specified number of days.
        /// This does not apply to Cloud logging, which is owned by the user - not
        /// Dialogflow.
        /// User must Set a value lower than Dialogflow's default 30d TTL. Setting a
        /// value higher than that has no effect.
        /// A missing value or setting to 0 also means we use Dialogflow's default
        /// TTL.
        /// Note: Interaction logging is a limited access feature. Talk to your
        /// Google representative to check availability for you.
        #[prost(int32, tag = "6")]
        RetentionWindowDays(i32),
    }
}
#[doc = r" Generated client implementations."]
pub mod security_settings_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for managing security settings for Dialogflow."]
    #[derive(Debug, Clone)]
    pub struct SecuritySettingsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SecuritySettingsServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
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
        ) -> SecuritySettingsServiceClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            SecuritySettingsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Create security settings in the specified location."]
        pub async fn create_security_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSecuritySettingsRequest>,
        ) -> Result<tonic::Response<super::SecuritySettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.SecuritySettingsService/CreateSecuritySettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified [SecuritySettings][google.cloud.dialogflow.cx.v3.SecuritySettings]."]
        #[doc = " The returned settings may be stale by up to 1 minute."]
        pub async fn get_security_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSecuritySettingsRequest>,
        ) -> Result<tonic::Response<super::SecuritySettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.SecuritySettingsService/GetSecuritySettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified [SecuritySettings][google.cloud.dialogflow.cx.v3.SecuritySettings]."]
        pub async fn update_security_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSecuritySettingsRequest>,
        ) -> Result<tonic::Response<super::SecuritySettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.SecuritySettingsService/UpdateSecuritySettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the list of all security settings in the specified location."]
        pub async fn list_security_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSecuritySettingsRequest>,
        ) -> Result<tonic::Response<super::ListSecuritySettingsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.SecuritySettingsService/ListSecuritySettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified [SecuritySettings][google.cloud.dialogflow.cx.v3.SecuritySettings]."]
        pub async fn delete_security_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSecuritySettingsRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.SecuritySettingsService/DeleteSecuritySettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Settings related to speech recognition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechToTextSettings {
    /// Whether to use speech adaptation for speech recognition.
    #[prost(bool, tag = "1")]
    pub enable_speech_adaptation: bool,
}
/// Agents are best described as Natural Language Understanding (NLU) modules
/// that transform user requests into actionable data. You can include agents
/// in your app, product, or service to determine user intent and respond to the
/// user in a natural way.
///
/// After you create an agent, you can add [Intents][google.cloud.dialogflow.cx.v3.Intent],
/// [Entity Types][google.cloud.dialogflow.cx.v3.EntityType], [Flows][google.cloud.dialogflow.cx.v3.Flow], [Fulfillments][google.cloud.dialogflow.cx.v3.Fulfillment],
/// [Webhooks][google.cloud.dialogflow.cx.v3.Webhook], and so on to manage the conversation flows..
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Agent {
    /// The unique identifier of the agent.
    /// Required for the [Agents.UpdateAgent][google.cloud.dialogflow.cx.v3.Agents.UpdateAgent] method. [Agents.CreateAgent][google.cloud.dialogflow.cx.v3.Agents.CreateAgent]
    /// populates the name automatically.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The human-readable name of the agent, unique within the location.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. Immutable. The default language of the agent as a language tag.
    /// See [Language
    /// Support](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// for a list of the currently supported language codes.
    /// This field cannot be set by the [Agents.UpdateAgent][google.cloud.dialogflow.cx.v3.Agents.UpdateAgent] method.
    #[prost(string, tag = "3")]
    pub default_language_code: ::prost::alloc::string::String,
    /// The list of all languages supported by the agent (except for the
    /// `default_language_code`).
    #[prost(string, repeated, tag = "4")]
    pub supported_language_codes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The time zone of the agent from the [time zone
    /// database](https://www.iana.org/time-zones), e.g., America/New_York,
    /// Europe/Paris.
    #[prost(string, tag = "5")]
    pub time_zone: ::prost::alloc::string::String,
    /// The description of the agent. The maximum length is 500 characters. If
    /// exceeded, the request is rejected.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    /// The URI of the agent's avatar. Avatars are used throughout the Dialogflow
    /// console and in the self-hosted [Web
    /// Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo)
    /// integration.
    #[prost(string, tag = "7")]
    pub avatar_uri: ::prost::alloc::string::String,
    /// Speech recognition related settings.
    #[prost(message, optional, tag = "13")]
    pub speech_to_text_settings: ::core::option::Option<SpeechToTextSettings>,
    /// Immutable. Name of the start flow in this agent. A start flow will be automatically
    /// created when the agent is created, and can only be deleted by deleting the
    /// agent.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "16")]
    pub start_flow: ::prost::alloc::string::String,
    /// Name of the [SecuritySettings][google.cloud.dialogflow.cx.v3.SecuritySettings] reference for the agent.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/securitySettings/<Security Settings ID>`.
    #[prost(string, tag = "17")]
    pub security_settings: ::prost::alloc::string::String,
    /// Indicates if stackdriver logging is enabled for the agent.
    #[prost(bool, tag = "18")]
    pub enable_stackdriver_logging: bool,
    /// Indicates if automatic spell correction is enabled in detect intent
    /// requests.
    #[prost(bool, tag = "20")]
    pub enable_spell_correction: bool,
}
/// The request message for [Agents.ListAgents][google.cloud.dialogflow.cx.v3.Agents.ListAgents].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAgentsRequest {
    /// Required. The location to list all agents for.
    /// Format: `projects/<Project ID>/locations/<Location ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for [Agents.ListAgents][google.cloud.dialogflow.cx.v3.Agents.ListAgents].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAgentsResponse {
    /// The list of agents. There will be a maximum number of items returned based
    /// on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub agents: ::prost::alloc::vec::Vec<Agent>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for [Agents.GetAgent][google.cloud.dialogflow.cx.v3.Agents.GetAgent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAgentRequest {
    /// Required. The name of the agent.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for [Agents.CreateAgent][google.cloud.dialogflow.cx.v3.Agents.CreateAgent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAgentRequest {
    /// Required. The location to create a agent for.
    /// Format: `projects/<Project ID>/locations/<Location ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The agent to create.
    #[prost(message, optional, tag = "2")]
    pub agent: ::core::option::Option<Agent>,
}
/// The request message for [Agents.UpdateAgent][google.cloud.dialogflow.cx.v3.Agents.UpdateAgent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAgentRequest {
    /// Required. The agent to update.
    #[prost(message, optional, tag = "1")]
    pub agent: ::core::option::Option<Agent>,
    /// The mask to control which fields get updated. If the mask is not present,
    /// all fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for [Agents.DeleteAgent][google.cloud.dialogflow.cx.v3.Agents.DeleteAgent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAgentRequest {
    /// Required. The name of the agent to delete.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for [Agents.ExportAgent][google.cloud.dialogflow.cx.v3.Agents.ExportAgent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportAgentRequest {
    /// Required. The name of the agent to export.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI to
    /// export the agent to. The format of this URI must be
    /// `gs://<bucket-name>/<object-name>`.
    /// If left unspecified, the serialized agent is returned inline.
    #[prost(string, tag = "2")]
    pub agent_uri: ::prost::alloc::string::String,
    /// Optional. Environment name. If not set, draft environment is assumed.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>`.
    #[prost(string, tag = "5")]
    pub environment: ::prost::alloc::string::String,
}
/// The response message for [Agents.ExportAgent][google.cloud.dialogflow.cx.v3.Agents.ExportAgent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportAgentResponse {
    /// The exported agent.
    #[prost(oneof = "export_agent_response::Agent", tags = "1, 2")]
    pub agent: ::core::option::Option<export_agent_response::Agent>,
}
/// Nested message and enum types in `ExportAgentResponse`.
pub mod export_agent_response {
    /// The exported agent.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Agent {
        /// The URI to a file containing the exported agent. This field is populated
        /// only if `agent_uri` is specified in [ExportAgentRequest][google.cloud.dialogflow.cx.v3.ExportAgentRequest].
        #[prost(string, tag = "1")]
        AgentUri(::prost::alloc::string::String),
        /// Uncompressed raw byte content for agent.
        #[prost(bytes, tag = "2")]
        AgentContent(::prost::alloc::vec::Vec<u8>),
    }
}
/// The request message for [Agents.RestoreAgent][google.cloud.dialogflow.cx.v3.Agents.RestoreAgent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreAgentRequest {
    /// Required. The name of the agent to restore into.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Agent restore mode. If not specified, `KEEP` is assumed.
    #[prost(enumeration = "restore_agent_request::RestoreOption", tag = "5")]
    pub restore_option: i32,
    /// Required. The agent to restore.
    #[prost(oneof = "restore_agent_request::Agent", tags = "2, 3")]
    pub agent: ::core::option::Option<restore_agent_request::Agent>,
}
/// Nested message and enum types in `RestoreAgentRequest`.
pub mod restore_agent_request {
    /// Restore option.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RestoreOption {
        /// Unspecified. Treated as KEEP.
        Unspecified = 0,
        /// Always respect the settings from the exported agent file. It may cause
        /// a restoration failure if some settings (e.g. model type) are not
        /// supported in the target agent.
        Keep = 1,
        /// Fallback to default settings if some settings are not supported in the
        /// target agent.
        Fallback = 2,
    }
    /// Required. The agent to restore.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Agent {
        /// The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI
        /// to restore agent from. The format of this URI must be
        /// `gs://<bucket-name>/<object-name>`.
        #[prost(string, tag = "2")]
        AgentUri(::prost::alloc::string::String),
        /// Uncompressed raw byte content for agent.
        #[prost(bytes, tag = "3")]
        AgentContent(::prost::alloc::vec::Vec<u8>),
    }
}
/// The request message for [Agents.ValidateAgent][google.cloud.dialogflow.cx.v3.Agents.ValidateAgent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateAgentRequest {
    /// Required. The agent to validate.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If not specified, the agent's default language is used.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
}
/// The request message for [Agents.GetAgentValidationResult][google.cloud.dialogflow.cx.v3.Agents.GetAgentValidationResult].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAgentValidationResultRequest {
    /// Required. The agent name.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/validationResult`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If not specified, the agent's default language is used.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
}
/// The response message for [Agents.GetAgentValidationResult][google.cloud.dialogflow.cx.v3.Agents.GetAgentValidationResult].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgentValidationResult {
    /// The unique identifier of the agent validation result.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/validationResult`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Contains all flow validation results.
    #[prost(message, repeated, tag = "2")]
    pub flow_validation_results: ::prost::alloc::vec::Vec<FlowValidationResult>,
}
#[doc = r" Generated client implementations."]
pub mod agents_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for managing [Agents][google.cloud.dialogflow.cx.v3.Agent]."]
    #[derive(Debug, Clone)]
    pub struct AgentsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AgentsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
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
        ) -> AgentsClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            AgentsClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns the list of all agents in the specified location."]
        pub async fn list_agents(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAgentsRequest>,
        ) -> Result<tonic::Response<super::ListAgentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Agents/ListAgents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified agent."]
        pub async fn get_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAgentRequest>,
        ) -> Result<tonic::Response<super::Agent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Agents/GetAgent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an agent in the specified location."]
        #[doc = ""]
        #[doc = " Note: You should always train flows prior to sending them queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn create_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAgentRequest>,
        ) -> Result<tonic::Response<super::Agent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Agents/CreateAgent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified agent."]
        #[doc = ""]
        #[doc = " Note: You should always train flows prior to sending them queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn update_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAgentRequest>,
        ) -> Result<tonic::Response<super::Agent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Agents/UpdateAgent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified agent."]
        pub async fn delete_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAgentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Agents/DeleteAgent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Exports the specified agent to a binary file."]
        pub async fn export_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportAgentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.cx.v3.Agents/ExportAgent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Restores the specified agent from a binary file."]
        #[doc = ""]
        #[doc = " Replaces the current agent with a new one. Note that all existing resources"]
        #[doc = " in agent (e.g. intents, entity types, flows) will be removed."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " Note: You should always train flows prior to sending them queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn restore_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreAgentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.cx.v3.Agents/RestoreAgent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Validates the specified agent and creates or updates validation results."]
        #[doc = " The agent in draft version is validated. Please call this API after the"]
        #[doc = " training is completed to get the complete validation results."]
        pub async fn validate_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateAgentRequest>,
        ) -> Result<tonic::Response<super::AgentValidationResult>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Agents/ValidateAgent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the latest agent validation result. Agent validation is performed"]
        #[doc = " when ValidateAgent is called."]
        pub async fn get_agent_validation_result(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAgentValidationResultRequest>,
        ) -> Result<tonic::Response<super::AgentValidationResult>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Agents/GetAgentValidationResult",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Information for a word recognized by the speech recognizer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechWordInfo {
    /// The word this info is for.
    #[prost(string, tag = "3")]
    pub word: ::prost::alloc::string::String,
    /// Time offset relative to the beginning of the audio that corresponds to the
    /// start of the spoken word. This is an experimental feature and the accuracy
    /// of the time offset can vary.
    #[prost(message, optional, tag = "1")]
    pub start_offset: ::core::option::Option<::prost_types::Duration>,
    /// Time offset relative to the beginning of the audio that corresponds to the
    /// end of the spoken word. This is an experimental feature and the accuracy of
    /// the time offset can vary.
    #[prost(message, optional, tag = "2")]
    pub end_offset: ::core::option::Option<::prost_types::Duration>,
    /// The Speech confidence between 0.0 and 1.0 for this word. A higher number
    /// indicates an estimated greater likelihood that the recognized word is
    /// correct. The default of 0.0 is a sentinel value indicating that confidence
    /// was not set.
    ///
    /// This field is not guaranteed to be fully stable over time for the same
    /// audio input. Users should also not rely on it to always be provided.
    #[prost(float, tag = "4")]
    pub confidence: f32,
}
/// Instructs the speech recognizer on how to process the audio content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAudioConfig {
    /// Required. Audio encoding of the audio content to process.
    #[prost(enumeration = "AudioEncoding", tag = "1")]
    pub audio_encoding: i32,
    /// Sample rate (in Hertz) of the audio content sent in the query.
    /// Refer to
    /// [Cloud Speech API
    /// documentation](https://cloud.google.com/speech-to-text/docs/basics) for
    /// more details.
    #[prost(int32, tag = "2")]
    pub sample_rate_hertz: i32,
    /// Optional. If `true`, Dialogflow returns [SpeechWordInfo][google.cloud.dialogflow.cx.v3.SpeechWordInfo] in
    /// [StreamingRecognitionResult][google.cloud.dialogflow.cx.v3.StreamingRecognitionResult] with information about the recognized speech
    /// words, e.g. start and end time offsets. If false or unspecified, Speech
    /// doesn't return any word-level information.
    #[prost(bool, tag = "13")]
    pub enable_word_info: bool,
    /// Optional. A list of strings containing words and phrases that the speech
    /// recognizer should recognize with higher likelihood.
    ///
    /// See [the Cloud Speech
    /// documentation](https://cloud.google.com/speech-to-text/docs/basics#phrase-hints)
    /// for more details.
    #[prost(string, repeated, tag = "4")]
    pub phrase_hints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Which Speech model to select for the given request. Select the
    /// model best suited to your domain to get best results. If a model is not
    /// explicitly specified, then we auto-select a model based on the parameters
    /// in the InputAudioConfig.
    /// If enhanced speech model is enabled for the agent and an enhanced
    /// version of the specified model for the language does not exist, then the
    /// speech is recognized using the standard version of the specified model.
    /// Refer to
    /// [Cloud Speech API
    /// documentation](https://cloud.google.com/speech-to-text/docs/basics#select-model)
    /// for more details.
    #[prost(string, tag = "7")]
    pub model: ::prost::alloc::string::String,
    /// Optional. Which variant of the [Speech model][google.cloud.dialogflow.cx.v3.InputAudioConfig.model] to use.
    #[prost(enumeration = "SpeechModelVariant", tag = "10")]
    pub model_variant: i32,
    /// Optional. If `false` (default), recognition does not cease until the
    /// client closes the stream.
    /// If `true`, the recognizer will detect a single spoken utterance in input
    /// audio. Recognition ceases when it detects the audio's voice has
    /// stopped or paused. In this case, once a detected intent is received, the
    /// client should close the stream and start a new request with a new stream as
    /// needed.
    /// Note: This setting is relevant only for streaming methods.
    #[prost(bool, tag = "8")]
    pub single_utterance: bool,
}
/// Description of which voice to use for speech synthesis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoiceSelectionParams {
    /// Optional. The name of the voice. If not set, the service will choose a
    /// voice based on the other parameters such as language_code and
    /// [ssml_gender][google.cloud.dialogflow.cx.v3.VoiceSelectionParams.ssml_gender].
    ///
    /// For the list of available voices, please refer to [Supported voices and
    /// languages](https://cloud.google.com/text-to-speech/docs/voices).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The preferred gender of the voice. If not set, the service will
    /// choose a voice based on the other parameters such as language_code and
    /// [name][google.cloud.dialogflow.cx.v3.VoiceSelectionParams.name]. Note that this is only a preference, not requirement. If a
    /// voice of the appropriate gender is not available, the synthesizer
    /// substitutes a voice with a different gender rather than failing the
    /// request.
    #[prost(enumeration = "SsmlVoiceGender", tag = "2")]
    pub ssml_gender: i32,
}
/// Configuration of how speech should be synthesized.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynthesizeSpeechConfig {
    /// Optional. Speaking rate/speed, in the range [0.25, 4.0]. 1.0 is the normal
    /// native speed supported by the specific voice. 2.0 is twice as fast, and
    /// 0.5 is half as fast. If unset(0.0), defaults to the native 1.0 speed. Any
    /// other values < 0.25 or > 4.0 will return an error.
    #[prost(double, tag = "1")]
    pub speaking_rate: f64,
    /// Optional. Speaking pitch, in the range [-20.0, 20.0]. 20 means increase 20
    /// semitones from the original pitch. -20 means decrease 20 semitones from the
    /// original pitch.
    #[prost(double, tag = "2")]
    pub pitch: f64,
    /// Optional. Volume gain (in dB) of the normal native volume supported by the
    /// specific voice, in the range [-96.0, 16.0]. If unset, or set to a value of
    /// 0.0 (dB), will play at normal native signal amplitude. A value of -6.0 (dB)
    /// will play at approximately half the amplitude of the normal native signal
    /// amplitude. A value of +6.0 (dB) will play at approximately twice the
    /// amplitude of the normal native signal amplitude. We strongly recommend not
    /// to exceed +10 (dB) as there's usually no effective increase in loudness for
    /// any value greater than that.
    #[prost(double, tag = "3")]
    pub volume_gain_db: f64,
    /// Optional. An identifier which selects 'audio effects' profiles that are
    /// applied on (post synthesized) text to speech. Effects are applied on top of
    /// each other in the order they are given.
    #[prost(string, repeated, tag = "5")]
    pub effects_profile_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The desired voice of the synthesized audio.
    #[prost(message, optional, tag = "4")]
    pub voice: ::core::option::Option<VoiceSelectionParams>,
}
/// Instructs the speech synthesizer how to generate the output audio content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputAudioConfig {
    /// Required. Audio encoding of the synthesized audio content.
    #[prost(enumeration = "OutputAudioEncoding", tag = "1")]
    pub audio_encoding: i32,
    /// Optional. The synthesis sample rate (in hertz) for this audio. If not
    /// provided, then the synthesizer will use the default sample rate based on
    /// the audio encoding. If this is different from the voice's natural sample
    /// rate, then the synthesizer will honor this request by converting to the
    /// desired sample rate (which might result in worse audio quality).
    #[prost(int32, tag = "2")]
    pub sample_rate_hertz: i32,
    /// Optional. Configuration of how speech should be synthesized.
    #[prost(message, optional, tag = "3")]
    pub synthesize_speech_config: ::core::option::Option<SynthesizeSpeechConfig>,
}
/// Audio encoding of the audio content sent in the conversational query request.
/// Refer to the
/// [Cloud Speech API
/// documentation](https://cloud.google.com/speech-to-text/docs/basics) for more
/// details.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AudioEncoding {
    /// Not specified.
    Unspecified = 0,
    /// Uncompressed 16-bit signed little-endian samples (Linear PCM).
    Linear16 = 1,
    /// [`FLAC`](https://xiph.org/flac/documentation.html) (Free Lossless Audio
    /// Codec) is the recommended encoding because it is lossless (therefore
    /// recognition is not compromised) and requires only about half the
    /// bandwidth of `LINEAR16`. `FLAC` stream encoding supports 16-bit and
    /// 24-bit samples, however, not all fields in `STREAMINFO` are supported.
    Flac = 2,
    /// 8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law.
    Mulaw = 3,
    /// Adaptive Multi-Rate Narrowband codec. `sample_rate_hertz` must be 8000.
    Amr = 4,
    /// Adaptive Multi-Rate Wideband codec. `sample_rate_hertz` must be 16000.
    AmrWb = 5,
    /// Opus encoded audio frames in Ogg container
    /// ([OggOpus](https://wiki.xiph.org/OggOpus)).
    /// `sample_rate_hertz` must be 16000.
    OggOpus = 6,
    /// Although the use of lossy encodings is not recommended, if a very low
    /// bitrate encoding is required, `OGG_OPUS` is highly preferred over
    /// Speex encoding. The [Speex](https://speex.org/) encoding supported by
    /// Dialogflow API has a header byte in each block, as in MIME type
    /// `audio/x-speex-with-header-byte`.
    /// It is a variant of the RTP Speex encoding defined in
    /// [RFC 5574](https://tools.ietf.org/html/rfc5574).
    /// The stream is a sequence of blocks, one block per RTP packet. Each block
    /// starts with a byte containing the length of the block, in bytes, followed
    /// by one or more frames of Speex data, padded to an integral number of
    /// bytes (octets) as specified in RFC 5574. In other words, each RTP header
    /// is replaced with a single byte containing the block length. Only Speex
    /// wideband is supported. `sample_rate_hertz` must be 16000.
    SpeexWithHeaderByte = 7,
}
/// Variant of the specified [Speech model][google.cloud.dialogflow.cx.v3.InputAudioConfig.model] to use.
///
/// See the [Cloud Speech
/// documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models)
/// for which models have different variants. For example, the "phone_call" model
/// has both a standard and an enhanced variant. When you use an enhanced model,
/// you will generally receive higher quality results than for a standard model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SpeechModelVariant {
    /// No model variant specified. In this case Dialogflow defaults to
    /// USE_BEST_AVAILABLE.
    Unspecified = 0,
    /// Use the best available variant of the [Speech
    /// model][InputAudioConfig.model] that the caller is eligible for.
    ///
    /// Please see the [Dialogflow
    /// docs](https://cloud.google.com/dialogflow/docs/data-logging) for
    /// how to make your project eligible for enhanced models.
    UseBestAvailable = 1,
    /// Use standard model variant even if an enhanced model is available.  See the
    /// [Cloud Speech
    /// documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models)
    /// for details about enhanced models.
    UseStandard = 2,
    /// Use an enhanced model variant:
    ///
    /// * If an enhanced variant does not exist for the given
    ///   [model][google.cloud.dialogflow.cx.v3.InputAudioConfig.model] and request language, Dialogflow falls
    ///   back to the standard variant.
    ///
    ///   The [Cloud Speech
    ///   documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models)
    ///   describes which models have enhanced variants.
    ///
    /// * If the API caller isn't eligible for enhanced models, Dialogflow returns
    ///   an error.  Please see the [Dialogflow
    ///   docs](https://cloud.google.com/dialogflow/docs/data-logging)
    ///   for how to make your project eligible.
    UseEnhanced = 3,
}
/// Gender of the voice as described in
/// [SSML voice element](https://www.w3.org/TR/speech-synthesis11/#edef_voice).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SsmlVoiceGender {
    /// An unspecified gender, which means that the client doesn't care which
    /// gender the selected voice will have.
    Unspecified = 0,
    /// A male voice.
    Male = 1,
    /// A female voice.
    Female = 2,
    /// A gender-neutral voice.
    Neutral = 3,
}
/// Audio encoding of the output audio format in Text-To-Speech.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OutputAudioEncoding {
    /// Not specified.
    Unspecified = 0,
    /// Uncompressed 16-bit signed little-endian samples (Linear PCM).
    /// Audio content returned as LINEAR16 also contains a WAV header.
    Linear16 = 1,
    /// MP3 audio at 32kbps.
    Mp3 = 2,
    /// MP3 audio at 64kbps.
    Mp364Kbps = 4,
    /// Opus encoded audio wrapped in an ogg container. The result will be a
    /// file which can be played natively on Android, and in browsers (at least
    /// Chrome and Firefox). The quality of the encoding is considerably higher
    /// than MP3 while using approximately the same bitrate.
    OggOpus = 3,
    /// 8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law.
    Mulaw = 5,
}
/// Entities are extracted from user input and represent parameters that are
/// meaningful to your application. For example, a date range, a proper name
/// such as a geographic location or landmark, and so on. Entities represent
/// actionable data for your application.
///
/// When you define an entity, you can also include synonyms that all map to
/// that entity. For example, "soft drink", "soda", "pop", and so on.
///
/// There are three types of entities:
///
/// *   **System** - entities that are defined by the Dialogflow API for common
///     data types such as date, time, currency, and so on. A system entity is
///     represented by the `EntityType` type.
///
/// *   **Custom** - entities that are defined by you that represent
///     actionable data that is meaningful to your application. For example,
///     you could define a `pizza.sauce` entity for red or white pizza sauce,
///     a `pizza.cheese` entity for the different types of cheese on a pizza,
///     a `pizza.topping` entity for different toppings, and so on. A custom
///     entity is represented by the `EntityType` type.
///
/// *   **User** - entities that are built for an individual user such as
///     favorites, preferences, playlists, and so on. A user entity is
///     represented by the [SessionEntityType][google.cloud.dialogflow.cx.v3.SessionEntityType] type.
///
/// For more information about entity types, see the [Dialogflow
/// documentation](https://cloud.google.com/dialogflow/docs/entities-overview).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityType {
    /// The unique identifier of the entity type.
    /// Required for [EntityTypes.UpdateEntityType][google.cloud.dialogflow.cx.v3.EntityTypes.UpdateEntityType].
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/entityTypes/<Entity Type ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The human-readable name of the entity type, unique within the agent.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. Indicates the kind of entity type.
    #[prost(enumeration = "entity_type::Kind", tag = "3")]
    pub kind: i32,
    /// Indicates whether the entity type can be automatically expanded.
    #[prost(enumeration = "entity_type::AutoExpansionMode", tag = "4")]
    pub auto_expansion_mode: i32,
    /// The collection of entity entries associated with the entity type.
    #[prost(message, repeated, tag = "5")]
    pub entities: ::prost::alloc::vec::Vec<entity_type::Entity>,
    /// Collection of exceptional words and phrases that shouldn't be matched.
    /// For example, if you have a size entity type with entry `giant`(an
    /// adjective), you might consider adding `giants`(a noun) as an exclusion.
    /// If the kind of entity type is `KIND_MAP`, then the phrases specified by
    /// entities and excluded phrases should be mutually exclusive.
    #[prost(message, repeated, tag = "6")]
    pub excluded_phrases: ::prost::alloc::vec::Vec<entity_type::ExcludedPhrase>,
    /// Enables fuzzy entity extraction during classification.
    #[prost(bool, tag = "7")]
    pub enable_fuzzy_extraction: bool,
    /// Indicates whether parameters of the entity type should be redacted in log.
    /// If redaction is enabled, page parameters and intent parameters referring to
    /// the entity type will be replaced by parameter name when logging.
    #[prost(bool, tag = "9")]
    pub redact: bool,
}
/// Nested message and enum types in `EntityType`.
pub mod entity_type {
    /// An **entity entry** for an associated entity type.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entity {
        /// Required. The primary value associated with this entity entry.
        /// For example, if the entity type is *vegetable*, the value could be
        /// *scallions*.
        ///
        /// For `KIND_MAP` entity types:
        ///
        /// *   A canonical value to be used in place of synonyms.
        ///
        /// For `KIND_LIST` entity types:
        ///
        /// *   A string that can contain references to other entity types (with or
        ///     without aliases).
        #[prost(string, tag = "1")]
        pub value: ::prost::alloc::string::String,
        /// Required. A collection of value synonyms. For example, if the entity type
        /// is *vegetable*, and `value` is *scallions*, a synonym could be *green
        /// onions*.
        ///
        /// For `KIND_LIST` entity types:
        ///
        /// *   This collection must contain exactly one synonym equal to `value`.
        #[prost(string, repeated, tag = "2")]
        pub synonyms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// An excluded entity phrase that should not be matched.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExcludedPhrase {
        /// Required. The word or phrase to be excluded.
        #[prost(string, tag = "1")]
        pub value: ::prost::alloc::string::String,
    }
    /// Represents kinds of entities.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        /// Not specified. This value should be never used.
        Unspecified = 0,
        /// Map entity types allow mapping of a group of synonyms to a canonical
        /// value.
        Map = 1,
        /// List entity types contain a set of entries that do not map to canonical
        /// values. However, list entity types can contain references to other entity
        /// types (with or without aliases).
        List = 2,
        /// Regexp entity types allow to specify regular expressions in entries
        /// values.
        Regexp = 3,
    }
    /// Represents different entity type expansion modes. Automated expansion
    /// allows an agent to recognize values that have not been explicitly listed in
    /// the entity (for example, new kinds of shopping list items).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AutoExpansionMode {
        /// Auto expansion disabled for the entity.
        Unspecified = 0,
        /// Allows an agent to recognize values that have not been explicitly
        /// listed in the entity.
        Default = 1,
    }
}
/// The request message for [EntityTypes.ListEntityTypes][google.cloud.dialogflow.cx.v3.EntityTypes.ListEntityTypes].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntityTypesRequest {
    /// Required. The agent to list all entity types for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The language to list entity types for. The following fields are language
    /// dependent:
    ///
    /// *   `EntityType.entities.value`
    /// *   `EntityType.entities.synonyms`
    /// *   `EntityType.excluded_phrases.value`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for [EntityTypes.ListEntityTypes][google.cloud.dialogflow.cx.v3.EntityTypes.ListEntityTypes].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntityTypesResponse {
    /// The list of entity types. There will be a maximum number of items returned
    /// based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub entity_types: ::prost::alloc::vec::Vec<EntityType>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for [EntityTypes.GetEntityType][google.cloud.dialogflow.cx.v3.EntityTypes.GetEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntityTypeRequest {
    /// Required. The name of the entity type.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/entityTypes/<Entity Type ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The language to retrieve the entity type for. The following fields are
    /// language dependent:
    ///
    /// *   `EntityType.entities.value`
    /// *   `EntityType.entities.synonyms`
    /// *   `EntityType.excluded_phrases.value`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
}
/// The request message for [EntityTypes.CreateEntityType][google.cloud.dialogflow.cx.v3.EntityTypes.CreateEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntityTypeRequest {
    /// Required. The agent to create a entity type for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The entity type to create.
    #[prost(message, optional, tag = "2")]
    pub entity_type: ::core::option::Option<EntityType>,
    /// The language of the following fields in `entity_type`:
    ///
    /// *   `EntityType.entities.value`
    /// *   `EntityType.entities.synonyms`
    /// *   `EntityType.excluded_phrases.value`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// The request message for [EntityTypes.UpdateEntityType][google.cloud.dialogflow.cx.v3.EntityTypes.UpdateEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEntityTypeRequest {
    /// Required. The entity type to update.
    #[prost(message, optional, tag = "1")]
    pub entity_type: ::core::option::Option<EntityType>,
    /// The language of the following fields in `entity_type`:
    ///
    /// *   `EntityType.entities.value`
    /// *   `EntityType.entities.synonyms`
    /// *   `EntityType.excluded_phrases.value`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// The mask to control which fields get updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for [EntityTypes.DeleteEntityType][google.cloud.dialogflow.cx.v3.EntityTypes.DeleteEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEntityTypeRequest {
    /// Required. The name of the entity type to delete.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/entityTypes/<Entity Type ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// This field has no effect for entity type not being used.
    /// For entity types that are used by intents or pages:
    ///
    /// *  If `force` is set to false, an error will be returned with message
    ///    indicating the referencing resources.
    /// *  If `force` is set to true, Dialogflow will remove the entity type, as
    ///    well as any references to the entity type (i.e. Page
    ///    [parameter][google.cloud.dialogflow.cx.v3.Form.Parameter] of the entity type will be changed to
    ///    '@sys.any' and intent [parameter][google.cloud.dialogflow.cx.v3.Intent.Parameter] of the entity type
    ///    will be removed).
    #[prost(bool, tag = "2")]
    pub force: bool,
}
#[doc = r" Generated client implementations."]
pub mod entity_types_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for managing [EntityTypes][google.cloud.dialogflow.cx.v3.EntityType]."]
    #[derive(Debug, Clone)]
    pub struct EntityTypesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> EntityTypesClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
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
        ) -> EntityTypesClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            EntityTypesClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns the list of all entity types in the specified agent."]
        pub async fn list_entity_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEntityTypesRequest>,
        ) -> Result<tonic::Response<super::ListEntityTypesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.EntityTypes/ListEntityTypes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified entity type."]
        pub async fn get_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEntityTypeRequest>,
        ) -> Result<tonic::Response<super::EntityType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.EntityTypes/GetEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an entity type in the specified agent."]
        #[doc = ""]
        #[doc = " Note: You should always train a flow prior to sending it queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn create_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEntityTypeRequest>,
        ) -> Result<tonic::Response<super::EntityType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.EntityTypes/CreateEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified entity type."]
        #[doc = ""]
        #[doc = " Note: You should always train a flow prior to sending it queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn update_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEntityTypeRequest>,
        ) -> Result<tonic::Response<super::EntityType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.EntityTypes/UpdateEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified entity type."]
        #[doc = ""]
        #[doc = " Note: You should always train a flow prior to sending it queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn delete_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEntityTypeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.EntityTypes/DeleteEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// An intent represents a user's intent to interact with a conversational agent.
///
/// You can provide information for the Dialogflow API to use to match user input
/// to an intent by adding training phrases (i.e., examples of user input) to
/// your intent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Intent {
    /// The unique identifier of the intent.
    /// Required for the [Intents.UpdateIntent][google.cloud.dialogflow.cx.v3.Intents.UpdateIntent] method. [Intents.CreateIntent][google.cloud.dialogflow.cx.v3.Intents.CreateIntent]
    /// populates the name automatically.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/intents/<Intent ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The human-readable name of the intent, unique within the agent.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The collection of training phrases the agent is trained on to identify the
    /// intent.
    #[prost(message, repeated, tag = "3")]
    pub training_phrases: ::prost::alloc::vec::Vec<intent::TrainingPhrase>,
    /// The collection of parameters associated with the intent.
    #[prost(message, repeated, tag = "4")]
    pub parameters: ::prost::alloc::vec::Vec<intent::Parameter>,
    /// The priority of this intent. Higher numbers represent higher
    /// priorities.
    ///
    /// - If the supplied value is unspecified or 0, the service
    ///   translates the value to 500,000, which corresponds to the
    ///   `Normal` priority in the console.
    /// - If the supplied value is negative, the intent is ignored
    ///   in runtime detect intent requests.
    #[prost(int32, tag = "5")]
    pub priority: i32,
    /// Indicates whether this is a fallback intent. Currently only default
    /// fallback intent is allowed in the agent, which is added upon agent
    /// creation.
    /// Adding training phrases to fallback intent is useful in the case of
    /// requests that are mistakenly matched, since training phrases assigned to
    /// fallback intents act as negative examples that triggers no-match event.
    #[prost(bool, tag = "6")]
    pub is_fallback: bool,
    /// The key/value metadata to label an intent. Labels can contain
    /// lowercase letters, digits and the symbols '-' and '_'. International
    /// characters are allowed, including letters from unicase alphabets. Keys must
    /// start with a letter. Keys and values can be no longer than 63 characters
    /// and no more than 128 bytes.
    ///
    /// Prefix "sys-" is reserved for Dialogflow defined labels. Currently allowed
    /// Dialogflow defined labels include:
    /// * sys-head
    /// * sys-contextual
    /// The above labels do not require value. "sys-head" means the intent is a
    /// head intent. "sys.contextual" means the intent is a contextual intent.
    #[prost(map = "string, string", tag = "7")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Human readable description for better understanding an intent like its
    /// scope, content, result etc. Maximum character limit: 140 characters.
    #[prost(string, tag = "8")]
    pub description: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Intent`.
pub mod intent {
    /// Represents an example that the agent is trained on to identify the intent.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TrainingPhrase {
        /// Output only. The unique identifier of the training phrase.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// Required. The ordered list of training phrase parts.
        /// The parts are concatenated in order to form the training phrase.
        ///
        /// Note: The API does not automatically annotate training phrases like the
        /// Dialogflow Console does.
        ///
        /// Note: Do not forget to include whitespace at part boundaries, so the
        /// training phrase is well formatted when the parts are concatenated.
        ///
        /// If the training phrase does not need to be annotated with parameters,
        /// you just need a single part with only the [Part.text][google.cloud.dialogflow.cx.v3.Intent.TrainingPhrase.Part.text] field set.
        ///
        /// If you want to annotate the training phrase, you must create multiple
        /// parts, where the fields of each part are populated in one of two ways:
        ///
        /// -   `Part.text` is set to a part of the phrase that has no parameters.
        /// -   `Part.text` is set to a part of the phrase that you want to annotate,
        ///     and the `parameter_id` field is set.
        #[prost(message, repeated, tag = "2")]
        pub parts: ::prost::alloc::vec::Vec<training_phrase::Part>,
        /// Indicates how many times this example was added to the intent.
        #[prost(int32, tag = "3")]
        pub repeat_count: i32,
    }
    /// Nested message and enum types in `TrainingPhrase`.
    pub mod training_phrase {
        /// Represents a part of a training phrase.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Part {
            /// Required. The text for this part.
            #[prost(string, tag = "1")]
            pub text: ::prost::alloc::string::String,
            /// The [parameter][google.cloud.dialogflow.cx.v3.Intent.Parameter] used to annotate this part of the
            /// training phrase. This field is required for annotated parts of the
            /// training phrase.
            #[prost(string, tag = "2")]
            pub parameter_id: ::prost::alloc::string::String,
        }
    }
    /// Represents an intent parameter.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Parameter {
        /// Required. The unique identifier of the parameter. This field
        /// is used by [training phrases][google.cloud.dialogflow.cx.v3.Intent.TrainingPhrase] to annotate their
        /// [parts][google.cloud.dialogflow.cx.v3.Intent.TrainingPhrase.Part].
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// Required. The entity type of the parameter.
        /// Format: `projects/-/locations/-/agents/-/entityTypes/<System Entity Type
        /// ID>` for system entity types (for example,
        /// `projects/-/locations/-/agents/-/entityTypes/sys.date`), or
        /// `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/entityTypes/<Entity Type ID>` for developer entity types.
        #[prost(string, tag = "2")]
        pub entity_type: ::prost::alloc::string::String,
        /// Indicates whether the parameter represents a list of values.
        #[prost(bool, tag = "3")]
        pub is_list: bool,
        /// Indicates whether the parameter content should be redacted in log. If
        /// redaction is enabled, the parameter content will be replaced by parameter
        /// name during logging.
        /// Note: the parameter content is subject to redaction if either parameter
        /// level redaction or [entity type level redaction][google.cloud.dialogflow.cx.v3.EntityType.redact] is
        /// enabled.
        #[prost(bool, tag = "4")]
        pub redact: bool,
    }
}
/// The request message for [Intents.ListIntents][google.cloud.dialogflow.cx.v3.Intents.ListIntents].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIntentsRequest {
    /// Required. The agent to list all intents for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The language to list intents for. The following fields are language
    /// dependent:
    ///
    /// *   `Intent.training_phrases.parts.text`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// The resource view to apply to the returned intent.
    #[prost(enumeration = "IntentView", tag = "5")]
    pub intent_view: i32,
    /// The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for [Intents.ListIntents][google.cloud.dialogflow.cx.v3.Intents.ListIntents].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIntentsResponse {
    /// The list of intents. There will be a maximum number of items returned based
    /// on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub intents: ::prost::alloc::vec::Vec<Intent>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for [Intents.GetIntent][google.cloud.dialogflow.cx.v3.Intents.GetIntent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIntentRequest {
    /// Required. The name of the intent.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/intents/<Intent ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The language to retrieve the intent for. The following fields are language
    /// dependent:
    ///
    /// *   `Intent.training_phrases.parts.text`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
}
/// The request message for [Intents.CreateIntent][google.cloud.dialogflow.cx.v3.Intents.CreateIntent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIntentRequest {
    /// Required. The agent to create an intent for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The intent to create.
    #[prost(message, optional, tag = "2")]
    pub intent: ::core::option::Option<Intent>,
    /// The language of the following fields in `intent`:
    ///
    /// *   `Intent.training_phrases.parts.text`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// The request message for [Intents.UpdateIntent][google.cloud.dialogflow.cx.v3.Intents.UpdateIntent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIntentRequest {
    /// Required. The intent to update.
    #[prost(message, optional, tag = "1")]
    pub intent: ::core::option::Option<Intent>,
    /// The language of the following fields in `intent`:
    ///
    /// *   `Intent.training_phrases.parts.text`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// The mask to control which fields get updated. If the mask is not present,
    /// all fields will be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for [Intents.DeleteIntent][google.cloud.dialogflow.cx.v3.Intents.DeleteIntent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteIntentRequest {
    /// Required. The name of the intent to delete.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/intents/<Intent ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the options for views of an intent.
/// An intent can be a sizable object. Therefore, we provide a resource view that
/// does not return training phrases in the response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IntentView {
    /// Not specified. Treated as INTENT_VIEW_FULL.
    Unspecified = 0,
    /// Training phrases field is not populated in the response.
    Partial = 1,
    /// All fields are populated.
    Full = 2,
}
#[doc = r" Generated client implementations."]
pub mod intents_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for managing [Intents][google.cloud.dialogflow.cx.v3.Intent]."]
    #[derive(Debug, Clone)]
    pub struct IntentsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> IntentsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
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
        ) -> IntentsClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            IntentsClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns the list of all intents in the specified agent."]
        pub async fn list_intents(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIntentsRequest>,
        ) -> Result<tonic::Response<super::ListIntentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Intents/ListIntents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified intent."]
        pub async fn get_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIntentRequest>,
        ) -> Result<tonic::Response<super::Intent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Intents/GetIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an intent in the specified agent."]
        #[doc = ""]
        #[doc = " Note: You should always train a flow prior to sending it queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn create_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateIntentRequest>,
        ) -> Result<tonic::Response<super::Intent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Intents/CreateIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified intent."]
        #[doc = ""]
        #[doc = " Note: You should always train a flow prior to sending it queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn update_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateIntentRequest>,
        ) -> Result<tonic::Response<super::Intent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Intents/UpdateIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified intent."]
        #[doc = ""]
        #[doc = " Note: You should always train a flow prior to sending it queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn delete_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteIntentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Intents/DeleteIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Session entity types are referred to as **User** entity types and are
/// entities that are built for an individual user such as favorites,
/// preferences, playlists, and so on.
///
/// You can redefine a session entity type at the session level to extend or
/// replace a [custom entity type][google.cloud.dialogflow.cx.v3.EntityType] at the user session level (we
/// refer to the entity types defined at the agent level as "custom entity
/// types").
///
/// Note: session entity types apply to all queries, regardless of the language.
///
/// For more information about entity types, see the [Dialogflow
/// documentation](https://cloud.google.com/dialogflow/docs/entities-overview).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionEntityType {
    /// Required. The unique identifier of the session entity type.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/agents/<Agent ID>/sessions/<Session ID>/entityTypes/<Entity Type
    /// ID>` or `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>/sessions/<Session ID>/entityTypes/<Entity
    /// Type ID>`. If `Environment ID` is not specified, we assume default 'draft'
    /// environment.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Indicates whether the additional data should override or supplement the
    /// custom entity type definition.
    #[prost(enumeration = "session_entity_type::EntityOverrideMode", tag = "3")]
    pub entity_override_mode: i32,
    /// Required. The collection of entities to override or supplement the custom entity
    /// type.
    #[prost(message, repeated, tag = "4")]
    pub entities: ::prost::alloc::vec::Vec<entity_type::Entity>,
}
/// Nested message and enum types in `SessionEntityType`.
pub mod session_entity_type {
    /// The types of modifications for the session entity type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EntityOverrideMode {
        /// Not specified. This value should be never used.
        Unspecified = 0,
        /// The collection of session entities overrides the collection of entities
        /// in the corresponding custom entity type.
        Override = 1,
        /// The collection of session entities extends the collection of entities in
        /// the corresponding custom entity type.
        ///
        /// Note: Even in this override mode calls to `ListSessionEntityTypes`,
        /// `GetSessionEntityType`, `CreateSessionEntityType` and
        /// `UpdateSessionEntityType` only return the additional entities added in
        /// this session entity type. If you want to get the supplemented list,
        /// please call [EntityTypes.GetEntityType][google.cloud.dialogflow.cx.v3.EntityTypes.GetEntityType] on the custom entity type
        /// and merge.
        Supplement = 2,
    }
}
/// The request message for [SessionEntityTypes.ListSessionEntityTypes][google.cloud.dialogflow.cx.v3.SessionEntityTypes.ListSessionEntityTypes].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSessionEntityTypesRequest {
    /// Required. The session to list all session entity types from.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/sessions/<Session ID>` or `projects/<Project ID>/locations/<Location
    /// ID>/agents/<Agent ID>/environments/<Environment ID>/sessions/<Session ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for [SessionEntityTypes.ListSessionEntityTypes][google.cloud.dialogflow.cx.v3.SessionEntityTypes.ListSessionEntityTypes].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSessionEntityTypesResponse {
    /// The list of session entity types. There will be a maximum number of items
    /// returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub session_entity_types: ::prost::alloc::vec::Vec<SessionEntityType>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for [SessionEntityTypes.GetSessionEntityType][google.cloud.dialogflow.cx.v3.SessionEntityTypes.GetSessionEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSessionEntityTypeRequest {
    /// Required. The name of the session entity type.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/sessions/<Session ID>/entityTypes/<Entity Type ID>` or
    /// `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>/sessions/<Session ID>/entityTypes/<Entity
    /// Type ID>`. If `Environment ID` is not specified, we assume default 'draft'
    /// environment.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for [SessionEntityTypes.CreateSessionEntityType][google.cloud.dialogflow.cx.v3.SessionEntityTypes.CreateSessionEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSessionEntityTypeRequest {
    /// Required. The session to create a session entity type for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/sessions/<Session ID>` or `projects/<Project ID>/locations/<Location
    /// ID>/agents/<Agent ID>/environments/<Environment ID>/sessions/<Session ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The session entity type to create.
    #[prost(message, optional, tag = "2")]
    pub session_entity_type: ::core::option::Option<SessionEntityType>,
}
/// The request message for [SessionEntityTypes.UpdateSessionEntityType][google.cloud.dialogflow.cx.v3.SessionEntityTypes.UpdateSessionEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSessionEntityTypeRequest {
    /// Required. The session entity type to update.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/sessions/<Session ID>/entityTypes/<Entity Type ID>` or
    /// `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>/sessions/<Session ID>/entityTypes/<Entity
    /// Type ID>`. If `Environment ID` is not specified, we assume default 'draft'
    /// environment.
    #[prost(message, optional, tag = "1")]
    pub session_entity_type: ::core::option::Option<SessionEntityType>,
    /// The mask to control which fields get updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for [SessionEntityTypes.DeleteSessionEntityType][google.cloud.dialogflow.cx.v3.SessionEntityTypes.DeleteSessionEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSessionEntityTypeRequest {
    /// Required. The name of the session entity type to delete.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/sessions/<Session ID>/entityTypes/<Entity Type ID>` or
    /// `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>/sessions/<Session ID>/entityTypes/<Entity
    /// Type ID>`. If `Environment ID` is not specified, we assume default 'draft'
    /// environment.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod session_entity_types_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for managing [SessionEntityTypes][google.cloud.dialogflow.cx.v3.SessionEntityType]."]
    #[derive(Debug, Clone)]
    pub struct SessionEntityTypesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SessionEntityTypesClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
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
        ) -> SessionEntityTypesClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            SessionEntityTypesClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns the list of all session entity types in the specified session."]
        pub async fn list_session_entity_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSessionEntityTypesRequest>,
        ) -> Result<tonic::Response<super::ListSessionEntityTypesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.SessionEntityTypes/ListSessionEntityTypes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified session entity type."]
        pub async fn get_session_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSessionEntityTypeRequest>,
        ) -> Result<tonic::Response<super::SessionEntityType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.SessionEntityTypes/GetSessionEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a session entity type."]
        pub async fn create_session_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSessionEntityTypeRequest>,
        ) -> Result<tonic::Response<super::SessionEntityType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.SessionEntityTypes/CreateSessionEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified session entity type."]
        pub async fn update_session_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSessionEntityTypeRequest>,
        ) -> Result<tonic::Response<super::SessionEntityType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.SessionEntityTypes/UpdateSessionEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified session entity type."]
        pub async fn delete_session_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSessionEntityTypeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.SessionEntityTypes/DeleteSessionEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The request to detect user's intent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectIntentRequest {
    /// Required. The name of the session this query is sent to.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/sessions/<Session ID>` or `projects/<Project ID>/locations/<Location
    /// ID>/agents/<Agent ID>/environments/<Environment ID>/sessions/<Session ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment.
    /// It's up to the API caller to choose an appropriate `Session ID`. It can be
    /// a random number or some type of session identifiers (preferably hashed).
    /// The length of the `Session ID` must not exceed 36 characters.
    ///
    /// For more information, see the [sessions
    /// guide](https://cloud.google.com/dialogflow/cx/docs/concept/session).
    ///
    /// Note: Always use agent versions for production traffic.
    /// See [Versions and
    /// environments](https://cloud.google.com/dialogflow/cx/docs/concept/version).
    #[prost(string, tag = "1")]
    pub session: ::prost::alloc::string::String,
    /// The parameters of this query.
    #[prost(message, optional, tag = "2")]
    pub query_params: ::core::option::Option<QueryParameters>,
    /// Required. The input specification.
    #[prost(message, optional, tag = "3")]
    pub query_input: ::core::option::Option<QueryInput>,
    /// Instructs the speech synthesizer how to generate the output audio.
    #[prost(message, optional, tag = "4")]
    pub output_audio_config: ::core::option::Option<OutputAudioConfig>,
}
/// The message returned from the DetectIntent method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectIntentResponse {
    /// Output only. The unique identifier of the response. It can be used to
    /// locate a response in the training example set or for reporting issues.
    #[prost(string, tag = "1")]
    pub response_id: ::prost::alloc::string::String,
    /// The result of the conversational query.
    #[prost(message, optional, tag = "2")]
    pub query_result: ::core::option::Option<QueryResult>,
    /// The audio data bytes encoded as specified in the request.
    /// Note: The output audio is generated based on the values of default platform
    /// text responses found in the
    /// [`query_result.response_messages`][google.cloud.dialogflow.cx.v3.QueryResult.response_messages] field. If
    /// multiple default text responses exist, they will be concatenated when
    /// generating audio. If no default platform text responses exist, the
    /// generated audio content will be empty.
    ///
    /// In some scenarios, multiple output audio fields may be present in the
    /// response structure. In these cases, only the top-most-level audio output
    /// has content.
    #[prost(bytes = "vec", tag = "4")]
    pub output_audio: ::prost::alloc::vec::Vec<u8>,
    /// The config used by the speech synthesizer to generate the output audio.
    #[prost(message, optional, tag = "5")]
    pub output_audio_config: ::core::option::Option<OutputAudioConfig>,
    /// Response type.
    #[prost(enumeration = "detect_intent_response::ResponseType", tag = "6")]
    pub response_type: i32,
    /// Indicates whether the partial response can be cancelled when a later
    /// response arrives. e.g. if the agent specified some music as partial
    /// response, it can be cancelled.
    #[prost(bool, tag = "7")]
    pub allow_cancellation: bool,
}
/// Nested message and enum types in `DetectIntentResponse`.
pub mod detect_intent_response {
    /// Represents different DetectIntentResponse types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResponseType {
        /// Not specified. This should never happen.
        Unspecified = 0,
        /// Partial response. e.g. Aggregated responses in a Fulfillment that enables
        /// `return_partial_response` can be returned as partial response.
        /// WARNING: partial response is not eligible for barge-in.
        Partial = 1,
        /// Final response.
        Final = 2,
    }
}
/// The top-level message sent by the client to the
/// [Sessions.StreamingDetectIntent][google.cloud.dialogflow.cx.v3.Sessions.StreamingDetectIntent] method.
///
/// Multiple request messages should be sent in order:
///
/// 1.  The first message must contain
/// [session][google.cloud.dialogflow.cx.v3.StreamingDetectIntentRequest.session],
///     [query_input][google.cloud.dialogflow.cx.v3.StreamingDetectIntentRequest.query_input] plus optionally
///     [query_params][google.cloud.dialogflow.cx.v3.StreamingDetectIntentRequest.query_params]. If the client
///     wants to receive an audio response, it should also contain
///     [output_audio_config][google.cloud.dialogflow.cx.v3.StreamingDetectIntentRequest.output_audio_config].
///
/// 2.  If [query_input][google.cloud.dialogflow.cx.v3.StreamingDetectIntentRequest.query_input] was set to
///     [query_input.audio.config][google.cloud.dialogflow.cx.v3.AudioInput.config], all subsequent messages
///     must contain [query_input.audio.audio][google.cloud.dialogflow.cx.v3.AudioInput.audio] to continue with
///     Speech recognition.
///     If you decide to rather detect an intent from text
///     input after you already started Speech recognition, please send a message
///     with [query_input.text][google.cloud.dialogflow.cx.v3.QueryInput.text].
///
///     However, note that:
///
///     * Dialogflow will bill you for the audio duration so far.
///     * Dialogflow discards all Speech recognition results in favor of the
///       input text.
///     * Dialogflow will use the language code from the first message.
///
/// After you sent all input, you must half-close or abort the request stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingDetectIntentRequest {
    /// The name of the session this query is sent to.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/sessions/<Session ID>` or `projects/<Project ID>/locations/<Location
    /// ID>/agents/<Agent ID>/environments/<Environment ID>/sessions/<Session ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment.
    /// It's up to the API caller to choose an appropriate `Session ID`. It can be
    /// a random number or some type of session identifiers (preferably hashed).
    /// The length of the `Session ID` must not exceed 36 characters.
    /// Note: session must be set in the first request.
    ///
    /// For more information, see the [sessions
    /// guide](https://cloud.google.com/dialogflow/cx/docs/concept/session).
    ///
    /// Note: Always use agent versions for production traffic.
    /// See [Versions and
    /// environments](https://cloud.google.com/dialogflow/cx/docs/concept/version).
    #[prost(string, tag = "1")]
    pub session: ::prost::alloc::string::String,
    /// The parameters of this query.
    #[prost(message, optional, tag = "2")]
    pub query_params: ::core::option::Option<QueryParameters>,
    /// Required. The input specification.
    #[prost(message, optional, tag = "3")]
    pub query_input: ::core::option::Option<QueryInput>,
    /// Instructs the speech synthesizer how to generate the output audio.
    #[prost(message, optional, tag = "4")]
    pub output_audio_config: ::core::option::Option<OutputAudioConfig>,
    /// Enable partial detect intent response. If this flag is not enabled,
    /// response stream still contains only one final `DetectIntentResponse` even
    /// if some `Fulfillment`s in the agent have been configured to return partial
    /// responses.
    #[prost(bool, tag = "5")]
    pub enable_partial_response: bool,
}
/// The top-level message returned from the `StreamingDetectIntent` method.
///
/// Multiple response messages can be returned in order:
///
/// 1.  If the input was set to streaming audio, the first one or more messages
///     contain `recognition_result`. Each `recognition_result` represents a more
///     complete transcript of what the user said. The last `recognition_result`
///     has `is_final` set to `true`.
///
/// 2.  If `enable_partial_response` is true, the following N messages
///     (currently 1 <= N <= 4) contain `detect_intent_response`. The first (N-1)
///     `detect_intent_response`s will have `response_type` set to `PARTIAL`.
///     The last `detect_intent_response` has `response_type` set to `FINAL`.
///     If `response_type` is false, response stream only contains
///     the final `detect_intent_response`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingDetectIntentResponse {
    /// The output response.
    #[prost(oneof = "streaming_detect_intent_response::Response", tags = "1, 2")]
    pub response: ::core::option::Option<streaming_detect_intent_response::Response>,
}
/// Nested message and enum types in `StreamingDetectIntentResponse`.
pub mod streaming_detect_intent_response {
    /// The output response.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// The result of speech recognition.
        #[prost(message, tag = "1")]
        RecognitionResult(super::StreamingRecognitionResult),
        /// The response from detect intent.
        #[prost(message, tag = "2")]
        DetectIntentResponse(super::DetectIntentResponse),
    }
}
/// Contains a speech recognition result corresponding to a portion of the audio
/// that is currently being processed or an indication that this is the end
/// of the single requested utterance.
///
/// Example:
///
/// 1.  transcript: "tube"
///
/// 2.  transcript: "to be a"
///
/// 3.  transcript: "to be"
///
/// 4.  transcript: "to be or not to be"
///     is_final: true
///
/// 5.  transcript: " that's"
///
/// 6.  transcript: " that is"
///
/// 7.  message_type: `END_OF_SINGLE_UTTERANCE`
///
/// 8.  transcript: " that is the question"
///     is_final: true
///
/// Only two of the responses contain final results (#4 and #8 indicated by
/// `is_final: true`). Concatenating these generates the full transcript: "to be
/// or not to be that is the question".
///
/// In each response we populate:
///
/// *  for `TRANSCRIPT`: `transcript` and possibly `is_final`.
///
/// *  for `END_OF_SINGLE_UTTERANCE`: only `message_type`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingRecognitionResult {
    /// Type of the result message.
    #[prost(enumeration = "streaming_recognition_result::MessageType", tag = "1")]
    pub message_type: i32,
    /// Transcript text representing the words that the user spoke.
    /// Populated if and only if `message_type` = `TRANSCRIPT`.
    #[prost(string, tag = "2")]
    pub transcript: ::prost::alloc::string::String,
    /// If `false`, the `StreamingRecognitionResult` represents an
    /// interim result that may change. If `true`, the recognizer will not return
    /// any further hypotheses about this piece of the audio. May only be populated
    /// for `message_type` = `TRANSCRIPT`.
    #[prost(bool, tag = "3")]
    pub is_final: bool,
    /// The Speech confidence between 0.0 and 1.0 for the current portion of audio.
    /// A higher number indicates an estimated greater likelihood that the
    /// recognized words are correct. The default of 0.0 is a sentinel value
    /// indicating that confidence was not set.
    ///
    /// This field is typically only provided if `is_final` is true and you should
    /// not rely on it being accurate or even set.
    #[prost(float, tag = "4")]
    pub confidence: f32,
    /// An estimate of the likelihood that the speech recognizer will
    /// not change its guess about this interim recognition result:
    /// * If the value is unspecified or 0.0, Dialogflow didn't compute the
    ///   stability. In particular, Dialogflow will only provide stability for
    ///   `TRANSCRIPT` results with `is_final = false`.
    /// * Otherwise, the value is in (0.0, 1.0] where 0.0 means completely
    ///   unstable and 1.0 means completely stable.
    #[prost(float, tag = "6")]
    pub stability: f32,
    /// Word-specific information for the words recognized by Speech in
    /// [transcript][google.cloud.dialogflow.cx.v3.StreamingRecognitionResult.transcript]. Populated if and only if `message_type` = `TRANSCRIPT` and
    /// [InputAudioConfig.enable_word_info] is set.
    #[prost(message, repeated, tag = "7")]
    pub speech_word_info: ::prost::alloc::vec::Vec<SpeechWordInfo>,
    /// Time offset of the end of this Speech recognition result relative to the
    /// beginning of the audio. Only populated for `message_type` =
    /// `TRANSCRIPT`.
    #[prost(message, optional, tag = "8")]
    pub speech_end_offset: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `StreamingRecognitionResult`.
pub mod streaming_recognition_result {
    /// Type of the response message.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MessageType {
        /// Not specified. Should never be used.
        Unspecified = 0,
        /// Message contains a (possibly partial) transcript.
        Transcript = 1,
        /// Event indicates that the server has detected the end of the user's speech
        /// utterance and expects no additional speech. Therefore, the server will
        /// not process additional audio (although it may subsequently return
        /// additional results). The client should stop sending additional audio
        /// data, half-close the gRPC connection, and wait for any additional results
        /// until the server closes the gRPC connection. This message is only sent if
        /// [`single_utterance`][google.cloud.dialogflow.cx.v3.InputAudioConfig.single_utterance] was set to
        /// `true`, and is not used otherwise.
        EndOfSingleUtterance = 2,
    }
}
/// Represents the parameters of a conversational query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParameters {
    /// The time zone of this conversational query from the [time zone
    /// database](https://www.iana.org/time-zones), e.g., America/New_York,
    /// Europe/Paris. If not provided, the time zone specified in the agent is
    /// used.
    #[prost(string, tag = "1")]
    pub time_zone: ::prost::alloc::string::String,
    /// The geo location of this conversational query.
    #[prost(message, optional, tag = "2")]
    pub geo_location: ::core::option::Option<super::super::super::super::r#type::LatLng>,
    /// Additional session entity types to replace or extend developer entity types
    /// with. The entity synonyms apply to all languages and persist for the
    /// session of this query.
    #[prost(message, repeated, tag = "3")]
    pub session_entity_types: ::prost::alloc::vec::Vec<SessionEntityType>,
    /// This field can be used to pass custom data into the webhook associated with
    /// the agent. Arbitrary JSON objects are supported.
    /// Some integrations that query a Dialogflow agent may provide additional
    /// information in the payload.
    /// In particular, for the Dialogflow Phone Gateway integration, this field has
    /// the form:
    /// ```
    /// {
    ///  "telephony": {
    ///    "caller_id": "+18558363987"
    ///  }
    /// }
    /// ```
    #[prost(message, optional, tag = "4")]
    pub payload: ::core::option::Option<::prost_types::Struct>,
    /// Additional parameters to be put into [session
    /// parameters][SessionInfo.parameters]. To remove a
    /// parameter from the session, clients should explicitly set the parameter
    /// value to null.
    ///
    /// You can reference the session parameters in the agent with the following
    /// format: $session.params.parameter-id.
    ///
    /// Depending on your protocol or client library language, this is a
    /// map, associative array, symbol table, dictionary, or JSON object
    /// composed of a collection of (MapKey, MapValue) pairs:
    ///
    /// -   MapKey type: string
    /// -   MapKey value: parameter name
    /// -   MapValue type:
    ///     -   If parameter's entity type is a composite entity: map
    ///     -   Else: depending on parameter value type, could be one of string,
    ///         number, boolean, null, list or map
    /// -   MapValue value:
    ///     -   If parameter's entity type is a composite entity:
    ///         map from composite entity property names to property values
    ///     -   Else: parameter value
    #[prost(message, optional, tag = "5")]
    pub parameters: ::core::option::Option<::prost_types::Struct>,
    /// The unique identifier of the [page][google.cloud.dialogflow.cx.v3.Page] to override the [current
    /// page][QueryResult.current_page] in the session.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/pages/<Page ID>`.
    ///
    /// If `current_page` is specified, the previous state of the session will be
    /// ignored by Dialogflow, including the [previous
    /// page][QueryResult.current_page] and the [previous session
    /// parameters][QueryResult.parameters].
    /// In most cases, [current_page][google.cloud.dialogflow.cx.v3.QueryParameters.current_page] and
    /// [parameters][google.cloud.dialogflow.cx.v3.QueryParameters.parameters] should be configured together to
    /// direct a session to a specific state.
    #[prost(string, tag = "6")]
    pub current_page: ::prost::alloc::string::String,
    /// Whether to disable webhook calls for this request.
    #[prost(bool, tag = "7")]
    pub disable_webhook: bool,
    /// Configures whether sentiment analysis should be performed. If not
    /// provided, sentiment analysis is not performed.
    #[prost(bool, tag = "8")]
    pub analyze_query_text_sentiment: bool,
    /// This field can be used to pass HTTP headers for a webhook
    /// call. These headers will be sent to webhook along with the headers that
    /// have been configured through Dialogflow web console. The headers defined
    /// within this field will overwrite the headers configured through Dialogflow
    /// console if there is a conflict. Header names are case-insensitive.
    /// Google's specified headers are not allowed. Including: "Host",
    /// "Content-Length", "Connection", "From", "User-Agent", "Accept-Encoding",
    /// "If-Modified-Since", "If-None-Match", "X-Forwarded-For", etc.
    #[prost(map = "string, string", tag = "10")]
    pub webhook_headers:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Represents the query input. It can contain one of:
///
/// 1.  A conversational query in the form of text.
///
/// 2.  An intent query that specifies which intent to trigger.
///
/// 3.  Natural language speech audio to be processed.
///
/// 4.  An event to be triggered.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInput {
    /// Required. The language of the input. See [Language
    /// Support](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// for a list of the currently supported language codes. Note that queries in
    /// the same session do not necessarily need to specify the same language.
    #[prost(string, tag = "4")]
    pub language_code: ::prost::alloc::string::String,
    /// Required. The input specification.
    #[prost(oneof = "query_input::Input", tags = "2, 3, 5, 6, 7")]
    pub input: ::core::option::Option<query_input::Input>,
}
/// Nested message and enum types in `QueryInput`.
pub mod query_input {
    /// Required. The input specification.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Input {
        /// The natural language text to be processed.
        #[prost(message, tag = "2")]
        Text(super::TextInput),
        /// The intent to be triggered.
        #[prost(message, tag = "3")]
        Intent(super::IntentInput),
        /// The natural language speech audio to be processed.
        #[prost(message, tag = "5")]
        Audio(super::AudioInput),
        /// The event to be triggered.
        #[prost(message, tag = "6")]
        Event(super::EventInput),
        /// The DTMF event to be handled.
        #[prost(message, tag = "7")]
        Dtmf(super::DtmfInput),
    }
}
/// Represents the result of a conversational query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResult {
    /// The language that was triggered during intent detection.
    /// See [Language
    /// Support](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// for a list of the currently supported language codes.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// The collected [session parameters][google.cloud.dialogflow.cx.v3.SessionInfo.parameters].
    ///
    /// Depending on your protocol or client library language, this is a
    /// map, associative array, symbol table, dictionary, or JSON object
    /// composed of a collection of (MapKey, MapValue) pairs:
    ///
    /// -   MapKey type: string
    /// -   MapKey value: parameter name
    /// -   MapValue type:
    ///     -   If parameter's entity type is a composite entity: map
    ///     -   Else: depending on parameter value type, could be one of string,
    ///         number, boolean, null, list or map
    /// -   MapValue value:
    ///     -   If parameter's entity type is a composite entity:
    ///         map from composite entity property names to property values
    ///     -   Else: parameter value
    #[prost(message, optional, tag = "3")]
    pub parameters: ::core::option::Option<::prost_types::Struct>,
    /// The list of rich messages returned to the client. Responses vary from
    /// simple text messages to more sophisticated, structured payloads used
    /// to drive complex logic.
    #[prost(message, repeated, tag = "4")]
    pub response_messages: ::prost::alloc::vec::Vec<ResponseMessage>,
    /// The list of webhook call status in the order of call sequence.
    #[prost(message, repeated, tag = "13")]
    pub webhook_statuses: ::prost::alloc::vec::Vec<super::super::super::super::rpc::Status>,
    /// The list of webhook payload in [WebhookResponse.payload][google.cloud.dialogflow.cx.v3.WebhookResponse.payload], in
    /// the order of call sequence. If some webhook call fails or doesn't return
    /// any payload, an empty `Struct` would be used instead.
    #[prost(message, repeated, tag = "6")]
    pub webhook_payloads: ::prost::alloc::vec::Vec<::prost_types::Struct>,
    /// The current [Page][google.cloud.dialogflow.cx.v3.Page]. Some, not all fields are filled in this message,
    /// including but not limited to `name` and `display_name`.
    #[prost(message, optional, tag = "7")]
    pub current_page: ::core::option::Option<Page>,
    /// The [Intent][google.cloud.dialogflow.cx.v3.Intent] that matched the conversational query. Some, not all fields
    /// are filled in this message, including but not limited to: `name` and
    /// `display_name`.
    /// This field is deprecated, please use [QueryResult.match][google.cloud.dialogflow.cx.v3.QueryResult.match] instead.
    #[deprecated]
    #[prost(message, optional, tag = "8")]
    pub intent: ::core::option::Option<Intent>,
    /// The intent detection confidence. Values range from 0.0 (completely
    /// uncertain) to 1.0 (completely certain).
    /// This value is for informational purpose only and is only used to
    /// help match the best intent within the classification threshold.
    /// This value may change for the same end-user expression at any time due to a
    /// model retraining or change in implementation.
    /// This field is deprecated, please use [QueryResult.match][google.cloud.dialogflow.cx.v3.QueryResult.match] instead.
    #[deprecated]
    #[prost(float, tag = "9")]
    pub intent_detection_confidence: f32,
    /// Intent match result, could be an intent or an event.
    #[prost(message, optional, tag = "15")]
    pub r#match: ::core::option::Option<Match>,
    /// The free-form diagnostic info. For example, this field could contain
    /// webhook call latency. The string keys of the Struct's fields map can change
    /// without notice.
    #[prost(message, optional, tag = "10")]
    pub diagnostic_info: ::core::option::Option<::prost_types::Struct>,
    /// The sentiment analyss result, which depends on
    /// [`analyze_query_text_sentiment`]
    /// [google.cloud.dialogflow.cx.v3.QueryParameters.analyze_query_text_sentiment], specified in the request.
    #[prost(message, optional, tag = "17")]
    pub sentiment_analysis_result: ::core::option::Option<SentimentAnalysisResult>,
    /// The original conversational query.
    #[prost(oneof = "query_result::Query", tags = "1, 11, 12, 14")]
    pub query: ::core::option::Option<query_result::Query>,
}
/// Nested message and enum types in `QueryResult`.
pub mod query_result {
    /// The original conversational query.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        /// If [natural language text][google.cloud.dialogflow.cx.v3.TextInput] was provided as input, this field
        /// will contain a copy of the text.
        #[prost(string, tag = "1")]
        Text(::prost::alloc::string::String),
        /// If an [intent][google.cloud.dialogflow.cx.v3.IntentInput] was provided as input, this field will
        /// contain a copy of the intent identifier.
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/intents/<Intent ID>`.
        #[prost(string, tag = "11")]
        TriggerIntent(::prost::alloc::string::String),
        /// If [natural language speech audio][google.cloud.dialogflow.cx.v3.AudioInput] was provided as input,
        /// this field will contain the transcript for the audio.
        #[prost(string, tag = "12")]
        Transcript(::prost::alloc::string::String),
        /// If an [event][google.cloud.dialogflow.cx.v3.EventInput] was provided as input, this field will contain
        /// the name of the event.
        #[prost(string, tag = "14")]
        TriggerEvent(::prost::alloc::string::String),
    }
}
/// Represents the natural language text to be processed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextInput {
    /// Required. The UTF-8 encoded natural language text to be processed. Text length must
    /// not exceed 256 characters.
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
}
/// Represents the intent to trigger programmatically rather than as a result of
/// natural language processing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentInput {
    /// Required. The unique identifier of the intent.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/intents/<Intent ID>`.
    #[prost(string, tag = "1")]
    pub intent: ::prost::alloc::string::String,
}
/// Represents the natural speech audio to be processed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioInput {
    /// Required. Instructs the speech recognizer how to process the speech audio.
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<InputAudioConfig>,
    /// The natural language speech audio to be processed.
    /// A single request can contain up to 1 minute of speech audio data.
    /// The [transcribed text][google.cloud.dialogflow.cx.v3.QueryResult.transcript] cannot contain more than 256
    /// bytes.
    ///
    /// For non-streaming audio detect intent, both `config` and `audio` must be
    /// provided.
    /// For streaming audio detect intent, `config` must be provided in
    /// the first request and `audio` must be provided in all following requests.
    #[prost(bytes = "vec", tag = "2")]
    pub audio: ::prost::alloc::vec::Vec<u8>,
}
/// Represents the event to trigger.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventInput {
    /// Name of the event.
    #[prost(string, tag = "1")]
    pub event: ::prost::alloc::string::String,
}
/// Represents the input for dtmf event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DtmfInput {
    /// The dtmf digits.
    #[prost(string, tag = "1")]
    pub digits: ::prost::alloc::string::String,
    /// The finish digit (if any).
    #[prost(string, tag = "2")]
    pub finish_digit: ::prost::alloc::string::String,
}
/// Represents one match result of [MatchIntent][].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Match {
    /// The [Intent][google.cloud.dialogflow.cx.v3.Intent] that matched the query. Some, not all fields are filled in
    /// this message, including but not limited to: `name` and `display_name`. Only
    /// filled for [`INTENT`][google.cloud.dialogflow.cx.v3.Match.MatchType] match type.
    #[prost(message, optional, tag = "1")]
    pub intent: ::core::option::Option<Intent>,
    /// The event that matched the query. Only filled for
    /// [`EVENT`][google.cloud.dialogflow.cx.v3.Match.MatchType] match type.
    #[prost(string, tag = "6")]
    pub event: ::prost::alloc::string::String,
    /// The collection of parameters extracted from the query.
    ///
    /// Depending on your protocol or client library language, this is a
    /// map, associative array, symbol table, dictionary, or JSON object
    /// composed of a collection of (MapKey, MapValue) pairs:
    ///
    /// -   MapKey type: string
    /// -   MapKey value: parameter name
    /// -   MapValue type:
    ///     -   If parameter's entity type is a composite entity: map
    ///     -   Else: depending on parameter value type, could be one of string,
    ///         number, boolean, null, list or map
    /// -   MapValue value:
    ///     -   If parameter's entity type is a composite entity:
    ///         map from composite entity property names to property values
    ///     -   Else: parameter value
    #[prost(message, optional, tag = "2")]
    pub parameters: ::core::option::Option<::prost_types::Struct>,
    /// Final text input which was matched during MatchIntent. This value can be
    /// different from original input sent in request because of spelling
    /// correction or other processing.
    #[prost(string, tag = "3")]
    pub resolved_input: ::prost::alloc::string::String,
    /// Type of this [Match][google.cloud.dialogflow.cx.v3.Match].
    #[prost(enumeration = "r#match::MatchType", tag = "4")]
    pub match_type: i32,
    /// The confidence of this match. Values range from 0.0 (completely uncertain)
    /// to 1.0 (completely certain).
    /// This value is for informational purpose only and is only used to help match
    /// the best intent within the classification threshold. This value may change
    /// for the same end-user expression at any time due to a model retraining or
    /// change in implementation.
    #[prost(float, tag = "5")]
    pub confidence: f32,
}
/// Nested message and enum types in `Match`.
pub mod r#match {
    /// Type of a Match.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MatchType {
        /// Not specified. Should never be used.
        Unspecified = 0,
        /// The query was matched to an intent.
        Intent = 1,
        /// The query directly triggered an intent.
        DirectIntent = 2,
        /// The query was used for parameter filling.
        ParameterFilling = 3,
        /// No match was found for the query.
        NoMatch = 4,
        /// Indicates an empty query.
        NoInput = 5,
        /// The query directly triggered an event.
        Event = 6,
    }
}
/// Request of [MatchIntent][].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchIntentRequest {
    /// Required. The name of the session this query is sent to.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/sessions/<Session ID>` or `projects/<Project ID>/locations/<Location
    /// ID>/agents/<Agent ID>/environments/<Environment ID>/sessions/<Session ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment.
    /// It's up to the API caller to choose an appropriate `Session ID`. It can be
    /// a random number or some type of session identifiers (preferably hashed).
    /// The length of the `Session ID` must not exceed 36 characters.
    ///
    /// For more information, see the [sessions
    /// guide](https://cloud.google.com/dialogflow/cx/docs/concept/session).
    #[prost(string, tag = "1")]
    pub session: ::prost::alloc::string::String,
    /// The parameters of this query.
    #[prost(message, optional, tag = "2")]
    pub query_params: ::core::option::Option<QueryParameters>,
    /// Required. The input specification.
    #[prost(message, optional, tag = "3")]
    pub query_input: ::core::option::Option<QueryInput>,
}
/// Response of [MatchIntent][].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchIntentResponse {
    /// Match results, if more than one, ordered descendingly by the confidence
    /// we have that the particular intent matches the query.
    #[prost(message, repeated, tag = "4")]
    pub matches: ::prost::alloc::vec::Vec<Match>,
    /// The current [Page][google.cloud.dialogflow.cx.v3.Page]. Some, not all fields are filled in this message,
    /// including but not limited to `name` and `display_name`.
    #[prost(message, optional, tag = "5")]
    pub current_page: ::core::option::Option<Page>,
    /// The original conversational query.
    #[prost(oneof = "match_intent_response::Query", tags = "1, 2, 3, 6")]
    pub query: ::core::option::Option<match_intent_response::Query>,
}
/// Nested message and enum types in `MatchIntentResponse`.
pub mod match_intent_response {
    /// The original conversational query.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        /// If [natural language text][google.cloud.dialogflow.cx.v3.TextInput] was provided as input, this field
        /// will contain a copy of the text.
        #[prost(string, tag = "1")]
        Text(::prost::alloc::string::String),
        /// If an [intent][google.cloud.dialogflow.cx.v3.IntentInput] was provided as input, this field will
        /// contain a copy of the intent identifier.
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/intents/<Intent ID>`.
        #[prost(string, tag = "2")]
        TriggerIntent(::prost::alloc::string::String),
        /// If [natural language speech audio][google.cloud.dialogflow.cx.v3.AudioInput] was provided as input,
        /// this field will contain the transcript for the audio.
        #[prost(string, tag = "3")]
        Transcript(::prost::alloc::string::String),
        /// If an [event][google.cloud.dialogflow.cx.v3.EventInput] was provided as input, this field will
        /// contain a copy of the event name.
        #[prost(string, tag = "6")]
        TriggerEvent(::prost::alloc::string::String),
    }
}
/// Request of [FulfillIntent][]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FulfillIntentRequest {
    /// Must be same as the corresponding MatchIntent request, otherwise the
    /// behavior is undefined.
    #[prost(message, optional, tag = "1")]
    pub match_intent_request: ::core::option::Option<MatchIntentRequest>,
    /// The matched intent/event to fulfill.
    #[prost(message, optional, tag = "2")]
    pub r#match: ::core::option::Option<Match>,
    /// Instructs the speech synthesizer how to generate output audio.
    #[prost(message, optional, tag = "3")]
    pub output_audio_config: ::core::option::Option<OutputAudioConfig>,
}
/// Response of [FulfillIntent][]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FulfillIntentResponse {
    /// Output only. The unique identifier of the response. It can be used to
    /// locate a response in the training example set or for reporting issues.
    #[prost(string, tag = "1")]
    pub response_id: ::prost::alloc::string::String,
    /// The result of the conversational query.
    #[prost(message, optional, tag = "2")]
    pub query_result: ::core::option::Option<QueryResult>,
    /// The audio data bytes encoded as specified in the request.
    /// Note: The output audio is generated based on the values of default platform
    /// text responses found in the
    /// [`query_result.response_messages`][google.cloud.dialogflow.cx.v3.QueryResult.response_messages] field. If
    /// multiple default text responses exist, they will be concatenated when
    /// generating audio. If no default platform text responses exist, the
    /// generated audio content will be empty.
    ///
    /// In some scenarios, multiple output audio fields may be present in the
    /// response structure. In these cases, only the top-most-level audio output
    /// has content.
    #[prost(bytes = "vec", tag = "3")]
    pub output_audio: ::prost::alloc::vec::Vec<u8>,
    /// The config used by the speech synthesizer to generate the output audio.
    #[prost(message, optional, tag = "4")]
    pub output_audio_config: ::core::option::Option<OutputAudioConfig>,
}
/// The result of sentiment analysis. Sentiment analysis inspects user input
/// and identifies the prevailing subjective opinion, especially to determine a
/// user's attitude as positive, negative, or neutral.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SentimentAnalysisResult {
    /// Sentiment score between -1.0 (negative sentiment) and 1.0 (positive
    /// sentiment).
    #[prost(float, tag = "1")]
    pub score: f32,
    /// A non-negative number in the [0, +inf) range, which represents the absolute
    /// magnitude of sentiment, regardless of score (positive or negative).
    #[prost(float, tag = "2")]
    pub magnitude: f32,
}
#[doc = r" Generated client implementations."]
pub mod sessions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A session represents an interaction with a user. You retrieve user input"]
    #[doc = " and pass it to the [DetectIntent][google.cloud.dialogflow.cx.v3.Sessions.DetectIntent] method to determine"]
    #[doc = " user intent and respond."]
    #[derive(Debug, Clone)]
    pub struct SessionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SessionsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
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
        ) -> SessionsClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            SessionsClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Processes a natural language query and returns structured, actionable data"]
        #[doc = " as a result. This method is not idempotent, because it may cause session"]
        #[doc = " entity types to be updated, which in turn might affect results of future"]
        #[doc = " queries."]
        #[doc = ""]
        #[doc = " Note: Always use agent versions for production traffic."]
        #[doc = " See [Versions and"]
        #[doc = " environments](https://cloud.google.com/dialogflow/cx/docs/concept/version)."]
        pub async fn detect_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::DetectIntentRequest>,
        ) -> Result<tonic::Response<super::DetectIntentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Sessions/DetectIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Processes a natural language query in audio format in a streaming fashion"]
        #[doc = " and returns structured, actionable data as a result. This method is only"]
        #[doc = " available via the gRPC API (not REST)."]
        #[doc = ""]
        #[doc = " Note: Always use agent versions for production traffic."]
        #[doc = " See [Versions and"]
        #[doc = " environments](https://cloud.google.com/dialogflow/cx/docs/concept/version)."]
        pub async fn streaming_detect_intent(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::StreamingDetectIntentRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StreamingDetectIntentResponse>>,
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
                "/google.cloud.dialogflow.cx.v3.Sessions/StreamingDetectIntent",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " Returns preliminary intent match results, doesn't change the session"]
        #[doc = " status."]
        pub async fn match_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::MatchIntentRequest>,
        ) -> Result<tonic::Response<super::MatchIntentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Sessions/MatchIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Fulfills a matched intent returned by [MatchIntent][google.cloud.dialogflow.cx.v3.Sessions.MatchIntent]."]
        #[doc = " Must be called after [MatchIntent][google.cloud.dialogflow.cx.v3.Sessions.MatchIntent], with input from"]
        #[doc = " [MatchIntentResponse][google.cloud.dialogflow.cx.v3.MatchIntentResponse]. Otherwise, the behavior is undefined."]
        pub async fn fulfill_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::FulfillIntentRequest>,
        ) -> Result<tonic::Response<super::FulfillIntentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Sessions/FulfillIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// An TransitionRouteGroup represents a group of
/// [`TransitionRoutes`][google.cloud.dialogflow.cx.v3.TransitionRoute] to be used by a [Page][google.cloud.dialogflow.cx.v3.Page].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitionRouteGroup {
    /// The unique identifier of the transition route group.
    /// [TransitionRouteGroups.CreateTransitionRouteGroup][google.cloud.dialogflow.cx.v3.TransitionRouteGroups.CreateTransitionRouteGroup] populates the name
    /// automatically.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/transitionRouteGroups/<Transition Route Group ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The human-readable name of the transition route group, unique within
    /// the [Agent][google.cloud.dialogflow.cx.v3.Agent]. The display name can be no longer than 30 characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Transition routes associated with the [TransitionRouteGroup][google.cloud.dialogflow.cx.v3.TransitionRouteGroup].
    #[prost(message, repeated, tag = "5")]
    pub transition_routes: ::prost::alloc::vec::Vec<TransitionRoute>,
}
/// The request message for [TransitionRouteGroups.ListTransitionRouteGroups][google.cloud.dialogflow.cx.v3.TransitionRouteGroups.ListTransitionRouteGroups].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransitionRouteGroupsRequest {
    /// Required. The flow to list all transition route groups for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The language to list transition route groups for. The following fields are
    /// language dependent:
    ///
    /// *  `TransitionRouteGroup.transition_routes.trigger_fulfillment.messages`
    /// *
    /// `TransitionRouteGroup.transition_routes.trigger_fulfillment.conditional_cases`
    ///
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "4")]
    pub language_code: ::prost::alloc::string::String,
}
/// The response message for [TransitionRouteGroups.ListTransitionRouteGroups][google.cloud.dialogflow.cx.v3.TransitionRouteGroups.ListTransitionRouteGroups].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransitionRouteGroupsResponse {
    /// The list of transition route groups. There will be a maximum number of
    /// items returned based on the page_size field in the request. The list may in
    /// some cases be empty or contain fewer entries than page_size even if this
    /// isn't the last page.
    #[prost(message, repeated, tag = "1")]
    pub transition_route_groups: ::prost::alloc::vec::Vec<TransitionRouteGroup>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for [TransitionRouteGroups.GetTransitionRouteGroup][google.cloud.dialogflow.cx.v3.TransitionRouteGroups.GetTransitionRouteGroup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransitionRouteGroupRequest {
    /// Required. The name of the [TransitionRouteGroup][google.cloud.dialogflow.cx.v3.TransitionRouteGroup].
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/transitionRouteGroups/<Transition Route Group ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The language to retrieve the transition route group for. The following
    /// fields are language dependent:
    ///
    /// *  `TransitionRouteGroup.transition_routes.trigger_fulfillment.messages`
    /// *
    /// `TransitionRouteGroup.transition_routes.trigger_fulfillment.conditional_cases`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
}
/// The request message for [TransitionRouteGroups.CreateTransitionRouteGroup][google.cloud.dialogflow.cx.v3.TransitionRouteGroups.CreateTransitionRouteGroup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTransitionRouteGroupRequest {
    /// Required. The flow to create an [TransitionRouteGroup][google.cloud.dialogflow.cx.v3.TransitionRouteGroup] for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The transition route group to create.
    #[prost(message, optional, tag = "2")]
    pub transition_route_group: ::core::option::Option<TransitionRouteGroup>,
    /// The language of the following fields in `TransitionRouteGroup`:
    ///
    /// *  `TransitionRouteGroup.transition_routes.trigger_fulfillment.messages`
    /// *
    /// `TransitionRouteGroup.transition_routes.trigger_fulfillment.conditional_cases`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// The request message for [TransitionRouteGroups.UpdateTransitionRouteGroup][google.cloud.dialogflow.cx.v3.TransitionRouteGroups.UpdateTransitionRouteGroup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTransitionRouteGroupRequest {
    /// Required. The transition route group to update.
    #[prost(message, optional, tag = "1")]
    pub transition_route_group: ::core::option::Option<TransitionRouteGroup>,
    /// The mask to control which fields get updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The language of the following fields in `TransitionRouteGroup`:
    ///
    /// *  `TransitionRouteGroup.transition_routes.trigger_fulfillment.messages`
    /// *
    /// `TransitionRouteGroup.transition_routes.trigger_fulfillment.conditional_cases`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// The request message for [TransitionRouteGroups.DeleteTransitionRouteGroup][google.cloud.dialogflow.cx.v3.TransitionRouteGroups.DeleteTransitionRouteGroup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTransitionRouteGroupRequest {
    /// Required. The name of the [TransitionRouteGroup][google.cloud.dialogflow.cx.v3.TransitionRouteGroup] to delete.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/transitionRouteGroups/<Transition Route Group ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// This field has no effect for transition route group that no page is using.
    /// If the transition route group is referenced by any page:
    ///
    /// *  If `force` is set to false, an error will be returned with message
    ///    indicating pages that reference the transition route group.
    /// *  If `force` is set to true, Dialogflow will remove the transition route
    ///    group, as well as any reference to it.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
#[doc = r" Generated client implementations."]
pub mod transition_route_groups_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for managing [TransitionRouteGroups][google.cloud.dialogflow.cx.v3.TransitionRouteGroup]."]
    #[derive(Debug, Clone)]
    pub struct TransitionRouteGroupsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TransitionRouteGroupsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
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
        ) -> TransitionRouteGroupsClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            TransitionRouteGroupsClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns the list of all transition route groups in the specified flow."]
        pub async fn list_transition_route_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTransitionRouteGroupsRequest>,
        ) -> Result<tonic::Response<super::ListTransitionRouteGroupsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.TransitionRouteGroups/ListTransitionRouteGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified [TransitionRouteGroup][google.cloud.dialogflow.cx.v3.TransitionRouteGroup]."]
        pub async fn get_transition_route_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransitionRouteGroupRequest>,
        ) -> Result<tonic::Response<super::TransitionRouteGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.TransitionRouteGroups/GetTransitionRouteGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an [TransitionRouteGroup][google.cloud.dialogflow.cx.v3.TransitionRouteGroup] in the specified flow."]
        #[doc = ""]
        #[doc = " Note: You should always train a flow prior to sending it queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn create_transition_route_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTransitionRouteGroupRequest>,
        ) -> Result<tonic::Response<super::TransitionRouteGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.TransitionRouteGroups/CreateTransitionRouteGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified [TransitionRouteGroup][google.cloud.dialogflow.cx.v3.TransitionRouteGroup]."]
        #[doc = ""]
        #[doc = " Note: You should always train a flow prior to sending it queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn update_transition_route_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTransitionRouteGroupRequest>,
        ) -> Result<tonic::Response<super::TransitionRouteGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.TransitionRouteGroups/UpdateTransitionRouteGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified [TransitionRouteGroup][google.cloud.dialogflow.cx.v3.TransitionRouteGroup]."]
        #[doc = ""]
        #[doc = " Note: You should always train a flow prior to sending it queries. See the"]
        #[doc = " [training"]
        #[doc = " documentation](https://cloud.google.com/dialogflow/cx/docs/concept/training)."]
        pub async fn delete_transition_route_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTransitionRouteGroupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.TransitionRouteGroups/DeleteTransitionRouteGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Represents a test case.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestCase {
    /// The unique identifier of the test case.
    /// [TestCases.CreateTestCase][google.cloud.dialogflow.cx.v3.TestCases.CreateTestCase] will populate the name automatically.
    /// Otherwise use format: `projects/<Project ID>/locations/<LocationID>/agents/
    /// <AgentID>/testCases/<TestCase ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Tags are short descriptions that users may apply to test cases for
    /// organizational and filtering purposes. Each tag should start with "#" and
    /// has a limit of 30 characters.
    #[prost(string, repeated, tag = "2")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The human-readable name of the test case, unique within the agent. Limit of
    /// 200 characters.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Additional freeform notes about the test case. Limit of 400 characters.
    #[prost(string, tag = "4")]
    pub notes: ::prost::alloc::string::String,
    /// Config for the test case.
    #[prost(message, optional, tag = "13")]
    pub test_config: ::core::option::Option<TestConfig>,
    /// The conversation turns uttered when the test case was created, in
    /// chronological order. These include the canonical set of agent utterances
    /// that should occur when the agent is working properly.
    #[prost(message, repeated, tag = "5")]
    pub test_case_conversation_turns: ::prost::alloc::vec::Vec<ConversationTurn>,
    /// Output only. When the test was created.
    #[prost(message, optional, tag = "10")]
    pub creation_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The latest test result.
    #[prost(message, optional, tag = "12")]
    pub last_test_result: ::core::option::Option<TestCaseResult>,
}
/// Represents a result from running a test case in an agent environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestCaseResult {
    /// The resource name for the test case result. Format:
    /// `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/testCases/
    /// <TestCase ID>/results/<TestCaseResult ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Environment where the test was run. If not set, it indicates the draft
    /// environment.
    #[prost(string, tag = "2")]
    pub environment: ::prost::alloc::string::String,
    /// The conversation turns uttered during the test case replay in chronological
    /// order.
    #[prost(message, repeated, tag = "3")]
    pub conversation_turns: ::prost::alloc::vec::Vec<ConversationTurn>,
    /// Whether the test case passed in the agent environment.
    #[prost(enumeration = "TestResult", tag = "4")]
    pub test_result: i32,
    /// The time that the test was run.
    #[prost(message, optional, tag = "5")]
    pub test_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Represents configurations for a test case.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestConfig {
    /// Session parameters to be compared when calculating differences.
    #[prost(string, repeated, tag = "1")]
    pub tracking_parameters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Flow name. If not set, default start flow is assumed.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "2")]
    pub flow: ::prost::alloc::string::String,
}
/// One interaction between a human and virtual agent. The human provides some
/// input and the virtual agent provides a response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversationTurn {
    /// The user input.
    #[prost(message, optional, tag = "1")]
    pub user_input: ::core::option::Option<conversation_turn::UserInput>,
    /// The virtual agent output.
    #[prost(message, optional, tag = "2")]
    pub virtual_agent_output: ::core::option::Option<conversation_turn::VirtualAgentOutput>,
}
/// Nested message and enum types in `ConversationTurn`.
pub mod conversation_turn {
    /// The input from the human user.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserInput {
        /// Supports [text input][google.cloud.dialogflow.cx.v3.QueryInput.text], [event input][google.cloud.dialogflow.cx.v3.QueryInput.event],
        /// [dtmf input][google.cloud.dialogflow.cx.v3.QueryInput.dtmf] in the test case.
        #[prost(message, optional, tag = "5")]
        pub input: ::core::option::Option<super::QueryInput>,
        /// Parameters that need to be injected into the conversation during intent
        /// detection.
        #[prost(message, optional, tag = "2")]
        pub injected_parameters: ::core::option::Option<::prost_types::Struct>,
        /// If webhooks should be allowed to trigger in response to the user
        /// utterance. Often if parameters are injected, webhooks should not be
        /// enabled.
        #[prost(bool, tag = "3")]
        pub is_webhook_enabled: bool,
        /// Whether sentiment analysis is enabled.
        #[prost(bool, tag = "7")]
        pub enable_sentiment_analysis: bool,
    }
    /// The output from the virtual agent.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VirtualAgentOutput {
        /// The session parameters available to the bot at this point.
        #[prost(message, optional, tag = "4")]
        pub session_parameters: ::core::option::Option<::prost_types::Struct>,
        /// Output only. If this is part of a [result conversation
        /// turn][TestCaseResult.conversation_turns], the list of differences
        /// between the original run and the replay for this output, if any.
        #[prost(message, repeated, tag = "5")]
        pub differences: ::prost::alloc::vec::Vec<super::TestRunDifference>,
        /// Required. Input only. The diagnostic
        /// [info][Session.DetectIntentResponse.QueryResult.diagnostic_info]
        /// output for the turn.
        #[prost(message, optional, tag = "6")]
        pub diagnostic_info: ::core::option::Option<::prost_types::Struct>,
        /// The [Intent][google.cloud.dialogflow.cx.v3.Intent] that triggered the response. Only name and displayName
        /// will be set.
        #[prost(message, optional, tag = "7")]
        pub triggered_intent: ::core::option::Option<super::Intent>,
        /// The [Page][google.cloud.dialogflow.cx.v3.Page] on which the utterance was spoken. Only name and displayName
        /// will be set.
        #[prost(message, optional, tag = "8")]
        pub current_page: ::core::option::Option<super::Page>,
        /// The [text][google.cloud.dialogflow.cx.v3.ResponseMessage.Text] responses from the agent for the turn.
        #[prost(message, repeated, tag = "9")]
        pub text_responses: ::prost::alloc::vec::Vec<super::response_message::Text>,
        /// Response error from the agent in the test result. If set, other output
        /// is empty.
        #[prost(message, optional, tag = "10")]
        pub status: ::core::option::Option<super::super::super::super::super::rpc::Status>,
    }
}
/// The description of differences between original and replayed agent output.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestRunDifference {
    /// The type of diff.
    #[prost(enumeration = "test_run_difference::DiffType", tag = "1")]
    pub r#type: i32,
    /// A description of the diff, showing the actual output vs expected output.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TestRunDifference`.
pub mod test_run_difference {
    /// What part of the message replay differs from the test case.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DiffType {
        /// Should never be used.
        Unspecified = 0,
        /// The intent.
        Intent = 1,
        /// The page.
        Page = 2,
        /// The parameters.
        Parameters = 3,
        /// The message utterance.
        Utterance = 4,
    }
}
/// Transition coverage represents the percentage of all possible page
/// transitions (page-level transition routes and event handlers, excluding
/// transition route groups) present within any of a parent's test cases.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitionCoverage {
    /// The list of Transitions present in the agent.
    #[prost(message, repeated, tag = "1")]
    pub transitions: ::prost::alloc::vec::Vec<transition_coverage::Transition>,
    /// The percent of transitions in the agent that are covered.
    #[prost(float, tag = "2")]
    pub coverage_score: f32,
}
/// Nested message and enum types in `TransitionCoverage`.
pub mod transition_coverage {
    /// The source or target of a transition.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransitionNode {
        /// A TransitionNode can be either a page or a flow.
        #[prost(oneof = "transition_node::Kind", tags = "1, 2")]
        pub kind: ::core::option::Option<transition_node::Kind>,
    }
    /// Nested message and enum types in `TransitionNode`.
    pub mod transition_node {
        /// A TransitionNode can be either a page or a flow.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Kind {
            /// Indicates a transition to a [Page][google.cloud.dialogflow.cx.v3.Page]. Only some fields such as name and
            /// displayname will be set.
            #[prost(message, tag = "1")]
            Page(super::super::Page),
            /// Indicates a transition to a [Flow][google.cloud.dialogflow.cx.v3.Flow]. Only some fields such as name and
            /// displayname will be set.
            #[prost(message, tag = "2")]
            Flow(super::super::Flow),
        }
    }
    /// A transition in a page.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Transition {
        /// The start node of a transition.
        #[prost(message, optional, tag = "1")]
        pub source: ::core::option::Option<TransitionNode>,
        /// The index of a transition in the transition list. Starting from 0.
        #[prost(int32, tag = "4")]
        pub index: i32,
        /// The end node of a transition.
        #[prost(message, optional, tag = "2")]
        pub target: ::core::option::Option<TransitionNode>,
        /// Whether or not the transition is covered by at least one of the
        /// agent's test cases.
        #[prost(bool, tag = "3")]
        pub covered: bool,
        /// The detailed transition.
        #[prost(oneof = "transition::Detail", tags = "5, 6")]
        pub detail: ::core::option::Option<transition::Detail>,
    }
    /// Nested message and enum types in `Transition`.
    pub mod transition {
        /// The detailed transition.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Detail {
            /// Intent route or condition route.
            #[prost(message, tag = "5")]
            TransitionRoute(super::super::TransitionRoute),
            /// Event handler.
            #[prost(message, tag = "6")]
            EventHandler(super::super::EventHandler),
        }
    }
}
/// Transition route group coverage represents the percentage of all possible
/// transition routes present within any of a parent's test cases. The results
/// are grouped by the transition route group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitionRouteGroupCoverage {
    /// Transition route group coverages.
    #[prost(message, repeated, tag = "1")]
    pub coverages: ::prost::alloc::vec::Vec<transition_route_group_coverage::Coverage>,
    /// The percent of transition routes in all the transition route groups that
    /// are covered.
    #[prost(float, tag = "2")]
    pub coverage_score: f32,
}
/// Nested message and enum types in `TransitionRouteGroupCoverage`.
pub mod transition_route_group_coverage {
    /// Coverage result message for one transition route group.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Coverage {
        /// Transition route group metadata. Only name and displayName will be set.
        #[prost(message, optional, tag = "1")]
        pub route_group: ::core::option::Option<super::TransitionRouteGroup>,
        /// The list of transition routes and coverage in the transition route group.
        #[prost(message, repeated, tag = "2")]
        pub transitions: ::prost::alloc::vec::Vec<coverage::Transition>,
        /// The percent of transition routes in the transition route group that are
        /// covered.
        #[prost(float, tag = "3")]
        pub coverage_score: f32,
    }
    /// Nested message and enum types in `Coverage`.
    pub mod coverage {
        /// A transition coverage in a transition route group.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Transition {
            /// Intent route or condition route.
            #[prost(message, optional, tag = "1")]
            pub transition_route: ::core::option::Option<super::super::TransitionRoute>,
            /// Whether or not the transition route is covered by at least one of the
            /// agent's test cases.
            #[prost(bool, tag = "2")]
            pub covered: bool,
        }
    }
}
/// Intent coverage represents the percentage of all possible intents in the
/// agent that are triggered in any of a parent's test cases.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentCoverage {
    /// The list of Intents present in the agent
    #[prost(message, repeated, tag = "1")]
    pub intents: ::prost::alloc::vec::Vec<intent_coverage::Intent>,
    /// The percent of intents in the agent that are covered.
    #[prost(float, tag = "2")]
    pub coverage_score: f32,
}
/// Nested message and enum types in `IntentCoverage`.
pub mod intent_coverage {
    /// The agent's intent.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Intent {
        /// The intent full resource name
        #[prost(string, tag = "1")]
        pub intent: ::prost::alloc::string::String,
        /// Whether or not the intent is covered by at least one of the agent's
        /// test cases.
        #[prost(bool, tag = "2")]
        pub covered: bool,
    }
}
/// The request message for [TestCases.CalculateCoverage][google.cloud.dialogflow.cx.v3.TestCases.CalculateCoverage].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculateCoverageRequest {
    /// Required. The agent to calculate coverage for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "3")]
    pub agent: ::prost::alloc::string::String,
    /// Required. The type of coverage requested.
    #[prost(enumeration = "calculate_coverage_request::CoverageType", tag = "2")]
    pub r#type: i32,
}
/// Nested message and enum types in `CalculateCoverageRequest`.
pub mod calculate_coverage_request {
    /// The type of coverage score requested.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CoverageType {
        /// Should never be used.
        Unspecified = 0,
        /// Intent coverage.
        Intent = 1,
        /// Page transition coverage.
        PageTransition = 2,
        /// Transition route group coverage.
        TransitionRouteGroup = 3,
    }
}
/// The response message for [TestCases.CalculateCoverage][google.cloud.dialogflow.cx.v3.TestCases.CalculateCoverage].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculateCoverageResponse {
    /// The agent to calculate coverage for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "5")]
    pub agent: ::prost::alloc::string::String,
    /// The type of coverage requested.
    #[prost(oneof = "calculate_coverage_response::CoverageType", tags = "2, 4, 6")]
    pub coverage_type: ::core::option::Option<calculate_coverage_response::CoverageType>,
}
/// Nested message and enum types in `CalculateCoverageResponse`.
pub mod calculate_coverage_response {
    /// The type of coverage requested.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CoverageType {
        /// Intent coverage.
        #[prost(message, tag = "2")]
        IntentCoverage(super::IntentCoverage),
        /// Transition (excluding transition route groups) coverage.
        #[prost(message, tag = "4")]
        TransitionCoverage(super::TransitionCoverage),
        /// Transition route group coverage.
        #[prost(message, tag = "6")]
        RouteGroupCoverage(super::TransitionRouteGroupCoverage),
    }
}
/// The request message for [TestCases.ListTestCases][google.cloud.dialogflow.cx.v3.TestCases.ListTestCases].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTestCasesRequest {
    /// Required. The agent to list all pages for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return in a single page. By default 20.
    /// Note that when TestCaseView = FULL, the maximum page size allowed is 20.
    /// When TestCaseView = BASIC, the maximum page size allowed is 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Specifies whether response should include all fields or just the metadata.
    #[prost(enumeration = "list_test_cases_request::TestCaseView", tag = "4")]
    pub view: i32,
}
/// Nested message and enum types in `ListTestCasesRequest`.
pub mod list_test_cases_request {
    /// Specifies how much test case information to include in the response.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TestCaseView {
        /// The default / unset value.
        /// The API will default to the BASIC view.
        Unspecified = 0,
        /// Include basic metadata about the test case, but not the conversation
        /// turns. This is the default value.
        Basic = 1,
        /// Include everything.
        Full = 2,
    }
}
/// The response message for [TestCases.ListTestCases][google.cloud.dialogflow.cx.v3.TestCases.ListTestCases].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTestCasesResponse {
    /// The list of test cases. There will be a maximum number of items returned
    /// based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub test_cases: ::prost::alloc::vec::Vec<TestCase>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for [TestCases.BatchDeleteTestCases][google.cloud.dialogflow.cx.v3.TestCases.BatchDeleteTestCases].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteTestCasesRequest {
    /// Required. The agent to delete test cases from.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Format of test case names: `projects/<Project ID>/locations/
    /// <Location ID>/agents/<AgentID>/testCases/<TestCase ID>`.
    #[prost(string, repeated, tag = "3")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request message for [TestCases.CreateTestCase][google.cloud.dialogflow.cx.v3.TestCases.CreateTestCase].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTestCaseRequest {
    /// Required. The agent to create the test case for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The test case to create.
    #[prost(message, optional, tag = "2")]
    pub test_case: ::core::option::Option<TestCase>,
}
/// The request message for [TestCases.UpdateTestCase][google.cloud.dialogflow.cx.v3.TestCases.UpdateTestCase].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTestCaseRequest {
    /// Required. The test case to update.
    #[prost(message, optional, tag = "1")]
    pub test_case: ::core::option::Option<TestCase>,
    /// Required. The mask to specify which fields should be updated. The
    /// [`creationTime`][google.cloud.dialogflow.cx.v3.TestCase.creation_time] and
    /// [`lastTestResult`][google.cloud.dialogflow.cx.v3.TestCase.last_test_result] cannot be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for [TestCases.GetTestCase][google.cloud.dialogflow.cx.v3.TestCases.GetTestCase].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTestCaseRequest {
    /// Required. The name of the testcase.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/testCases/<TestCase ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for [TestCases.RunTestCase][google.cloud.dialogflow.cx.v3.TestCases.RunTestCase].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunTestCaseRequest {
    /// Required. Format of test case name to run: `projects/<Project ID>/locations/
    /// <Location ID>/agents/<AgentID>/testCases/<TestCase ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Environment name. If not set, draft environment is assumed.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>`.
    #[prost(string, tag = "2")]
    pub environment: ::prost::alloc::string::String,
}
/// The response message for [TestCases.RunTestCase][google.cloud.dialogflow.cx.v3.TestCases.RunTestCase].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunTestCaseResponse {
    /// The result.
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<TestCaseResult>,
}
/// Metadata returned for the [TestCases.RunTestCase][google.cloud.dialogflow.cx.v3.TestCases.RunTestCase] long running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunTestCaseMetadata {}
/// The request message for [TestCases.BatchRunTestCases][google.cloud.dialogflow.cx.v3.TestCases.BatchRunTestCases].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRunTestCasesRequest {
    /// Required. Agent name. Format: `projects/<Project ID>/locations/<Location ID>/agents/
    /// <AgentID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. If not set, draft environment is assumed. Format: `projects/<Project
    /// ID>/locations/<Location ID>/agents/<Agent ID>/environments/<Environment
    /// ID>`.
    #[prost(string, tag = "2")]
    pub environment: ::prost::alloc::string::String,
    /// Required. Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/testCases/<TestCase ID>`.
    #[prost(string, repeated, tag = "3")]
    pub test_cases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The response message for [TestCases.BatchRunTestCases][google.cloud.dialogflow.cx.v3.TestCases.BatchRunTestCases].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRunTestCasesResponse {
    /// The test case results. The detailed
    /// [conversation turns][google.cloud.dialogflow.cx.v3.TestCaseResult.conversation_turns] are empty in this
    /// response.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<TestCaseResult>,
}
/// Metadata returned for the [TestCases.BatchRunTestCases][google.cloud.dialogflow.cx.v3.TestCases.BatchRunTestCases] long running
/// operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRunTestCasesMetadata {
    /// The test errors.
    #[prost(message, repeated, tag = "1")]
    pub errors: ::prost::alloc::vec::Vec<TestError>,
}
/// Error info for running a test.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestError {
    /// The test case resource name.
    #[prost(string, tag = "1")]
    pub test_case: ::prost::alloc::string::String,
    /// The status associated with the test.
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// The timestamp when the test was completed.
    #[prost(message, optional, tag = "3")]
    pub test_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The request message for [TestCases.ImportTestCases][google.cloud.dialogflow.cx.v3.TestCases.ImportTestCases].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportTestCasesRequest {
    /// Required. The agent to import test cases to.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The source to import.
    #[prost(oneof = "import_test_cases_request::Source", tags = "2, 3")]
    pub source: ::core::option::Option<import_test_cases_request::Source>,
}
/// Nested message and enum types in `ImportTestCasesRequest`.
pub mod import_test_cases_request {
    /// Required. The source to import.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI
        /// to import test cases from. The format of this URI must be
        /// `gs://<bucket-name>/<object-name>`.
        #[prost(string, tag = "2")]
        GcsUri(::prost::alloc::string::String),
        /// Uncompressed raw byte content for test cases.
        #[prost(bytes, tag = "3")]
        Content(::prost::alloc::vec::Vec<u8>),
    }
}
/// The response message for [TestCases.ImportTestCases][google.cloud.dialogflow.cx.v3.TestCases.ImportTestCases].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportTestCasesResponse {
    /// The unique identifiers of the new test cases.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/testCases/<TestCase ID>`.
    #[prost(string, repeated, tag = "1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Metadata returned for the [TestCases.ImportTestCases][google.cloud.dialogflow.cx.v3.TestCases.ImportTestCases] long running
/// operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportTestCasesMetadata {
    /// Errors for failed test cases.
    #[prost(message, repeated, tag = "1")]
    pub errors: ::prost::alloc::vec::Vec<TestCaseError>,
}
/// Error info for importing a test.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestCaseError {
    /// The test case.
    #[prost(message, optional, tag = "1")]
    pub test_case: ::core::option::Option<TestCase>,
    /// The status associated with the test case.
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The request message for [TestCases.ExportTestCases][google.cloud.dialogflow.cx.v3.TestCases.ExportTestCases].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportTestCasesRequest {
    /// Required. The agent where to export test cases from.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The data format of the exported test cases. If not specified, `BLOB` is
    /// assumed.
    #[prost(enumeration = "export_test_cases_request::DataFormat", tag = "3")]
    pub data_format: i32,
    /// The filter expression used to filter exported test cases, see
    /// [API Filtering](https://aip.dev/160). The expression is case insensitive
    /// and supports the following syntax:
    ///
    ///   name = <value> [OR name = <value>] ...
    ///
    /// For example:
    ///
    /// *   "name = t1 OR name = t2" matches the test case with the exact resource
    ///     name "t1" or "t2".
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// The destination to export.
    #[prost(oneof = "export_test_cases_request::Destination", tags = "2")]
    pub destination: ::core::option::Option<export_test_cases_request::Destination>,
}
/// Nested message and enum types in `ExportTestCasesRequest`.
pub mod export_test_cases_request {
    /// Data format of the exported test cases.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DataFormat {
        /// Unspecified format.
        Unspecified = 0,
        /// Raw bytes.
        Blob = 1,
        /// JSON format.
        Json = 2,
    }
    /// The destination to export.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI to
        /// export the test cases to. The format of this URI must be
        /// `gs://<bucket-name>/<object-name>`. If unspecified, the serialized test
        /// cases is returned inline.
        #[prost(string, tag = "2")]
        GcsUri(::prost::alloc::string::String),
    }
}
/// The response message for [TestCases.ExportTestCases][google.cloud.dialogflow.cx.v3.TestCases.ExportTestCases].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportTestCasesResponse {
    /// The exported test cases.
    #[prost(oneof = "export_test_cases_response::Destination", tags = "1, 2")]
    pub destination: ::core::option::Option<export_test_cases_response::Destination>,
}
/// Nested message and enum types in `ExportTestCasesResponse`.
pub mod export_test_cases_response {
    /// The exported test cases.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// The URI to a file containing the exported test cases. This field is
        /// populated only if `gcs_uri` is specified in
        /// [ExportTestCasesRequest][google.cloud.dialogflow.cx.v3.ExportTestCasesRequest].
        #[prost(string, tag = "1")]
        GcsUri(::prost::alloc::string::String),
        /// Uncompressed raw byte content for test cases.
        #[prost(bytes, tag = "2")]
        Content(::prost::alloc::vec::Vec<u8>),
    }
}
/// Metadata returned for the [TestCases.ExportTestCases][google.cloud.dialogflow.cx.v3.TestCases.ExportTestCases] long running
/// operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportTestCasesMetadata {}
/// The request message for [TestCases.ListTestCaseResults][google.cloud.dialogflow.cx.v3.TestCases.ListTestCaseResults].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTestCaseResultsRequest {
    /// Required. The test case to list results for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/
    /// testCases/<TestCase ID>`. Specify a `-` as a wildcard for TestCase ID to
    /// list results across multiple test cases.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The filter expression used to filter test case results. See
    /// [API Filtering](https://aip.dev/160).
    ///
    /// The expression is case insensitive. Only 'AND' is supported for logical
    /// operators. The supported syntax is listed below in detail:
    ///
    ///   <field> <operator> <value> [AND <field> <operator> <value>] ...
    ///   [AND latest]
    ///
    /// The supported fields and operators are:
    /// field                 operator
    /// `environment`         `=`, `IN`  (Use value `draft` for draft environment)
    /// `test_time`           `>`, `<`
    ///
    /// `latest` only returns the latest test result in all results for each test
    /// case.
    ///
    /// Examples:
    /// *   "environment=draft AND latest" matches the latest test result for each
    ///     test case in the draft environment.
    /// *   "environment IN (e1,e2)" matches any test case results with an
    ///     environment resource name of either "e1" or "e2".
    /// *   "test_time > 1602540713" matches any test case results with test time
    ///     later than a unix timestamp in seconds 1602540713.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// The response message for [TestCases.ListTestCaseResults][google.cloud.dialogflow.cx.v3.TestCases.ListTestCaseResults].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTestCaseResultsResponse {
    /// The list of test case results.
    #[prost(message, repeated, tag = "1")]
    pub test_case_results: ::prost::alloc::vec::Vec<TestCaseResult>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for [TestCases.GetTestCaseResult][google.cloud.dialogflow.cx.v3.TestCases.GetTestCaseResult].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTestCaseResultRequest {
    /// Required. The name of the testcase.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/testCases/<TestCase ID>/results/<TestCaseResult ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The test result for a test case and an agent environment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TestResult {
    /// Not specified. Should never be used.
    Unspecified = 0,
    /// The test passed.
    Passed = 1,
    /// The test did not pass.
    Failed = 2,
}
#[doc = r" Generated client implementations."]
pub mod test_cases_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for managing [Test Cases][google.cloud.dialogflow.cx.v3.TestCase] and"]
    #[doc = " [Test Case Results][google.cloud.dialogflow.cx.v3.TestCaseResult]."]
    #[derive(Debug, Clone)]
    pub struct TestCasesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TestCasesClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
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
        ) -> TestCasesClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            TestCasesClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Fetches a list of test cases for a given agent."]
        pub async fn list_test_cases(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTestCasesRequest>,
        ) -> Result<tonic::Response<super::ListTestCasesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.TestCases/ListTestCases",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Batch deletes test cases."]
        pub async fn batch_delete_test_cases(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteTestCasesRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.TestCases/BatchDeleteTestCases",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a test case."]
        pub async fn get_test_case(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTestCaseRequest>,
        ) -> Result<tonic::Response<super::TestCase>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.TestCases/GetTestCase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a test case for the given agent."]
        pub async fn create_test_case(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTestCaseRequest>,
        ) -> Result<tonic::Response<super::TestCase>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.TestCases/CreateTestCase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified test case."]
        pub async fn update_test_case(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTestCaseRequest>,
        ) -> Result<tonic::Response<super::TestCase>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.TestCases/UpdateTestCase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Kicks off a test case run."]
        pub async fn run_test_case(
            &mut self,
            request: impl tonic::IntoRequest<super::RunTestCaseRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.cx.v3.TestCases/RunTestCase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Kicks off a batch run of test cases."]
        pub async fn batch_run_test_cases(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchRunTestCasesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.cx.v3.TestCases/BatchRunTestCases",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Calculates the test coverage for an agent."]
        pub async fn calculate_coverage(
            &mut self,
            request: impl tonic::IntoRequest<super::CalculateCoverageRequest>,
        ) -> Result<tonic::Response<super::CalculateCoverageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.TestCases/CalculateCoverage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Imports the test cases from a Cloud Storage bucket or a local file. It"]
        #[doc = " always creates new test cases and won't overwite any existing ones. The"]
        #[doc = " provided ID in the imported test case is neglected."]
        pub async fn import_test_cases(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportTestCasesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.cx.v3.TestCases/ImportTestCases",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Exports the test cases under the agent to a Cloud Storage bucket or a local"]
        #[doc = " file. Filter can be applied to export a subset of test cases."]
        pub async fn export_test_cases(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportTestCasesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.cx.v3.TestCases/ExportTestCases",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Fetches a list of results for a given test case."]
        pub async fn list_test_case_results(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTestCaseResultsRequest>,
        ) -> Result<tonic::Response<super::ListTestCaseResultsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.TestCases/ListTestCaseResults",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a test case result."]
        pub async fn get_test_case_result(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTestCaseResultRequest>,
        ) -> Result<tonic::Response<super::TestCaseResult>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.TestCases/GetTestCaseResult",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Represents an environment for an agent. You can create multiple versions
/// of your agent and publish them to separate environments. When you edit an
/// agent, you are editing the draft agent. At any point, you can save the draft
/// agent as an agent version, which is an immutable snapshot of your agent. When
/// you save the draft agent, it is published to the default environment. When
/// you create agent versions, you can publish them to custom environments. You
/// can create a variety of custom environments for testing, development,
/// production, etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    /// The name of the environment.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The human-readable name of the environment (unique in an agent). Limit of
    /// 64 characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The human-readable description of the environment. The maximum length is
    /// 500 characters. If exceeded, the request is rejected.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Required. A list of configurations for flow versions. You should include version
    /// configs for all flows that are reachable from [`Start
    /// Flow`][Agent.start_flow] in the agent. Otherwise, an error will be
    /// returned.
    #[prost(message, repeated, tag = "6")]
    pub version_configs: ::prost::alloc::vec::Vec<environment::VersionConfig>,
    /// Output only. Update time of this environment.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Environment`.
pub mod environment {
    /// Configuration for the version.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VersionConfig {
        /// Required. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/flows/<Flow ID>/versions/<Version ID>.
        #[prost(string, tag = "1")]
        pub version: ::prost::alloc::string::String,
    }
}
/// The request message for [Environments.ListEnvironments][google.cloud.dialogflow.cx.v3.Environments.ListEnvironments].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnvironmentsRequest {
    /// Required. The [Agent][google.cloud.dialogflow.cx.v3.Agent] to list all environments for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return in a single page. By default 20 and
    /// at most 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for [Environments.ListEnvironments][google.cloud.dialogflow.cx.v3.Environments.ListEnvironments].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnvironmentsResponse {
    /// The list of environments. There will be a maximum number of items
    /// returned based on the page_size field in the request. The list may in some
    /// cases be empty or contain fewer entries than page_size even if this isn't
    /// the last page.
    #[prost(message, repeated, tag = "1")]
    pub environments: ::prost::alloc::vec::Vec<Environment>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for [Environments.GetEnvironment][google.cloud.dialogflow.cx.v3.Environments.GetEnvironment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEnvironmentRequest {
    /// Required. The name of the [Environment][google.cloud.dialogflow.cx.v3.Environment].
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for [Environments.CreateEnvironment][google.cloud.dialogflow.cx.v3.Environments.CreateEnvironment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEnvironmentRequest {
    /// Required. The [Agent][google.cloud.dialogflow.cx.v3.Agent] to create an [Environment][google.cloud.dialogflow.cx.v3.Environment] for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The environment to create.
    #[prost(message, optional, tag = "2")]
    pub environment: ::core::option::Option<Environment>,
}
/// The request message for [Environments.UpdateEnvironment][google.cloud.dialogflow.cx.v3.Environments.UpdateEnvironment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEnvironmentRequest {
    /// Required. The environment to update.
    #[prost(message, optional, tag = "1")]
    pub environment: ::core::option::Option<Environment>,
    /// Required. The mask to control which fields get updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for [Environments.DeleteEnvironment][google.cloud.dialogflow.cx.v3.Environments.DeleteEnvironment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEnvironmentRequest {
    /// Required. The name of the [Environment][google.cloud.dialogflow.cx.v3.Environment] to delete.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for [Environments.LookupEnvironmentHistory][google.cloud.dialogflow.cx.v3.Environments.LookupEnvironmentHistory].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupEnvironmentHistoryRequest {
    /// Required. Resource name of the environment to look up the history for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for [Environments.LookupEnvironmentHistory][google.cloud.dialogflow.cx.v3.Environments.LookupEnvironmentHistory].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupEnvironmentHistoryResponse {
    /// Represents a list of snapshots for an environment. Time of the snapshots is
    /// stored in [`update_time`][google.cloud.dialogflow.cx.v3.Environment.update_time].
    #[prost(message, repeated, tag = "1")]
    pub environments: ::prost::alloc::vec::Vec<Environment>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Represents a result from running a test case in an agent environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContinuousTestResult {
    /// The resource name for the continuous test result. Format:
    /// `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment
    /// ID>/continuousTestResults/<ContinuousTestResult ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The result of this continuous test run, i.e. whether all the tests in this
    /// continuous test run pass or not.
    #[prost(
        enumeration = "continuous_test_result::AggregatedTestResult",
        tag = "2"
    )]
    pub result: i32,
    /// A list of individual test case results names in this continuous test run.
    #[prost(string, repeated, tag = "3")]
    pub test_case_results: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Time when the continuous testing run starts.
    #[prost(message, optional, tag = "4")]
    pub run_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `ContinuousTestResult`.
pub mod continuous_test_result {
    /// The overall result for a continuous test run in an agent environment.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AggregatedTestResult {
        /// Not specified. Should never be used.
        Unspecified = 0,
        /// All the tests passed.
        Passed = 1,
        /// At least one test did not pass.
        Failed = 2,
    }
}
/// The request message for [Environments.RunContinuousTest][google.cloud.dialogflow.cx.v3.Environments.RunContinuousTest].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunContinuousTestRequest {
    /// Required. Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>`.
    #[prost(string, tag = "1")]
    pub environment: ::prost::alloc::string::String,
}
/// The response message for [Environments.RunContinuousTest][google.cloud.dialogflow.cx.v3.Environments.RunContinuousTest].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunContinuousTestResponse {
    /// The result for a continuous test run.
    #[prost(message, optional, tag = "1")]
    pub continuous_test_result: ::core::option::Option<ContinuousTestResult>,
}
/// Metadata returned for the [Environments.RunContinuousTest][google.cloud.dialogflow.cx.v3.Environments.RunContinuousTest] long running
/// operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunContinuousTestMetadata {
    /// The test errors.
    #[prost(message, repeated, tag = "1")]
    pub errors: ::prost::alloc::vec::Vec<TestError>,
}
/// The request message for [Environments.ListContinuousTestResults][google.cloud.dialogflow.cx.v3.Environments.ListContinuousTestResults].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContinuousTestResultsRequest {
    /// Required. The environment to list results for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/
    /// environments/<Environment ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for [Environments.ListTestCaseResults][].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContinuousTestResultsResponse {
    /// The list of continuous test results.
    #[prost(message, repeated, tag = "1")]
    pub continuous_test_results: ::prost::alloc::vec::Vec<ContinuousTestResult>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod environments_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for managing [Environments][google.cloud.dialogflow.cx.v3.Environment]."]
    #[derive(Debug, Clone)]
    pub struct EnvironmentsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> EnvironmentsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
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
        ) -> EnvironmentsClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            EnvironmentsClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns the list of all environments in the specified [Agent][google.cloud.dialogflow.cx.v3.Agent]."]
        pub async fn list_environments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEnvironmentsRequest>,
        ) -> Result<tonic::Response<super::ListEnvironmentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Environments/ListEnvironments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified [Environment][google.cloud.dialogflow.cx.v3.Environment]."]
        pub async fn get_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEnvironmentRequest>,
        ) -> Result<tonic::Response<super::Environment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Environments/GetEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an [Environment][google.cloud.dialogflow.cx.v3.Environment] in the specified [Agent][google.cloud.dialogflow.cx.v3.Agent]."]
        pub async fn create_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEnvironmentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.cx.v3.Environments/CreateEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified [Environment][google.cloud.dialogflow.cx.v3.Environment]."]
        pub async fn update_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEnvironmentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.cx.v3.Environments/UpdateEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified [Environment][google.cloud.dialogflow.cx.v3.Environment]."]
        pub async fn delete_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEnvironmentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Environments/DeleteEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Looks up the history of the specified [Environment][google.cloud.dialogflow.cx.v3.Environment]."]
        pub async fn lookup_environment_history(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupEnvironmentHistoryRequest>,
        ) -> Result<tonic::Response<super::LookupEnvironmentHistoryResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Environments/LookupEnvironmentHistory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Kicks off a continuous test under the specified [Environment][google.cloud.dialogflow.cx.v3.Environment]."]
        pub async fn run_continuous_test(
            &mut self,
            request: impl tonic::IntoRequest<super::RunContinuousTestRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.cx.v3.Environments/RunContinuousTest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Fetches a list of continuous test results for a given environment."]
        pub async fn list_continuous_test_results(
            &mut self,
            request: impl tonic::IntoRequest<super::ListContinuousTestResultsRequest>,
        ) -> Result<tonic::Response<super::ListContinuousTestResultsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Environments/ListContinuousTestResults",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Represents an experiment in an environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Experiment {
    /// The name of the experiment.
    /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>/experiments/<Experiment ID>..
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The human-readable name of the experiment (unique in an environment). Limit
    /// of 64 characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The human-readable description of the experiment.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The current state of the experiment.
    /// Transition triggered by Expriments.StartExperiment: PENDING->RUNNING.
    /// Transition triggered by Expriments.CancelExperiment: PENDING->CANCELLED or
    /// RUNNING->CANCELLED.
    #[prost(enumeration = "experiment::State", tag = "4")]
    pub state: i32,
    /// The definition of the experiment.
    #[prost(message, optional, tag = "5")]
    pub definition: ::core::option::Option<experiment::Definition>,
    /// Inference result of the experiment.
    #[prost(message, optional, tag = "6")]
    pub result: ::core::option::Option<experiment::Result>,
    /// Creation time of this experiment.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Start time of this experiment.
    #[prost(message, optional, tag = "8")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// End time of this experiment.
    #[prost(message, optional, tag = "9")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Last update time of this experiment.
    #[prost(message, optional, tag = "10")]
    pub last_update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Maximum number of days to run the experiment/rollout. If auto-rollout is
    /// not enabled, default value and maximum will be 30 days. If auto-rollout is
    /// enabled, default value and maximum will be 6 days.
    #[prost(message, optional, tag = "11")]
    pub experiment_length: ::core::option::Option<::prost_types::Duration>,
    /// The history of updates to the experiment variants.
    #[prost(message, repeated, tag = "12")]
    pub variants_history: ::prost::alloc::vec::Vec<VariantsHistory>,
}
/// Nested message and enum types in `Experiment`.
pub mod experiment {
    /// Definition of the experiment.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Definition {
        /// The condition defines which subset of sessions are selected for
        /// this experiment. If not specified, all sessions are eligible. E.g.
        /// "query_input.language_code=en" See the [conditions
        /// reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition).
        #[prost(string, tag = "1")]
        pub condition: ::prost::alloc::string::String,
        /// The variants of the experiment. We currently only support single variant
        /// experiment.
        #[prost(oneof = "definition::Variants", tags = "2")]
        pub variants: ::core::option::Option<definition::Variants>,
    }
    /// Nested message and enum types in `Definition`.
    pub mod definition {
        /// The variants of the experiment. We currently only support single variant
        /// experiment.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Variants {
            /// The flow versions as the variants of this experiment.
            #[prost(message, tag = "2")]
            VersionVariants(super::super::VersionVariants),
        }
    }
    /// The inference result which includes an objective metric to optimize and the
    /// confidence interval.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        /// Version variants and metrics.
        #[prost(message, repeated, tag = "1")]
        pub version_metrics: ::prost::alloc::vec::Vec<result::VersionMetrics>,
        /// The last time the experiment's stats data was updated. Will have default
        /// value if stats have never been computed for this experiment.
        #[prost(message, optional, tag = "2")]
        pub last_update_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// Nested message and enum types in `Result`.
    pub mod result {
        /// A confidence interval is a range of possible values for the experiment
        /// objective you are trying to measure.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ConfidenceInterval {
            /// The confidence level used to construct the interval, i.e. there is X%
            /// chance that the true value is within this interval.
            #[prost(double, tag = "1")]
            pub confidence_level: f64,
            /// The percent change between an experiment metric's value and the value
            /// for its control.
            #[prost(double, tag = "2")]
            pub ratio: f64,
            /// Lower bound of the interval.
            #[prost(double, tag = "3")]
            pub lower_bound: f64,
            /// Upper bound of the interval.
            #[prost(double, tag = "4")]
            pub upper_bound: f64,
        }
        /// Metric and corresponding confidence intervals.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Metric {
            /// Ratio-based metric type. Only one of type or count_type is specified in
            /// each Metric.
            #[prost(enumeration = "MetricType", tag = "1")]
            pub r#type: i32,
            /// Count-based metric type. Only one of type or count_type is specified in
            /// each Metric.
            #[prost(enumeration = "CountType", tag = "5")]
            pub count_type: i32,
            /// The probability that the treatment is better than all other treatments
            /// in the experiment
            #[prost(message, optional, tag = "3")]
            pub confidence_interval: ::core::option::Option<ConfidenceInterval>,
            /// The actual value of the metric.
            #[prost(oneof = "metric::Value", tags = "2, 4")]
            pub value: ::core::option::Option<metric::Value>,
        }
        /// Nested message and enum types in `Metric`.
        pub mod metric {
            /// The actual value of the metric.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Value {
                /// Ratio value of a metric.
                #[prost(double, tag = "2")]
                Ratio(f64),
                /// Count value of a metric.
                #[prost(double, tag = "4")]
                Count(f64),
            }
        }
        /// Version variant and associated metrics.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct VersionMetrics {
            /// The name of the flow [Version][google.cloud.dialogflow.cx.v3.Version].
            /// Format: `projects/<Project ID>/locations/<Location
            /// ID>/agents/<Agent ID>/flows/<Flow ID>/versions/<Version ID>`.
            #[prost(string, tag = "1")]
            pub version: ::prost::alloc::string::String,
            /// The metrics and corresponding confidence intervals in the inference
            /// result.
            #[prost(message, repeated, tag = "2")]
            pub metrics: ::prost::alloc::vec::Vec<Metric>,
            /// Number of sessions that were allocated to this version.
            #[prost(int32, tag = "3")]
            pub session_count: i32,
        }
        /// Types of ratio-based metric for Dialogflow experiment.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum MetricType {
            /// Metric unspecified.
            MetricUnspecified = 0,
            /// Percentage of contained sessions without user calling back in 24 hours.
            ContainedSessionNoCallbackRate = 1,
            /// Percentage of sessions that were handed to a human agent.
            LiveAgentHandoffRate = 2,
            /// Percentage of sessions with the same user calling back.
            CallbackSessionRate = 3,
            /// Percentage of sessions where user hung up.
            AbandonedSessionRate = 4,
            /// Percentage of sessions reached Dialogflow 'END_PAGE' or
            /// 'END_SESSION'.
            SessionEndRate = 5,
        }
        /// Types of count-based metric for Dialogflow experiment.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum CountType {
            /// Count type unspecified.
            Unspecified = 0,
            /// Total number of occurrences of a 'NO_MATCH'.
            TotalNoMatchCount = 1,
            /// Total number of turn counts.
            TotalTurnCount = 2,
            /// Average turn count in a session.
            AverageTurnCount = 3,
        }
    }
    /// The state of the experiment.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// State unspecified.
        Unspecified = 0,
        /// The experiment is created but not started yet.
        Draft = 1,
        /// The experiment is running.
        Running = 2,
        /// The experiment is done.
        Done = 3,
    }
}
/// A list of flow version variants.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionVariants {
    /// A list of flow version variants.
    #[prost(message, repeated, tag = "1")]
    pub variants: ::prost::alloc::vec::Vec<version_variants::Variant>,
}
/// Nested message and enum types in `VersionVariants`.
pub mod version_variants {
    /// A single flow version with specified traffic allocation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Variant {
        /// The name of the flow version.
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/flows/<Flow ID>/versions/<Version ID>`.
        #[prost(string, tag = "1")]
        pub version: ::prost::alloc::string::String,
        /// Percentage of the traffic which should be routed to this
        /// version of flow. Traffic allocation for a single flow must sum up to 1.0.
        #[prost(float, tag = "2")]
        pub traffic_allocation: f32,
        /// Whether the variant is for the control group.
        #[prost(bool, tag = "3")]
        pub is_control_group: bool,
    }
}
/// The history of variants update.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VariantsHistory {
    /// Update time of the variants.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The variants updated. We currently only support single variant
    /// experiment.
    #[prost(oneof = "variants_history::Variants", tags = "1")]
    pub variants: ::core::option::Option<variants_history::Variants>,
}
/// Nested message and enum types in `VariantsHistory`.
pub mod variants_history {
    /// The variants updated. We currently only support single variant
    /// experiment.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Variants {
        /// The flow versions as the variants.
        #[prost(message, tag = "1")]
        VersionVariants(super::VersionVariants),
    }
}
/// The request message for [Experiments.ListExperiments][google.cloud.dialogflow.cx.v3.Experiments.ListExperiments].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExperimentsRequest {
    /// Required. The [Environment][google.cloud.dialogflow.cx.v3.Environment] to list all environments for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return in a single page. By default 20 and
    /// at most 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for [Experiments.ListExperiments][google.cloud.dialogflow.cx.v3.Experiments.ListExperiments].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExperimentsResponse {
    /// The list of experiments. There will be a maximum number of items
    /// returned based on the page_size field in the request. The list may in some
    /// cases be empty or contain fewer entries than page_size even if this isn't
    /// the last page.
    #[prost(message, repeated, tag = "1")]
    pub experiments: ::prost::alloc::vec::Vec<Experiment>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for [Experiments.GetExperiment][google.cloud.dialogflow.cx.v3.Experiments.GetExperiment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExperimentRequest {
    /// Required. The name of the [Environment][google.cloud.dialogflow.cx.v3.Environment].
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>/experiments/<Experiment ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for [Experiments.CreateExperiment][google.cloud.dialogflow.cx.v3.Experiments.CreateExperiment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateExperimentRequest {
    /// Required. The [Agent][google.cloud.dialogflow.cx.v3.Agent] to create an [Environment][google.cloud.dialogflow.cx.v3.Environment] for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The experiment to create.
    #[prost(message, optional, tag = "2")]
    pub experiment: ::core::option::Option<Experiment>,
}
/// The request message for [Experiments.UpdateExperiment][google.cloud.dialogflow.cx.v3.Experiments.UpdateExperiment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateExperimentRequest {
    /// Required. The experiment to update.
    #[prost(message, optional, tag = "1")]
    pub experiment: ::core::option::Option<Experiment>,
    /// Required. The mask to control which fields get updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for [Experiments.DeleteExperiment][google.cloud.dialogflow.cx.v3.Experiments.DeleteExperiment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteExperimentRequest {
    /// Required. The name of the [Environment][google.cloud.dialogflow.cx.v3.Environment] to delete.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>/experiments/<Experiment ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for [Experiments.StartExperiment][google.cloud.dialogflow.cx.v3.Experiments.StartExperiment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartExperimentRequest {
    /// Required. Resource name of the experiment to start.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>/experiments/<Experiment ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for [Experiments.StopExperiment][google.cloud.dialogflow.cx.v3.Experiments.StopExperiment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopExperimentRequest {
    /// Required. Resource name of the experiment to stop.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>/experiments/<Experiment ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod experiments_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for managing [Experiments][google.cloud.dialogflow.cx.v3.Experiment]."]
    #[derive(Debug, Clone)]
    pub struct ExperimentsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ExperimentsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
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
        ) -> ExperimentsClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ExperimentsClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns the list of all experiments in the specified [Environment][google.cloud.dialogflow.cx.v3.Environment]."]
        pub async fn list_experiments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExperimentsRequest>,
        ) -> Result<tonic::Response<super::ListExperimentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Experiments/ListExperiments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified [Experiment][google.cloud.dialogflow.cx.v3.Experiment]."]
        pub async fn get_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExperimentRequest>,
        ) -> Result<tonic::Response<super::Experiment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Experiments/GetExperiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an [Experiment][google.cloud.dialogflow.cx.v3.Experiment] in the specified [Environment][google.cloud.dialogflow.cx.v3.Environment]."]
        pub async fn create_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateExperimentRequest>,
        ) -> Result<tonic::Response<super::Experiment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Experiments/CreateExperiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified [Experiment][google.cloud.dialogflow.cx.v3.Experiment]."]
        pub async fn update_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateExperimentRequest>,
        ) -> Result<tonic::Response<super::Experiment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Experiments/UpdateExperiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified [Experiment][google.cloud.dialogflow.cx.v3.Experiment]."]
        pub async fn delete_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteExperimentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Experiments/DeleteExperiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Starts the specified [Experiment][google.cloud.dialogflow.cx.v3.Experiment]. This rpc only changes the state of"]
        #[doc = " experiment from PENDING to RUNNING."]
        pub async fn start_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::StartExperimentRequest>,
        ) -> Result<tonic::Response<super::Experiment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Experiments/StartExperiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Stops the specified [Experiment][google.cloud.dialogflow.cx.v3.Experiment]. This rpc only changes the state of"]
        #[doc = " experiment from RUNNING to DONE."]
        pub async fn stop_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::StopExperimentRequest>,
        ) -> Result<tonic::Response<super::Experiment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Experiments/StopExperiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Metadata associated with the long running operation for
/// [Versions.CreateVersion][google.cloud.dialogflow.cx.v3.Versions.CreateVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVersionOperationMetadata {
    /// Name of the created version.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/versions/<Version ID>`.
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
}
/// Represents a version of a flow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/versions/<Version ID>. Version ID is a self-increasing
    /// number generated by Dialogflow upon version creation.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The human-readable name of the version. Limit of 64 characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The description of the version. The maximum length is 500 characters. If
    /// exceeded, the request is rejected.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The NLU settings of the flow at version creation.
    #[prost(message, optional, tag = "4")]
    pub nlu_settings: ::core::option::Option<NluSettings>,
    /// Output only. Create time of the version.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The state of this version. This field is read-only and cannot be set by
    /// create and update methods.
    #[prost(enumeration = "version::State", tag = "6")]
    pub state: i32,
}
/// Nested message and enum types in `Version`.
pub mod version {
    /// The state of the version.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not specified. This value is not used.
        Unspecified = 0,
        /// Version is not ready to serve (e.g. training is running).
        Running = 1,
        /// Training has succeeded and this version is ready to serve.
        Succeeded = 2,
        /// Version training failed.
        Failed = 3,
    }
}
/// The request message for [Versions.ListVersions][google.cloud.dialogflow.cx.v3.Versions.ListVersions].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVersionsRequest {
    /// Required. The [Flow][google.cloud.dialogflow.cx.v3.Flow] to list all versions for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return in a single page. By default 20 and
    /// at most 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for [Versions.ListVersions][google.cloud.dialogflow.cx.v3.Versions.ListVersions].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVersionsResponse {
    /// A list of versions. There will be a maximum number of items returned based
    /// on the page_size field in the request. The list may in some cases be empty
    /// or contain fewer entries than page_size even if this isn't the last page.
    #[prost(message, repeated, tag = "1")]
    pub versions: ::prost::alloc::vec::Vec<Version>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for [Versions.GetVersion][google.cloud.dialogflow.cx.v3.Versions.GetVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionRequest {
    /// Required. The name of the [Version][google.cloud.dialogflow.cx.v3.Version].
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/versions/<Version ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for [Versions.CreateVersion][google.cloud.dialogflow.cx.v3.Versions.CreateVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVersionRequest {
    /// Required. The [Flow][google.cloud.dialogflow.cx.v3.Flow] to create an [Version][google.cloud.dialogflow.cx.v3.Version] for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The version to create.
    #[prost(message, optional, tag = "2")]
    pub version: ::core::option::Option<Version>,
}
/// The request message for [Versions.UpdateVersion][google.cloud.dialogflow.cx.v3.Versions.UpdateVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVersionRequest {
    /// Required. The version to update.
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<Version>,
    /// Required. The mask to control which fields get updated. Currently only `description`
    /// and `display_name` can be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for [Versions.DeleteVersion][google.cloud.dialogflow.cx.v3.Versions.DeleteVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVersionRequest {
    /// Required. The name of the [Version][google.cloud.dialogflow.cx.v3.Version] to delete.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/versions/<Version ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for [Versions.LoadVersion][google.cloud.dialogflow.cx.v3.Versions.LoadVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadVersionRequest {
    /// Required. The [Version][google.cloud.dialogflow.cx.v3.Version] to be loaded to draft flow.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/versions/<Version ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// This field is used to prevent accidental overwrite of other agent
    /// resources, which can potentially impact other flow's behavior. If
    /// `allow_override_agent_resources` is false, conflicted agent-level resources
    /// will not be overridden (i.e. intents, entities, webhooks).
    #[prost(bool, tag = "2")]
    pub allow_override_agent_resources: bool,
}
#[doc = r" Generated client implementations."]
pub mod versions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for managing [Versions][google.cloud.dialogflow.cx.v3.Version]."]
    #[derive(Debug, Clone)]
    pub struct VersionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VersionsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
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
        ) -> VersionsClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            VersionsClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns the list of all versions in the specified [Flow][google.cloud.dialogflow.cx.v3.Flow]."]
        pub async fn list_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVersionsRequest>,
        ) -> Result<tonic::Response<super::ListVersionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Versions/ListVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified [Version][google.cloud.dialogflow.cx.v3.Version]."]
        pub async fn get_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVersionRequest>,
        ) -> Result<tonic::Response<super::Version>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Versions/GetVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a [Version][google.cloud.dialogflow.cx.v3.Version] in the specified [Flow][google.cloud.dialogflow.cx.v3.Flow]."]
        pub async fn create_version(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVersionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.cx.v3.Versions/CreateVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified [Version][google.cloud.dialogflow.cx.v3.Version]."]
        pub async fn update_version(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVersionRequest>,
        ) -> Result<tonic::Response<super::Version>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Versions/UpdateVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified [Version][google.cloud.dialogflow.cx.v3.Version]."]
        pub async fn delete_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteVersionRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Versions/DeleteVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Loads resources in the specified version to the draft flow."]
        pub async fn load_version(
            &mut self,
            request: impl tonic::IntoRequest<super::LoadVersionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.cx.v3.Versions/LoadVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Webhooks host the developer's business logic. During a session, webhooks
/// allow the developer to use the data extracted by Dialogflow's natural
/// language processing to generate dynamic responses, validate collected data,
/// or trigger actions on the backend.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Webhook {
    /// The unique identifier of the webhook.
    /// Required for the [Webhooks.UpdateWebhook][google.cloud.dialogflow.cx.v3.Webhooks.UpdateWebhook] method.
    /// [Webhooks.CreateWebhook][google.cloud.dialogflow.cx.v3.Webhooks.CreateWebhook] populates the name automatically.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/webhooks/<Webhook ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The human-readable name of the webhook, unique within the agent.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Webhook execution timeout. Execution is considered failed if Dialogflow
    /// doesn't receive a response from webhook at the end of the timeout period.
    /// Defaults to 5 seconds, maximum allowed timeout is 30 seconds.
    #[prost(message, optional, tag = "6")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// Indicates whether the webhook is disabled.
    #[prost(bool, tag = "5")]
    pub disabled: bool,
    /// Required. The webhook configuration.
    #[prost(oneof = "webhook::Webhook", tags = "4, 7")]
    pub webhook: ::core::option::Option<webhook::Webhook>,
}
/// Nested message and enum types in `Webhook`.
pub mod webhook {
    /// Represents configuration for a generic web service.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GenericWebService {
        /// Required. The webhook URI for receiving POST requests. It must use https protocol.
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
        /// The user name for HTTP Basic authentication.
        #[deprecated]
        #[prost(string, tag = "2")]
        pub username: ::prost::alloc::string::String,
        /// The password for HTTP Basic authentication.
        #[deprecated]
        #[prost(string, tag = "3")]
        pub password: ::prost::alloc::string::String,
        /// The HTTP request headers to send together with webhook
        /// requests.
        #[prost(map = "string, string", tag = "4")]
        pub request_headers: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
    /// Represents configuration for a [Service
    /// Directory](https://cloud.google.com/service-directory) service.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServiceDirectoryConfig {
        /// Required. The name of [Service
        /// Directory](https://cloud.google.com/service-directory) service.
        /// Format: `projects/<Project ID>/locations/<Location
        /// ID>/namespaces/<Namespace ID>/services/<Service ID>`.
        /// `Location ID` of the service directory must be the same as the location
        /// of the agent.
        #[prost(string, tag = "1")]
        pub service: ::prost::alloc::string::String,
        /// Generic Service configuration of this webhook.
        #[prost(message, optional, tag = "2")]
        pub generic_web_service: ::core::option::Option<GenericWebService>,
    }
    /// Required. The webhook configuration.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Webhook {
        /// Configuration for a generic web service.
        #[prost(message, tag = "4")]
        GenericWebService(GenericWebService),
        /// Configuration for a [Service
        /// Directory](https://cloud.google.com/service-directory) service.
        #[prost(message, tag = "7")]
        ServiceDirectory(ServiceDirectoryConfig),
    }
}
/// The request message for [Webhooks.ListWebhooks][google.cloud.dialogflow.cx.v3.Webhooks.ListWebhooks].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWebhooksRequest {
    /// Required. The agent to list all webhooks for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for [Webhooks.ListWebhooks][google.cloud.dialogflow.cx.v3.Webhooks.ListWebhooks].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWebhooksResponse {
    /// The list of webhooks. There will be a maximum number of items returned
    /// based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub webhooks: ::prost::alloc::vec::Vec<Webhook>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for [Webhooks.GetWebhook][google.cloud.dialogflow.cx.v3.Webhooks.GetWebhook].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWebhookRequest {
    /// Required. The name of the webhook.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/webhooks/<Webhook ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for [Webhooks.CreateWebhook][google.cloud.dialogflow.cx.v3.Webhooks.CreateWebhook].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWebhookRequest {
    /// Required. The agent to create a webhook for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The webhook to create.
    #[prost(message, optional, tag = "2")]
    pub webhook: ::core::option::Option<Webhook>,
}
/// The request message for [Webhooks.UpdateWebhook][google.cloud.dialogflow.cx.v3.Webhooks.UpdateWebhook].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWebhookRequest {
    /// Required. The webhook to update.
    #[prost(message, optional, tag = "1")]
    pub webhook: ::core::option::Option<Webhook>,
    /// The mask to control which fields get updated. If the mask is not present,
    /// all fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for [Webhooks.DeleteWebhook][google.cloud.dialogflow.cx.v3.Webhooks.DeleteWebhook].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWebhookRequest {
    /// Required. The name of the webhook to delete.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/webhooks/<Webhook ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// This field has no effect for webhook not being used.
    /// For webhooks that are used by pages/flows/transition route groups:
    ///
    /// *  If `force` is set to false, an error will be returned with message
    ///    indicating the referenced resources.
    /// *  If `force` is set to true, Dialogflow will remove the webhook, as well
    ///    as any references to the webhook (i.e. [Webhook][google.cloud.dialogflow.cx.v3.Fulfillment.webhook]
    ///    and [tag][google.cloud.dialogflow.cx.v3.Fulfillment.tag]in fulfillments that point to this webhook
    ///    will be removed).
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// The request message for a webhook call. The request is sent as a JSON object
/// and the field names will be presented in camel cases.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebhookRequest {
    /// Always present. The unique identifier of the [DetectIntentResponse][google.cloud.dialogflow.cx.v3.DetectIntentResponse] that
    /// will be returned to the API caller.
    #[prost(string, tag = "1")]
    pub detect_intent_response_id: ::prost::alloc::string::String,
    /// The language code specified in the [original
    /// request][QueryInput.language_code].
    #[prost(string, tag = "15")]
    pub language_code: ::prost::alloc::string::String,
    /// Always present. Information about the fulfillment that triggered this
    /// webhook call.
    #[prost(message, optional, tag = "6")]
    pub fulfillment_info: ::core::option::Option<webhook_request::FulfillmentInfo>,
    /// Information about the last matched intent.
    #[prost(message, optional, tag = "3")]
    pub intent_info: ::core::option::Option<webhook_request::IntentInfo>,
    /// Information about page status.
    #[prost(message, optional, tag = "4")]
    pub page_info: ::core::option::Option<PageInfo>,
    /// Information about session status.
    #[prost(message, optional, tag = "5")]
    pub session_info: ::core::option::Option<SessionInfo>,
    /// The list of rich message responses to present to the user. Webhook can
    /// choose to append or replace this list in
    /// [WebhookResponse.fulfillment_response][google.cloud.dialogflow.cx.v3.WebhookResponse.fulfillment_response];
    #[prost(message, repeated, tag = "7")]
    pub messages: ::prost::alloc::vec::Vec<ResponseMessage>,
    /// Custom data set in [QueryParameters.payload][google.cloud.dialogflow.cx.v3.QueryParameters.payload].
    #[prost(message, optional, tag = "8")]
    pub payload: ::core::option::Option<::prost_types::Struct>,
    /// The sentiment analysis result of the current user request. The field is
    /// filled when sentiment analysis is configured to be enabled for the request.
    #[prost(message, optional, tag = "9")]
    pub sentiment_analysis_result: ::core::option::Option<webhook_request::SentimentAnalysisResult>,
    /// The original conversational query.
    #[prost(oneof = "webhook_request::Query", tags = "10, 11, 12, 14")]
    pub query: ::core::option::Option<webhook_request::Query>,
}
/// Nested message and enum types in `WebhookRequest`.
pub mod webhook_request {
    /// Represents fulfillment information communicated to the webhook.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FulfillmentInfo {
        /// Always present. The tag used to identify which fulfillment is being
        /// called.
        #[prost(string, tag = "1")]
        pub tag: ::prost::alloc::string::String,
    }
    /// Represents intent information communicated to the webhook.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IntentInfo {
        /// Always present. The unique identifier of the last matched
        /// [intent][google.cloud.dialogflow.cx.v3.Intent].
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/intents/<Intent ID>`.
        #[prost(string, tag = "1")]
        pub last_matched_intent: ::prost::alloc::string::String,
        /// Always present. The display name of the last matched [intent][google.cloud.dialogflow.cx.v3.Intent].
        #[prost(string, tag = "3")]
        pub display_name: ::prost::alloc::string::String,
        /// Parameters identified as a result of intent matching. This is a map of
        /// the name of the identified parameter to the value of the parameter
        /// identified from the user's utterance. All parameters defined in the
        /// matched intent that are identified will be surfaced here.
        #[prost(map = "string, message", tag = "2")]
        pub parameters: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            intent_info::IntentParameterValue,
        >,
        /// The confidence of the matched intent. Values range from 0.0 (completely
        /// uncertain) to 1.0 (completely certain).
        #[prost(float, tag = "4")]
        pub confidence: f32,
    }
    /// Nested message and enum types in `IntentInfo`.
    pub mod intent_info {
        /// Represents a value for an intent parameter.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct IntentParameterValue {
            /// Always present. Original text value extracted from user utterance.
            #[prost(string, tag = "1")]
            pub original_value: ::prost::alloc::string::String,
            /// Always present. Structured value for the parameter extracted from user
            /// utterance.
            #[prost(message, optional, tag = "2")]
            pub resolved_value: ::core::option::Option<::prost_types::Value>,
        }
    }
    /// Represents the result of sentiment analysis.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SentimentAnalysisResult {
        /// Sentiment score between -1.0 (negative sentiment) and 1.0 (positive
        /// sentiment).
        #[prost(float, tag = "1")]
        pub score: f32,
        /// A non-negative number in the [0, +inf) range, which represents the
        /// absolute magnitude of sentiment, regardless of score (positive or
        /// negative).
        #[prost(float, tag = "2")]
        pub magnitude: f32,
    }
    /// The original conversational query.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        /// If [natural language text][google.cloud.dialogflow.cx.v3.TextInput] was provided as input, this field
        /// will contain a copy of the text.
        #[prost(string, tag = "10")]
        Text(::prost::alloc::string::String),
        /// If an [intent][google.cloud.dialogflow.cx.v3.IntentInput] was provided as input, this field will
        /// contain a copy of the intent identifier.
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/intents/<Intent ID>`.
        #[prost(string, tag = "11")]
        TriggerIntent(::prost::alloc::string::String),
        /// If [natural language speech audio][google.cloud.dialogflow.cx.v3.AudioInput] was provided as input,
        /// this field will contain the transcript for the audio.
        #[prost(string, tag = "12")]
        Transcript(::prost::alloc::string::String),
        /// If an [event][google.cloud.dialogflow.cx.v3.EventInput] was provided as input, this field will contain
        /// the name of the event.
        #[prost(string, tag = "14")]
        TriggerEvent(::prost::alloc::string::String),
    }
}
/// The response message for a webhook call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebhookResponse {
    /// The fulfillment response to send to the user. This field can be omitted by
    /// the webhook if it does not intend to send any response to the user.
    #[prost(message, optional, tag = "1")]
    pub fulfillment_response: ::core::option::Option<webhook_response::FulfillmentResponse>,
    /// Information about page status. This field can be omitted by the webhook if
    /// it does not intend to modify page status.
    #[prost(message, optional, tag = "2")]
    pub page_info: ::core::option::Option<PageInfo>,
    /// Information about session status. This field can be omitted by the webhook
    /// if it does not intend to modify session status.
    #[prost(message, optional, tag = "3")]
    pub session_info: ::core::option::Option<SessionInfo>,
    /// Value to append directly to [QueryResult.webhook_payloads][google.cloud.dialogflow.cx.v3.QueryResult.webhook_payloads].
    #[prost(message, optional, tag = "4")]
    pub payload: ::core::option::Option<::prost_types::Struct>,
    /// The target to transition to. This can be set optionally to indicate an
    /// immediate transition to a different page in the same host flow, or a
    /// different flow in the same agent.
    #[prost(oneof = "webhook_response::Transition", tags = "5, 6")]
    pub transition: ::core::option::Option<webhook_response::Transition>,
}
/// Nested message and enum types in `WebhookResponse`.
pub mod webhook_response {
    /// Represents a fulfillment response to the user.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FulfillmentResponse {
        /// The list of rich message responses to present to the user.
        #[prost(message, repeated, tag = "1")]
        pub messages: ::prost::alloc::vec::Vec<super::ResponseMessage>,
        /// Merge behavior for `messages`.
        #[prost(enumeration = "fulfillment_response::MergeBehavior", tag = "2")]
        pub merge_behavior: i32,
    }
    /// Nested message and enum types in `FulfillmentResponse`.
    pub mod fulfillment_response {
        /// Defines merge behavior for `messages`.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum MergeBehavior {
            /// Not specified. `APPEND` will be used.
            Unspecified = 0,
            /// `messages` will be appended to the list of messages waiting to be sent
            /// to the user.
            Append = 1,
            /// `messages` will replace the list of messages waiting to be sent to the
            /// user.
            Replace = 2,
        }
    }
    /// The target to transition to. This can be set optionally to indicate an
    /// immediate transition to a different page in the same host flow, or a
    /// different flow in the same agent.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Transition {
        /// The target page to transition to.
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/flows/<Flow ID>/pages/<Page ID>`.
        #[prost(string, tag = "5")]
        TargetPage(::prost::alloc::string::String),
        /// The target flow to transition to.
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/flows/<Flow ID>`.
        #[prost(string, tag = "6")]
        TargetFlow(::prost::alloc::string::String),
    }
}
/// Represents page information communicated to and from the webhook.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageInfo {
    /// Always present for [WebhookRequest][google.cloud.dialogflow.cx.v3.WebhookRequest]. Ignored for [WebhookResponse][google.cloud.dialogflow.cx.v3.WebhookResponse].
    /// The unique identifier of the current page.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/pages/<Page ID>`.
    #[prost(string, tag = "1")]
    pub current_page: ::prost::alloc::string::String,
    /// Optional for both [WebhookRequest][google.cloud.dialogflow.cx.v3.WebhookRequest] and [WebhookResponse][google.cloud.dialogflow.cx.v3.WebhookResponse].
    /// Information about the form.
    #[prost(message, optional, tag = "3")]
    pub form_info: ::core::option::Option<page_info::FormInfo>,
}
/// Nested message and enum types in `PageInfo`.
pub mod page_info {
    /// Represents form information.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FormInfo {
        /// Optional for both [WebhookRequest][google.cloud.dialogflow.cx.v3.WebhookRequest] and [WebhookResponse][google.cloud.dialogflow.cx.v3.WebhookResponse].
        /// The parameters contained in the form. Note that the webhook cannot add
        /// or remove any form parameter.
        #[prost(message, repeated, tag = "2")]
        pub parameter_info: ::prost::alloc::vec::Vec<form_info::ParameterInfo>,
    }
    /// Nested message and enum types in `FormInfo`.
    pub mod form_info {
        /// Represents parameter information.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ParameterInfo {
            /// Always present for [WebhookRequest][google.cloud.dialogflow.cx.v3.WebhookRequest]. Required for
            /// [WebhookResponse][google.cloud.dialogflow.cx.v3.WebhookResponse].
            /// The human-readable name of the parameter, unique within the form. This
            /// field cannot be modified by the webhook.
            #[prost(string, tag = "1")]
            pub display_name: ::prost::alloc::string::String,
            /// Optional for both [WebhookRequest][google.cloud.dialogflow.cx.v3.WebhookRequest] and [WebhookResponse][google.cloud.dialogflow.cx.v3.WebhookResponse].
            /// Indicates whether the parameter is required. Optional parameters will
            /// not trigger prompts; however, they are filled if the user specifies
            /// them. Required parameters must be filled before form filling concludes.
            #[prost(bool, tag = "2")]
            pub required: bool,
            /// Always present for [WebhookRequest][google.cloud.dialogflow.cx.v3.WebhookRequest]. Required for
            /// [WebhookResponse][google.cloud.dialogflow.cx.v3.WebhookResponse]. The state of the parameter. This field can be set
            /// to [INVALID][google.cloud.dialogflow.cx.v3.PageInfo.FormInfo.ParameterInfo.ParameterState.INVALID] by
            /// the webhook to invalidate the parameter; other values set by the
            /// webhook will be ignored.
            #[prost(enumeration = "parameter_info::ParameterState", tag = "3")]
            pub state: i32,
            /// Optional for both [WebhookRequest][google.cloud.dialogflow.cx.v3.WebhookRequest] and [WebhookResponse][google.cloud.dialogflow.cx.v3.WebhookResponse].
            /// The value of the parameter. This field can be set by the webhook to
            /// change the parameter value.
            #[prost(message, optional, tag = "4")]
            pub value: ::core::option::Option<::prost_types::Value>,
            /// Optional for [WebhookRequest][google.cloud.dialogflow.cx.v3.WebhookRequest]. Ignored for [WebhookResponse][google.cloud.dialogflow.cx.v3.WebhookResponse].
            /// Indicates if the parameter value was just collected on the last
            /// conversation turn.
            #[prost(bool, tag = "5")]
            pub just_collected: bool,
        }
        /// Nested message and enum types in `ParameterInfo`.
        pub mod parameter_info {
            /// Represents the state of a parameter.
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum ParameterState {
                /// Not specified. This value should be never used.
                Unspecified = 0,
                /// Indicates that the parameter does not have a value.
                Empty = 1,
                /// Indicates that the parameter value is invalid. This field can be used
                /// by the webhook to invalidate the parameter and ask the server to
                /// collect it from the user again.
                Invalid = 2,
                /// Indicates that the parameter has a value.
                Filled = 3,
            }
        }
    }
}
/// Represents session information communicated to and from the webhook.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionInfo {
    /// Always present for [WebhookRequest][google.cloud.dialogflow.cx.v3.WebhookRequest]. Ignored for [WebhookResponse][google.cloud.dialogflow.cx.v3.WebhookResponse].
    /// The unique identifier of the [session][google.cloud.dialogflow.cx.v3.DetectIntentRequest.session]. This
    /// field can be used by the webhook to identify a session.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/sessions/<Session ID>` or `projects/<Project ID>/locations/<Location
    /// ID>/agents/<Agent ID>/environments/<Environment ID>/sessions/<Session ID>`
    /// if environment is specified.
    #[prost(string, tag = "1")]
    pub session: ::prost::alloc::string::String,
    /// Optional for [WebhookRequest][google.cloud.dialogflow.cx.v3.WebhookRequest]. Optional for [WebhookResponse][google.cloud.dialogflow.cx.v3.WebhookResponse].
    /// All parameters collected from forms and intents during the session.
    /// Parameters can be created, updated, or removed by the webhook. To remove a
    /// parameter from the session, the webhook should explicitly set the parameter
    /// value to null in [WebhookResponse][google.cloud.dialogflow.cx.v3.WebhookResponse]. The map is keyed by parameters'
    /// display names.
    #[prost(map = "string, message", tag = "2")]
    pub parameters:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
}
#[doc = r" Generated client implementations."]
pub mod webhooks_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for managing [Webhooks][google.cloud.dialogflow.cx.v3.Webhook]."]
    #[derive(Debug, Clone)]
    pub struct WebhooksClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> WebhooksClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
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
        ) -> WebhooksClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            WebhooksClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns the list of all webhooks in the specified agent."]
        pub async fn list_webhooks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWebhooksRequest>,
        ) -> Result<tonic::Response<super::ListWebhooksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Webhooks/ListWebhooks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified webhook."]
        pub async fn get_webhook(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWebhookRequest>,
        ) -> Result<tonic::Response<super::Webhook>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Webhooks/GetWebhook",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a webhook in the specified agent."]
        pub async fn create_webhook(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWebhookRequest>,
        ) -> Result<tonic::Response<super::Webhook>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Webhooks/CreateWebhook",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified webhook."]
        pub async fn update_webhook(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWebhookRequest>,
        ) -> Result<tonic::Response<super::Webhook>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Webhooks/UpdateWebhook",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified webhook."]
        pub async fn delete_webhook(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWebhookRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3.Webhooks/DeleteWebhook",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
