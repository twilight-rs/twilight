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
use twilight_model::gateway::{
    event::{GatewayEvent, GatewayEventDeserializer},
    OpCode,
};

/// Parse JSON into a gateway event without existing knowledge of its underlying
/// parts.
///
/// Returns [`None`] if the event is not contained inside of `event_types`.
///
/// # Errors
///
/// Returns a [`ReceiveMessageErrorType::Deserializing`] error if the payload
/// could not be deserialized.
pub fn parse(
    event_types: EventTypeFlags,
    json: String,
) -> Result<Option<GatewayEvent>, ReceiveMessageError> {
    let gateway_deserializer =
        GatewayEventDeserializer::from_json(&json).expect("Shard::process asserted valid opcode");

    let opcode = if let Some(opcode) = OpCode::from(gateway_deserializer.op()) {
        opcode
    } else {
        let opcode = gateway_deserializer.op();

        return Err(ReceiveMessageError {
            kind: ReceiveMessageErrorType::Deserializing { event: json },
            source: Some(format!("unknown opcode: {opcode}").into()),
        });
    };

    let event_type = gateway_deserializer.event_type();

    let event_flag = if let Ok(event_flag) = EventTypeFlags::try_from((opcode, event_type)) {
        event_flag
    } else {
        let opcode = opcode as u8;
        let source = format!("unknown opcode/dispatch event type: {opcode}/{event_type:?}");

        return Err(ReceiveMessageError {
            kind: ReceiveMessageErrorType::Deserializing { event: json },
            source: Some(source.into()),
        });
    };

    if event_types.contains(event_flag) {
        #[cfg(feature = "simd-json")]
        let gateway_deserializer = gateway_deserializer.into_owned();
        #[cfg(feature = "simd-json")]
        let mut bytes = json.into_bytes();

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
        let mut json_deserializer = serde_json::Deserializer::from_str(&json);

        gateway_deserializer
            .deserialize(&mut json_deserializer)
            .map(Some)
            .map_err(|source| ReceiveMessageError {
                kind: ReceiveMessageErrorType::Deserializing {
                    #[cfg(feature = "simd-json")]
                    event: String::from_utf8_lossy(&bytes).into_owned(),
                    #[cfg(not(feature = "simd-json"))]
                    event: json,
                },
                source: Some(Box::new(source)),
            })
    } else {
        Ok(None)
    }
}
