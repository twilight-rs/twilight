//! Utilities for knowing and parsing the current connection stage of a shard.
//!
//! Included is the [`Stage`], which is an enum representing the connection
//! stage with variants such as [`Connected`] or [`Disconnected`].
//!
//! The [`Stage`] also has some parsing capability, so an error type for
//! conversion reasons is included.
//!
//! [`Connected`]: Stage::Connected
//! [`Disconnected`]: Stage::Disconnected

use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Reason for a failure while parsing a value into a [`Stage`].
#[derive(Debug)]
pub struct StageConversionError {
    kind: StageConversionErrorType,
}

impl StageConversionError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &StageConversionErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        StageConversionErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for StageConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            StageConversionErrorType::InvalidInteger { value } => {
                f.write_str("The integer ")?;
                Display::fmt(value, f)?;

                f.write_str(" is invalid")
            }
        }
    }
}

impl Error for StageConversionError {}

/// Type of [`StageConversionError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum StageConversionErrorType {
    /// The integer isn't one that maps to a stage. For example, 7 might not map
    /// to a Stage variant.
    InvalidInteger {
        /// The value that was provided.
        value: u8,
    },
}

/// The current connection stage of a [`Shard`].
///
/// [`Shard`]: super::Shard
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[non_exhaustive]
#[repr(u8)]
pub enum Stage {
    /// Indicator that a shard is now fully connected to the gateway.
    Connected,
    /// Indicator that a shard is now disconnected and may or may not reconnect
    /// based on whether the connection was explicitly shutdown.
    Disconnected,
    /// Indicator that a shard is now handshaking with the gateway to initiate a
    /// connection.
    Handshaking,
    /// Indicator that a shard is now identifying with the gateway to make a new
    /// session.
    Identifying,
    /// Indicator that a shard is now resuming a session.
    Resuming,
}

impl Default for Stage {
    fn default() -> Self {
        Self::Disconnected
    }
}

impl Display for Stage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(match self {
            Self::Connected => "Connected",
            Self::Disconnected => "Disconnected",
            Self::Handshaking => "Handshaking",
            Self::Identifying => "Identifying",
            Self::Resuming => "Resuming",
        })
    }
}

impl TryFrom<u8> for Stage {
    type Error = StageConversionError;

    fn try_from(num: u8) -> Result<Self, Self::Error> {
        Ok(match num {
            0 => Self::Connected,
            1 => Self::Disconnected,
            2 => Self::Handshaking,
            3 => Self::Identifying,
            4 => Self::Resuming,
            other => {
                return Err(StageConversionError {
                    kind: StageConversionErrorType::InvalidInteger { value: other },
                })
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Stage;
    use static_assertions::assert_impl_all;
    use std::{
        error::Error,
        fmt::{Debug, Display},
        hash::Hash,
    };

    assert_impl_all!(
        Stage: Clone,
        Copy,
        Debug,
        Default,
        Display,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Send,
        Sync,
        TryFrom<u8>,
    );

    #[test]
    fn conversion() -> Result<(), Box<dyn Error>> {
        assert_eq!(Stage::Connected, Stage::try_from(0)?);
        assert_eq!(Stage::Disconnected, Stage::try_from(1)?);
        assert_eq!(Stage::Handshaking, Stage::try_from(2)?);
        assert_eq!(Stage::Identifying, Stage::try_from(3)?);
        assert_eq!(Stage::Resuming, Stage::try_from(4)?);
        assert!(Stage::try_from(5).is_err());

        Ok(())
    }

    #[test]
    fn formatting() {
        assert_eq!("Connected", Stage::Connected.to_string());
        assert_eq!("Disconnected", Stage::Disconnected.to_string());
        assert_eq!("Handshaking", Stage::Handshaking.to_string());
        assert_eq!("Identifying", Stage::Identifying.to_string());
        assert_eq!("Resuming", Stage::Resuming.to_string());
    }
}
