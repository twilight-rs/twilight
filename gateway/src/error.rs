//! Errors returned by shard operations.

use crate::{compression::CompressionError, json::GatewayEventParsingError};
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};
use twilight_model::gateway::CloseCode;

/// Received gateway message couldn't be processed.
#[derive(Debug)]
pub struct ProcessError {
    /// Type of error.
    pub(crate) kind: ProcessErrorType,
    /// Source error if available.
    pub(crate) source: Option<Box<dyn Error + Send + Sync>>,
}

impl ProcessError {
    /// Shortcut to create a new error from a message compression error.
    pub(crate) fn from_compression(source: CompressionError) -> Self {
        Self {
            kind: ProcessErrorType::Compression,
            source: Some(Box::new(source)),
        }
    }

    /// Shortcut to create a new error from a gateway event parsing error.
    pub(crate) fn from_json(source: GatewayEventParsingError) -> Self {
        Self {
            kind: ProcessErrorType::Deserializing,
            source: Some(Box::new(source)),
        }
    }

    /// Shortcut to create a new error from a message sending error.
    pub(crate) fn from_send(source: SendError) -> Self {
        Self {
            kind: ProcessErrorType::SendingMessage,
            source: Some(Box::new(source)),
        }
    }
}

impl Display for ProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            ProcessErrorType::Compression => {
                f.write_str("compression failed because the payload may be invalid")
            }
            ProcessErrorType::Deserializing => {
                f.write_str("payload isn't a recognized gateway event")
            }
            ProcessErrorType::ParsingPayload => f.write_str("payload could not be parsed as json"),
            ProcessErrorType::SendingMessage => {
                f.write_str("failed to send a message over the websocket")
            }
        }
    }
}

impl Error for ProcessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`ProcessError`] that occurred.
#[derive(Debug)]
pub enum ProcessErrorType {
    /// Message could not be decompressed.
    Compression,
    /// Received gateway event failed to be deserialized.
    ///
    /// The message payload is likely an unrecognized type that is not yet
    /// supported.
    Deserializing,
    /// There was an error parsing a GatewayEvent payload.
    ParsingPayload,
    /// Message could not be sent over the Websocket connection.
    ///
    /// This may happen when the shard sends heartbeats or attempts to identify
    /// a new gateway session.
    SendingMessage,
}

/// Receiving the next Websocket message failed.
#[derive(Debug)]
pub struct ReceiveMessageError {
    /// Type of error.
    pub(crate) kind: ReceiveMessageErrorType,
    /// Source error if available.
    pub(crate) source: Option<Box<dyn Error + Send + Sync>>,
}

impl ReceiveMessageError {
    /// Shortcut to create a new error from a fatal close code.
    pub(crate) fn from_fatally_closed(close_code: u16) -> Self {
        Self {
            kind: ReceiveMessageErrorType::FatallyClosed { close_code },
            source: None,
        }
    }

    /// Shortcut to create a new error from a gateway event parsing error.
    pub(crate) fn from_json(source: GatewayEventParsingError) -> Self {
        Self {
            kind: ReceiveMessageErrorType::Deserializing,
            source: Some(Box::new(source)),
        }
    }

    /// Shortcut to create a new error from a shard initialization error.
    pub(crate) fn from_reconnect(source: ShardInitializeError) -> Self {
        Self {
            kind: ReceiveMessageErrorType::Reconnect,
            source: Some(Box::new(source)),
        }
    }

    /// Shortcut to create a new error from a message sending error.
    pub(crate) fn from_send(source: SendError) -> Self {
        Self {
            kind: ReceiveMessageErrorType::SendingMessage,
            source: Some(Box::new(source)),
        }
    }

    /// Whether the error is fatal.
    ///
    /// If the error is fatal then further attempts to use the shard will return
    /// more fatal errors.
    pub fn is_fatal(&self) -> bool {
        if let ReceiveMessageErrorType::FatallyClosed { close_code } = self.kind() {
            !CloseCode::try_from(*close_code).map_or(false, CloseCode::can_reconnect)
        } else {
            false
        }
    }

    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &ReceiveMessageErrorType {
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
        ReceiveMessageErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for ReceiveMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            ReceiveMessageErrorType::Client => f.write_str("websocket client error"),
            ReceiveMessageErrorType::Decompressing => {
                f.write_str("failed to decompress the message because it may be invalid")
            }
            ReceiveMessageErrorType::Deserializing => {
                f.write_str("message is an unrecognized payload")
            }
            ReceiveMessageErrorType::FatallyClosed { close_code } => {
                f.write_str("shard fatally closed: ")?;

                if let Ok(code) = CloseCode::try_from(close_code) {
                    Display::fmt(&code, f)
                } else {
                    Display::fmt(&close_code, f)
                }
            }
            ReceiveMessageErrorType::Process => {
                f.write_str("failed to internally process the received message")
            }
            ReceiveMessageErrorType::Reconnect => f.write_str("failed to reconnect to the gateway"),
            ReceiveMessageErrorType::SendingMessage => {
                f.write_str("failed to send a message over the websocket")
            }
        }
    }
}

