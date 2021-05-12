use serde_repr::{Deserialize_repr, Serialize_repr};

use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Denotes the type integers of possible interactions.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#interaction-interactiontype
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
}

impl InteractionType {
    pub fn kind(self) -> &'static str {
        match self {
            InteractionType::Ping => "Ping",
            InteractionType::ApplicationCommand => "ApplicationCommand",
        }
    }
}

#[derive(Debug)]
pub struct UnknownInteractionTypeError {
    value: u8,
}

impl Display for UnknownInteractionTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "unknown interaction type: {}", self.value)
    }
}

impl TryFrom<u8> for InteractionType {
    type Error = UnknownInteractionTypeError;

    fn try_from(i: u8) -> Result<Self, Self::Error> {
        match i {
            1 => Ok(Self::Ping),
            2 => Ok(Self::ApplicationCommand),
            n => Err(UnknownInteractionTypeError { value: n }),
        }
    }
}
