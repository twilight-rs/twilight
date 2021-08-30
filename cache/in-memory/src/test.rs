use crate::InMemoryCache;
use twilight_model::{
    channel::{
        message::{Message, MessageFlags, MessageType},
        ChannelType, GuildChannel, Reaction, ReactionType, TextChannel,
    },
    gateway::payload::incoming::{MessageCreate, ReactionAdd},
    guild::{Emoji, Member, PartialMember, Permissions, Role},
    id::{ChannelId, EmojiId, GuildId, MessageId, RoleId, UserId},
    user::{CurrentUser, User},
    voice::VoiceState,
};

pub fn cache_with_message_and_reactions() -> InMemoryCache {
    let cache = InMemoryCache::new();

    let msg = Message {
        activity: None,
        application: None,
        application_id: None,
        attachments: Vec::new(),
        author: User {
            accent_color: None,
            avatar: Some("".to_owned()),
            banner: None,
            bot: false,
            discriminator: 1,
            email: None,
            flags: None,
            id: UserId::new(3).expect("non zero"),
            locale: None,
            mfa_enabled: None,
            name: "test".to_owned(),
            premium_type: None,
            public_flags: None,
            system: None,
            verified: None,
        },
        channel_id: ChannelId::new(2).expect("non zero"),
        components: Vec::new(),
        content: "ping".to_owned(),
        edited_timestamp: None,
        embeds: Vec::new(),
        flags: Some(MessageFlags::empty()),
        guild_id: Some(GuildId::new(1).expect("non zero")),
        id: MessageId::new(4).expect("non zero"),
        interaction: None,
        kind: MessageType::Regular,
        member: Some(PartialMember {
            deaf: false,
            joined_at: None,
            mute: false,
            nick: Some("member nick".to_owned()),
            permissions: None,
            premium_since: None,
            roles: Vec::new(),
            user: None,
        }),
        mention_channels: Vec::new(),
        mention_everyone: false,
        mention_roles: Vec::new(),
        mentions: Vec::new(),
        pinned: false,
        reactions: Vec::new(),
        reference: None,
        sticker_items: Vec::new(),
        thread: None,
        referenced_message: None,
        timestamp: String::new(),
        tts: false,
        webhook_id: None,
    };

    cache.update(&MessageCreate(msg));

    let mut reaction = ReactionAdd(Reaction {
        channel_id: ChannelId::new(2).expect("non zero"),
        emoji: ReactionType::Unicode {
            name: "😀".to_owned(),
        },
        guild_id: Some(GuildId::new(1).expect("non zero")),
        member: Some(Member {
            deaf: false,
            guild_id: GuildId::new(1).expect("non zero"),
            hoisted_role: None,
            joined_at: None,
            mute: false,
            nick: Some("member nick".to_owned()),
            pending: false,
            premium_since: None,
            roles: Vec::new(),
            user: User {
                accent_color: None,
                avatar: Some("".to_owned()),
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                id: UserId::new(3).expect("non zero"),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            },
        }),
        message_id: MessageId::new(4).expect("non zero"),
        user_id: UserId::new(3).expect("non zero"),
    });

    cache.update(&reaction);

    reaction.member.replace(Member {
        deaf: false,
        guild_id: GuildId::new(1).expect("non zero"),
        hoisted_role: None,
        joined_at: None,
        mute: false,
        nick: None,
        pending: false,
        premium_since: None,
        roles: Vec::new(),
        user: User {
            accent_color: None,
            avatar: Some("".to_owned()),
            banner: None,
            bot: false,
            discriminator: 2,
            email: None,
            flags: None,
            id: UserId::new(5).expect("non zero"),
            locale: None,
            mfa_enabled: None,
            name: "test".to_owned(),
            premium_type: None,
            public_flags: None,
            system: None,
            verified: None,
        },
    });
    reaction.user_id = UserId::new(5).expect("non zero");

    cache.update(&reaction);

    reaction.emoji = ReactionType::Unicode {
        name: "🗺️".to_owned(),
    };

    cache.update(&reaction);

    cache
}

pub fn current_user(id: u64) -> CurrentUser {
    CurrentUser {
        accent_color: Some(16711680),
        avatar: None,
        banner: None,
        bot: true,
        discriminator: 9876,
        email: None,
        id: UserId::new(id).expect("non zero"),
        mfa_enabled: true,
        name: "test".to_owned(),
        verified: Some(true),
        premium_type: None,
        public_flags: None,
        flags: None,
        locale: None,
    }
}

pub fn emoji(id: EmojiId, user: Option<User>) -> Emoji {
    Emoji {
        animated: false,
        available: true,
        id,
        managed: false,
        name: "test".to_owned(),
        require_colons: true,
        roles: Vec::new(),
        user,
    }
}

pub fn guild_channel_text() -> (GuildId, ChannelId, GuildChannel) {
    let guild_id = GuildId::new(1).expect("non zero");
    let channel_id = ChannelId::new(2).expect("non zero");
    let channel = GuildChannel::Text(TextChannel {
        guild_id: Some(guild_id),
        id: channel_id,
        kind: ChannelType::GuildText,
        last_message_id: None,
        last_pin_timestamp: None,
        name: "test".to_owned(),
        nsfw: false,
        parent_id: None,
        permission_overwrites: Vec::new(),
        position: 3,
        rate_limit_per_user: None,
        topic: None,
    });

    (guild_id, channel_id, channel)
}

pub fn member(id: UserId, guild_id: GuildId) -> Member {
    Member {
        deaf: false,
        guild_id,
        hoisted_role: None,
        joined_at: None,
        mute: false,
        nick: None,
        pending: false,
        premium_since: None,
        roles: Vec::new(),
        user: user(id),
    }
}

pub fn role(id: RoleId) -> Role {
    Role {
        color: 0,
        hoist: false,
        id,
        managed: false,
        mentionable: false,
        name: "test".to_owned(),
        permissions: Permissions::empty(),
        position: 0,
        tags: None,
    }
}

pub fn voice_state(
    guild_id: GuildId,
    channel_id: Option<ChannelId>,
    user_id: UserId,
) -> VoiceState {
    VoiceState {
        channel_id,
        deaf: false,
        guild_id: Some(guild_id),
        member: None,
        mute: true,
        self_deaf: false,
        self_mute: true,
        self_stream: false,
        session_id: "a".to_owned(),
        suppress: false,
        token: None,
        user_id,
        request_to_speak_timestamp: Some("2021-04-21T22:16:50+0000".to_owned()),
    }
}

pub fn user(id: UserId) -> User {
    User {
        accent_color: None,
        avatar: None,
        banner: Some("06c16474723fe537c283b8efa61a30c8".to_owned()),
        bot: false,
        discriminator: 1,
        email: None,
        flags: None,
        id,
        locale: None,
        mfa_enabled: None,
        name: "user".to_owned(),
        premium_type: None,
        public_flags: None,
        system: None,
        verified: None,
    }
}
