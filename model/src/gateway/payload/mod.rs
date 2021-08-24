//! Payloads for sending commands to and receiving events from the Discord
//! Gateway API.
//!
//! An example of an incoming event might be [`RoleCreate`] while an outgoing
//! event might be [`UpdateVoiceState`].
//!
//! Refer to [Discord Docs / Commands and Events][1] for Discord's documentation
//! about incoming and outgoing events.
//!
//! [`RoleCreate`]: incoming::RoleCreate
//! [`UpdateVoiceState`]: outgoing::UpdateVoiceState
//! [1]: https://discord.com/developers/docs/topics/gateway#commands-and-events

pub mod incoming;
pub mod outgoing;
