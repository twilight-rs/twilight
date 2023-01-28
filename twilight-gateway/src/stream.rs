//! Utilities for managing collections of shards.
//!
//! # Concurrency
//!
//! Multiple shards' events or websocket messages may be concurrently streamed
//! via [`ShardEventStream`] or [`ShardMessageStream`] respectively, returning a
//! mutable reference to the yielded shard and its item. The yielded shard is
//! later returned to the stream in its [`Drop`] implementation. The streams are
//! currently implemented via the `futures::stream::FuturesUnordered` type.
//!
//! This is the recommended way to run multiple shards.
//!
//! # Parallelism
//!
//! A multi-threaded executor is able to run tasks in parallel, but splitting
//! shards across tasks requires channels to communicate between them, for
//! example, to coordinate shutdowns. It is therefore **not** recommended unless
//! required for performance reasons (a single core should, on a reasonably
//! performant CPU, be capable of handling tens of thousands of Discord events
//! per second).
//!
//! See the [gateway-parallel] example for how to implement this.
//!
//! [gateway-parallel]: https://github.com/twilight-rs/twilight/blob/main/examples/gateway-parallel.rs

use crate::{error::ReceiveMessageError, message::Message, Shard};
use futures_util::{
    future::BoxFuture,
    stream::{FuturesUnordered, Stream, StreamExt},
};
use std::{
    ops::{Deref, DerefMut},
    pin::Pin,
    sync::mpsc,
    task::{Context, Poll},
};
use twilight_model::gateway::event::Event;

/// Generic list of unordered futures producing an item for each shard.
type FutureList<'a, Item> = FuturesUnordered<BoxFuture<'a, NextItemOutput<'a, Item>>>;

/// Stream selecting the next gateway event from a collection of shards.
///
/// See the crate root documentation for examples.
pub struct ShardEventStream<'a> {
    /// Set of futures resolving to the next event of each shard.
    futures: FutureList<'a, Event>,
    /// Sender to include in [`ShardRef`].
    sender: mpsc::Sender<&'a mut Shard>,
    /// Receiver to re-insert shards into to the stream.
    receiver: mpsc::Receiver<&'a mut Shard>,
}

impl<'a> ShardEventStream<'a> {
    /// Create a new stream producing events from a set of shards.
    pub fn new(shards: impl Iterator<Item = &'a mut Shard>) -> Self {
        let (sender, receiver) = mpsc::channel();
        let mut this = Self {
            futures: FuturesUnordered::new(),
            sender,
            receiver,
        };

        for shard in shards {
            this.add_shard(shard);
        }

        this
    }

    /// Add a shard to the stream to produce a gateway event.
    fn add_shard(&mut self, shard: &'a mut Shard) {
        self.futures.push(Box::pin(async {
            let result = shard.next_event().await;

            NextItemOutput { result, shard }
        }));
    }
}

impl<'a> Stream for ShardEventStream<'a> {
    type Item = (ShardRef<'a>, Result<Event, ReceiveMessageError>);

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        while let Some(shard) = self.receiver.try_iter().next() {
            self.add_shard(shard);
        }

        match self.futures.poll_next_unpin(cx) {
            Poll::Ready(Some(output)) => Poll::Ready(Some((
                ShardRef {
                    channel: self.sender.clone(),
                    shard: Some(output.shard),
                },
                output.result,
            ))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Stream selecting the next websocket message from a collection of shards.
///
/// See the crate root documentation for examples.
pub struct ShardMessageStream<'a> {
    /// Set of futures resolving to the next message of each shard.
    futures: FutureList<'a, Message>,
    /// Sender to include in [`ShardRef`].
    sender: mpsc::Sender<&'a mut Shard>,
    /// Receiver to re-insert shards into the stream.
    receiver: mpsc::Receiver<&'a mut Shard>,
}

impl<'a> ShardMessageStream<'a> {
    /// Create a new stream producing websocket messages from a set of shards.
    pub fn new(shards: impl Iterator<Item = &'a mut Shard>) -> Self {
        let (sender, receiver) = mpsc::channel();
        let mut this = Self {
            futures: FuturesUnordered::new(),
            sender,
            receiver,
        };

        for shard in shards {
            this.add_shard(shard);
        }

        this
    }

    /// Add a shard to the stream to produce a websocket message.
    fn add_shard(&mut self, shard: &'a mut Shard) {
        self.futures.push(Box::pin(async {
            let result = shard.next_message().await;

            NextItemOutput { result, shard }
        }));
    }
}

impl<'a> Stream for ShardMessageStream<'a> {
    type Item = (ShardRef<'a>, Result<Message, ReceiveMessageError>);

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        while let Some(shard) = self.receiver.try_iter().next() {
            self.add_shard(shard);
        }

        match self.futures.poll_next_unpin(cx) {
            Poll::Ready(Some(output)) => Poll::Ready(Some((
                ShardRef {
                    channel: self.sender.clone(),
                    shard: Some(output.shard),
                },
                output.result,
            ))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Guard dereferencing to the shard that produced an event or message.
///
/// Note that manually causing the destructor to [not be called] will cause the
/// shard to not be re-inserted into the stream.
///
/// [not be called]: std::mem::forget
pub struct ShardRef<'a> {
    /// Sender pointing back to the parent stream.
    channel: mpsc::Sender<&'a mut Shard>,
    /// Mutable reference to the shard that produced an event or message.
    shard: Option<&'a mut Shard>,
}

impl Deref for ShardRef<'_> {
    type Target = Shard;

    fn deref(&self) -> &Self::Target {
        self.shard.as_ref().unwrap()
    }
}

impl DerefMut for ShardRef<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.shard.as_mut().unwrap()
    }
}

impl Drop for ShardRef<'_> {
    fn drop(&mut self) {
        if let Some(shard) = self.shard.take() {
            _ = self.channel.send(shard);
        }
    }
}

/// Output of a stream, such as [`ShardMessageStream`].
struct NextItemOutput<'a, Item> {
    /// Result of the future.
    result: Result<Item, ReceiveMessageError>,
    /// Shard that produced the result.
    shard: &'a mut Shard,
}

#[cfg(test)]
mod tests {
    use super::{ShardEventStream, ShardMessageStream, ShardRef};
    use futures_util::Stream;
    use static_assertions::assert_impl_all;
    use std::ops::{Deref, DerefMut};

    assert_impl_all!(ShardEventStream<'_>: Send, Stream, Unpin);
    assert_impl_all!(ShardMessageStream<'_>: Send, Stream, Unpin);
    assert_impl_all!(ShardRef<'_>: Deref, DerefMut, Send);
}
