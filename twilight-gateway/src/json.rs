//! Function wrappers for deserializing and serializing events and commands.

pub use serde_json::from_str;
#[cfg(not(feature = "simd-json"))]
pub use serde_json::to_string;
#[cfg(feature = "simd-json")]
pub use simd_json::to_string;

use crate::{
    error::{ReceiveMessageError, ReceiveMessageErrorType},
    EventTypeFlags, Message,
};
use serde::de::DeserializeSeed;
use twilight_model::gateway::{
    event::{Event, GatewayEventDeserializer},
    OpCode,
};

/// Deserialize a websocket message into an event if `wanted_event_types`
/// contains its type.
///
/// Close messages are always considered wanted and map onto
/// [`Event::GatewayClose`].
///
/// # Errors
///
/// Returns a [`ReceiveMessageErrorType::Deserializing`] error if the *known*
/// event could not be deserialized.
pub fn deserialize_wanted(
    message: Message,
    wanted_event_types: EventTypeFlags,
) -> Result<Option<Event>, ReceiveMessageError> {
    let event = match message {
        Message::Close(frame) => return Ok(Some(Event::GatewayClose(frame))),
        Message::Text(event) => event,
    };

    let Some(gateway_deserializer) = GatewayEventDeserializer::from_json(&event) else {
        return Err(ReceiveMessageError {
            kind: ReceiveMessageErrorType::Deserializing { event },
            source: None,
        });
    };

    let Some(opcode) = OpCode::from(gateway_deserializer.op()) else {
        return Ok(None);
    };

    let event_type = gateway_deserializer.event_type();

    let Ok(event_type) = EventTypeFlags::try_from((opcode, event_type)) else {
        return Ok(None);
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
            .map(|event| Some(event.into()))
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
