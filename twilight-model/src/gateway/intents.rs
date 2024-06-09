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
    /// multiple intents, create a union using the `|` operator. See
    /// [Discord Docs/Gateway Intents].
    ///
    /// [Discord Docs/Gateway Intents]: https://discord.com/developers/docs/topics/gateway#gateway-intents
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
        ///  - [`THREAD_CREATE`]
        ///  - [`THREAD_UPDATE`]
        ///  - [`THREAD_DELETE`]
        ///  - [`THREAD_LIST_SYNC`]
        ///  - [`THREAD_MEMBER_UPDATE`]
        ///  - [`THREAD_MEMBERS_UPDATE`]
        ///  - [`STAGE_INSTANCE_CREATE`]
        ///  - [`STAGE_INSTANCE_UPDATE`]
        ///  - [`STAGE_INSTANCE_DELETE`]
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
        /// [`THREAD_CREATE`]: super::event::Event::ThreadCreate
        /// [`THREAD_UPDATE`]: super::event::Event::ThreadUpdate
        /// [`THREAD_DELETE`]: super::event::Event::ThreadDelete
        /// [`THREAD_LIST_SYNC`]: super::event::Event::ThreadListSync
        /// [`THREAD_MEMBER_UPDATE`]: super::event::Event::ThreadMemberUpdate
        /// [`THREAD_MEMBERS_UPDATE`]: super::event::Event::ThreadMembersUpdate
        /// [`STAGE_INSTANCE_CREATE`]: super::event::Event::StageInstanceCreate
        /// [`STAGE_INSTANCE_UPDATE`]: super::event::Event::StageInstanceUpdate
        /// [`STAGE_INSTANCE_DELETE`]: super::event::Event::StageInstanceDelete
        const GUILDS = 1;
        /// Guild members intent.
        ///
        /// This intent is privileged. See [Discord Docs/Privileged Intents].
        ///
        /// Event(s) received:
        ///  - [`GUILD_MEMBER_ADD`]
        ///  - [`GUILD_MEMBER_UPDATE`]
        ///  - [`GUILD_MEMBER_REMOVE`]
        ///  - [`THREAD_MEMBERS_UPDATE`]
        ///
        /// [Discord Docs/Privileged Intents]: https://discord.com/developers/docs/topics/gateway#privileged-intents
        /// [`GUILD_MEMBER_ADD`]: super::event::Event::MemberAdd
        /// [`GUILD_MEMBER_UPDATE`]: super::event::Event::MemberUpdate
        /// [`GUILD_MEMBER_REMOVE`]: super::event::Event::MemberRemove
        /// [`THREAD_MEMBERS_UPDATE`]: super::event::Event::ThreadMembersUpdate
        const GUILD_MEMBERS = 1 << 1;
        /// Guild moderation intent.
        ///
        /// Event(s) received:
        ///  - [`GUILD_AUDIT_LOG_ENTRY_CREATE`]
        ///  - [`GUILD_BAN_ADD`]
        ///  - [`GUILD_BAN_REMOVE`]
        ///
        /// [`GUILD_AUDIT_LOG_ENTRY_CREATE`]: super::event::Event::
        /// [`GUILD_BAN_ADD`]: super::event::Event::BanAdd
        /// [`GUILD_BAN_REMOVE`]: super::event::Event::BanRemove
        const GUILD_MODERATION = 1 << 2;
        /// Guild emojis and stickers intent.
        ///
        /// Event(s) received:
        ///  - [`GUILD_EMOJIS_UPDATE`]
        ///  - [`GUILD_STICKERS_UPDATE`]
        ///
        /// [`GUILD_EMOJIS_UPDATE`]: super::event::Event::GuildEmojisUpdate
        /// [`GUILD_STICKERS_UPDATE`]: super::event::Event::GuildStickersUpdate
        const GUILD_EMOJIS_AND_STICKERS = 1 << 3;
        /// Guild integrations intent.
        ///
        /// Event(s) received:
        ///  - [`GUILD_INTEGRATIONS_UPDATE`]
        ///  - [`INTEGRATION_CREATE`]
        ///  - [`INTEGRATION_UPDATE`]
        ///  - [`INTEGRATION_DELETE`]
        ///
        /// [`GUILD_INTEGRATIONS_UPDATE`]: super::event::Event::GuildIntegrationsUpdate
        /// [`INTEGRATION_CREATE`]: super::event::Event::IntegrationCreate
        /// [`INTEGRATION_UPDATE`]: super::event::Event::IntegrationUpdate
        /// [`INTEGRATION_DELETE`]: super::event::Event::IntegrationDelete
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
        /// This intent is privileged. See [Discord Docs/Privileged Intents].
        ///
        /// Event(s) received:
        ///  - [`PRESENCE_UPDATE`]
        ///
        /// [Discord Docs/Privileged Intents]: https://discord.com/developers/docs/topics/gateway#privileged-intents
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
        /// Message content intent.
        ///
        /// This intent is privileged. See [Discord Docs/Privileged Intents].
        ///
        /// This intent allows you to receive the contents of all messages.
        ///
        /// [Discord Docs/Privileged Intents]: https://discord.com/developers/docs/topics/gateway#privileged-intents
        const MESSAGE_CONTENT = 1 << 15;
        /// Guild scheduled events intent.
        ///
        /// Event(s) received:
        ///
        /// - [`GUILD_SCHEDULED_EVENT_CREATE`]
        /// - [`GUILD_SCHEDULED_EVENT_UPDATE`]
        /// - [`GUILD_SCHEDULED_EVENT_DELETE`]
        /// - [`GUILD_SCHEDULED_EVENT_USER_ADD`]
        /// - [`GUILD_SCHEDULED_EVENT_USER_REMOVE`]
        ///
        /// [`GUILD_SCHEDULED_EVENT_CREATE`]: super::event::Event::GuildScheduledEventCreate
        /// [`GUILD_SCHEDULED_EVENT_UPDATE`]: super::event::Event::GuildScheduledEventDelete
        /// [`GUILD_SCHEDULED_EVENT_DELETE`]: super::event::Event::GuildScheduledEventUpdate
        /// [`GUILD_SCHEDULED_EVENT_USER_ADD`]: super::event::Event::GuildScheduledEventUserAdd
        /// [`GUILD_SCHEDULED_EVENT_USER_REMOVE`]: super::event::Event::GuildScheduledEventUserRemove
        const GUILD_SCHEDULED_EVENTS = 1 << 16;
        /// Auto moderation configuration intent.
        ///
        /// Event(s) received:
        /// - [`AUTO_MODERATION_RULE_CREATE`]
        /// - [`AUTO_MODERATION_RULE_DELETE`]
        /// - [`AUTO_MODERATION_RULE_UPDATE`]
        ///
        /// [`AUTO_MODERATION_RULE_CREATE`]: super::event::Event::AutoModerationRuleCreate
        /// [`AUTO_MODERATION_RULE_DELETE`]: super::event::Event::AutoModerationRuleDelete
        /// [`AUTO_MODERATION_RULE_UPDATE`]: super::event::Event::AutoModerationRuleUpdate
        const AUTO_MODERATION_CONFIGURATION = 1 << 20;
        /// Auto moderation execution event.
        ///
        /// Event(s) received:
        /// - [`AUTO_MODERATION_ACTION_EXECUTION`]
        ///
        /// [`AUTO_MODERATION_ACTION_EXECUTION`]: super::event::Event::AutoModerationActionExecution
        const AUTO_MODERATION_EXECUTION = 1 << 21;
        /// Guild polls intent.
        ///
        /// Event(s) received:
        /// - [`MESSAGE_POLL_VOTE_ADD`]
        /// - [`MESSAGE_POLL_VOTE_REMOVE`]
        ///
        /// [`MESSAGE_POLL_VOTE_ADD`]: super::event::Event::MessagePollVoteAdd
        /// [`MESSAGE_POLL_VOTE_REMOVE`]: super::event::Event::MessagePollVoteRemove
        const GUILD_MESSAGE_POLLS = 1 << 24;
        /// Direct message polls intent.
        ///
        /// Event(s) received:
        /// - [`MESSAGE_POLL_VOTE_ADD`]
        /// - [`MESSAGE_POLL_VOTE_REMOVE`]
        ///
        /// [`MESSAGE_POLL_VOTE_ADD`]: super::event::Event::MessagePollVoteAdd
        /// [`MESSAGE_POLL_VOTE_REMOVE`]: super::event::Event::MessagePollVoteRemove
        const DIRECT_MESSAGE_POLLS = 1 << 25;
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
    #![allow(deprecated)]

    use super::Intents;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_impl_all, const_assert_eq};
    use std::{
        fmt::{Binary, Debug, LowerHex, Octal, UpperHex},
        hash::Hash,
        ops::{
            BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Sub, SubAssign,
        },
    };

    assert_impl_all!(
        Intents: Binary,
        BitAnd,
        BitAndAssign,
        BitOr,
        BitOrAssign,
        BitXor,
        BitXorAssign,
        Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Extend<Intents>,
        FromIterator<Intents>,
        Hash,
        LowerHex,
        Not,
        Octal,
        PartialEq,
        Send,
        Serialize,
        Sub,
        SubAssign,
        Sync,
        UpperHex
    );
    const_assert_eq!(Intents::GUILDS.bits(), 1);
    const_assert_eq!(Intents::GUILD_MEMBERS.bits(), 1 << 1);
    const_assert_eq!(Intents::GUILD_MODERATION.bits(), 1 << 2);
    const_assert_eq!(Intents::GUILD_EMOJIS_AND_STICKERS.bits(), 1 << 3);
    const_assert_eq!(Intents::GUILD_INTEGRATIONS.bits(), 1 << 4);
    const_assert_eq!(Intents::GUILD_WEBHOOKS.bits(), 1 << 5);
    const_assert_eq!(Intents::GUILD_INVITES.bits(), 1 << 6);
    const_assert_eq!(Intents::GUILD_VOICE_STATES.bits(), 1 << 7);
    const_assert_eq!(Intents::GUILD_PRESENCES.bits(), 1 << 8);
    const_assert_eq!(Intents::GUILD_MESSAGES.bits(), 1 << 9);
    const_assert_eq!(Intents::GUILD_MESSAGE_REACTIONS.bits(), 1 << 10);
    const_assert_eq!(Intents::GUILD_MESSAGE_TYPING.bits(), 1 << 11);
    const_assert_eq!(Intents::DIRECT_MESSAGES.bits(), 1 << 12);
    const_assert_eq!(Intents::DIRECT_MESSAGE_REACTIONS.bits(), 1 << 13);
    const_assert_eq!(Intents::DIRECT_MESSAGE_TYPING.bits(), 1 << 14);
    const_assert_eq!(Intents::MESSAGE_CONTENT.bits(), 1 << 15);
    const_assert_eq!(Intents::GUILD_SCHEDULED_EVENTS.bits(), 1 << 16);
    const_assert_eq!(Intents::AUTO_MODERATION_CONFIGURATION.bits(), 1 << 20);
    const_assert_eq!(Intents::AUTO_MODERATION_EXECUTION.bits(), 1 << 21);
    const_assert_eq!(Intents::GUILD_MESSAGE_POLLS.bits(), 1 << 24);
    const_assert_eq!(Intents::DIRECT_MESSAGE_POLLS.bits(), 1 << 25);

    #[test]
    fn serde() {
        serde_test::assert_tokens(
            &Intents::MESSAGE_CONTENT,
            &[Token::U64(Intents::MESSAGE_CONTENT.bits())],
        );
        // Deserialization truncates unknown bits.
        serde_test::assert_de_tokens(&Intents::empty(), &[Token::U64(1 << 63)]);
    }
}
