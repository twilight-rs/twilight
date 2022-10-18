//! Various utility futures used by the [`Shard`].
//!
//! These tend to be used to get around lifetime and borrow requirements, but
//! are also sometimes used to simplify logic.
//!
//! [`Shard`]: crate::Shard

use crate::{message::Message, ratelimiter::Permit};
use futures_util::{
    future::FutureExt,
    stream::{Next, Stream},
};
use pin_project_lite::pin_project;
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

/// Resolved value from polling a [`NextMessageFuture`].
///
/// **Be sure** to keep variants in sync with documented precedence in
/// [`NextMessageFuture`]!
pub enum NextMessageFutureOutput {
    /// Message has been received from the Websocket connection.
    ///
    /// If no message is present then the stream has ended and a new connection
    /// will need to be made.
    Message(Option<TungsteniteMessage>),
    /// Heartbeat must now be sent to Discord.
    SendHeartbeat,
    /// Message has been received from the user to be relayed over the Websocket
    /// connection.
    UserChannelMessage(Message),
}

pin_project! {
    /// Future to determine the next action when [`Shard::next_message`] is
    /// called.
    ///
    /// Polled futures are given a consistent precedence, from first to last:
    ///
    /// - [sending a heartbeat to Discord][1];
    /// - [relaying a user's message][2] over the Websocket message;
    /// - [receiving a message][3] from Discord
    ///
    /// **Be sure** to keep documented precedence in sync with variants in
    /// [`NextMessageFutureOutput`]!
    ///
    /// [1]: NextMessageFutureOutput::SendHeartbeat
    /// [2]: NextMessageFutureOutput::UserChannelMessage
    /// [3]: NextMessageFutureOutput::Message
    /// [`Shard::next_message`]: crate::Shard::next_message
    pub struct NextMessageFuture<'a, F, St> {
        // Future resolving when the user has sent a message over the channel,
        // to be relayed over the Websocket connection.
        channel_receive_future: &'a mut UnboundedReceiver<Message>,
        // Future resolving when the next Websocket message has been received.
        message_future: Next<'a, St>,
        // Future resolving to a ratelimit permit, if a ratelimiter is enabled.
        #[pin]
        ratelimit_permit: Option<F>,
        // `true` when the `ratelimit_permit` future has completed, indicating
        // it should not be polled again.
        ratelimit_permit_completed: bool,
        // Future resolving when the [`Shard`] must sent a heartbeat.
        //
        // [`Shard`]: crate::Shard
        tick_heartbeat_future: TickHeartbeatFuture,
    }
}

impl<'a, F, St> NextMessageFuture<'a, F, St> {
    /// Initialize a new series of futures determining the next action to take.
    pub fn new(
        rx: &'a mut UnboundedReceiver<Message>,
        message_future: Next<'a, St>,
        ratelimit_permit: Option<F>,
        maybe_heartbeat_interval: Option<Duration>,
        maybe_last_sent: Option<Instant>,
    ) -> Self {
        Self {
            channel_receive_future: rx,
            message_future,
            ratelimit_permit,
            ratelimit_permit_completed: false,
            tick_heartbeat_future: TickHeartbeatFuture::new(
                maybe_heartbeat_interval,
                maybe_last_sent,
            ),
        }
    }
}

impl<'a, F, St> Future for NextMessageFuture<'a, F, St>
where
    F: Future<Output = Permit<'a>>,
    St: Stream<Item = Result<TungsteniteMessage, TungsteniteError>> + Unpin,
{
    type Output = NextMessageFutureOutput;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        if this.tick_heartbeat_future.poll_unpin(cx).is_ready() {
            return Poll::Ready(NextMessageFutureOutput::SendHeartbeat);
        }

        let ratelimited = this
            .ratelimit_permit
            .as_pin_mut()
            .map_or(false, |permit_future| {
                if *this.ratelimit_permit_completed {
                    return false;
                }

                match permit_future.poll(cx) {
                    Poll::Ready(permit) => {
                        permit.forget();
                        *this.ratelimit_permit_completed = true;
                        false
                    }
                    Poll::Pending => true,
                }
            });

        if !ratelimited {
            if let Poll::Ready(maybe_message) = this.channel_receive_future.poll_recv(cx) {
                let message = maybe_message.expect("shard owns channel");

                return Poll::Ready(NextMessageFutureOutput::UserChannelMessage(message));
            }
        }

        if let Poll::Ready(maybe_try_message) = this.message_future.poll_unpin(cx) {
            let maybe_message = maybe_try_message.and_then(Result::ok);

            return Poll::Ready(NextMessageFutureOutput::Message(maybe_message));
        }

        Poll::Pending
    }
}

