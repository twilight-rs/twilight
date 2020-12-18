use serde_repr::{Deserialize_repr, Serialize_repr};

use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};

/*
 * # InteractionType
 *
 * | Name               | Value |
 * |--------------------|-------|
 * | Ping               | 1     |
 * | ApplicationCommand | 2     |
 */

/// Types of interactions available
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
}

#[derive(Debug)]
pub struct UnknownInteractionTypeError {
    value: u8,
}

impl Display for UnknownInteractionTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Got unknown interaction type: {}", self.value)
    }    
}

impl TryFrom<u8> for InteractionType {
    type Error = UnknownInteractionTypeError;

    fn try_from(i: u8) -> Result<Self, Self::Error> {
        match i {
            1 => Ok(Self::Ping),
            2 => Ok(Self::ApplicationCommand),
            n => {
                Err(UnknownInteractionTypeError {
                    value: n,
                })
            },
        }
    }
}
