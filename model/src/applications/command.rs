use crate::applications::CommandOption;
use crate::id::{ApplicationId, CommandId};
use serde::{Deserialize, Serialize};

/// An executable command by a user in a server.
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

/// The data received when an ApplicationCommand interaction is executed.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#interaction-applicationcommandinteractiondata
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Default)]
pub struct CommandData {
    pub id: CommandId,
    pub name: String,
    #[serde(default)]
    pub options: Vec<CommandDataOption>,
}

/// The data received when a user fills in a command option.
///
/// Note: user, channel, and role option types will be returned as a String
/// option here.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#interaction-applicationcommandinteractiondataoption
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CommandDataOption {
    String {
        name: String,
        value: String,
    },
    Integer {
        name: String,
        value: i64,
    },
    Boolean {
        name: String,
        value: bool,
    },
    SubCommand {
        name: String,
        options: Vec<CommandDataOption>,
    },
}

impl CommandDataOption {
    pub fn kind(&self) -> &'static str {
        match self {
            CommandDataOption::String { .. } => "String",
            CommandDataOption::Integer { .. } => "Integer",
            CommandDataOption::Boolean { .. } => "Boolean",
            CommandDataOption::SubCommand { .. } => "SubCommand",
        }
    }
}
