//! Raw Websocket frame.
//!
//! This is mostly equivalent to the underlying websocket library's message, but
//! this intermediary exists to prevent exposing it in the public API. Messages
//! constructed are equivalent to what the underlying library will receive. The
//! input will not be checked and will be passed directly to the underlying
//! websocket library.

use std::borrow::Cow;

/// Information about a close message.
///
/// A close frame can be constructed via [`CloseFrame::new`]. A default close
/// frame for causing a [full session disconnect] and for
/// [causing a session resume] are provided.
///
/// [causing a session resume]: CloseFrame::RESUME
/// [full session disconnect]: CloseFrame::NORMAL
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct CloseFrame<'a> {
    /// Reason for the close.
    pub code: u16,
    /// Textual representation of the reason the connection is being closed.
    #[cfg_attr(feature = "rkyv", with(rkyv::with::AsOwned))]
    pub reason: Cow<'a, str>,
}

impl<'a> CloseFrame<'a> {
    /// Normal close code indicating the shard will not be reconnecting soon.
    ///
    /// This frame will cause Discord to invalidate your session. If you intend
    /// to resume your session soon, use [`RESUME`].
    ///
    /// [`RESUME`]: Self::RESUME
    pub const NORMAL: Self = Self::new(1000, "closing connection");

    /// Close code indicating the shard will be reconnecting soon.
    ///
    /// This frame will cause Discord to keep your session alive. If you
    /// **don't** intend to resume your session soon, use [`NORMAL`].
    ///
    /// [`NORMAL`]: Self::NORMAL
    pub const RESUME: Self = Self::new(4000, "resuming connection");

    /// Construct a close frame from a code and a reason why.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::gateway::CloseFrame;
    ///
    /// let frame = CloseFrame::new(1000, "reason here");
    ///
    /// assert_eq!(1000, frame.code);
    /// assert_eq!("reason here", frame.reason);
    /// ```
    pub const fn new(code: u16, reason: &'a str) -> Self {
        Self {
            code,
            reason: Cow::Borrowed(reason),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CloseFrame;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(
        CloseFrame<'_>:
        Clone,
        Debug,
        Eq,
        PartialEq,
    );
}
