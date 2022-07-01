use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};
use twilight_model::gateway::Intents;

use crate::{compression::CompressionError, json::GatewayEventParsingError};

#[derive(Debug)]
pub struct ProcessError {
    pub(crate) kind: ProcessErrorType,
    pub(crate) source: Option<Box<dyn Error + Send + Sync>>,
}

impl ProcessError {
    pub(crate) fn from_compression(source: CompressionError) -> Self {
        Self {
            kind: ProcessErrorType::Compression,
            source: Some(Box::new(source)),
        }
    }

    pub(crate) fn from_json(source: GatewayEventParsingError) -> Self {
        Self {
            kind: ProcessErrorType::Deserializing,
            source: Some(Box::new(source)),
        }
    }

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
    Compression,
    Deserializing,
    /// There was an error parsing a GatewayEvent payload.
    ParsingPayload,
    SendingMessage,
}

/// Receiving the next Websocket message failed.
#[derive(Debug)]
pub struct ReceiveMessageError {
    pub(crate) kind: ReceiveMessageErrorType,
    pub(crate) source: Option<Box<dyn Error + Send + Sync>>,
}

impl ReceiveMessageError {
    pub(crate) fn from_json(source: GatewayEventParsingError) -> Self {
        Self {
            kind: ReceiveMessageErrorType::Deserializing,
            source: Some(Box::new(source)),
        }
    }

    pub(crate) fn from_send(source: SendError) -> Self {
        Self {
            kind: ReceiveMessageErrorType::SendingMessage,
            source: Some(Box::new(source)),
        }
    }

    pub(crate) fn with_reconnect(source: ShardInitializeError) -> Self {
        Self {
            kind: ReceiveMessageErrorType::Reconnect,
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
    Deserializing,
    /// Processing the message failed.
    ///
    /// The associated error downcasts to [`ProcessError`].
    Process,
    Reconnect,
    SendingMessage,
}

/// Sending a command failed.
#[derive(Debug)]
pub struct SendError {
    pub(crate) kind: SendErrorType,
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
    pub(crate) kind: ShardInitializeErrorType,
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
            ShardInitializeErrorType::AuthorizationInvalid { shard_id, .. } => {
                f.write_str("the authorization token for shard ")?;
                Display::fmt(shard_id, f)?;

                f.write_str(" is invalid")
            }
            ShardInitializeErrorType::Establishing => {
                f.write_str("establishing the connection failed")
            }
            ShardInitializeErrorType::IntentsDisallowed { intents, shard_id } => {
                f.write_str("at least one of the intents (")?;
                Debug::fmt(intents, f)?;
                f.write_str(") for shard ")?;
                Display::fmt(shard_id, f)?;

                f.write_str(" are disallowed")
            }
            ShardInitializeErrorType::IntentsInvalid { intents, shard_id } => {
                f.write_str("at least one of the intents (")?;
                Debug::fmt(intents, f)?;
                f.write_str(") for shard ")?;
                Display::fmt(shard_id, f)?;

                f.write_str(" are invalid")
            }
            ShardInitializeErrorType::ParsingGatewayUrl { url } => {
                f.write_str("the gateway url `")?;
                f.write_str(url)?;

                f.write_str("` is invalid")
            }
            ShardInitializeErrorType::RetrievingGatewayUrl => {
                f.write_str("retrieving the gateway URL via HTTP failed")
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
    /// Provided authorization token is invalid.
    AuthorizationInvalid { shard_id: u64, token: String },
    /// Establishing a connection to the gateway failed.
    Establishing,
    /// Current user isn't allowed to use at least one of the configured
    /// intents.
    ///
    /// The intents are provided.
    IntentsDisallowed {
        /// The configured intents for the shard.
        intents: Intents,
        /// The ID of the shard.
        shard_id: u64,
    },
    /// Configured intents aren't supported by Discord's gateway.
    ///
    /// The intents are provided.
    IntentsInvalid {
        /// Configured intents for the shard.
        intents: Intents,
        /// ID of the shard.
        shard_id: u64,
    },
    /// Parsing the gateway URL provided by Discord to connect to the gateway
    /// failed due to an invalid URL.
    ParsingGatewayUrl {
        /// URL that couldn't be parsed.
        url: String,
    },
    /// Retrieving the gateway URL via the Twilight HTTP client failed.
    RetrievingGatewayUrl,
}
