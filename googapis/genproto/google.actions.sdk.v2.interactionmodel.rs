/// Defines a handler to be executed after an event. Examples of events are
/// intent and condition based events in a scene.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventHandler {
    /// Name of the webhook handler to call.
    #[prost(string, tag = "1")]
    pub webhook_handler: std::string::String,
    /// Prompts can either be inlined or referenced by name.
    #[prost(oneof = "event_handler::Prompt", tags = "2, 3")]
    pub prompt: ::std::option::Option<event_handler::Prompt>,
}
pub mod event_handler {
    /// Prompts can either be inlined or referenced by name.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Prompt {
        /// Inlined static prompt. Can contain references to string resources in
        /// bundles.
        #[prost(message, tag = "2")]
        StaticPrompt(super::prompt::StaticPrompt),
        /// Name of the static prompt to invoke.
        #[prost(string, tag = "3")]
        StaticPromptName(std::string::String),
    }
}
/// Defines a global intent handler. Global intent events are scoped to the
/// entire Actions project and may be overridden by intent handlers in a scene.
/// Intent names must be unique within an Actions project.
///
/// Global intents can be matched anytime during a session, allowing users to
/// access common flows like  "get help" or "go back home." They can also be
/// used to deep link users into specific flows when they invoke an Action.
///
/// Note, the intent name is specified in the name of the file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalIntentEvent {
    /// Optional. Destination scene which the conversation should jump to. The state of the
    /// current scene is destroyed on the transition.
    #[prost(string, tag = "1")]
    pub transition_to_scene: std::string::String,
    /// Optional. Event handler which is triggered when the intent is matched. Should execute
    /// before transitioning to the destination scene. Useful to generate Prompts
    /// in response to events.
    #[prost(message, optional, tag = "2")]
    pub handler: ::std::option::Option<EventHandler>,
}
/// Intents map open-ended user input to structured objects. Spoken
/// phrases are matched to intents with Google's Natural Language Understanding
/// (NLU). Intent matches can trigger events in your conversation design to
/// progress the user's conversation.
/// The intent name is specified in the name of the file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Intent {
    /// The list of parameters within the training phrases. All parameters must be
    /// defined here to be used in the training phrase.
    #[prost(message, repeated, tag = "1")]
    pub parameters: ::std::vec::Vec<intent::IntentParameter>,
    /// Training phrases allow Google’s NLU to automatically match intents with
    /// user input. The more unique phrases that are provided, the better chance
    /// this intent will be matched.
    /// The following is the format of training phrase part which are annotated.
    /// Note that `auto` field is optional and the default behavior when `auto` is
    /// not specified is equivalent to `auto=false`.
    /// `($<paramName> '<sample text>' auto=<true or false>)`
    /// `auto = true` means the part was auto annotated by NLU.
    /// `auto = false` means the part was annotated by the user. This is the
    ///     default when auto is not specified.
    /// Example:
    /// "Book a flight from ($source 'San Francisco' auto=false) to ($dest
    /// 'Vancouver')"
    #[prost(string, repeated, tag = "2")]
    pub training_phrases: ::std::vec::Vec<std::string::String>,
}
pub mod intent {
    /// Definition of a parameter which can be used inside training phrases.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IntentParameter {
        /// Required. Unique name of the intent parameter. Can be used in conditions and
        /// responses to reference intent parameters extracted by NLU with
        /// $intent.params.[name].resolved
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        /// The type of the intent parameter.
        #[prost(oneof = "intent_parameter::ParameterType", tags = "2")]
        pub parameter_type: ::std::option::Option<intent_parameter::ParameterType>,
    }
    pub mod intent_parameter {
        /// The type of the intent parameter.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ParameterType {
            /// Optional. Declares the data type of this parameter.
            /// This should not be set for built-in intents.
            #[prost(message, tag = "2")]
            Type(super::super::r#type::ClassReference),
        }
    }
}
/// Registers events that trigger as the result of a true condition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConditionalEvent {
    /// Required. Filter condition for this event to trigger. If condition is evaluated to
    /// true then the associated `handler` will be triggered.
    /// The following variable references are supported:
    ///   `$session` - To reference data in session storage.
    ///   `$user` - To reference data in user storage.
    /// The following boolean operators are supported (with examples):
    ///   `&&` - `session.params.counter > 0 && session.params.counter < 100`
    ///   `||` - `session.params.foo == "John" || session.params.counter == "Adam"`
    ///   `!`  - `!(session.params.counter == 5)`
    /// The following comparisons are supported:
    ///   `==`, `!=`, `<`, `>`, `<=`, `>=`
    /// The following list and string operators are supported (with examples):
    ///   `in`        - "Watermelon" in `session.params.fruitList`
    ///   `size`      - `size(session.params.fruitList) > 2`
    ///   `substring` - `session.params.fullName.contains("John")`
    #[prost(string, tag = "1")]
    pub condition: std::string::String,
    /// Optional. Destination scene which the conversation should jump to when the associated
    /// condition is evaluated to true. The state of the current scene is destroyed
    /// on the transition.
    #[prost(string, tag = "2")]
    pub transition_to_scene: std::string::String,
    /// Optional. Event handler which is triggered when the associated condition is evaluated
    /// to `true`. Should execute before transitioning to the destination scene.
    /// Useful to generate Prompts in response to events.
    #[prost(message, optional, tag = "3")]
    pub handler: ::std::option::Option<EventHandler>,
}
/// Registers Events which trigger as the result of an intent match.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentEvent {
    /// Required. Intent triggering the event.
    #[prost(string, tag = "1")]
    pub intent: std::string::String,
    /// Optional. Destination scene which the conversation should jump to. The state of the
    /// current scene is destroyed on the transition.
    #[prost(string, tag = "2")]
    pub transition_to_scene: std::string::String,
    /// Optional. Event handler which is triggered when the intent is matched. Should execute
    /// before transitioning to the destination scene. Useful to generate prompts
    /// in response to events.
    #[prost(message, optional, tag = "3")]
    pub handler: ::std::option::Option<EventHandler>,
}
/// Configuration for a slot. Slots are single units of data that can be filled
/// through natural language (ie. intent parameters), session parameters, and
/// other sources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Slot {
    /// Required. Name of the slot.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Declares the data type of this slot.
    #[prost(message, optional, tag = "2")]
    pub r#type: ::std::option::Option<r#type::ClassReference>,
    /// Optional. Indicates whether the slot is required to be filled before
    /// advancing. Required slots that are not filled will trigger a customizable
    /// prompt to the user.
    #[prost(bool, tag = "3")]
    pub required: bool,
    /// Optional. Registers Prompts for different stages of slot filling.
    #[prost(message, optional, tag = "4")]
    pub prompt_settings: ::std::option::Option<slot::PromptSettings>,
    /// Optional. Commit behavior associated with the slot.
    #[prost(message, optional, tag = "5")]
    pub commit_behavior: ::std::option::Option<slot::CommitBehavior>,
    /// Optional. Additional configuration associated with the slot which is
    /// used for filling the slot. The format of the config is specific to the
    /// type of the slot. Resource references to user or session parameter can be
    /// added to this config. This config is needed for filling slots related to
    /// transactions and user engagement.
    ///
    /// Example:
    ///  For a slot of type actions.type.CompletePurchaseValue, the following
    ///  config proposes a digital good order with a reference to a client defined
    ///  session parameter `userSelectedSkuId`:
    ///
    ///    {
    ///      "@type": "type.googleapis.com/
    ///                  google.actions.transactions.v3.CompletePurchaseValueSpec",
    ///      "skuId": {
    ///        "skuType": "SKU_TYPE_IN_APP",
    ///        "id": "$session.params.userSelectedSkuId",
    ///        "packageName": "com.example.company"
    ///      }
    ///    }
    #[prost(message, optional, tag = "6")]
    pub config: ::std::option::Option<::prost_types::Value>,
    /// Optional. Configuration to populate a default value for this slot.
    #[prost(message, optional, tag = "7")]
    pub default_value: ::std::option::Option<slot::DefaultValue>,
}
pub mod slot {
    /// A single place where slot prompts are defined.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PromptSettings {
        /// Prompt for the slot value itself. Example: "What size did you want?"
        #[prost(message, optional, tag = "1")]
        pub initial_prompt: ::std::option::Option<super::EventHandler>,
        /// Prompt to give when the user's input does not match the expected
        /// value type for the slot for the first time. Example: "Sorry, I
        /// didn't get that."
        #[prost(message, optional, tag = "2")]
        pub no_match_prompt1: ::std::option::Option<super::EventHandler>,
        /// Prompt to give when the user's input does not match the expected
        /// value type for the slot for the second time. Example: "Sorry, I
        /// didn't get that."
        #[prost(message, optional, tag = "3")]
        pub no_match_prompt2: ::std::option::Option<super::EventHandler>,
        /// Prompt to give when the user's input does not match the expected
        /// value type for the slot for the last time. Example: "Sorry, I
        /// didn't get that."
        #[prost(message, optional, tag = "4")]
        pub no_match_final_prompt: ::std::option::Option<super::EventHandler>,
        /// Prompt to give when the user does not provide an input for the first
        /// time. Example: "Sorry, I didn't get that."
        #[prost(message, optional, tag = "5")]
        pub no_input_prompt1: ::std::option::Option<super::EventHandler>,
        /// Prompt to give when the user does not provide an input for the second
        /// time. Example: "Sorry, I didn't get that."
        #[prost(message, optional, tag = "6")]
        pub no_input_prompt2: ::std::option::Option<super::EventHandler>,
        /// Prompt to give when the user does not provide an input for the last
        /// time. Example: "Sorry, I didn't get that."
        #[prost(message, optional, tag = "7")]
        pub no_input_final_prompt: ::std::option::Option<super::EventHandler>,
    }
    /// Message describing the commit behavior associated with the slot after it
    /// has been successfully filled.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CommitBehavior {
        /// The session parameter to write the slot value after it is filled. Note
        /// that nested paths are not currently supported. "$$" is used to write the
        /// slot value to a session parameter with same name as the slot.
        /// Eg: write_session_param = "fruit" corresponds to "$session.params.fruit".
        /// write_session_param = "ticket" corresponds to "$session.params.ticket".
        #[prost(string, tag = "1")]
        pub write_session_param: std::string::String,
    }
    /// Configuration to populate a default value for this slot.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DefaultValue {
        /// Optional. The session parameter to be used to initialize the slot value, if it has
        /// a non-empty value. The type of the value must match the type of the slot.
        /// Note that nested paths are not currently supported.
        /// Eg: `session_param = "fruit"` corresponds to `$session.params.fruit`.
        /// `session_param = "ticket"` corresponds to `$session.params.ticket`.
        #[prost(string, tag = "1")]
        pub session_param: std::string::String,
        /// Optional. Constant default value for the slot. This will only be used if a value
        /// for this slot was not populated through the `session_param`. The
        /// type for this value must match the type of the slot.
        #[prost(message, optional, tag = "2")]
        pub constant: ::std::option::Option<::prost_types::Value>,
    }
}
/// Scene is the basic unit of control flow when designing a conversation. They
/// can be chained together with other scenes, generate prompts for the end user,
/// and define slots.
/// The scene name is specified in the name of the file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scene {
    /// Handler to invoke when transitioning into this scene.
    #[prost(message, optional, tag = "1")]
    pub on_enter: ::std::option::Option<EventHandler>,
    /// The list of events that trigger based on intents. These events can
    /// be triggered at any time after the on_load Handler has been called.
    /// Important - these events define the set of intents which are scoped to
    /// this scene and will take precedence over any globally defined events that
    /// have the same intents or their triggering phrases. Intent names must be
    /// unique within a scene.
    #[prost(message, repeated, tag = "2")]
    pub intent_events: ::std::vec::Vec<IntentEvent>,
    /// The list of events to trigger based on conditional statements. These are
    /// evaluated after the form has been filled or immediately after on_load if
    /// this scene does not have a form (evaluation is only done once). Only the
    /// first matching event will be triggered.
    #[prost(message, repeated, tag = "3")]
    pub conditional_events: ::std::vec::Vec<ConditionalEvent>,
    /// Ordered list of slots. Each slot defines the type of data
    /// that it will resolve and configuration to customize the experience of this
    /// resolution (e.g. prompts).
    #[prost(message, repeated, tag = "4")]
    pub slots: ::std::vec::Vec<Slot>,
    /// Handler called when there is a change in state of a slot not
    /// caused by updates within another Handler. This allows slots to be
    /// invalidated, the scene invalidated or other changes to scene state.
    #[prost(message, optional, tag = "5")]
    pub on_slot_updated: ::std::option::Option<EventHandler>,
}
