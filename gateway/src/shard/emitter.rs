use super::json;
use crate::{Event, EventTypeFlags};
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use twilight_model::gateway::event::shard::Payload;

#[derive(Debug)]
pub struct EmitJsonError {
    kind: EmitJsonErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl EmitJsonError {
    pub fn into_parts(self) -> (EmitJsonErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for EmitJsonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            EmitJsonErrorType::EventTypeUnknown { event_type, op } => {
                f.write_str("provided event type (")?;
                Debug::fmt(event_type, f)?;
                f.write_str(")/op (")?;
                Display::fmt(op, f)?;

                f.write_str(") pair is unknown")
            }
            EmitJsonErrorType::Parsing => f.write_str("parsing a gateway event failed"),
        }
    }
}

impl Error for EmitJsonError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`EmitJsonError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum EmitJsonErrorType {
    /// Provided event type and/or opcode combination doesn't match a known
    /// event type flag.
    EventTypeUnknown {
        /// Received dispatch event type.
        event_type: Option<String>,
        /// Received opcode.
        op: u8,
    },
    /// Parsing a a gateway event failed.
    Parsing,
}

/// Emitter over a listener with some useful things on top to abstract common
/// operations.
#[derive(Clone, Debug)]
pub struct Emitter {
    event_types: EventTypeFlags,
    tx: UnboundedSender<Event>,
}

impl Emitter {
    /// Create a new emitter for events and bytes.
    pub fn new(event_types: EventTypeFlags) -> (Self, UnboundedReceiver<Event>) {
        let (tx, rx) = mpsc::unbounded_channel();

        (Self { event_types, tx }, rx)
    }

    /// Whether the configured event types include an individual event type.
    #[inline]
    pub const fn wants(&self, event_type: EventTypeFlags) -> bool {
        self.event_types.contains(event_type)
    }

    /// Send some bytes to the listener if it has subscribed to
    /// [`EventTypeFlags::SHARD_PAYLOAD`].
    ///
    /// Shard payload events aren't subscribed to by default and must be opted
    /// in to. If the listener has subscribed to them, then the input bytes will
    /// be cloned. This means that for most users, this will be a cheap check.
    ///
    /// [`EventTypeFlags::SHARD_PAYLOAD`]: crate::EventTypeFlags::SHARD_PAYLOAD
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
    pub fn bytes(&self, bytes: &[u8]) {
        if self.wants(EventTypeFlags::SHARD_PAYLOAD) {
            self.send(Event::ShardPayload(Payload {
                bytes: bytes.to_vec(),
            }))
        }
    }

    /// Send an event to the listener if it has subscribed to its event type.
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
    pub fn event(&self, event: Event) {
        let event_type = EventTypeFlags::from(event.kind());

        if self.wants(event_type) {
            self.send(event);
        }
    }

    /// Emit a JSON payload that hasn't been deserialized yet, but only if the
    /// listener wants the event type.
    ///
    /// # Errors
    ///
    /// Returns a [`EmitJsonErrorType::EventTypeUnknown`] error type if the
    /// event type is unknown.
    ///
    /// Returns a [`EmitJsonErrorType::Parsing`] error type if the combination
    /// of the provided opcode, sequence, event type, and JSON could not be
    /// parsed into an event.
    pub fn json(
        &self,
        op: u8,
        seq: Option<u64>,
        event_type: Option<&str>,
        json: &mut [u8],
    ) -> Result<(), EmitJsonError> {
        let flag = EventTypeFlags::try_from((op, event_type)).map_err(|(op, event_type)| {
            EmitJsonError {
                kind: EmitJsonErrorType::EventTypeUnknown {
                    event_type: event_type.map(ToOwned::to_owned),
                    op,
                },
                source: None,
            }
        })?;

        if self.wants(flag) {
            let gateway_event =
                json::parse_gateway_event(op, seq, event_type, json).map_err(|source| {
                    EmitJsonError {
                        kind: EmitJsonErrorType::Parsing,
                        source: Some(Box::new(source)),
                    }
                })?;
            self.event(Event::from(gateway_event));
        }

        Ok(())
    }

    fn send(&self, event: Event) {
        let _res = self.tx.send(event);
    }
}

#[cfg(test)]
mod tests {
    use super::Emitter;
    use crate::{Event, EventTypeFlags};

    #[test]
    fn test_bytes_send() {
        let (emitter, mut rx) = Emitter::new(EventTypeFlags::SHARD_PAYLOAD);
        emitter.bytes(&[1]);

        assert!(rx.try_recv().is_ok());
        assert!(rx.try_recv().is_err());
    }

    #[test]
    fn test_event_sends_to_rx() {
        let (emitter, mut rx) = Emitter::new(EventTypeFlags::default());
        emitter.event(Event::GatewayReconnect);

        assert!(rx.try_recv().is_ok());

        // now check that the event didn't send the event twice
        assert!(rx.try_recv().is_err());
    }
}
