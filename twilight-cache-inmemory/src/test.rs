use crate::DefaultInMemoryCache;
use twilight_model::{
    channel::{
        message::{
            sticker::{Sticker, StickerFormatType, StickerType},
            EmojiReactionType, Message, MessageFlags, MessageType,
        },
        Channel, ChannelType,
    },
    gateway::{
        payload::incoming::{MessageCreate, ReactionAdd},
        GatewayReaction,
    },
    guild::{
        scheduled_event::{EntityType, GuildScheduledEvent, PrivacyLevel, Status},
        AfkTimeout, DefaultMessageNotificationLevel, Emoji, ExplicitContentFilter, Guild, Member,
        MemberFlags, MfaLevel, NSFWLevel, PartialMember, Permissions, PremiumTier, Role, RoleFlags,
        SystemChannelFlags, VerificationLevel,
    },
    id::{
        marker::{
            ChannelMarker, EmojiMarker, GuildMarker, RoleMarker, ScheduledEventMarker,
            StickerMarker, UserMarker,
        },
        Id,
    },
    user::{CurrentUser, User},
    util::{ImageHash, Timestamp},
    voice::VoiceState,
};

pub fn cache() -> DefaultInMemoryCache {
    DefaultInMemoryCache::new()
}

#[allow(clippy::too_many_lines, deprecated)]
pub fn cache_with_message_and_reactions() -> DefaultInMemoryCache {
    let joined_at = Some(Timestamp::from_secs(1_632_072_645).expect("non zero"));
    let cache = DefaultInMemoryCache::new();
    let avatar = ImageHash::parse(b"6961d9f1fdb5880bf4a3ec6348d3bbcf").unwrap();
    let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

    let msg = Message {
        activity: None,
        application: None,
        application_id: None,
        attachments: Vec::new(),
        author: User {
            accent_color: None,
            avatar: Some(avatar),
            avatar_decoration: None,
            avatar_decoration_data: None,
            banner: None,
            bot: false,
            discriminator: 1,
            email: None,
            flags: None,
            global_name: Some("test".to_owned()),
            id: Id::new(3),
            locale: None,
            mfa_enabled: None,
            name: "test".to_owned(),
            premium_type: None,
            public_flags: None,
            system: None,
            verified: None,
        },
        call: None,
        channel_id: Id::new(2),
        components: Vec::new(),
        content: "ping".to_owned(),
        edited_timestamp: None,
        embeds: Vec::new(),
        flags: Some(MessageFlags::empty()),
        guild_id: Some(Id::new(1)),
        id: Id::new(4),
        interaction: None,
        interaction_metadata: None,
        kind: MessageType::Regular,
        member: Some(PartialMember {
            avatar: None,
            communication_disabled_until: None,
            deaf: false,
            flags,
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
        message_snapshots: Vec::new(),
        pinned: false,
        poll: None,
        reactions: Vec::new(),
        reference: None,
        referenced_message: None,
        role_subscription_data: None,
        sticker_items: Vec::new(),
        timestamp: Timestamp::from_secs(1_632_072_645).expect("non zero"),
        thread: None,
        tts: false,
        webhook_id: None,
    };

    cache.update(&MessageCreate(msg));

    let mut reaction = ReactionAdd(GatewayReaction {
        burst: false,
        burst_colors: Vec::new(),
        channel_id: Id::new(2),
        emoji: EmojiReactionType::Unicode {
            name: "😀".to_owned(),
        },
        guild_id: Some(Id::new(1)),
        member: Some(Member {
            avatar: None,
            communication_disabled_until: None,
            deaf: false,
            flags,
            joined_at,
            mute: false,
            nick: Some("member nick".to_owned()),
            pending: false,
            premium_since: None,
            roles: Vec::new(),
            user: User {
                accent_color: None,
                avatar: Some(avatar),
                avatar_decoration: None,
                avatar_decoration_data: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                global_name: Some("test".to_owned()),
                id: Id::new(3),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            },
        }),
        message_author_id: Some(Id::new(7)),
        message_id: Id::new(4),
        user_id: Id::new(3),
    });

    cache.update(&reaction);

    let user_5_input = b"ef678abdee09d8dfb14e83381983d5e4";
    let user_5_avatar = ImageHash::parse(user_5_input).unwrap();

    reaction.member.replace(Member {
        avatar: None,
        communication_disabled_until: None,
        deaf: false,
        flags,
        joined_at,
        mute: false,
        nick: None,
        pending: false,
        premium_since: None,
        roles: Vec::new(),
        user: User {
            accent_color: None,
            avatar: Some(user_5_avatar),
            avatar_decoration: None,
            avatar_decoration_data: None,
            banner: None,
            bot: false,
            discriminator: 2,
            email: None,
            flags: None,
            global_name: Some("test".to_owned()),
            id: Id::new(5),
            locale: None,
            mfa_enabled: None,
            name: "test".to_owned(),
            premium_type: None,
            public_flags: None,
            system: None,
            verified: None,
        },
    });
    reaction.user_id = Id::new(5);

    cache.update(&reaction);

    reaction.emoji = EmojiReactionType::Unicode {
        name: "🗺️".to_owned(),
    };

    cache.update(&reaction);

    reaction.emoji = EmojiReactionType::Custom {
        animated: true,
        id: Id::new(6),
        name: Some("custom".to_owned()),
    };

    cache.update(&reaction);

    cache
}

pub fn current_user(id: u64) -> CurrentUser {
    CurrentUser {
        accent_color: Some(0xFF_00_00),
        avatar: None,
        banner: None,
        bot: true,
        discriminator: 9876,
        email: None,
        id: Id::new(id),
        mfa_enabled: true,
        name: "test".to_owned(),
        verified: Some(true),
        premium_type: None,
        public_flags: None,
        flags: None,
        locale: None,
        global_name: None,
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

pub fn guild_channel_text() -> (Id<GuildMarker>, Id<ChannelMarker>, Channel) {
    let guild_id = Id::new(1);
    let channel_id = Id::new(2);
    let channel = Channel {
        application_id: None,
        applied_tags: None,
        available_tags: None,
        bitrate: None,
        default_auto_archive_duration: None,
        default_forum_layout: None,
        default_reaction_emoji: None,
        default_sort_order: None,
        default_thread_rate_limit_per_user: None,
        flags: None,
        guild_id: Some(guild_id),
        icon: None,
        id: channel_id,
        invitable: None,
        kind: ChannelType::GuildText,
        last_message_id: None,
        last_pin_timestamp: None,
        managed: None,
        member: None,
        member_count: None,
        message_count: None,
        name: Some("test".to_owned()),
        newly_created: None,
        nsfw: Some(false),
        owner_id: None,
        parent_id: None,
        permission_overwrites: Some(Vec::new()),
        position: Some(3),
        rate_limit_per_user: None,
        recipients: None,
        rtc_region: None,
        status: None,
        thread_metadata: None,
        topic: None,
        user_limit: None,
        video_quality_mode: None,
    };

    (guild_id, channel_id, channel)
}

pub fn member(id: Id<UserMarker>) -> Member {
    let joined_at = Some(Timestamp::from_secs(1_632_072_645).expect("non zero"));
    let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

    Member {
        avatar: None,
        communication_disabled_until: None,
        deaf: false,
        flags,
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
        flags: RoleFlags::empty(),
        tags: None,
        unicode_emoji: None,
    }
}

pub const fn sticker(id: Id<StickerMarker>, guild_id: Id<GuildMarker>) -> Sticker {
    Sticker {
        available: false,
        description: None,
        format_type: StickerFormatType::Png,
        guild_id: Some(guild_id),
        id,
        kind: StickerType::Standard,
        name: String::new(),
        pack_id: None,
        sort_value: None,
        tags: String::new(),
        user: None,
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
        self_video: false,
        session_id: "a".to_owned(),
        suppress: false,
        user_id,
        request_to_speak_timestamp: Some(Timestamp::from_secs(1_632_072_645).expect("non zero")),
    }
}

pub fn user(id: Id<UserMarker>) -> User {
    let banner_hash = b"16ed037ab6dae5e1739f15c745d12454";
    let banner = ImageHash::parse(banner_hash).expect("valid hash");

    User {
        accent_color: None,
        avatar: None,
        avatar_decoration: None,
        avatar_decoration_data: None,
        banner: Some(banner),
        bot: false,
        discriminator: 1,
        email: None,
        flags: None,
        global_name: Some("test".to_owned()),
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

pub fn guild(id: Id<GuildMarker>, member_count: Option<u64>) -> Guild {
    Guild {
        afk_channel_id: None,
        afk_timeout: AfkTimeout::FIFTEEN_MINUTES,
        application_id: None,
        approximate_member_count: None,
        approximate_presence_count: None,
        banner: None,
        channels: Vec::new(),
        default_message_notifications: DefaultMessageNotificationLevel::Mentions,
        description: None,
        discovery_splash: None,
        emojis: Vec::new(),
        explicit_content_filter: ExplicitContentFilter::None,
        features: Vec::new(),
        guild_scheduled_events: Vec::new(),
        icon: None,
        id,
        joined_at: None,
        large: false,
        max_members: None,
        max_presences: None,
        max_stage_video_channel_users: None,
        max_video_channel_users: None,
        member_count,
        members: Vec::new(),
        mfa_level: MfaLevel::None,
        name: "test".to_owned(),
        nsfw_level: NSFWLevel::Default,
        owner_id: Id::new(1),
        owner: None,
        permissions: None,
        preferred_locale: "en_us".to_owned(),
        premium_progress_bar_enabled: false,
        premium_subscription_count: None,
        premium_tier: PremiumTier::None,
        presences: Vec::new(),
        public_updates_channel_id: None,
        roles: Vec::new(),
        rules_channel_id: None,
        safety_alerts_channel_id: Some(Id::new(2)),
        splash: None,
        stage_instances: Vec::new(),
        stickers: Vec::new(),
        system_channel_flags: SystemChannelFlags::empty(),
        system_channel_id: None,
        threads: Vec::new(),
        unavailable: Some(false),
        vanity_url_code: None,
        verification_level: VerificationLevel::VeryHigh,
        voice_states: Vec::new(),
        widget_channel_id: None,
        widget_enabled: None,
    }
}

pub fn guild_scheduled_event(
    id: Id<ScheduledEventMarker>,
    guild_id: Id<GuildMarker>,
    user_count: Option<u64>,
) -> GuildScheduledEvent {
    GuildScheduledEvent {
        channel_id: None,
        creator: None,
        creator_id: None,
        description: None,
        entity_id: None,
        entity_metadata: None,
        entity_type: EntityType::External,
        guild_id,
        id,
        image: None,
        name: "test".to_owned(),
        privacy_level: PrivacyLevel::GuildOnly,
        scheduled_end_time: None,
        scheduled_start_time: Timestamp::from_secs(789).unwrap(),
        status: Status::Completed,
        user_count,
    }
}
