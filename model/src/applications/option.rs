//use crate::id::{ApplicationId, CommandId};
use super::CommandOptionChoice;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/**
| Field           | Type                                    |
|-----------------|-----------------------------------------|
| type            | int                                     |
| name\*          | string                                  |
| description     | string                                  |
| default\*\*     | bool                                    |
| required\*\*\*  | bool                                    |
| choices         | array of ApplicationCommandOptionChoice |
| options\*\*\*\* | array of ApplicationCommandOption       |
**/
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CommandOption {
    #[serde(rename = "type")]
    kind: OptionType,
    name: String,
    description: String,
    default: bool,
    required: bool,
    choices: Vec<CommandOptionChoice>,
    options: Vec<CommandOption>,
}



/*
| Name              | Value |
|-------------------|-------|
| SUB_COMMAND       | 1     |
| SUB_COMMAND_GROUP | 2     |
| STRING            | 3     |
| INTEGER           | 4     |
| BOOLEAN           | 5     |
| USER              | 6     |
| CHANNEL           | 7     |
| ROLE              | 8     |
*/
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
pub enum OptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
}
