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
use std::str;
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
    #[cfg_attr(not(feature = "simd-json"), allow(unused_mut))]
    let mut bytes = json.into_bytes();
    let json = str::from_utf8(&bytes).unwrap();

    let gateway_deserializer =
        GatewayEventDeserializer::from_json(json).expect("Shard::process asserted valid opcode");

    #[cfg(feature = "simd-json")]
    let (gateway_deserializer, mut json_deserializer) = {
        let gateway_deserializer = gateway_deserializer.into_owned();

        let json_deserializer = match simd_json::Deserializer::from_slice(&mut bytes) {
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

        (gateway_deserializer, json_deserializer)
    };

    #[cfg(not(feature = "simd-json"))]
    let mut json_deserializer = serde_json::Deserializer::from_str(json);

    let opcode = match OpCode::new(gateway_deserializer.op()) {
        opcode => opcode,
        #[allow(unused)]
        _ => {
            // todo!()
            return Err(ReceiveMessageError {
                kind: ReceiveMessageErrorType::Deserializing {
                    event: String::from_utf8_lossy(&bytes).into_owned(),
                },
                source: Some(format!("unknown opcode: {}", gateway_deserializer.op()).into()),
            });
        }
    };

    let event_type = gateway_deserializer.event_type();

    let event_flag = match EventTypeFlags::try_from((opcode, event_type)) {
        Ok(event_flag) => event_flag,
        Err(_) => {
            return Err(ReceiveMessageError {
                kind: ReceiveMessageErrorType::Deserializing {
                    event: String::from_utf8_lossy(&bytes).into_owned(),
                },
                source: Some(
                    format!(
                        "unknown opcode/dispatch event type: {}/{event_type:?}",
                        u8::from(opcode),
                    )
                    .into(),
                ),
            })
        }
    };

    if event_types.contains(event_flag) {
        gateway_deserializer
            .deserialize(&mut json_deserializer)
            .map(Some)
            .map_err(|source| ReceiveMessageError {
                kind: ReceiveMessageErrorType::Deserializing {
                    event: String::from_utf8_lossy(&bytes).into_owned(),
                },
                source: Some(Box::new(source)),
            })
    } else {
        Ok(None)
    }
}
