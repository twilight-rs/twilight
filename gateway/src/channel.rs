//! Channel for users to [messages] across threads when calling [`Shard::send`]
//! is not possible.
//!
//! [messages]: crate::message::Message
//! [`Shard::send`]: crate::Shard::send

use crate::{
    command::{self, Command},
    error::{SendError, SendErrorType},
    message::Message,
};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

/// Message channel between the user and the shard for sending outgoing
/// commands.
#[derive(Debug)]
pub struct MessageChannel {
    /// Receiving half for shards to receive users' messages.
    rx: UnboundedReceiver<Message>,
    /// Sending half for users to send messages via shards.
    tx: UnboundedSender<Message>,
}

impl MessageChannel {
    /// Initialize a new unbounded mpsc channel.
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        Self { rx, tx }
    }

    /// Mutable reference to the receiving half for shards.
    pub fn rx_mut(&mut self) -> &mut UnboundedReceiver<Message> {
        &mut self.rx
    }

    /// Clone of the sending half for users.
    pub fn sender(&self) -> MessageSender {
        MessageSender {
            tx: self.tx.clone(),
        }
    }
}

/// Channel to send Websocket [`Message`]s over a [`Shard`] to the Discord
/// gateway.
///
/// [`Shard`]: crate::Shard
#[derive(Clone, Debug)]
pub struct MessageSender {
    /// Sending half of the channel for the user to send messages to a shard.
    tx: UnboundedSender<Message>,
}

impl MessageSender {
    /// Whether the channel is closed.
    ///
    /// The channel will only be closed if the associated shard has been
    /// dropped.
    pub fn is_closed(&self) -> bool {
        self.tx.is_closed()
    }

    /// Send a command to the associated shard.
    ///
    /// # Errors
    ///
    /// Returns a [`SendErrorType::Sending`] error type if the message could
    /// not be sent over the websocket. This indicates the shard is either
    /// currently restarting or closed and will restart.
    ///
    /// Returns a [`SendErrorType::Serializing`] error type if the provided
    /// command failed to serialize.
    pub fn command(&self, command: &impl Command) -> Result<(), SendError> {
        let message = command::prepare(command)?;

        self.send(message).map_err(|_| SendError {
            kind: SendErrorType::Sending,
            source: None,
        })
    }

    /// Send a raw Websocket message over the shard.
    ///
    /// # Errors
    ///
    /// Returns a [`SendErrorType::Sending`] error type if the message could
    /// not be sent over the websocket. This indicates the shard is either
    /// currently restarting or closed and will restart.
    pub fn send(&self, message: Message) -> Result<(), SendError> {
        self.tx.send(message).map_err(|_| SendError {
            kind: SendErrorType::Sending,
            source: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::message::Message;

    use super::{MessageChannel, MessageSender};
    use static_assertions::assert_impl_all;
    use std::{error::Error, fmt::Debug};
    use twilight_model::{
        gateway::payload::outgoing::{Heartbeat, RequestGuildMembers},
        id::Id,
    };

    assert_impl_all!(MessageChannel: Debug, Send, Sync);
    assert_impl_all!(MessageSender: Clone, Debug, Send, Sync);

    #[test]
    fn channel_sending() -> Result<(), Box<dyn Error>> {
        let mut channel = MessageChannel::new();
        let sender = channel.sender();
        assert!(channel.rx_mut().try_recv().is_err());

        let request = RequestGuildMembers::builder(Id::new(1)).query("", None);
        let heartbeat = Heartbeat::new(30_000);
        let heartbeat_bytes = serde_json::to_vec(&heartbeat)?;
        assert!(sender.command(&request).is_ok());
        assert!(sender
            .send(Message::Binary(heartbeat_bytes.clone()))
            .is_ok());

        match channel.rx_mut().try_recv()? {
            Message::Binary(bytes) => assert_eq!(request, serde_json::from_slice(&bytes)?),
            other => panic!("message isn't binary: {:?}", other),
        }

        match channel.rx_mut().try_recv()? {
            Message::Binary(bytes) => assert_eq!(heartbeat, serde_json::from_slice(&bytes)?),
            other => panic!("message isn't binary: {:?}", other),
        }

        assert!(!sender.is_closed());
        drop(channel);
        assert!(sender.is_closed());

        assert!(sender.command(&heartbeat).is_err());
        assert!(sender.send(Message::Binary(heartbeat_bytes)).is_err());

        Ok(())
    }
}
