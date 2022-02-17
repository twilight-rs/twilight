use crate::{
    application::{
        command::CommandType,
        interaction::{
            application_command::CommandInteractionDataResolved,
            application_command_autocomplete::ApplicationCommandAutocompleteDataOption,
        },
    },
    id::{
        marker::{CommandMarker, GenericMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

/// Data received when an [`ApplicationCommandAutocomplete`] interaction is executed.
///
/// See [Discord Docs/Interaction Object].
///
/// [`ApplicationCommandAutocomplete`]: crate::application::interaction::Interaction::ApplicationCommandAutocomplete
/// [Discord Docs/Interaction Object]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-data-structure
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ApplicationCommandAutocompleteData {
    /// ID of the command.
    pub id: Id<CommandMarker>,
    /// Name of the command.
    pub name: String,
    /// Type of the command.
    #[serde(rename = "type")]
    pub kind: CommandType,
    /// List of parsed options specified by the user.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<ApplicationCommandAutocompleteDataOption>,
    /// Data sent if any of the options are Discord types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<CommandInteractionDataResolved>,
}
