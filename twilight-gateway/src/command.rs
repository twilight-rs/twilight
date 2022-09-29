//! Sealed Command trait to denote what can be provided to [`Shard::command`].
//!
//! [`Shard::command`]: crate::Shard::command

use crate::{json, message::Message};
use twilight_model::gateway::payload::outgoing::{
    identify::Identify, resume::Resume, Heartbeat, RequestGuildMembers, UpdatePresence,
    UpdateVoiceState,
};

mod private {
    //! Private module to provide a sealed trait depended on by [`Command`],
    //! disallowing consumers from implementing it.
    //!
    //! [`Command`]: super::Command

    use serde::Serialize;
    use twilight_model::gateway::payload::outgoing::{
        identify::Identify, resume::Resume, Heartbeat, RequestGuildMembers, UpdatePresence,
        UpdateVoiceState,
    };

    /// Sealed trait to prevent users from implementing the Command trait.
    pub trait Sealed: Serialize {}

    impl Sealed for Heartbeat {}
    impl Sealed for Identify {}
    impl Sealed for RequestGuildMembers {}
    impl Sealed for Resume {}
    impl Sealed for UpdatePresence {}
    impl Sealed for UpdateVoiceState {}
}

/// Trait marker to denote what can be provided to [`Shard::command`].
///
/// This trait exists to make it easier to determine what commands can be sent
/// to the Discord Gateway API.
///
/// To send an arbitrary command to the Discord Gateway API then [`Shard::send`]
/// may be used.
///
/// [`Shard::command`]: crate::Shard::command
/// [`Shard::send`]: crate::Shard::send
pub trait Command: private::Sealed {}

impl Command for Heartbeat {}
impl Command for Identify {}
impl Command for RequestGuildMembers {}
impl Command for Resume {}
impl Command for UpdatePresence {}
impl Command for UpdateVoiceState {}

/// Prepare a command for sending by serializing it and creating a message.
pub fn prepare(command: &impl Command) -> Message {
    json::to_vec(command)
        .map(Message::Binary)
        .expect("command's serialize impl never errors")
}

#[cfg(test)]
mod tests {
    use super::Command;
    use crate::message::Message;
    use static_assertions::assert_impl_all;
    use twilight_model::gateway::payload::outgoing::{
        identify::Identify, resume::Resume, Heartbeat, RequestGuildMembers, UpdatePresence,
        UpdateVoiceState,
    };

    assert_impl_all!(Heartbeat: Command);
    assert_impl_all!(Identify: Command);
    assert_impl_all!(RequestGuildMembers: Command);
    assert_impl_all!(Resume: Command);
    assert_impl_all!(UpdatePresence: Command);
    assert_impl_all!(UpdateVoiceState: Command);

    #[test]
    fn prepare() {
        let heartbeat = Heartbeat::new(30_000);
        let bytes = serde_json::to_vec(&heartbeat).unwrap();
        let message = super::prepare(&heartbeat);

        assert_eq!(message, Message::Binary(bytes));
    }
}
