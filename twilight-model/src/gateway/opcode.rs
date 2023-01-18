use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter, Result as FmtResult};

/// Gateway event opcodes.
///
/// The documentation is written from a client's perspective.
///
/// [`PRESENCE_UPDATE`], [`REQUEST_GUILD_MEMBERS`], and [`VOICE_STATE_UPDATE`] are
/// not requiried for establishing or maintaining a gateway connection.
///
/// [`PRESENCE_UPDATE`]: Self::PRESENCE_UPDATE
/// [`REQUEST_GUILD_MEMBERS`]: Self::REQUEST_GUILD_MEMBERS
/// [`VOICE_STATE_UPDATE`]: Self::VOICE_STATE_UPDATE
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct OpCode(u8);

impl OpCode {
    /// [`DispatchEvent`] and sequence number.
    ///
    /// Will only be received after establishing or resuming a session.
    ///
    /// [`DispatchEvent`]: super::event::DispatchEvent
    pub const DISPATCH: Self = Self::new(0);

    /// Periodically sent to maintain the connection and may be received to
    /// immediately request one.
    pub const HEARTBEAT: Self = Self::new(1);

    /// Start a new session.
    pub const IDENTIFY: Self = Self::new(2);

    /// Request to update the client's presence.
    pub const PRESENCE_UPDATE: Self = Self::new(3);

    /// Request to join, leave or move between voice channels.
    pub const VOICE_STATE_UPDATE: Self = Self::new(4);

    /// Resume a previously disconnected session, skipping over [`IDENTIFY`].
    ///
    /// [`IDENTIFY`]: Self::IDENTIFY
    pub const RESUME: Self = Self::new(6);

    /// Indicates that a reconnect is required.
    pub const RECONNECT: Self = Self::new(7);

    /// Request a list of members for a guild.
    pub const REQUEST_GUILD_MEMBERS: Self = Self::new(8);

    /// Received when the session is invalidated.
    pub const INVALID_SESSION: Self = Self::new(9);

    /// Received after connecting, contains the heartbeat interval.
    pub const HELLO: Self = Self::new(10);

    /// Received in response to sending a [`HEARTBEAT`].
    ///
    /// [`HEARTBEAT`]: Self::HEARTBEAT
    pub const HEARTBEAT_ACK: Self = Self::new(11);

    /// Create a new opcode from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`DISPATCH`][`Self::DISPATCH`].
    pub const fn new(opcode: u8) -> Self {
        Self(opcode)
    }

    /// Retrieve the value of the opcode.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::gateway::OpCode;
    ///
    /// assert_eq!(2, OpCode::IDENTIFY.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::DISPATCH => "DISPATCH",
            Self::HEARTBEAT => "HEARTBEAT",
            Self::HEARTBEAT_ACK => "HEARTBEAT_ACK",
            Self::HELLO => "HELLO",
            Self::IDENTIFY => "IDENTIFY",
            Self::INVALID_SESSION => "INVALID_SESSION",
            Self::PRESENCE_UPDATE => "PRESENCE_UPDATE",
            Self::RECONNECT => "RECONNECT",
            Self::REQUEST_GUILD_MEMBERS => "REQUEST_GUILD_MEMBERS",
            Self::RESUME => "RESUME",
            Self::VOICE_STATE_UPDATE => "VOICE_STATE_UPDATE",
            _ => return None,
        })
    }
}

impl Debug for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("OpCode")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("OpCode").field(&self.0).finish()
        }
    }
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::OpCode;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(
        OpCode: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );

    const MAP: &[(OpCode, u8)] = &[
        (OpCode::DISPATCH, 0),
        (OpCode::HEARTBEAT, 1),
        (OpCode::IDENTIFY, 2),
        (OpCode::PRESENCE_UPDATE, 3),
        (OpCode::VOICE_STATE_UPDATE, 4),
        (OpCode::RESUME, 6),
        (OpCode::RECONNECT, 7),
        (OpCode::REQUEST_GUILD_MEMBERS, 8),
        (OpCode::INVALID_SESSION, 9),
        (OpCode::HELLO, 10),
        (OpCode::HEARTBEAT_ACK, 11),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[Token::NewtypeStruct { name: "OpCode" }, Token::U8(*num)],
            );
            assert_eq!(*kind, OpCode::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
