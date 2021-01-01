//! Send raw websocket messages over the websocket.
//!
//! This is mostly equivalent to the underlying websocket library's message, but
//! this intermediary exists to prevent exposing it in the public API. Messages
//! constructed are equivalent to what the underlying library will receive. The
//! input will not be checked and will be passed directly to the underlying
//! websocket library.

use async_tungstenite::tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame as TungsteniteCloseFrame},
    Message as TungsteniteMessage,
};
use std::borrow::Cow;

/// Information about a close message, if any.
///
/// A close frame can be constructed via its `From` implementations.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct CloseFrame<'a> {
    /// Reason for the close.
    pub code: u16,
    /// Textual representation of the reason the connection is being closed.
    pub reason: Cow<'a, str>,
}

/// Construct a close frame from a code and a reason why.
///
/// # Examples
///
/// ```
/// use twilight_gateway::shard::raw_message::CloseFrame;
///
/// let frame = CloseFrame::from((1000, "reason here"));
///
/// assert_eq!(1000, frame.code);
/// assert_eq!("reason here", frame.reason);
/// ```
impl<'a, T: Into<Cow<'a, str>>> From<(u16, T)> for CloseFrame<'a> {
    fn from((code, reason): (u16, T)) -> Self {
        Self {
            code,
            reason: reason.into(),
        }
    }
}

/// Message to send over the connection to the remote.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Message {
    /// Binary websocket message.
    Binary(Vec<u8>),
    /// Close message with an optional frame including information about the
    /// reason for the close.
    Close(Option<CloseFrame<'static>>),
    /// Ping message with a payload.
    ///
    /// The payload must have a length less than 125 bytes.
    Ping(Vec<u8>),
    /// Pong message with a payload.
    ///
    /// The payload must have a length less than 125 bytes.
    Pong(Vec<u8>),
    /// Text websocket message.
    Text(String),
}

impl Message {
    pub(super) fn into_tungstenite(self) -> TungsteniteMessage {
        match self {
            Self::Binary(bytes) => TungsteniteMessage::Binary(bytes),
            Self::Close(close) => {
                TungsteniteMessage::Close(close.map(|close| TungsteniteCloseFrame {
                    code: CloseCode::from(close.code),
                    reason: close.reason,
                }))
            }
            Self::Ping(bytes) => TungsteniteMessage::Ping(bytes),
            Self::Pong(bytes) => TungsteniteMessage::Pong(bytes),
            Self::Text(string) => TungsteniteMessage::Text(string),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CloseFrame, Message};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;

    assert_fields!(CloseFrame<'_>: code, reason);
    assert_impl_all!(
        CloseFrame<'_>:
        Clone,
        Debug,
        Eq,
        From<(u16, &'static str)>,
        From<(u16, String)>,
        PartialEq,
    );
    assert_impl_all!(Message: Clone, Debug, Eq, PartialEq);
}
