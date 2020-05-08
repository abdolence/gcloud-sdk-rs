/// Log content of an action on a recommendation. This includes Mark* actions, as
/// well as ApplyRule actions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionLog {
    /// Required. User that executed this action. Eg, foo@gmail.com
    #[prost(string, tag = "1")]
    pub actor: std::string::String,
    /// Required. Action that was taken by the actor. Eg, MarkCompleted.
    #[prost(
        enumeration = "super::super::v1::recommendation_state_info::State",
        tag = "2"
    )]
    pub state: i32,
    /// Optional. Metadata that was included with the action that was taken.
    #[prost(map = "string, string", tag = "3")]
    pub state_metadata: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Required. Name of the recommendation which was acted on. Eg, :
    /// 'projects/foo/locations/global/recommenders/roleReco/recommendations/r1'
    #[prost(string, tag = "4")]
    pub recommendation_name: std::string::String,
}
