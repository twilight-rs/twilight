//! Events that the cluster emits to event streams.
//!
//! Included is the larger [`Event`] exposed to event streams. It contains
//! variants with all of the possible events that can come in: new channels,
//! heartbeat acknowledgements, "meta" events of when a shard disconnects or
//! connects, etc.
//!
//! Also included is the [`EventType`] bitflags, which can be used to identify
//! the type of an event and to filter events from event streams via
//! [`ClusterBuilder::event_types`].
//!
//! [`EventType`]: twilight_model::gateway::event::EventType
//! [`ClusterBuilder::event_types`]: crate::cluster::ClusterBuilder::event_types

use crate::shard::Events as ShardEvents;
use futures_util::stream::{SelectAll, Stream};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use twilight_model::gateway::event::Event;

/// Stream of events from a [`Cluster`].
///
/// Unlike the shard's [`Events`] stream this does not include the event types
/// that may produce events; refer to the event types of each individual shard
/// for their event types.
///
/// This implements [`futures_util::stream::Stream`].
///
/// # Examples
///
/// Refer to [`Cluster`] for an example of how to use this.
///
/// [`Cluster`]: super::Cluster
/// [`Events`]: crate::shard::Events
#[derive(Debug)]
pub struct Events {
    stream: SelectAll<ShardEventsWithId>,
}

impl Events {
    /// Create a new stream of shards' events.
    pub(super) const fn new(stream: SelectAll<ShardEventsWithId>) -> Self {
        Self { stream }
    }
}

impl Stream for Events {
    type Item = (u64, Event);

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.stream).poll_next(cx)
    }
}

/// Poll a shard's [`Events`] stream, mapping the result to the shard's ID.
///
/// [`Events`]: crate::shard::Events
#[derive(Debug)]
pub struct ShardEventsWithId {
    id: u64,
    stream: ShardEvents,
}

impl ShardEventsWithId {
    /// Create a new stream with shard's ID and event stream.
    pub(super) const fn new(id: u64, stream: ShardEvents) -> Self {
        Self { id, stream }
    }
}

impl Stream for ShardEventsWithId {
    type Item = (u64, Event);

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.stream).poll_next(cx) {
            Poll::Ready(Some(event)) => Poll::Ready(Some((self.id, event))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Events;
    use futures_util::stream::Stream;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(Events: Debug, Send, Stream, Sync);
}
