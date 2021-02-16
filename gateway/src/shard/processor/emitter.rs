use super::super::json;
use crate::{listener::Listeners, EventTypeFlags};
use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::gateway::event::{shard::Payload, Event};

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
            EmitJsonErrorType::EventTypeUnknown { event_type, op } => f.write_fmt(format_args!(
                "provided event type ({:?})/op ({}) pair is unknown",
                event_type, op,
            )),
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

/// Emitter over a map of listeners with some useful things on top to abstract
/// common operations.
#[derive(Clone, Debug)]
pub struct Emitter {
    listeners: Listeners<Event>,
}

impl Emitter {
    /// Create a new emitter for events and bytes.
    pub fn new(listeners: Listeners<Event>) -> Self {
        Self { listeners }
    }

    /// Consume the emitter, returning the inner listeners.
    pub fn into_listeners(self) -> Listeners<Event> {
        self.listeners
    }

    /// Determine if any of the listeners want a certain event type.
    pub fn wants(&self, event_type: EventTypeFlags) -> bool {
        self.listeners.event_types().contains(event_type)
    }

    /// Send some bytes to listeners that have subscribed to shard payloads.
    ///
    /// Shard payload events aren't subscribed to by default and must be opted in
    /// to. If a listener has subscribed to them, then the input bytes will be
    /// cloned. This means that for most users, this will be a cheap check.
    #[tracing::instrument(level = "trace")]
    pub fn bytes(&self, bytes: &[u8]) {
        if !self.wants(EventTypeFlags::SHARD_PAYLOAD) {
            return;
        }

        self.send(EventTypeFlags::SHARD_PAYLOAD, |_| {
            Event::ShardPayload(Payload {
                bytes: bytes.to_vec(),
            })
        });
    }

    /// Send an event to listeners that have subscribed to its event type.
    #[tracing::instrument(level = "trace")]
    pub fn event(&self, event: Event) {
        let event_type = EventTypeFlags::from(event.kind());

        if !self.wants(event_type) {
            return;
        }

        let listener_count = self.listeners.len();
        let mut event = Some(event);

        self.send(event_type, |idx| {
            // We conditionally move out the event from its Option here to avoid
            // unnecessary clones on all but the last listener.
            //
            // If there are 2 listeners, then the first will be given a clone of
            // the event. The last one will then be given ownership of the event.
            if idx == listener_count {
                tracing::trace!("moving event to send to listener");

                event.take().unwrap()
            } else {
                tracing::trace!("cloning event to send to listener");

                event.clone().unwrap()
            }
        })
    }

    /// Emit a JSON payload that hasn't been deserialized yet, but only if at
    /// least one of the listeners wants the event type.
    pub fn json(
        &self,
        op: u8,
        seq: Option<u64>,
        event_type: Option<&str>,
        json: &mut str,
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

        if !self.wants(flag) {
            return Ok(());
        }

        let gateway_event =
            json::parse_gateway_event(op, seq, event_type, json).map_err(|source| {
                EmitJsonError {
                    kind: EmitJsonErrorType::Parsing,
                    source: Some(Box::new(source)),
                }
            })?;
        self.event(Event::from(gateway_event));

        Ok(())
    }

    fn send(&self, event_type: EventTypeFlags, mut f: impl FnMut(usize) -> Event) {
        let listener_count = self.listeners.len();
        let mut idx = 0;

        let span = tracing::trace_span!(
            "beginning to iterate over listeners",
            ?event_type,
            ?listener_count,
        );
        let _span_enter = span.enter();

        self.listeners.all().retain(|id, listener| {
            let span = tracing::trace_span!("sending to listener", %id, ?event_type);
            let _span_enter = span.enter();

            idx += 1;

            if !listener.wants(event_type) {
                tracing::trace!("listener doesn't want event type");

                return !listener.tx.is_closed();
            }

            listener.tx.unbounded_send(f(idx)).is_ok()
        });
    }
}

#[cfg(test)]
mod tests {
    use super::Emitter;
    use crate::{listener::Listeners, Event, EventTypeFlags};

    #[test]
    fn test_bytes_send() {
        let listeners = Listeners::default();
        let mut rx = listeners.add(EventTypeFlags::SHARD_PAYLOAD);
        let emitter = Emitter::new(listeners);
        emitter.bytes(&[1]);
        assert_eq!(1, emitter.listeners.len());

        assert!(matches!(rx.try_next(), Ok(Some(_))));
        assert!(rx.try_next().is_err());
    }

    #[test]
    fn test_event_removes_closed_channels() {
        let listeners = Listeners::default();
        listeners.add(EventTypeFlags::default());
        let emitter = Emitter::new(listeners);
        emitter.event(Event::GatewayReconnect);
        assert!(emitter.listeners.all().is_empty());
    }

    #[test]
    fn test_event_sends_to_rxs() {
        let listeners = Listeners::default();
        let mut rx1 = listeners.add(EventTypeFlags::default());
        let mut rx2 = listeners.add(EventTypeFlags::default());
        let emitter = Emitter::new(listeners);
        emitter.event(Event::GatewayReconnect);
        assert_eq!(2, emitter.listeners.len());

        assert!(matches!(rx1.try_next(), Ok(Some(_))));
        assert!(matches!(rx2.try_next(), Ok(Some(_))));

        // now check that they didn't send the event twice
        assert!(rx1.try_next().is_err());
        assert!(rx2.try_next().is_err());
    }
}
