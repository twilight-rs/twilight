use crate::shard::Event;
use futures::channel::mpsc::UnboundedReceiver;
use futures::stream::Stream;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct Events {
    _rx: UnboundedReceiver<(u64, Event)>,
}

impl Stream for Events {
    /// The ID of the shard and the event that the shard received.
    type Item = (u64, Event);

    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Pending
    }
}
