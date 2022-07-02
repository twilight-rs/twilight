//! Send raw websocket messages over the websocket.
//!
//! This is mostly equivalent to the underlying websocket library's message, but
//! this intermediary exists to prevent exposing it in the public API. Messages
//! constructed are equivalent to what the underlying library will receive. The
//! input will not be checked and will be passed directly to the underlying
//! websocket library.

use std::borrow::Cow;
use tokio_tungstenite::tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame as TungsteniteCloseFrame},
    Message as TungsteniteMessage,
};

/// Information about a close message, if any.
///
/// A close frame can be constructed via its `From` implementations.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct CloseFrame<'a> {
    /// Reason for the close.
    code: u16,
    /// Textual representation of the reason the connection is being closed.
    reason: Cow<'a, str>,
}

impl<'a> CloseFrame<'a> {
    /// Normal close code indicating the shard will not be reconnecting soon.
    ///
    /// Sending [`Message::Close`] with this frame will cause Discord to
    /// invalidate your session. If you intend to resume your session soon,
    /// use [`RESUME`].
    ///
    /// [`RESUME`]: Self::RESUME
    pub const NORMAL: Self = Self {
        code: 1000,
        reason: Cow::Borrowed("closing connection"),
    };

    /// Close code indicating the shard will be reconnecting soon.
    ///
    /// Sending [`Message::Close`] with this frame will cause Discord to keep
    /// your session alive. If you **don't** intend to resume your session soon,
    /// use [`NORMAL`].
    ///
    /// [`NORMAL`]: Self::NORMAL
    pub const RESUME: Self = Self {
        code: 4000,
        reason: Cow::Borrowed("resuming connection"),
    };

    /// Convert a `tungstenite` close frame into a `twilight` close frame.
    pub(crate) fn from_tungstenite(tungstenite: TungsteniteCloseFrame<'a>) -> Self {
        Self {
            code: u16::from(tungstenite.code),
            reason: tungstenite.reason,
        }
    }

    /// Convert a `twilight` close frame into a `tungstenite` close frame.
    pub(crate) fn into_tungstenite(self) -> TungsteniteCloseFrame<'a> {
        TungsteniteCloseFrame {
            code: CloseCode::from(self.code),
            reason: self.reason,
        }
    }

    /// Close code of the frame.
    pub const fn code(&self) -> u16 {
        self.code
    }

    /// Reason for the close.
    pub fn reason(&self) -> &str {
        self.reason.as_ref()
    }
}

/// Construct a close frame from a code and a reason why.
///
/// # Examples
///
/// ```
/// use twilight_gateway::message::CloseFrame;
///
/// let frame = CloseFrame::from((1000, "reason here"));
///
/// assert_eq!(1000, frame.code());
/// assert_eq!("reason here", frame.reason());
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
    /// Text websocket message.
    Text(String),
}

impl Message {
    /// Convert a `tungstenite` websocket message into a `twilight` websocket
    /// message.
    pub(crate) fn from_tungstenite(tungstenite: TungsteniteMessage) -> Option<Self> {
        match tungstenite {
            TungsteniteMessage::Binary(bytes) => Some(Self::Binary(bytes)),
            TungsteniteMessage::Close(maybe_close) => {
                let close = maybe_close.map(CloseFrame::from_tungstenite);

                Some(Self::Close(close))
            }
            TungsteniteMessage::Text(string) => Some(Self::Text(string)),
            TungsteniteMessage::Frame(_)
            | TungsteniteMessage::Ping(_)
            | TungsteniteMessage::Pong(_) => None,
        }
    }

    /// Convert a `twilight` websocket message into a `tungstenite` websocket
    /// message.
    pub(crate) fn into_tungstenite(self) -> TungsteniteMessage {
        match self {
            Self::Binary(bytes) => TungsteniteMessage::Binary(bytes),
            Self::Close(close) => {
                TungsteniteMessage::Close(close.map(CloseFrame::into_tungstenite))
            }
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
