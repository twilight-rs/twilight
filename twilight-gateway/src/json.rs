//! Function wrappers for deserializing and serializing events and commands.

#[cfg(not(feature = "simd-json"))]
pub use serde_json::to_string;
#[cfg(feature = "simd-json")]
pub use simd_json::to_string;

pub use serde_json::from_str;

use crate::EventTypeFlags;
use serde::de::DeserializeSeed;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    str,
};
use twilight_model::gateway::{
    event::{GatewayEvent, GatewayEventDeserializer},
    OpCode,
};

/// Parsing of a gateway event failed, likely due to a type being unrecognized.
#[derive(Debug)]
pub struct GatewayEventParsingError {
    /// Type of error.
    pub(crate) kind: GatewayEventParsingErrorType,
    /// Source error if available.
    pub(crate) source: Option<Box<dyn Error + Send + Sync>>,
}

impl GatewayEventParsingError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &GatewayEventParsingErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        GatewayEventParsingErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for GatewayEventParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            GatewayEventParsingErrorType::Deserializing => {
                f.write_str("deserializing gateway event as json failed")
            }
            GatewayEventParsingErrorType::PayloadInvalid => {
                f.write_str("payload is an invalid json structure")
            }
        }
    }
}

impl Error for GatewayEventParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`GatewayEventParsingError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum GatewayEventParsingErrorType {
    /// Deserializing the GatewayEvent payload from JSON failed.
    Deserializing,
    /// The payload received from Discord was an unrecognized or invalid
    /// structure.
    ///
    /// The payload was either invalid JSON or did not contain the necessary
    /// "op" key in the object.
    PayloadInvalid,
}

/// Parse JSON into a gateway event without existing knowledge of its underlying
/// parts.
///
/// Returns [`None`] if the event is not contained inside of `event_types`.
///
/// # Errors
///
/// Returns a [`GatewayEventParsingErrorType::Deserializing`] error type if the
/// payload failed to deserialize.
///
/// Returns a [`GatewayEventParsingErrorType::PayloadInvalid`] error type if the
/// payload wasn't a valid `GatewayEvent` data structure, such as due to not
/// being UTF-8 valid.
pub fn parse(
    event_types: EventTypeFlags,
    json: String,
) -> Result<Option<GatewayEvent>, GatewayEventParsingError> {
    #[cfg_attr(not(feature = "simd-json"), allow(unused_mut))]
    let mut bytes = json.into_bytes();
    let json = str::from_utf8(&bytes).unwrap();

    let gateway_deserializer =
        GatewayEventDeserializer::from_json(json).ok_or(GatewayEventParsingError {
            kind: GatewayEventParsingErrorType::PayloadInvalid,
            source: None,
        })?;

    #[cfg(feature = "simd-json")]
    let (gateway_deserializer, mut json_deserializer) = {
        let gateway_deserializer = gateway_deserializer.into_owned();

        let json_deserializer = simd_json::Deserializer::from_slice(&mut bytes).map_err(|_| {
            GatewayEventParsingError {
                kind: GatewayEventParsingErrorType::PayloadInvalid,
                source: None,
            }
        })?;

        (gateway_deserializer, json_deserializer)
    };

    #[cfg(not(feature = "simd-json"))]
    let mut json_deserializer = serde_json::Deserializer::from_str(json);

    let opcode = OpCode::from(gateway_deserializer.op()).ok_or(GatewayEventParsingError {
        kind: GatewayEventParsingErrorType::PayloadInvalid,
        source: None,
    })?;

    let event_flag = EventTypeFlags::try_from((opcode, gateway_deserializer.event_type()))
        .map_err(|_| GatewayEventParsingError {
            kind: GatewayEventParsingErrorType::PayloadInvalid,
            source: None,
        })?;

    if event_types.contains(event_flag) {
        gateway_deserializer
            .deserialize(&mut json_deserializer)
            .map(Some)
            .map_err(|source| {
                tracing::error!("invalid JSON: {}", String::from_utf8_lossy(&bytes));

                GatewayEventParsingError {
                    kind: GatewayEventParsingErrorType::Deserializing,
                    source: Some(Box::new(source)),
                }
            })
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::{GatewayEventParsingError, GatewayEventParsingErrorType};
    use static_assertions::assert_impl_all;
    use std::{error::Error, fmt::Debug};

    assert_impl_all!(GatewayEventParsingErrorType: Debug, Send, Sync);
    assert_impl_all!(GatewayEventParsingError: Error, Send, Sync);
}
