//! Channel for users to send messages when calling [`Shard::send`] isn't
//! possible.
//!
//! [`Shard::send`]: crate::Shard::send

use crate::{command::Command, json, CloseFrame};
use tokio::sync::mpsc;

/// Channel between a user and shard for sending outgoing gateway messages.
#[derive(Debug)]
pub struct MessageChannel {
    /// Receiving half for shards to receive users' close frames.
    pub close_rx: mpsc::Receiver<CloseFrame<'static>>,
    /// Sending half for users to send close frames via shards.
    pub close_tx: mpsc::Sender<CloseFrame<'static>>,
    /// Receiving half for shards to receive users' commands.
    pub command_rx: mpsc::UnboundedReceiver<String>,
    /// Sending half for users to send commands via shards.
    pub command_tx: mpsc::UnboundedSender<String>,
}

impl MessageChannel {
    /// Initialize a new message channel.
    pub fn new() -> Self {
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let (close_tx, close_rx) = mpsc::channel(1);

        Self {
            close_rx,
            close_tx,
            command_rx,
            command_tx,
        }
    }

    /// Clone of the senders.
    pub fn sender(&self) -> MessageSender {
        MessageSender {
            close: self.close_tx.clone(),
            command: self.command_tx.clone(),
        }
    }
}

/// Channel to send messages over a [`Shard`] to the Discord gateway.
///
/// [`Shard`]: crate::Shard
#[derive(Clone, Debug)]
pub struct MessageSender {
    /// Sending half of the close channel.
    close: mpsc::Sender<CloseFrame<'static>>,
    /// Sending half of the command channel.
    command: mpsc::UnboundedSender<String>,
}

impl MessageSender {
    /// Whether the channel is closed.
    ///
    /// The channel will only be closed if the associated shard has been
    /// dropped.
    pub fn is_closed(&self) -> bool {
        self.command.is_closed()
    }

    /// Send a command to the associated shard.
    ///
    /// # Panics
    ///
    /// Panics if the channel is closed.
    #[track_caller]
    pub fn command(&self, command: &impl Command) {
        self.send(json::to_string(command).expect("serialization cannot fail"));
    }

    /// Send a JSON encoded gateway event to the associated shard.
    ///
    /// # Panics
    ///
    /// Panics if the channel is closed.
    #[track_caller]
    pub fn send(&self, json: String) {
        self.command.send(json).expect("channel should be open");
    }

    /// Send a Websocket close frame to the associated shard.
    ///
    /// Subsequent calls may be queued up to be sent once the shard's
    /// reestablished a Websocket connection or ignored if the queue is full.
    /// The internal queue capacity is currently `1`.
    ///
    /// See the [`Shard::close`] docs for further information.
    ///
    /// # Panics
    ///
    /// Panics if the channel is closed.
    ///
    /// [`Shard::close`]: crate::Shard::close
    #[track_caller]
    pub fn close(&self, close_frame: CloseFrame<'static>) {
        if let Err(e @ mpsc::error::TrySendError::Closed(_)) = self.close.try_send(close_frame) {
            panic!("channel should be open: {e:?}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MessageChannel, MessageSender};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(MessageChannel: Debug, Send, Sync);
    assert_impl_all!(MessageSender: Clone, Debug, Send, Sync);
}
