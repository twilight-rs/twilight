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
#[cfg(not(feature = "simd-json"))]
use twilight_model::gateway::event::GatewayEventDeserializer as EventDeserializer;
#[cfg(feature = "simd-json")]
use twilight_model::gateway::event::GatewayEventDeserializerOwned as EventDeserializer;
use twilight_model::gateway::{event::GatewayEvent, OpCode};

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

    #[cfg(feature = "simd-json")]
    let (gateway_deserializer, mut json_deserializer) = {
        let gateway_deserializer =
            EventDeserializer::from_json(json).expect("Shard::process asserted valid opcode");

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
    let (gateway_deserializer, mut json_deserializer) = {
        let gateway_deserializer =
            EventDeserializer::from_json(json).expect("Shard::process asserted valid opcode");

        let json_deserializer = serde_json::Deserializer::from_str(json);

        (gateway_deserializer, json_deserializer)
    };

    let opcode = match OpCode::from(gateway_deserializer.op()) {
        Some(opcode) => opcode,
        None => {
            return Err(ReceiveMessageError {
                kind: ReceiveMessageErrorType::Deserializing {
                    event: String::from_utf8_lossy(&bytes).into_owned(),
                },
                source: Some(format!("unknown opcode: {}", gateway_deserializer.op()).into()),
            })
        }
    };

    let event_type = gateway_deserializer.event_type_ref();

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
                        opcode as u8
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
