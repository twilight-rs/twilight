//! Errors returned by shard operations.

use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};
use twilight_model::gateway::Intents;

use crate::{compression::CompressionError, json::GatewayEventParsingError};

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
        match &self.kind {
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
        match &self.kind {
            ReceiveMessageErrorType::Client => f.write_str("websocket client error"),
            ReceiveMessageErrorType::Decompressing => {
                f.write_str("failed to decompress the message because it may be invalid")
            }
            ReceiveMessageErrorType::Deserializing => {
                f.write_str("message is an unrecognized payload")
            }
            ReceiveMessageErrorType::Process => {
                f.write_str("failed to internally process the received message")
            }
            ReceiveMessageErrorType::Reconnect => f.write_str("failed to reconnect to the gateway"),
            ReceiveMessageErrorType::SendingMessage => {
                f.write_str("failed to send a message over the websocket")
            }
            ReceiveMessageErrorType::AuthorizationInvalid => {
                f.write_str("authorization for the shard is invalid")
            }
            ReceiveMessageErrorType::IntentsDisallowed { intents } => {
                f.write_str("at least one of the intents (")?;
                Debug::fmt(intents, f)?;
                f.write_str(") for the shard are disallowed, possibly because the application may not have access to all of them")
            }
            ReceiveMessageErrorType::IntentsInvalid { intents } => {
                f.write_str("at least one of the intents (")?;
                Debug::fmt(intents, f)?;
                f.write_str(") for the shard are invalid")
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
    /// Provided authorization token is invalid.
    AuthorizationInvalid,
    /// Current user isn't allowed to use at least one of the configured
    /// intents.
    ///
    /// The intents are provided.
    IntentsDisallowed {
        /// Configured intents for the shard.
        intents: Intents,
    },
    /// Configured intents aren't supported by Discord's gateway.
    ///
    /// The intents are provided.
    IntentsInvalid {
        /// Configured intents for the shard.
        intents: Intents,
    },
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
        match &self.kind {
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
}