/// Future that will resolve when the shard must send its next heartbeat.
///
/// The duration of the future is defined by taking the heartbeat interval defined
/// by the [`Ready`] event, and subtracting the duration since the
/// [last heartbeat] was sent. If a [`Ready`] event has not yet been received
/// then the future will never be ready, and if the duration has already passed
/// then it will immediately be ready.
///
/// This future must always take precedence over other actions in order to
/// maintain the [gateway session]!
///
/// [gateway session]: crate::Session
/// [`Ready`]: twilight_model::gateway::payload::incoming::Ready
pub struct TickHeartbeatFuture {
    /// Inner future that will resolve after some time, defined by the type-level
    /// documentation.
    inner: Option<Pin<Box<Sleep>>>,
}

impl TickHeartbeatFuture {
    /// Initialize a new unpolled future that will resolve when the next
    /// heartbeat must be sent.
    fn new(maybe_heartbeat_interval: Option<Duration>, maybe_last_sent: Option<Instant>) -> Self {
        let inner = match (maybe_heartbeat_interval, maybe_last_sent) {
            (Some(heartbeat_interval), Some(last_sent)) => Some(Box::pin(time::sleep(
                heartbeat_interval.saturating_sub(last_sent.elapsed()),
            ))),
            (Some(heartbeat_interval), None) => {
                // First heartbeat should have some jitter, see
                // https://discord.com/developers/docs/topics/gateway#heartbeat-interval
                Some(Box::pin(time::sleep(
                    heartbeat_interval.mul_f64(rand::random()),
                )))
            }
            (None, _) => None,
        };

        Self { inner }
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

/// Future that will resolve when the delay for a reconnect passes.
///
/// The duration of the future is defined by the number of attempts at
/// reconnecting that have already been made. The math behind it is
/// `2 ^ attempts`, maxing out at `MAX_WAIT_SECONDS`.
pub async fn reconnect_delay(reconnect_attempts: u8) {
    /// The maximum wait before resolving, in seconds.
    const MAX_WAIT_SECONDS: u8 = 128;

    let wait = 2_u8
        .saturating_pow(reconnect_attempts.into())
        .min(MAX_WAIT_SECONDS);

    time::sleep(Duration::from_secs(wait.into())).await;
}

#[cfg(test)]
mod tests {
    use super::NextMessageFuture;
    use crate::{message::Message, ratelimiter::CommandRatelimiter};
    use futures::StreamExt;
    use tokio::{
        sync::mpsc,
        time::{self, Duration},
    };

    #[tokio::test]
    async fn ratelimiter_permit_poll_completed_no_panic() {
        let mut ratelimiter = CommandRatelimiter::new(Duration::from_millis(42_500));
        let (tx, mut rx) = mpsc::unbounded_channel();
        let mut stream = futures_util::stream::pending();

        assert_eq!(ratelimiter.available(), ratelimiter.max());
        for _ in 0..ratelimiter.max() - 1 {
            ratelimiter.acquire().await;
        }
        assert_eq!(ratelimiter.available(), 1);

        tokio::spawn(async move {
            // send a message after the `permit_future` has been polled and
            // returned `Poll::Ready`
            time::sleep(Duration::from_millis(50)).await;
            tx.send(Message::Close(None)).unwrap();
        });

        // `CommandRatelimiter::acquire` will be checked two times, assert that
        // this does not actually poll the future two times, causing a panic
        let _future = time::timeout(
            Duration::from_millis(150),
            NextMessageFuture::new(
                &mut rx,
                stream.next(),
                Some(ratelimiter.acquire()),
                None,
                None,
            ),
        )
        .await
        .unwrap();
    }
}
