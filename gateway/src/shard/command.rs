//! Sealed Command trait to denote what can be provided to [`Shard::command`].
//!
//! [`Shard::command`]: super::Shard::command

use twilight_model::gateway::payload::outgoing::{
    identify::Identify, resume::Resume, Heartbeat, RequestGuildMembers, UpdatePresence,
    UpdateVoiceState,
};

mod private {
    use serde::Serialize;
    use twilight_model::gateway::payload::outgoing::{
        identify::Identify, resume::Resume, Heartbeat, RequestGuildMembers, UpdatePresence,
        UpdateVoiceState,
    };

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
/// [`Shard::command`]: super::Shard::command
/// [`Shard::send`]: super::Shard::send
pub trait Command: private::Sealed {}

impl Command for Heartbeat {}
impl Command for Identify {}
impl Command for RequestGuildMembers {}
impl Command for Resume {}
impl Command for UpdatePresence {}
impl Command for UpdateVoiceState {}

#[cfg(test)]
mod tests {
    use super::Command;
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
}
