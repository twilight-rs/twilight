use crate::{listener::Listeners, EventTypeFlags};
use twilight_model::gateway::event::{shard::Payload, Event};

/// Send some bytes to listeners that have subscribed to shard payloads.
///
/// Shard payload events aren't subscribed to by default and must be opted in
/// to. If a listener has subscribed to them, then the input bytes will be
/// cloned. This means that for most users, this will be a cheap check.
#[tracing::instrument(level = "trace")]
pub fn bytes(listeners: &Listeners<Event>, bytes: &[u8]) {
    send_to_listeners(listeners, EventTypeFlags::SHARD_PAYLOAD, |_| {
        Event::ShardPayload(Payload {
            bytes: bytes.to_vec(),
        })
    });
}

/// Send an event to listeners that have subscribed to its event type.
#[tracing::instrument(level = "trace")]
pub fn event(listeners: &Listeners<Event>, event: Event) {
    let listener_count = listeners.len();
    let event_type = EventTypeFlags::from(event.kind());
    let mut event = Some(event);

    send_to_listeners(listeners, event_type, |idx| {
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

fn send_to_listeners(
    listeners: &Listeners<Event>,
    event_type: EventTypeFlags,
    mut f: impl FnMut(usize) -> Event,
) {
    let listener_count = listeners.len();
    let mut idx = 0;

    let span = tracing::trace_span!(
        "beginning to iterate over listeners",
        ?event_type,
        ?listener_count,
    );
    let _span_enter = span.enter();

    listeners.all().retain(|id, listener| {
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

#[cfg(test)]
mod tests {
    use crate::{listener::Listeners, Event, EventTypeFlags};

    #[test]
    fn test_bytes_send() {
        let listeners = Listeners::default();
        let mut rx = listeners.add(EventTypeFlags::SHARD_PAYLOAD);
        super::bytes(&listeners, &[1]);
        assert_eq!(1, listeners.len());

        assert!(matches!(rx.try_next(), Ok(Some(_))));
        assert!(rx.try_next().is_err());
    }

    #[test]
    fn test_event_removes_closed_channels() {
        let listeners = Listeners::default();
        let _ = listeners.add(EventTypeFlags::default());
        super::event(&listeners, Event::GatewayReconnect);
        assert!(listeners.all().is_empty());
    }

    #[test]
    fn test_event_sends_to_rxs() {
        let listeners = Listeners::default();
        let mut rx1 = listeners.add(EventTypeFlags::default());
        let mut rx2 = listeners.add(EventTypeFlags::default());
        super::event(&listeners, Event::GatewayReconnect);
        assert_eq!(2, listeners.len());

        assert!(matches!(rx1.try_next(), Ok(Some(_))));
        assert!(matches!(rx2.try_next(), Ok(Some(_))));

        // now check that they didn't send the event twice
        assert!(rx1.try_next().is_err());
        assert!(rx2.try_next().is_err());
    }
}
