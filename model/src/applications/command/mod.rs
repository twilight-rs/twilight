mod option;

pub use option::{
    BaseCommandOptionData, ChoiceCommandOptionData, CommandOption, CommandOptionChoice,
    CommandOptionType, OptionsCommandOptionData,
};

use crate::id::{ApplicationId, CommandId};
use serde::{Deserialize, Serialize};

/// Data sent to discord to create a command.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#applicationcommand
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Command {
    /* TODO: Should there be a specific struct in http where
     * this field is a Option, becuase it is only used when
     * creating commands.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<CommandId>,
    pub application_id: ApplicationId,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub options: Vec<CommandOption>,
}
