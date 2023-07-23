//! Channel for users to send messages when calling [`Shard::send`] isn't
//! possible.
//!
//! [`Shard::send`]: crate::Shard::send

use crate::{
    command::{self, Command},
    error::{SendError, SendErrorType},
    CloseFrame,
};
use tokio::sync::mpsc::{self, error::TrySendError};

/// Channel between a user and shard for sending outgoing gateway messages.
#[derive(Debug)]
pub struct MessageChannel {
    /// Receiving half for shards to receive users' close frames.
    pub close_rx: mpsc::Receiver<CloseFrame<'static>>,
    /// Sending half for users to send close frames via shards.
    close_tx: mpsc::Sender<CloseFrame<'static>>,
    /// Receiving half for shards to receive users' commands.
    pub command_rx: mpsc::UnboundedReceiver<String>,
    /// Sending half for users to send commands via shards.
    command_tx: mpsc::UnboundedSender<String>,
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

    /// Clone of the queues.
    pub fn message_queue(&self) -> MessageQueue {
        MessageQueue {
            close: self.close_tx.clone(),
            command: self.command_tx.clone(),
        }
    }

    /// Clone of the senders.
    #[deprecated(since = "0.15.3", note = "renamed to `message_queue()`, use that instead")]
    pub fn sender(&self) -> MessageQueue {
        MessageQueue {
            close: self.close_tx.clone(),
            command: self.command_tx.clone(),
        }
    }
}

/// Channel to send messages over a [`Shard`] to the Discord gateway.
///
/// Unlike the methods on [`Shard`], messages queued up through this are
/// conditionally sent when not ratelimited and identified (except for close
/// frames which are sent as long as the shard is connected to the Websocket).
///
/// [`Shard`]: crate::Shard
#[derive(Clone, Debug)]
pub struct MessageQueue {
    /// Sending half of the close channel.
    close: mpsc::Sender<CloseFrame<'static>>,
    /// Sending half of the command channel.
    command: mpsc::UnboundedSender<String>,
}

impl MessageQueue {
    /// Whether the channel is closed.
    ///
    /// The channel will only be closed if the associated shard has been
    /// dropped.
    pub fn is_closed(&self) -> bool {
        self.command.is_closed()
    }

    /// Send a command to the associated shard.
    ///
    /// # Errors
    ///
    /// Returns a [`SendErrorType::Sending`] error type if the channel is
    /// closed.
    ///
    /// Returns a [`SendErrorType::Serializing`] error type if the provided
    /// command failed to serialize.
    pub fn command(&self, command: &impl Command) -> Result<(), SendError> {
        let json = command::prepare(command)?;

        self.send(json)
    }

    /// Send a JSON encoded gateway event to the associated shard.
    ///
    /// # Errors
    ///
    /// Returns a [`SendErrorType::Sending`] error type if the channel is
    /// closed.
    pub fn send(&self, json: String) -> Result<(), SendError> {
        self.command.send(json).map_err(|_| SendError {
            kind: SendErrorType::Sending,
            source: None,
        })
    }

    /// Send a Websocket close frame to the associated shard.
    ///
    /// Subsequent calls may be queued up to be sent once the shard's
    /// reestablished a Websocket connection or ignored if the queue is full.
    /// The internal queue capacity is currently `1`.
    ///
    /// See the [`Shard::close`] docs for further information.
    ///
    /// # Errors
    ///
    /// Returns a [`SendErrorType::Sending`] error type if the channel is
    /// closed.
    ///
    /// [`Shard::close`]: crate::Shard::close
    pub fn close(&self, close_frame: CloseFrame<'static>) -> Result<(), SendError> {
        match self.close.try_send(close_frame) {
            Ok(()) | Err(TrySendError::Full(_)) => Ok(()),
            _ => Err(SendError {
                kind: SendErrorType::Sending,
                source: None,
            }),
        }
    }
}

/// Channel to send messages over a [`Shard`] to the Discord gateway.
///
/// Unlike the methods on [`Shard`], messages queued up through this are
/// conditionally sent when not ratelimited and identified (except for close
/// frames which are sent as long as the shard is connected to the Websocket).
///
/// [`Shard`]: crate::Shard
#[deprecated(since = "0.15.3", note = "renamed to `MessageQueue`, use that instead")]
pub type MessageSender = MessageQueue;

#[cfg(test)]
mod tests {
    use super::{MessageChannel, MessageQueue};
    use crate::json;
    use static_assertions::assert_impl_all;
    use std::{error::Error, fmt::Debug};
    use twilight_model::{
        gateway::{
            payload::outgoing::{Heartbeat, RequestGuildMembers},
            CloseFrame,
        },
        id::Id,
    };

    assert_impl_all!(MessageChannel: Debug, Send, Sync);
    assert_impl_all!(MessageQueue: Clone, Debug, Send, Sync);

    #[test]
    fn channel_sending() -> Result<(), Box<dyn Error>> {
        let mut channel = MessageChannel::new();
        let sender = channel.sender();
        assert!(channel.command_rx.try_recv().is_err());
        assert!(channel.close_rx.try_recv().is_err());

        let frame = CloseFrame::NORMAL;
        let request = RequestGuildMembers::builder(Id::new(1)).query("", None);
        let heartbeat = Heartbeat::new(Some(30_000));
        let heartbeat_string = json::to_string(&heartbeat)?;
        assert!(sender.command(&request).is_ok());
        assert!(sender.send(heartbeat_string.clone()).is_ok());
        assert!(sender.close(frame.clone()).is_ok());
        assert!(sender.close(frame.clone()).is_ok());

        assert_eq!(request, json::from_str(&channel.command_rx.try_recv()?)?);
        assert_eq!(heartbeat_string, channel.command_rx.try_recv()?);
        assert_eq!(frame, channel.close_rx.try_recv()?);
        assert!(channel.close_rx.try_recv().is_err());

        assert!(!sender.is_closed());
        drop(channel);
        assert!(sender.is_closed());

        assert!(sender.command(&request).is_err());
        assert!(sender.send(heartbeat_string).is_err());
        assert!(sender.close(frame).is_err());

        Ok(())
    }
}
