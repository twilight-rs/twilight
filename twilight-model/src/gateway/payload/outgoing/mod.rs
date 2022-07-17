//! Payloads that are outgoing to the Discord Gateway API.
//!
//! These are sent by shards to the Discord API. Some payloads are required for
//! the operation of a shard; these are [`Heartbeat`], [`Identify`], and
//! [`Resume`]. Others - such as [`RequestGuildMembers`] - are for users to use
//! in their own operations.
//!
//! Refer to [Discord Docs / Gateway Commands][1] for Discord's documentation
//! about incoming and outgoing events.
//!
//! [1]: https://discord.com/developers/docs/topics/gateway#commands-and-events-gateway-commands

pub mod identify;
pub mod request_guild_members;
pub mod resume;
pub mod update_presence;
pub mod update_voice_state;

mod heartbeat;

pub use self::{
    heartbeat::Heartbeat, identify::Identify, request_guild_members::RequestGuildMembers,
    resume::Resume, update_presence::UpdatePresence, update_voice_state::UpdateVoiceState,
};
