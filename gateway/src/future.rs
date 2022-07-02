//! Various utility futures used by the [`Shard`].
//!
//! These tend to be used to get around lifetime and borrow requirements, but
//! are also sometimes used to simplify logic.
//!
//! [`Shard`]: super::Shard

use crate::{message::Message, Connection, Shard};
use futures_util::{future::FutureExt, stream::Next};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};
use tokio::{
    sync::mpsc::UnboundedReceiver,
    time::{self, Sleep},
};
use tokio_tungstenite::tungstenite::{Error as TungsteniteError, Message as TungsteniteMessage};

pub enum NextMessageFutureReturn {
    GatewayMessage(Option<Result<TungsteniteMessage, TungsteniteError>>),
    SendHeartbeat,
    UserChannelMessage(Option<Message>),
}

pub struct NextMessageFuture<'a> {
    channel_receive_future: &'a mut UnboundedReceiver<Message>,
    message_future: Next<'a, Connection>,
    tick_heartbeat_future: TickHeartbeatFuture,
}

impl<'a> NextMessageFuture<'a> {
    pub fn new(
        rx: &'a mut UnboundedReceiver<Message>,
        message_future: Next<'a, Connection>,
        maybe_heartbeat_interval: Option<Duration>,
        maybe_last_sent: Option<Instant>,
    ) -> Self {
        Self {
            channel_receive_future: rx,
            message_future,
            tick_heartbeat_future: TickHeartbeatFuture::new(
                maybe_last_sent,
                maybe_heartbeat_interval,
            ),
        }
    }
}

impl Future for NextMessageFuture<'_> {
    type Output = NextMessageFutureReturn;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut();

        if let Poll::Ready(()) = this.tick_heartbeat_future.poll_unpin(cx) {
            return Poll::Ready(NextMessageFutureReturn::SendHeartbeat);
        }

        if let Poll::Ready(message) = this.channel_receive_future.poll_recv(cx) {
            return Poll::Ready(NextMessageFutureReturn::UserChannelMessage(message));
        }

        if let Poll::Ready(message) = this.message_future.poll_unpin(cx) {
            return Poll::Ready(NextMessageFutureReturn::GatewayMessage(message));
        }

        Poll::Pending
    }
}

#[must_use]
pub struct TickHeartbeatFuture {
    inner: Option<Pin<Box<Sleep>>>,
}

impl TickHeartbeatFuture {
    pub fn new(
        maybe_last_sent: Option<Instant>,
        maybe_heartbeat_interval: Option<Duration>,
    ) -> Self {
        let heartbeat_interval = if let Some(heartbeat_interval) = maybe_heartbeat_interval {
            heartbeat_interval
        } else {
            return Self { inner: None };
        };

        let remaining = if let Some(last_sent) = maybe_last_sent {
            let time_since = last_sent.elapsed();

            heartbeat_interval.saturating_sub(time_since)
        } else {
            Duration::ZERO
        };

        Self {
            inner: Some(Box::pin(time::sleep(remaining))),
        }
    }
}

impl Future for TickHeartbeatFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(inner) = self.inner.as_mut() {
            return inner.as_mut().poll(cx);
        }

        Poll::Pending
    }
}