impl Error for ReceiveMessageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`ReceiveMessageError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum ReceiveMessageErrorType {
    /// Websocket client received an error, such as as an I/O or TLS error.
    Client,
    /// Decompressing a frame from Discord failed.
    Decompressing,
    /// Received gateway event failed to be deserialized.
    ///
    /// The message payload is likely an unrecognized type that is not yet
    /// supported.
    Deserializing,
    /// Shard has been closed due to a fatal configuration error.
    FatallyClosed {
        /// Close code of the close message.
        ///
        /// The close code may be able to parse into [`CloseCode`] if it's a
        /// known close code. Unknown close codes are considered fatal.
        close_code: u16,
    },
    ///
    /// Processing the message failed.
    ///
    /// The associated error downcasts to [`ProcessError`].
    Process,
    /// Shard failed to reconnect to the gateway.
    Reconnect,
    /// Message could not be sent over the Websocket connection.
    ///
    /// This may happen when the shard sends heartbeats or attempts to identify
    /// a new gateway session.
    SendingMessage,
}

/// Sending a command failed.
#[derive(Debug)]
pub struct SendError {
    /// Type of error.
    pub(crate) kind: SendErrorType,
    /// Source error if available.
    pub(crate) source: Option<Box<dyn Error + Send + Sync>>,
}

impl SendError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &SendErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (SendErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, None)
    }
}

impl Display for SendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            SendErrorType::Sending => f.write_str("sending the message over the websocket failed"),
            SendErrorType::Serializing => f.write_str("serializing the value as json failed"),
        }
    }
}

impl Error for SendError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`SendError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum SendErrorType {
    /// Sending the payload over the WebSocket failed. This is indicative of a
    /// shutdown shard.
    Sending,
    /// Serializing the payload as JSON failed.
    Serializing,
}

/// Initializing a shard and connecting to the gateway failed.
#[derive(Debug)]
pub struct ShardInitializeError {
    /// Type of error.
    pub(crate) kind: ShardInitializeErrorType,
    /// Source error if available.
    pub(crate) source: Option<Box<dyn Error + Send + Sync>>,
}

impl ShardInitializeError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &ShardInitializeErrorType {
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
        ShardInitializeErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for ShardInitializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ShardInitializeErrorType::Establishing => {
                f.write_str("establishing the connection failed")
            }
            ShardInitializeErrorType::UrlInvalid { url } => {
                f.write_str("user provided url is invalid: ")?;

                f.write_str(url)
            }
        }
    }
}

impl Error for ShardInitializeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`ShardInitializeError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum ShardInitializeErrorType {
    /// Establishing a connection to the gateway failed.
    Establishing,
    /// Gateway URL provided via [`ConfigBuilder::gateway_url`] is invalid.
    ///
    /// [`ConfigBuilder::gateway_url`]: crate::config::ConfigBuilder::gateway_url
    UrlInvalid {
        /// Fully built URL with a specified API version, compression, and other
        /// features.
        url: String,
    },
}

#[cfg(test)]
mod tests {
    use super::{
        ProcessError, ProcessErrorType, ReceiveMessageError, ReceiveMessageErrorType, SendError,
        SendErrorType, ShardInitializeError, ShardInitializeErrorType,
    };
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{error::Error, fmt::Debug};

    assert_fields!(ReceiveMessageErrorType::FatallyClosed: close_code);
    assert_fields!(ShardInitializeErrorType::UrlInvalid: url);
    assert_impl_all!(ProcessErrorType: Debug, Send, Sync);
    assert_impl_all!(ProcessError: Error, Send, Sync);
    assert_impl_all!(ReceiveMessageErrorType: Debug, Send, Sync);
    assert_impl_all!(ReceiveMessageError: Error, Send, Sync);
    assert_impl_all!(SendErrorType: Debug, Send, Sync);
    assert_impl_all!(SendError: Error, Send, Sync);
    assert_impl_all!(ShardInitializeErrorType: Debug, Send, Sync);
    assert_impl_all!(ShardInitializeError: Error, Send, Sync);
}
