use serde::{
    de::{value, IntoDeserializer},
    Deserialize,
};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::InteractionEnvelopeParseError;

use std::convert::TryFrom;

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

impl TryFrom<u8> for InteractionType {
    type Error = InteractionEnvelopeParseError;

    fn try_from(i: u8) -> Result<Self, Self::Error> {
        Self::deserialize(i.into_deserializer())
            .map_err(|_: value::Error| InteractionEnvelopeParseError::UnknownType(i))
    }
}
