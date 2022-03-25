use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    pub struct ApplicationFlags: u64 {
        /// Intent required for bots in 100 guilds or more to receive
        /// [`PresenceUpdate`] events.
        ///
        /// [`PresenceUpdate`]: crate::gateway::payload::incoming::PresenceUpdate
        const GATEWAY_PRESENCE = 1 << 12;
        /// Intent required for bots in less than 100 guilds to receive
        /// [`PresenceUpdate`] events.
        ///
        /// [`PresenceUpdate`]: crate::gateway::payload::incoming::PresenceUpdate
        const GATEWAY_PRESENCE_LIMITED = 1 << 13;
        /// Intent required for bots in 100 guilds or more to receive
        /// member-related events like [`MemberAdd`].
        ///
        /// [`MemberAdd`]: crate::gateway::payload::incoming::MemberAdd
        const GATEWAY_GUILD_MEMBERS = 1 << 14;
        /// Intent required for bots in less than 100 guilds to receive
        /// member-related events like [`MemberAdd`].
        ///
        /// [`MemberAdd`]: crate::gateway::payload::incoming::MemberAdd
        const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;
        /// Indicates unusual growth of an app that prevents verification.
        const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;
        /// Indicates if an app is embedded within the Discord client.
        const EMBEDDED = 1 << 17;
        /// Intent required for bots in 100 guilds or more to receive
        /// [message content].
        ///
        /// [message content]: https://support-dev.discord.com/hc/en-us/articles/4404772028055
        const GATEWAY_MESSAGE_CONTENT = 1 << 18;
        /// Intent required for bots in less than 100 guilds to receive
        /// [message content].
        ///
        /// [message content]: https://support-dev.discord.com/hc/en-us/articles/4404772028055
        const GATEWAY_MESSAGE_CONTENT_LIMITED = 1 << 19;
    }
}

impl<'de> Deserialize<'de> for ApplicationFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for ApplicationFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}
