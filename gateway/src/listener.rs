use crate::EventTypeFlags;
use dashmap::DashMap;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

#[derive(Debug)]
pub struct Listener<T> {
    pub events: EventTypeFlags,
    pub tx: UnboundedSender<T>,
}

impl<T> Listener<T> {
    /// Return whether the listener wants (contains) an event type.
    pub fn wants(&self, event_type: EventTypeFlags) -> bool {
        self.events.contains(event_type)
    }
}

#[derive(Debug)]
struct ListenersRef<T> {
    // Bitflags of the event types that all listeners combined want.
    //
    // If listener 1 wants message creates and listener 2 wants message deletes,
    // then this will contain the bits of both.
    event_types: AtomicU64,
    id: AtomicU64,
    listeners: DashMap<u64, Listener<T>>,
}

impl<T> Default for ListenersRef<T> {
    fn default() -> Self {
        Self {
            event_types: AtomicU64::new(0),
            id: AtomicU64::new(0),
            listeners: DashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Listeners<T>(Arc<ListenersRef<T>>);

impl<T> Listeners<T> {
    pub fn add(&self, events: EventTypeFlags) -> UnboundedReceiver<T> {
        let id = self.0.id.fetch_add(1, Ordering::Release) + 1;
        let (tx, rx) = mpsc::unbounded_channel();

        self.0.listeners.insert(id, Listener { events, tx });
        self.recalculate_event_types();

        rx
    }

    pub fn all(&self) -> &DashMap<u64, Listener<T>> {
        &self.0.listeners
    }

    /// Return all of the event types that are being requested by listeners.
    ///
    /// If listener 1 has requested message creates and listener 2 has requested
    /// message deletes, then this returns bitflags with both flipped on.
    pub fn event_types(&self) -> EventTypeFlags {
        let bits = self.0.event_types.load(Ordering::SeqCst);

        EventTypeFlags::from_bits_truncate(bits)
    }

    /// Return the length of the listeners map.
    pub fn len(&self) -> usize {
        self.0.listeners.len()
    }

    pub fn remove_all(&self) {
        self.0.listeners.clear();
        self.recalculate_event_types();
    }

    fn recalculate_event_types(&self) {
        let flags = self
            .0
            .listeners
            .iter()
            .fold(EventTypeFlags::empty(), |mut acc, r| {
                acc.insert(r.events);

                acc
            });

        self.0.event_types.store(flags.bits(), Ordering::SeqCst);
    }
}

impl<T> Default for Listeners<T> {
    fn default() -> Self {
        Self(Arc::new(ListenersRef::default()))
    }
}

#[cfg(test)]
mod tests {
    use super::{EventTypeFlags, Listener, Listeners};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(Listeners<()>: Clone, Debug, Default, Send, Sync);
    assert_impl_all!(Listener<()>: Debug, Send, Sync);

    #[test]
    fn test_total_event_types() {
        let listeners: Listeners<()> = Listeners::default();
        listeners.add(EventTypeFlags::MESSAGE_CREATE);
        assert_eq!(EventTypeFlags::MESSAGE_CREATE, listeners.event_types());
        listeners.add(EventTypeFlags::MESSAGE_DELETE);
        assert_eq!(
            EventTypeFlags::MESSAGE_CREATE | EventTypeFlags::MESSAGE_DELETE,
            listeners.event_types(),
        );
        listeners.remove_all();
        assert!(listeners.event_types().is_empty());
    }
}
