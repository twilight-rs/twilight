use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Data received when a user partially fills in a command option.
///
/// [`value`]: Self::value
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ApplicationCommandAutocompleteDataOption {
    #[serde(default)]
    pub focused: bool,
    #[serde(rename = "type")]
    pub kind: ApplicationCommandAutocompleteDataOptionType,
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<ApplicationCommandAutocompleteDataOption>,
    pub value: Option<String>,
}

/// Type of option data received.
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum ApplicationCommandAutocompleteDataOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
    Attachment = 11,
}
