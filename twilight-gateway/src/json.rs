//! Function wrappers for deserializing and serializing events and commands.

pub use serde_json::from_str;
#[cfg(not(feature = "simd-json"))]
pub use serde_json::to_string;
#[cfg(feature = "simd-json")]
pub use simd_json::to_string;

use crate::{
    error::{ReceiveMessageError, ReceiveMessageErrorType},
    EventTypeFlags,
};
use serde::de::DeserializeSeed;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};
use twilight_model::gateway::{
    event::{GatewayEvent, GatewayEventDeserializer},
    OpCode,
};

/// Error occurred due to an unknown event and opcode pair.
#[derive(Debug)]
#[non_exhaustive]
pub(crate) struct UnknownEventError {
    /// Event type in the payload.
    pub event_type: Option<String>,
    /// Opcode in the payload.
    pub opcode: Option<u8>,
}

impl Display for UnknownEventError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("unknown opcode/dispatch event type: ")?;
        Debug::fmt(&self.opcode, f)?;
        f.write_str("/")?;

        Debug::fmt(&self.event_type, f)
    }
}

impl Error for UnknownEventError {}

/// Parse a JSON encoded event into a gateway event if its type is in
/// `wanted_event_types`.
///
/// This function can be used together with [`Shard::next_message`] for greater
/// control of the deserialization process without giving up the ease of use of
/// [`Shard::next_event`].
///
/// Returns [`None`] if the event type is not contained inside of
/// `wanted_event_types`.
///
/// # Errors
///
/// Returns a [`ReceiveMessageErrorType::Deserializing`] error if the event
/// could not be deserialized.
///
/// [`Shard::next_event`]: crate::Shard::next_event
/// [`Shard::next_message`]: crate::Shard::next_message
pub fn parse(
    event: String,
    wanted_event_types: EventTypeFlags,
) -> Result<Option<GatewayEvent>, ReceiveMessageError> {
    let gateway_deserializer =
        if let Some(gateway_deserializer) = GatewayEventDeserializer::from_json(&event) {
            gateway_deserializer
        } else {
            return Err(ReceiveMessageError {
                kind: ReceiveMessageErrorType::Deserializing { event },
                source: Some(Box::new(UnknownEventError {
                    event_type: None,
                    opcode: None,
                })),
            });
        };

    let opcode = if let Some(opcode) = OpCode::from(gateway_deserializer.op()) {
        opcode
    } else {
        let opcode = gateway_deserializer.op();

        return Err(ReceiveMessageError {
            kind: ReceiveMessageErrorType::Deserializing { event },
            source: Some(Box::new(UnknownEventError {
                event_type: None,
                opcode: Some(opcode),
            })),
        });
    };

    let event_type = gateway_deserializer.event_type();

    let event_type = if let Ok(event_type) = EventTypeFlags::try_from((opcode, event_type)) {
        event_type
    } else {
        let opcode = opcode as u8;
        let owned_event_type = event_type.map(ToOwned::to_owned);

        return Err(ReceiveMessageError {
            kind: ReceiveMessageErrorType::Deserializing { event },
            source: Some(Box::new(UnknownEventError {
                event_type: owned_event_type,
                opcode: Some(opcode),
            })),
        });
    };

    if wanted_event_types.contains(event_type) {
        #[cfg(feature = "simd-json")]
        let gateway_deserializer = gateway_deserializer.into_owned();
        #[cfg(feature = "simd-json")]
        let mut bytes = event.into_bytes();

        #[cfg(feature = "simd-json")]
        let mut json_deserializer = match simd_json::Deserializer::from_slice(&mut bytes) {
            Ok(deserializer) => deserializer,
            Err(source) => {
                return Err(ReceiveMessageError {
                    kind: ReceiveMessageErrorType::Deserializing {
                        event: String::from_utf8_lossy(&bytes).into_owned(),
                    },
                    source: Some(Box::new(source)),
                })
            }
        };

        #[cfg(not(feature = "simd-json"))]
        let mut json_deserializer = serde_json::Deserializer::from_str(&event);

        gateway_deserializer
            .deserialize(&mut json_deserializer)
            .map(Some)
            .map_err(|source| ReceiveMessageError {
                kind: ReceiveMessageErrorType::Deserializing {
                    #[cfg(feature = "simd-json")]
                    event: String::from_utf8_lossy(&bytes).into_owned(),
                    #[cfg(not(feature = "simd-json"))]
                    event,
                },
                source: Some(Box::new(source)),
            })
    } else {
        Ok(None)
    }
}
