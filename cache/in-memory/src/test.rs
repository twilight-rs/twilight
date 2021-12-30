use crate::InMemoryCache;
use twilight_model::{
    channel::{
        message::{Message, MessageFlags, MessageType},
        ChannelType, GuildChannel, Reaction, ReactionType, TextChannel,
    },
    datetime::Timestamp,
    gateway::payload::incoming::{MessageCreate, ReactionAdd},
    guild::{Emoji, Member, PartialMember, Permissions, Role},
    id::{
        marker::{ChannelMarker, EmojiMarker, GuildMarker, RoleMarker, UserMarker},
        Id,
    },
    user::{CurrentUser, User},
    voice::VoiceState,
};

pub fn cache_with_message_and_reactions() -> InMemoryCache {
    let joined_at = Timestamp::from_secs(1_632_072_645).expect("non zero");
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
            id: Id::new_checked(3),
            locale: None,
            mfa_enabled: None,
            name: "test".to_owned(),
            premium_type: None,
            public_flags: None,
            system: None,
            verified: None,
        },
        channel_id: Id::new_checked(2),
        components: Vec::new(),
        content: "ping".to_owned(),
        edited_timestamp: None,
        embeds: Vec::new(),
        flags: Some(MessageFlags::empty()),
        guild_id: Some(Id::new_checked(1)),
        id: Id::new_checked(4),
        interaction: None,
        kind: MessageType::Regular,
        member: Some(PartialMember {
            avatar: None,
            communication_disabled_until: None,
            deaf: false,
            joined_at,
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
        timestamp: Timestamp::from_secs(1_632_072_645).expect("non zero"),
        tts: false,
        webhook_id: None,
    };

    cache.update(&MessageCreate(msg));

    let mut reaction = ReactionAdd(Reaction {
        channel_id: Id::new_checked(2),
        emoji: ReactionType::Unicode {
            name: "😀".to_owned(),
        },
        guild_id: Some(Id::new_checked(1)),
        member: Some(Member {
            avatar: None,
            communication_disabled_until: None,
            deaf: false,
            guild_id: Id::new_checked(1),
            joined_at,
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
                id: Id::new_checked(3),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            },
        }),
        message_id: Id::new_checked(4),
        user_id: Id::new_checked(3),
    });

    cache.update(&reaction);

    reaction.member.replace(Member {
        avatar: None,
        communication_disabled_until: None,
        deaf: false,
        guild_id: Id::new_checked(1),
        joined_at,
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
            id: Id::new_checked(5),
            locale: None,
            mfa_enabled: None,
            name: "test".to_owned(),
            premium_type: None,
            public_flags: None,
            system: None,
            verified: None,
        },
    });
    reaction.user_id = Id::new_checked(5);

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
        id: Id::new_checked(id),
        mfa_enabled: true,
        name: "test".to_owned(),
        verified: Some(true),
        premium_type: None,
        public_flags: None,
        flags: None,
        locale: None,
    }
}

pub fn emoji(id: Id<EmojiMarker>, user: Option<User>) -> Emoji {
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

pub fn guild_channel_text() -> (Id<GuildMarker>, Id<ChannelMarker>, GuildChannel) {
    let guild_id = Id::new_checked(1);
    let channel_id = Id::new_checked(2);
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

pub fn member(id: Id<UserMarker>, guild_id: Id<GuildMarker>) -> Member {
    let joined_at = Timestamp::from_secs(1_632_072_645).expect("non zero");

    Member {
        avatar: None,
        communication_disabled_until: None,
        deaf: false,
        guild_id,
        joined_at,
        mute: false,
        nick: None,
        pending: false,
        premium_since: None,
        roles: Vec::new(),
        user: user(id),
    }
}

pub fn role(id: Id<RoleMarker>) -> Role {
    Role {
        color: 0,
        hoist: false,
        icon: None,
        id,
        managed: false,
        mentionable: false,
        name: "test".to_owned(),
        permissions: Permissions::empty(),
        position: 0,
        tags: None,
        unicode_emoji: None,
    }
}

pub fn voice_state(
    guild_id: Id<GuildMarker>,
    channel_id: Option<Id<ChannelMarker>>,
    user_id: Id<UserMarker>,
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
        request_to_speak_timestamp: Some(Timestamp::from_secs(1_632_072_645).expect("non zero")),
    }
}

pub fn user(id: Id<UserMarker>) -> User {
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
