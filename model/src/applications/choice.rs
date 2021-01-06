use serde::{Deserialize, Serialize};

/// Specifies an option that a user must choose from in a dropdown.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#applicationcommandoptionchoice
#[serde(untagged)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum CommandOptionChoice {
    String { name: String, value: String },
    Int { name: String, value: i64 },
}
