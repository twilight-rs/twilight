use crate::{
    listener::{Listener, Listeners},
    EventTypeFlags,
};
#[allow(unused_imports)]
use tracing::{debug, info, trace, warn};
use twilight_model::gateway::event::{shard::Payload, Event};

pub async fn bytes(listeners: Listeners<Event>, bytes: &[u8]) {
    for listener in listeners.all() {
        if listener.events.contains(EventTypeFlags::SHARD_PAYLOAD) {
            let event = Event::ShardPayload(Payload {
                bytes: bytes.to_owned(),
            });

            // If the channel isn't active, this'll be caught by event emissions
            // later.
            let _ = listener.tx.unbounded_send(event);
        }
    }
}

pub fn event(listeners: &Listeners<Event>, event: Event) {
    let listeners = listeners.all();
    let mut remove_listeners = Vec::new();

    // Take up to the last one so that we can later get the last and *move*
    // the event into the listener's channel, rather than clone it like we
    // do here.
    //
    // This avoids a clone, and for users with only 1 listener this will
    // entirely avoid cloning.
    let mut last = None;

    for (idx, guard) in listeners.iter().enumerate() {
        let id = *guard.key();
        let listener = guard.value();
        if idx == listeners.len() - 1 {
            last = Some(*guard.key());

            break;
        }

        let event_type = EventTypeFlags::from(event.kind());

        if !listener.events.contains(event_type) {
            trace!(
                "[ShardProcessor] Listener {} doesn't want event type {:?}",
                id,
                event_type,
            );

            continue;
        }

        if !_emit_to_listener(id, listener, event.clone()) {
            remove_listeners.push(id);
        }
    }

    if let Some(id) = last {
        if let Some(listener) = listeners.get(&id) {
            if !_emit_to_listener(id, listener.value(), event) {
                remove_listeners.push(id);
            }
        }
    }

    for id in &remove_listeners {
        debug!("[ShardProcessor] Removing listener {}", id);

        listeners.remove(id);
    }
}

/// Returns whether the channel is still active.
///
/// If the receiver dropped, return `false` so we know to remove it.
/// These are unbounded channels, so we know it's not because it's full.
fn _emit_to_listener(id: u64, listener: &Listener<Event>, event: Event) -> bool {
    let event_type = EventTypeFlags::from(event.kind());

    if !listener.events.contains(event_type) {
        trace!(
            "[ShardProcessor] Listener {} doesn't want event type {:?}",
            id,
            event_type,
        );

        return true;
    }

    listener.tx.unbounded_send(event).is_ok()
}
