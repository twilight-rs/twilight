use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    /// Gateway intents.
    ///
    /// Developers must specify intents when connecting to the gateway. The
    /// intents specified correspond with the events received. To specify
    /// multiple intents, create a union using the `|` operator. See [the
    /// discord docs] for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/topics/gateway#gateway-intents
    pub struct Intents: u64 {
        /// Guilds intent.
        ///
        /// Event(s) received:
        ///  - [`GUILD_CREATE`]
        ///  - [`GUILD_UPDATE`]
        ///  - [`GUILD_DELETE`]
        ///  - [`GUILD_ROLE_CREATE`]
        ///  - [`GUILD_ROLE_UPDATE`]
        ///  - [`GUILD_ROLE_DELETE`]
        ///  - [`CHANNEL_CREATE`]
        ///  - [`CHANNEL_UPDATE`]
        ///  - [`CHANNEL_DELETE`]
        ///  - [`CHANNEL_PINS_UPDATE`]
        ///
        /// [`GUILD_CREATE`]: ./event/enum.Event.html#variant.GuildCreate
        /// [`GUILD_UPDATE`]: ./event/enum.Event.html#variant.GuildUpdate
        /// [`GUILD_DELETE`]: ./event/enum.Event.html#variant.GuildDelete
        /// [`GUILD_ROLE_CREATE`]: ./event/enum.Event.html#variant.RoleCreate
        /// [`GUILD_ROLE_UPDATE`]: ./event/enum.Event.html#variant.RoleUpdate
        /// [`GUILD_ROLE_DELETE`]: ./event/enum.Event.html#variant.RoleDelete
        /// [`CHANNEL_CREATE`]: ./event/enum.Event.html#variant.ChannelCreate
        /// [`CHANNEL_UPDATE`]: ./event/enum.Event.html#variant.ChannelUpdate
        /// [`CHANNEL_DELETE`]: ./event/enum.Event.html#variant.ChannelDelete
        /// [`CHANNEL_PINS_UPDATE`]: ./event/enum.Event.html#variant.ChannelPinsUpdate
        const GUILDS = 1;
        /// Guild members intent.
        ///
        /// This intent is privileged. See [`the discord docs`] for more information.
        ///
        /// Event(s) received:
        ///  - [`GUILD_MEMBER_ADD`]
        ///  - [`GUILD_MEMBER_UPDATE`]
        ///  - [`GUILD_MEMBER_REMOVE`]
        ///
        /// [the discord docs]: https://discord.com/developers/docs/topics/gateway#privileged-intents
        /// [`GUILD_MEMBER_ADD`]: ./event/enum.Event.html#variant.MemberAdd
        /// [`GUILD_MEMBER_UPDATE`]: ./event/enum.Event.html#variant.MemberUpdate
        /// [`GUILD_MEMBER_REMOVE`]: ./event/enum.Event.html#variant.MemberRemove
        const GUILD_MEMBERS = 1 << 1;
        /// Guild bans intent.
        ///
        /// Event(s) received:
        ///  - [`GUILD_BAN_ADD`]
        ///  - [`GUILD_BAN_REMOVE`]
        ///
        /// [`GUILD_BAN_ADD`]: ./event/enum.Event.html#variant.BanAdd
        /// [`GUILD_BAN_REMOVE`]: ./event/enum.Event.html#variant.BanRemove
        const GUILD_BANS = 1 << 2;
        /// Guild emojis intent.
        ///
        /// Event(s) received:
        ///  - [`GUILD_EMOJIS_UPDATE`]
        ///
        /// [`GUILD_EMOJIS_UPDATE`]: ./event/enum.Event.html#variant.GuildEmojisUpdate
        const GUILD_EMOJIS = 1 << 3;
        /// Guild integrations intent.
        ///
        /// Event(s) received:
        ///  - [`GUILD_INTEGRATIONS_UPDATE`]
        ///
        /// [`GUILD_INTEGRATIONS_UPDATE`]: ./event/enum.Event.html#variant.GuildIntegrationsUpdate
        const GUILD_INTEGRATIONS = 1 << 4;
        /// Guild webhooks intent.
        ///
        /// Event(s) received:
        ///  - [`WEBHOOKS_UPDATE`]
        ///
        /// [`WEBHOOKS_UPDATE`]: ./event/enum.Event.html#variant.WebhooksUpdate
        const GUILD_WEBHOOKS = 1 << 5;
        /// Guild invites intent.
        ///
        /// Event(s) received:
        ///  - [`INVITE_CREATE`]
        ///  - [`INVITE_DELETE`]
        ///
        /// [`INVITE_CREATE`]: ./event/enum.Event.html#variant.InviteCreate
        /// [`INVITE_DELETE`]: ./event/enum.Event.html#variant.InviteDelete
        const GUILD_INVITES = 1 << 6;
        /// Guild voice states intent.
        ///
        /// Event(s) received:
        ///  - [`VOICE_STATE_UPDATE`]
        ///
        /// [`VOICE_STATE_UPDATE`]: ./event/enum.Event.html#variant.VoiceStateUpdate
        const GUILD_VOICE_STATES = 1 << 7;
        /// Guild presences intent.
        ///
        /// This intent is privileged. See [`the discord docs`] for more information.
        ///
        /// Event(s) received:
        ///  - [`PRESENCE_UPDATE`]
        ///
        /// [the discord docs]: https://discord.com/developers/docs/topics/gateway#privileged-intents
        /// [`PRESENCE_UPDATE`]: ./event/enum.Event.html#variant.PresenceUpdate
        const GUILD_PRESENCES = 1 << 8;
        /// Guild messages intent.
        ///
        /// Event(s) received:
        ///  - [`MESSAGE_CREATE`]
        ///  - [`MESSAGE_UPDATE`]
        ///  - [`MESSAGE_DELETE`]
        ///  - [`MESSAGE_DELETE_BULK`]
        ///
        /// [`MESSAGE_CREATE`]: ./event/enum.Event.html#variant.MessageCreate
        /// [`MESSAGE_UPDATE`]: ./event/enum.Event.html#variant.MessageUpdate
        /// [`MESSAGE_DELETE`]: ./event/enum.Event.html#variant.MessageDelete
        /// [`MESSAGE_DELETE_BULK`]: ./event/enum.Event.html#variant.MessageDeleteBulk
        const GUILD_MESSAGES = 1 << 9;
        /// Guild message reactions intent.
        ///
        /// Event(s) received:
        ///  - [`MESSAGE_REACTION_ADD`]
        ///  - [`MESSAGE_REACTION_REMOVE`]
        ///  - [`MESSAGE_REACTION_REMOVE_ALL`]
        ///  - [`MESSAGE_REACTION_REMOVE_EMOJI`]
        ///
        /// [`MESSAGE_REACTION_ADD`]: ./event/enum.Event.html#variant.ReactionAdd
        /// [`MESSAGE_REACTION_REMOVE`]: ./event/enum.Event.html#variant.ReactionRemove
        /// [`MESSAGE_REACTION_REMOVE_ALL`]: ./event/enum.Event.html#variant.ReactionRemoveAll
        /// [`MESSAGE_REACTION_REMOVE_EMOJI`]: ./event/enum.Event.html#variant.ReactionRemoveEmoji
        const GUILD_MESSAGE_REACTIONS = 1 << 10;
        /// Guild message typing intent.
        ///
        /// Event(s) received:
        ///  - [`TYPING_START`]
        ///
        /// [`TYPING_START`]: ./event/enum.Event.html#variant.TypingStart
        const GUILD_MESSAGE_TYPING = 1 << 11;
        /// Direct messages intent.
        ///
        /// Event(s) received:
        ///  - [`MESSAGE_CREATE`]
        ///  - [`MESSAGE_UPDATE`]
        ///  - [`MESSAGE_DELETE`]
        ///  - [`MESSAGE_DELETE_BULK`]
        ///
        /// This is different from the [`GUILD_MESSAGES`] intent in that the bot
        /// will receive message events from locations other than guilds.
        ///
        /// [`MESSAGE_CREATE`]: ./event/enum.Event.html#variant.MessageCreate
        /// [`MESSAGE_UPDATE`]: ./event/enum.Event.html#variant.MessageUpdate
        /// [`MESSAGE_DELETE`]: ./event/enum.Event.html#variant.MessageDelete
        /// [`MESSAGE_DELETE_BULK`]: ./event/enum.Event.html#variant.MessageDeleteBulk
        /// [`GUILD_MESSAGES`]: #associatedconstant.GUILD_MESSAGES
        const DIRECT_MESSAGES = 1 << 12;
        /// Direct message reactions intent.
        ///
        /// Event(s) received:
        ///  - [`MESSAGE_REACTION_ADD`]
        ///  - [`MESSAGE_REACTION_REMOVE`]
        ///  - [`MESSAGE_REACTION_REMOVE_ALL`]
        ///  - [`MESSAGE_REACTION_REMOVE_EMOJI`]
        ///
        /// This is different from the [`GUILD_MESSAGE_REACTIONS`] event in that
        /// the bot will receive message reaction events from locations other
        /// than guilds.
        ///
        /// [`MESSAGE_REACTION_ADD`]: ./event/enum.Event.html#variant.ReactionAdd
        /// [`MESSAGE_REACTION_REMOVE`]: ./event/enum.Event.html#variant.ReactionRemove
        /// [`MESSAGE_REACTION_REMOVE_ALL`]: ./event/enum.Event.html#variant.ReactionRemoveAll
        /// [`MESSAGE_REACTION_REMOVE_EMOJI`]: ./event/enum.Event.html#variant.ReactionRemoveEmoji
        /// [`GUILD_MESSAGE_REACTIONS`]: #associatedconstant.GUILD_MESSAGE_REACTIONS
        const DIRECT_MESSAGE_REACTIONS = 1 << 13;
        /// Direct message typing intent.
        ///
        /// Event(s) received:
        ///  - [`TYPING_START`]
        ///
        /// This is different from the [`GUILD_MESSAGE_TYPING`] intent in that
        /// the bot will receive typing start events from locations other than
        /// guilds.
        ///
        /// [`TYPING_START`]: ./event/enum.Event.html#variant.TypingStart
        /// [`GUILD_MESSAGE_TYPING`]: #associatedconstant.GUILD_MESSAGE_TYPING
        const DIRECT_MESSAGE_TYPING = 1 << 14;
    }
}

