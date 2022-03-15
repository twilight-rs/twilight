#[cfg(feature = "simd-json")]
pub use dep_simd_json::{from_slice, to_vec};
#[cfg(not(feature = "simd-json"))]
pub use serde_json::{from_slice, to_vec};

use serde::de::DeserializeSeed;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::gateway::event::{GatewayEvent, GatewayEventDeserializer};

#[derive(Debug)]
pub struct GatewayEventParsingError {
    pub(super) source: Option<Box<dyn Error + Send + Sync>>,
    pub(super) kind: GatewayEventParsingErrorType,
}

impl Display for GatewayEventParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
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

/// Parse a gateway event from a string with headers.
///
/// # Errors
///
/// Returns a [`GatewayEventParsingErrorType::Deserializing`] error type if the
/// payload failed to deserialize.
///
/// Returns a [`GatewayEventParsingErrorType::PayloadInvalid`] error type if the
/// payload wasn't a valid `GatewayEvent` data structure.
pub fn parse_gateway_event(
    op: u8,
    sequence: Option<u64>,
    event_type: Option<&str>,
    json: &mut [u8],
) -> Result<GatewayEvent, GatewayEventParsingError> {
    let gateway_deserializer = GatewayEventDeserializer::new(op, sequence, event_type);
    #[cfg(not(feature = "simd-json"))]
    let mut json_deserializer = serde_json::Deserializer::from_slice(json);
    #[cfg(feature = "simd-json")]
    let mut json_deserializer =
        simd_json::Deserializer::from_slice(json).map_err(|_| GatewayEventParsingError {
            kind: GatewayEventParsingErrorType::PayloadInvalid,
            source: None,
        })?;

    gateway_deserializer
        .deserialize(&mut json_deserializer)
        .map_err(|source| {
            #[cfg(feature = "tracing")]
            tracing::error!("invalid JSON: {}", String::from_utf8_lossy(json));

            GatewayEventParsingError {
                kind: GatewayEventParsingErrorType::Deserializing,
                source: Some(Box::new(source)),
            }
        })
}

#[cfg(test)]
mod tests {
    use super::{GatewayEventParsingError, GatewayEventParsingErrorType};
    use static_assertions::assert_impl_all;
    use std::{error::Error, fmt::Debug};

    assert_impl_all!(GatewayEventParsingErrorType: Debug, Send, Sync);
    assert_impl_all!(GatewayEventParsingError: Error, Send, Sync);
}
