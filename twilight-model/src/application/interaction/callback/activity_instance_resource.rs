use serde::Deserialize;

/// Represents the Activity launched by the interaction.
///
/// See [Discord Docs/Interaction Callback Activity Instance Resource].
///
/// [Discord Docs/Interaction Callback Activity Instance Resource]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-callback-interaction-callback-activity-instance-resource
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ActivityInstanceResource {
    /// Instance ID of the Activity if one was launched or joined.
    id: String,
}