impl<'de> Deserialize<'de> for Intents {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for Intents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

#[cfg(test)]
mod tests {
    use super::Intents;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&Intents::GUILDS, &[Token::U64(1)]);
        serde_test::assert_tokens(&Intents::GUILD_MEMBERS, &[Token::U64(1 << 1)]);
        serde_test::assert_tokens(&Intents::GUILD_BANS, &[Token::U64(1 << 2)]);
        serde_test::assert_tokens(&Intents::GUILD_EMOJIS, &[Token::U64(1 << 3)]);
        serde_test::assert_tokens(&Intents::GUILD_INTEGRATIONS, &[Token::U64(1 << 4)]);
        serde_test::assert_tokens(&Intents::GUILD_WEBHOOKS, &[Token::U64(1 << 5)]);
        serde_test::assert_tokens(&Intents::GUILD_INVITES, &[Token::U64(1 << 6)]);
        serde_test::assert_tokens(&Intents::GUILD_VOICE_STATES, &[Token::U64(1 << 7)]);
        serde_test::assert_tokens(&Intents::GUILD_PRESENCES, &[Token::U64(1 << 8)]);
        serde_test::assert_tokens(&Intents::GUILD_MESSAGES, &[Token::U64(1 << 9)]);
        serde_test::assert_tokens(&Intents::GUILD_MESSAGE_REACTIONS, &[Token::U64(1 << 10)]);
        serde_test::assert_tokens(&Intents::GUILD_MESSAGE_TYPING, &[Token::U64(1 << 11)]);
        serde_test::assert_tokens(&Intents::DIRECT_MESSAGES, &[Token::U64(1 << 12)]);
        serde_test::assert_tokens(&Intents::DIRECT_MESSAGE_REACTIONS, &[Token::U64(1 << 13)]);
        serde_test::assert_tokens(&Intents::DIRECT_MESSAGE_TYPING, &[Token::U64(1 << 14)]);
    }
}
