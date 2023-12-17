//! Sealed Command trait to denote what can be provided to [`Shard::command`].
//!
//! [`Shard::command`]: crate::Shard::command

use twilight_model::gateway::payload::outgoing::{
    RequestGuildMembers, UpdatePresence, UpdateVoiceState,
};

mod private {
    //! Private module to provide a sealed trait depended on by [`Command`],
    //! disallowing consumers from implementing it.
    //!
    //! [`Command`]: super::Command

    use serde::Serialize;
    use twilight_model::gateway::payload::outgoing::{
        RequestGuildMembers, UpdatePresence, UpdateVoiceState,
    };

    /// Sealed trait to prevent users from implementing the Command trait.
    pub trait Sealed: Serialize {}

    impl Sealed for RequestGuildMembers {}
    impl Sealed for UpdatePresence {}
    impl Sealed for UpdateVoiceState {}
}

/// Trait marker denoting what can be provided to [`Shard::command`].
///
/// This trait exists to make it easier to determine what commands can be sent
/// to the Discord Gateway API.
///
/// This is deliberately not implemented for [`Heartbeat`], [`Identify`], and
/// [`Resume`] due to shards automatically sending them as necessary.
///
/// To send an arbitrary command to the Discord Gateway API then [`Shard::send`]
/// may be used.
///
/// [`Heartbeat`]: twilight_model::gateway::payload::outgoing::Heartbeat
/// [`Identify`]: twilight_model::gateway::payload::outgoing::Identify
/// [`Resume`]: twilight_model::gateway::payload::outgoing::Resume
/// [`Shard::command`]: crate::Shard::command
/// [`Shard::send`]: crate::Shard::send
pub trait Command: private::Sealed {}

impl Command for RequestGuildMembers {}
impl Command for UpdatePresence {}
impl Command for UpdateVoiceState {}

#[cfg(test)]
mod tests {
    use super::Command;
    use static_assertions::assert_impl_all;
    use twilight_model::gateway::payload::outgoing::{
        RequestGuildMembers, UpdatePresence, UpdateVoiceState,
    };

    assert_impl_all!(RequestGuildMembers: Command);
    assert_impl_all!(UpdatePresence: Command);
    assert_impl_all!(UpdateVoiceState: Command);
}
