//! Raw websocket message.
//!
//! This is mostly equivalent to the underlying websocket library's message, but
//! this intermediary exists to prevent exposing it in the public API. Messages
//! constructed are equivalent to what the underlying library will receive. The
//! input will not be checked and will be passed directly to the underlying
//! websocket library.

use tokio_tungstenite::tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame as TungsteniteCloseFrame},
    Message as TungsteniteMessage,
};
use twilight_model::gateway::CloseFrame;

/// Message to send over the connection to the remote.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    /// Close message with an optional frame including information about the
    /// reason for the close.
    Close(Option<CloseFrame<'static>>),
    /// Text websocket message.
    ///
    /// Should always be a JSON payload.
    Text(String),
}

impl Message {
    /// Convert a `tungstenite` websocket message into a `twilight` websocket
    /// message.
    pub(crate) fn from_tungstenite(tungstenite: TungsteniteMessage) -> Option<Self> {
        match tungstenite {
            TungsteniteMessage::Close(frame) => Some(Self::Close(frame.map(|frame| CloseFrame {
                code: frame.code.into(),
                reason: frame.reason,
            }))),
            TungsteniteMessage::Text(string) => Some(Self::Text(string)),
            TungsteniteMessage::Binary(_)
            | TungsteniteMessage::Frame(_)
            | TungsteniteMessage::Ping(_)
            | TungsteniteMessage::Pong(_) => None,
        }
    }

    /// Convert a `twilight` websocket message into a `tungstenite` websocket
    /// message.
    pub(crate) fn into_tungstenite(self) -> TungsteniteMessage {
        match self {
            Self::Close(frame) => {
                TungsteniteMessage::Close(frame.map(|frame| TungsteniteCloseFrame {
                    code: CloseCode::from(frame.code),
                    reason: frame.reason,
                }))
            }
            Self::Text(string) => TungsteniteMessage::Text(string),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Message;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(Message: Clone, Debug, Eq, PartialEq);
}
