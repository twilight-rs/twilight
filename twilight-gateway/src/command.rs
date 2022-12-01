//! Sealed Command trait to denote what can be provided to [`Shard::command`].
//!
//! [`Shard::command`]: crate::Shard::command

use crate::{
    error::{SendError, SendErrorType},
    json,
};
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

impl Command for RequestGuildMembers {}
impl Command for UpdatePresence {}
impl Command for UpdateVoiceState {}

/// Prepare a command for sending by serializing it.
///
/// # Errors
///
/// Returns a [`SendErrorType::Serializing`] error type if the provided value
/// failed to serialize into JSON.
pub fn prepare(command: &impl private::Sealed) -> Result<String, SendError> {
    json::to_string(command).map_err(|source| SendError {
        source: Some(Box::new(source)),
        kind: SendErrorType::Serializing,
    })
}

#[cfg(test)]
mod tests {
    use super::Command;
    use crate::json;
    use static_assertions::assert_impl_all;
    use twilight_model::gateway::payload::outgoing::{
        Heartbeat, RequestGuildMembers, UpdatePresence, UpdateVoiceState,
    };

    assert_impl_all!(RequestGuildMembers: Command);
    assert_impl_all!(UpdatePresence: Command);
    assert_impl_all!(UpdateVoiceState: Command);

    #[test]
    fn prepare() {
        let heartbeat = Heartbeat::new(Some(30_000));
        let string = json::to_string(&heartbeat).unwrap();
        let message = super::prepare(&heartbeat).unwrap();

        assert_eq!(message, string);
    }
}
