//! Various utility futures used by the [`Shard`].
//!
//! These tend to be used to get around lifetime and borrow requirements, but
//! are also sometimes used to simplify logic.
//!
//! [`Shard`]: crate::Shard

use crate::{connection::Connection, CloseFrame, CommandRatelimiter, ConnectionStatus};
use futures_util::{future::FutureExt, stream::Next};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{
    sync::mpsc,
    task::JoinHandle,
    time::{self, Duration, Interval},
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
    Message(Option<Result<TungsteniteMessage, TungsteniteError>>),
    /// Heartbeat must now be sent to Discord.
    SendHeartbeat,
    /// Identify may now be sent to Discord.
    SendIdentify,
    /// Close frame has been received from the user to be relayed over the
    /// Websocket connection.
    UserClose(CloseFrame<'static>),
    /// Message has been received from the user to be relayed over the Websocket
    /// connection.
    UserCommand(String),
}

/// Future to determine the next action when [`Shard::next_message`] is called.
///
/// Polled futures are given a consistent precedence, from first to last polled:
///
/// - [relaying a user's close frame][1] over the Websocket connection;
/// - [sending a heartbeat to Discord][2];
/// - [sending an identify to Discord][3];
/// - [relaying a user's message][4] over the Websocket connection;
/// - [receiving a message][5] from Discord
///
/// **Be sure** to keep documented precedence in sync with variants in
/// [`NextMessageFutureOutput`]!
///
/// [1]: NextMessageFutureOutput::UserClose
/// [2]: NextMessageFutureOutput::SendHeartbeat
/// [3]: NextMessageFutureOutput::SendIdentify
/// [4]: NextMessageFutureOutput::UserCommand
/// [5]: NextMessageFutureOutput::Message
/// [`Shard::next_message`]: crate::Shard::next_message
pub struct NextMessageFuture<'a> {
    /// Receiver of user sent close frames to be relayed over the Websocket
    /// connection.
    close_receiver: &'a mut mpsc::Receiver<CloseFrame<'static>>,
    /// Receiver of user sent commands to be relayed over the Websocket
    /// connection.
    command_receiver: &'a mut mpsc::UnboundedReceiver<String>,
    /// Heartbeat interval, if enadbled.
    heartbeat_interval: Option<&'a mut Interval>,
    /// Identify queue background task handle.
    identify_handle: Option<&'a mut JoinHandle<()>>,
    /// Future resolving when the next Websocket message has been received.
    message_future: Next<'a, Connection>,
    /// Command ratelimiter, if enabled.
    ratelimiter: Option<&'a mut CommandRatelimiter>,
    /// Shard's connection status.
    status: &'a ConnectionStatus,
}

impl<'a> NextMessageFuture<'a> {
    /// Initialize a new series of futures determining the next action to take.
    pub fn new(
        close_receiver: &'a mut mpsc::Receiver<CloseFrame<'static>>,
        command_receiver: &'a mut mpsc::UnboundedReceiver<String>,
        status: &'a ConnectionStatus,
        identify_handle: Option<&'a mut JoinHandle<()>>,
        message_future: Next<'a, Connection>,
        heartbeat_interval: Option<&'a mut Interval>,
        ratelimiter: Option<&'a mut CommandRatelimiter>,
    ) -> Self {
        Self {
            close_receiver,
            command_receiver,
            heartbeat_interval,
            identify_handle,
            message_future,
            ratelimiter,
            status,
        }
    }
}

impl Future for NextMessageFuture<'_> {
    type Output = NextMessageFutureOutput;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut();

        if !(this.status.is_disconnected() || this.status.is_fatally_closed()) {
            if let Poll::Ready(frame) = this.close_receiver.poll_recv(cx) {
                return Poll::Ready(NextMessageFutureOutput::UserClose(
                    frame.expect("shard owns channel"),
                ));
            }
        }

        if this
            .heartbeat_interval
            .as_mut()
            .map_or(false, |heartbeater| heartbeater.poll_tick(cx).is_ready())
        {
            return Poll::Ready(NextMessageFutureOutput::SendHeartbeat);
        }

        let ratelimited = this.ratelimiter.as_mut().map_or(false, |ratelimiter| {
            ratelimiter.poll_available(cx).is_pending()
        });

        // Must poll to register waker.
        if !ratelimited
            && this
                .identify_handle
                .as_mut()
                .map_or(false, |handle| handle.poll_unpin(cx).is_ready())
        {
            return Poll::Ready(NextMessageFutureOutput::SendIdentify);
        }

        if !ratelimited && this.status.is_identified() {
            if let Poll::Ready(message) = this.command_receiver.poll_recv(cx) {
                return Poll::Ready(NextMessageFutureOutput::UserCommand(
                    message.expect("shard owns channel"),
                ));
            }
        }

        if let Poll::Ready(maybe_try_message) = this.message_future.poll_unpin(cx) {
            return Poll::Ready(NextMessageFutureOutput::Message(maybe_try_message));
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
