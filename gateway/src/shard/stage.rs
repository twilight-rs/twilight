//! Utilities for knowing and parsing the current connection stage of a
//! [`Shard`].
//!
//! Included is the [`Stage`], which is an enum representing the connection
//! stage with variants such as [`Connecting`] or [`Disconnected`].
//!
//! The `Stage` also has some parsing capability, so an error type for
//! conversion reasons is included.
//!
//! [`Connecting`]: enum.Stage.html#variant.Connecting
//! [`Disconnected`]: enum.Stage.html#variant.Disconnected
//! [`Shard`]: ../struct.Shard.html
//! [`Stage`]: enum.Stage.html

use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Reason for a failure while parsing a value into a [`Stage`].
///
/// [`Stage`]: enum.Stage.html
#[derive(Clone, Debug)]
pub enum StageConversionError {
    /// The integer isn't one that maps to a stage. For example, 7 might not map
    /// to a Stage variant.
    InvalidInteger {
        /// The value that was provided.
        value: u8,
    },
}

impl Display for StageConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::InvalidInteger { value } => write!(f, "The integer {} is invalid", value),
        }
    }
}

impl Error for StageConversionError {}

/// The current connection stage of a [`Shard`].
///
/// [`Shard`]: struct.Shard.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
            other => return Err(StageConversionError::InvalidInteger { value: other }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Stage;
    use std::{convert::TryFrom, error::Error};

    #[test]
    fn test_conversion() -> Result<(), Box<dyn Error>> {
        assert_eq!(Stage::Connected, Stage::try_from(0)?);
        assert_eq!(Stage::Disconnected, Stage::try_from(1)?);
        assert_eq!(Stage::Handshaking, Stage::try_from(2)?);
        assert_eq!(Stage::Identifying, Stage::try_from(3)?);
        assert_eq!(Stage::Resuming, Stage::try_from(4)?);
        assert!(Stage::try_from(5).is_err());

        Ok(())
    }

    #[test]
    fn test_formatting() {
        assert_eq!("Connected", Stage::Connected.to_string());
        assert_eq!("Disconnected", Stage::Disconnected.to_string());
        assert_eq!("Handshaking", Stage::Handshaking.to_string());
        assert_eq!("Identifying", Stage::Identifying.to_string());
        assert_eq!("Resuming", Stage::Resuming.to_string());
    }
}
