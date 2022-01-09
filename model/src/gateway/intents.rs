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
    /// Discord Docs] for more information.
    ///
    /// [the Discord Docs]: https://discord.com/developers/docs/topics/gateway#gateway-intents
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
        /// [`GUILD_CREATE`]: super::event::Event::GuildCreate
        /// [`GUILD_UPDATE`]: super::event::Event::GuildUpdate
        /// [`GUILD_DELETE`]: super::event::Event::GuildDelete
        /// [`GUILD_ROLE_CREATE`]: super::event::Event::RoleCreate
        /// [`GUILD_ROLE_UPDATE`]: super::event::Event::RoleUpdate
        /// [`GUILD_ROLE_DELETE`]: super::event::Event::RoleDelete
        /// [`CHANNEL_CREATE`]: super::event::Event::ChannelCreate
        /// [`CHANNEL_UPDATE`]: super::event::Event::ChannelUpdate
        /// [`CHANNEL_DELETE`]: super::event::Event::ChannelDelete
        /// [`CHANNEL_PINS_UPDATE`]: super::event::Event::ChannelPinsUpdate
        const GUILDS = 1;
        /// Guild members intent.
        ///
        /// This intent is privileged. See [the Discord Docs] for more information.
        ///
        /// Event(s) received:
        ///  - [`GUILD_MEMBER_ADD`]
        ///  - [`GUILD_MEMBER_UPDATE`]
        ///  - [`GUILD_MEMBER_REMOVE`]
        ///
        /// [the Discord Docs]: https://discord.com/developers/docs/topics/gateway#privileged-intents
        /// [`GUILD_MEMBER_ADD`]: super::event::Event::MemberAdd
        /// [`GUILD_MEMBER_UPDATE`]: super::event::Event::MemberUpdate
        /// [`GUILD_MEMBER_REMOVE`]: super::event::Event::MemberRemove
        const GUILD_MEMBERS = 1 << 1;
        /// Guild bans intent.
        ///
        /// Event(s) received:
        ///  - [`GUILD_BAN_ADD`]
        ///  - [`GUILD_BAN_REMOVE`]
        ///
        /// [`GUILD_BAN_ADD`]: super::event::Event::BanAdd
        /// [`GUILD_BAN_REMOVE`]: super::event::Event::BanRemove
        const GUILD_BANS = 1 << 2;
        /// Guild emojis intent.
        ///
        /// Event(s) received:
        ///  - [`GUILD_EMOJIS_UPDATE`]
        ///
        /// [`GUILD_EMOJIS_UPDATE`]: super::event::Event::GuildEmojisUpdate
        const GUILD_EMOJIS = 1 << 3;
        /// Guild integrations intent.
        ///
        /// Event(s) received:
        ///  - [`GUILD_INTEGRATIONS_UPDATE`]
        ///
        /// [`GUILD_INTEGRATIONS_UPDATE`]: super::event::Event::GuildIntegrationsUpdate
        const GUILD_INTEGRATIONS = 1 << 4;
        /// Guild webhooks intent.
        ///
        /// Event(s) received:
        ///  - [`WEBHOOKS_UPDATE`]
        ///
        /// [`WEBHOOKS_UPDATE`]: super::event::Event::WebhooksUpdate
        const GUILD_WEBHOOKS = 1 << 5;
        /// Guild invites intent.
        ///
        /// Event(s) received:
        ///  - [`INVITE_CREATE`]
        ///  - [`INVITE_DELETE`]
        ///
        /// [`INVITE_CREATE`]: super::event::Event::InviteCreate
        /// [`INVITE_DELETE`]: super::event::Event::InviteDelete
        const GUILD_INVITES = 1 << 6;
        /// Guild voice states intent.
        ///
        /// Event(s) received:
        ///  - [`VOICE_STATE_UPDATE`]
        ///
        /// [`VOICE_STATE_UPDATE`]: super::event::Event::VoiceStateUpdate
        const GUILD_VOICE_STATES = 1 << 7;
        /// Guild presences intent.
        ///
        /// This intent is privileged. See [the Discord Docs] for more information.
        ///
        /// Event(s) received:
        ///  - [`PRESENCE_UPDATE`]
        ///
        /// [the Discord Docs]: https://discord.com/developers/docs/topics/gateway#privileged-intents
        /// [`PRESENCE_UPDATE`]: super::event::Event::PresenceUpdate
        const GUILD_PRESENCES = 1 << 8;
        /// Guild messages intent.
        ///
        /// Event(s) received:
        ///  - [`MESSAGE_CREATE`]
        ///  - [`MESSAGE_UPDATE`]
        ///  - [`MESSAGE_DELETE`]
        ///  - [`MESSAGE_DELETE_BULK`]
        ///
        /// [`MESSAGE_CREATE`]: super::event::Event::MessageCreate
        /// [`MESSAGE_UPDATE`]: super::event::Event::MessageUpdate
        /// [`MESSAGE_DELETE`]: super::event::Event::MessageDelete
        /// [`MESSAGE_DELETE_BULK`]: super::event::Event::MessageDeleteBulk
        const GUILD_MESSAGES = 1 << 9;
        /// Guild message reactions intent.
        ///
        /// Event(s) received:
        ///  - [`MESSAGE_REACTION_ADD`]
        ///  - [`MESSAGE_REACTION_REMOVE`]
        ///  - [`MESSAGE_REACTION_REMOVE_ALL`]
        ///  - [`MESSAGE_REACTION_REMOVE_EMOJI`]
        ///
        /// [`MESSAGE_REACTION_ADD`]: super::event::Event::ReactionAdd
        /// [`MESSAGE_REACTION_REMOVE`]: super::event::Event::ReactionRemove
        /// [`MESSAGE_REACTION_REMOVE_ALL`]: super::event::Event::ReactionRemoveAll
        /// [`MESSAGE_REACTION_REMOVE_EMOJI`]: super::event::Event::ReactionRemoveEmoji
        const GUILD_MESSAGE_REACTIONS = 1 << 10;
        /// Guild message typing intent.
        ///
        /// Event(s) received:
        ///  - [`TYPING_START`]
        ///
        /// [`TYPING_START`]: super::event::Event::TypingStart
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
        /// [`MESSAGE_CREATE`]: super::event::Event::MessageCreate
        /// [`MESSAGE_UPDATE`]: super::event::Event::MessageUpdate
        /// [`MESSAGE_DELETE`]: super::event::Event::MessageDelete
        /// [`MESSAGE_DELETE_BULK`]: super::event::Event::MessageDeleteBulk
        /// [`GUILD_MESSAGES`]: Self::GUILD_MESSAGES
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
        /// [`MESSAGE_REACTION_ADD`]: super::event::Event::ReactionAdd
        /// [`MESSAGE_REACTION_REMOVE`]: super::event::Event::ReactionRemove
        /// [`MESSAGE_REACTION_REMOVE_ALL`]: super::event::Event::ReactionRemoveAll
        /// [`MESSAGE_REACTION_REMOVE_EMOJI`]: super::event::Event::ReactionRemoveEmoji
        /// [`GUILD_MESSAGE_REACTIONS`]: Self::GUILD_MESSAGE_REACTIONS
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
        /// [`TYPING_START`]: super::event::Event::TypingStart
        /// [`GUILD_MESSAGE_TYPING`]: Self::GUILD_MESSAGE_TYPING
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
