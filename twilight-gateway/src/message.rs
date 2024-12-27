//! Raw websocket message.
//!
//! This is mostly equivalent to the underlying websocket library's message, but
//! this intermediary exists to prevent exposing it in the public API. Messages
//! constructed are equivalent to what the underlying library will receive. The
//! input will not be checked and will be passed directly to the underlying
//! websocket library.

use std::borrow::Cow;

use tokio_websockets::{CloseCode, Message as WebsocketMessage};
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
    /// Close message indicating the connection was closed abnormally.
    pub(crate) const ABNORMAL_CLOSE: Self = Self::Close(Some(CloseFrame::new(1006, "")));

    /// Whether the message is a close message.
    pub const fn is_close(&self) -> bool {
        matches!(self, Self::Close(_))
    }

    /// Whether the message is a text message.
    pub const fn is_text(&self) -> bool {
        matches!(self, Self::Text(_))
    }

    /// Convert a `tokio-websockets` websocket message into a `twilight` websocket
    /// message.
    pub(crate) fn from_websocket_msg(msg: &WebsocketMessage) -> Option<Self> {
        if msg.is_close() {
            let (code, reason) = msg.as_close().unwrap();

            let frame = (code != CloseCode::NO_STATUS_RECEIVED).then(|| CloseFrame {
                code: code.into(),
                reason: Cow::Owned(reason.to_string()),
            });

            Some(Self::Close(frame))
        } else if msg.is_text() {
            Some(Self::Text(msg.as_text().unwrap().to_owned()))
        } else {
            None
        }
    }

    /// Convert a `twilight` websocket message into a `tokio-websockets` websocket
    /// message.
    pub(crate) fn into_websocket_msg(self) -> WebsocketMessage {
        match self {
            Self::Close(frame) => WebsocketMessage::close(
                frame
                    .as_ref()
                    .and_then(|f| CloseCode::try_from(f.code).ok()),
                frame.map(|f| f.reason).as_deref().unwrap_or_default(),
            ),
            Self::Text(string) => WebsocketMessage::text(string),
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
