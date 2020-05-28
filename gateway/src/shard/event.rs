#![allow(clippy::wildcard_imports)]
//! Events that the shard emits to event streams.
//!
//! Included is the larger [`Event`] exposed to event streams. It contains
//! variants with all of the possible events that can come in: new channels,
//! heartbeat acknowledgements, "meta" events of when the shard disconnects or
//! connects, etc.
//!
//! Also included is the [`EventType`] bitflags, which can be used to identify
//! the type of an event and to filter events from event streams via
//! [`Shard::some_events`].
//!
//! [`Event`]: enum.Event.html
//! [`Shard::some_events`]: ../struct.Shard.html#method.some_events

use crate::EventTypeFlags;
use futures::{
    channel::mpsc::UnboundedReceiver,
    stream::{Stream, StreamExt},
};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use twilight_model::gateway::event::Event;

/// A stream of events from a [`Shard`].
///
/// The events of this stream may or may not be filtered. You can check the
/// event types returned by [`Events::event_types`] to see what events can come
/// in through this stream.
///
/// [`Events::event_types`]: #method.event_types
/// [`Shard`]: ../struct.Shard.html
pub struct Events {
    event_types: EventTypeFlags,
    rx: UnboundedReceiver<Event>,
}

impl Events {
    pub(super) fn new(event_types: EventTypeFlags, rx: UnboundedReceiver<Event>) -> Self {
        Self { event_types, rx }
    }

    /// Returns the event types that can be passed to this stream.
    pub fn event_types(&self) -> EventTypeFlags {
        self.event_types
    }
}

impl Stream for Events {
    type Item = Event;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_next_unpin(cx)
    }
}
