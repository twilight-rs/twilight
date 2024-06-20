use crate::{
    channel::{thread::ThreadMetadata, Attachment, ChannelType, Message},
    guild::{MemberFlags, Permissions, Role},
    id::{
        marker::{AttachmentMarker, ChannelMarker, MessageMarker, RoleMarker, UserMarker},
        Id,
    },
    user::User,
    util::{ImageHash, Timestamp},
};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;

/// Resolved mentioned resources.
///
/// See [Discord Docs/Resolved Data Structure].
///
/// [`ApplicationCommand`]: crate::application::interaction::InteractionType::ApplicationCommand
/// [Discord Docs/Resolved Data Structure]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-resolved-data-structure
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InteractionDataResolved {
    /// Map of resolved attachments.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub attachments: HashMap<Id<AttachmentMarker>, Attachment>,
    /// Map of resolved channels.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub channels: HashMap<Id<ChannelMarker>, InteractionChannel>,
    /// Map of resolved members.
    ///
    /// Resolved members' ID will map to a resolved user as well.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub members: HashMap<Id<UserMarker>, InteractionMember>,
    /// Map of resolved messages.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub messages: HashMap<Id<MessageMarker>, Message>,
    /// Map of resolved roles.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub roles: HashMap<Id<RoleMarker>, Role>,
    /// Map of resolved users.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub users: HashMap<Id<UserMarker>, User>,
}

/// Partial channel resolved from an [`Interaction`].
///
/// [`Interaction`]: crate::application::interaction::Interaction
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InteractionChannel {
    /// ID of the channel.
    pub id: Id<ChannelMarker>,
    /// Type of the channel.
    ///
    /// This can be used to determine what fields *might* be available.
    #[serde(rename = "type")]
    pub kind: ChannelType,
    /// Name of the channel.
    pub name: String,
    /// ID of the channel the thread was created in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Id<ChannelMarker>>,
    /// Computed permissions, including overwrites, for the invoking user in the channel.
    pub permissions: Permissions,
    /// Metadata about a thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_metadata: Option<ThreadMetadata>,
}

/// Partial member resolved from an [`Interaction`].
///
/// [`Interaction`]: crate::application::interaction::Interaction
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InteractionMember {
    /// Member's guild avatar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<ImageHash>,
    /// If the member is timed out, when the timeout will expire.
    pub communication_disabled_until: Option<Timestamp>,
    /// Flags for the member.
    ///
    /// Defaults to an empty bitfield.
    pub flags: MemberFlags,
    /// Member guild join date.
    pub joined_at: Option<Timestamp>,
    /// Member nickname.
    pub nick: Option<String>,
    /// Whether the user has yet to pass the guild's Membership Screening
    /// requirements.
    pub pending: bool,
    /// Total permissions of the member in this channel including overwrites
    pub permissions: Permissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<Timestamp>,
    /// Member roles.
    #[serde(default)]
    pub roles: Vec<Id<RoleMarker>>,
}

#[cfg(test)]
mod tests {
    use super::{InteractionChannel, InteractionDataResolved, InteractionMember};
    use crate::{
        channel::{
            message::{
                sticker::{MessageSticker, StickerFormatType},
                MessageFlags, MessageType,
            },
            Attachment, ChannelType, Message,
        },
        guild::{MemberFlags, PartialMember, Permissions, Role, RoleFlags},
        id::Id,
        test::image_hash,
        user::{PremiumType, User, UserFlags},
        util::datetime::{Timestamp, TimestampParseError},
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[test]
    #[allow(clippy::too_many_lines, deprecated)]
    fn test_data_resolved() -> Result<(), TimestampParseError> {
        let joined_at = Some(Timestamp::from_str("2021-08-10T12:18:37.000000+00:00")?);
        let timestamp = Timestamp::from_str("2020-02-02T02:02:02.020000+00:00")?;
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

        let value = InteractionDataResolved {
            attachments: IntoIterator::into_iter([(
                Id::new(400),
                Attachment {
                    content_type: Some("image/png".to_owned()),
                    ephemeral: true,
                    filename: "rainbow_dash.png".to_owned(),
                    flags: None,
                    description: None,
                    duration_secs: None,
                    height: Some(2674),
                    id: Id::new(400),
                    proxy_url: "https://proxy.example.com/rainbow_dash.png".to_owned(),
                    size: 13370,
                    url: "https://example.com/rainbow_dash.png".to_owned(),
                    waveform: None,
                    width: Some(1337),
                },
            )])
            .collect(),
            channels: IntoIterator::into_iter([(
                Id::new(100),
                InteractionChannel {
                    id: Id::new(100),
                    kind: ChannelType::GuildText,
                    name: "channel name".into(),
                    parent_id: None,
                    permissions: Permissions::empty(),
                    thread_metadata: None,
                },
            )])
            .collect(),
            members: IntoIterator::into_iter([(
                Id::new(300),
                InteractionMember {
                    avatar: None,
                    communication_disabled_until: None,
                    flags,
                    joined_at,
                    nick: None,
                    pending: false,
                    permissions: Permissions::empty(),
                    premium_since: None,
                    roles: Vec::new(),
                },
            )])
            .collect(),
            messages: IntoIterator::into_iter([(
                Id::new(4),
                Message {
                    activity: None,
                    application: None,
                    application_id: None,
                    attachments: Vec::new(),
                    author: User {
                        accent_color: None,
                        avatar: Some(image_hash::AVATAR),
                        avatar_decoration: None,
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
                        flags,
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
                    referenced_message: None,
                    role_subscription_data: None,
                    sticker_items: vec![MessageSticker {
                        format_type: StickerFormatType::Png,
                        id: Id::new(1),
                        name: "sticker name".to_owned(),
                    }],
                    timestamp,
                    thread: None,
                    tts: false,
                    webhook_id: None,
                },
            )])
            .collect(),
            roles: IntoIterator::into_iter([(
                Id::new(400),
                Role {
                    color: 0,
                    hoist: true,
                    icon: None,
                    id: Id::new(400),
                    managed: false,
                    mentionable: true,
                    name: "test".to_owned(),
                    permissions: Permissions::ADMINISTRATOR,
                    position: 12,
                    flags: RoleFlags::empty(),
                    tags: None,
                    unicode_emoji: None,
                },
            )])
            .collect(),
            users: IntoIterator::into_iter([(
                Id::new(300),
                User {
                    accent_color: None,
                    avatar: Some(image_hash::AVATAR),
                    avatar_decoration: None,
                    banner: None,
                    bot: false,
                    discriminator: 1,
                    email: Some("address@example.com".to_owned()),
                    flags: Some(UserFlags::PREMIUM_EARLY_SUPPORTER | UserFlags::VERIFIED_DEVELOPER),
                    global_name: Some("test".to_owned()),
                    id: Id::new(300),
                    locale: Some("en-us".to_owned()),
                    mfa_enabled: Some(true),
                    name: "test".to_owned(),
                    premium_type: Some(PremiumType::Nitro),
                    public_flags: Some(
                        UserFlags::PREMIUM_EARLY_SUPPORTER | UserFlags::VERIFIED_DEVELOPER,
                    ),
                    system: None,
                    verified: Some(true),
                },
            )])
            .collect(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InteractionDataResolved",
                    len: 6,
                },
                Token::Str("attachments"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("400"),
                Token::Struct {
                    name: "Attachment",
                    len: 9,
                },
                Token::Str("content_type"),
                Token::Some,
                Token::Str("image/png"),
                Token::Str("ephemeral"),
                Token::Bool(true),
                Token::Str("filename"),
                Token::Str("rainbow_dash.png"),
                Token::Str("height"),
                Token::Some,
                Token::U64(2674),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("400"),
                Token::Str("proxy_url"),
                Token::Str("https://proxy.example.com/rainbow_dash.png"),
                Token::Str("size"),
                Token::U64(13370),
                Token::Str("url"),
                Token::Str("https://example.com/rainbow_dash.png"),
                Token::Str("width"),
                Token::Some,
                Token::U64(1337),
                Token::StructEnd,
                Token::MapEnd,
                Token::Str("channels"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("100"),
                Token::Struct {
                    name: "InteractionChannel",
                    len: 4,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("100"),
                Token::Str("type"),
                Token::U8(0),
                Token::Str("name"),
                Token::Str("channel name"),
                Token::Str("permissions"),
                Token::Str("0"),
                Token::StructEnd,
                Token::MapEnd,
                Token::Str("members"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("300"),
                Token::Struct {
                    name: "InteractionMember",
                    len: 7,
                },
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("flags"),
                Token::U64(flags.bits()),
                Token::Str("joined_at"),
                Token::Some,
                Token::Str("2021-08-10T12:18:37.000000+00:00"),
                Token::Str("nick"),
                Token::None,
                Token::Str("pending"),
                Token::Bool(false),
                Token::Str("permissions"),
                Token::Str("0"),
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
                Token::MapEnd,
                Token::Str("messages"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Struct {
                    name: "Message",
                    len: 18,
                },
                Token::Str("attachments"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("author"),
                Token::Struct {
                    name: "User",
                    len: 9,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("avatar_decoration"),
                Token::None,
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("global_name"),
                Token::Some,
                Token::Str("test"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("content"),
                Token::Str("ping"),
                Token::Str("edited_timestamp"),
                Token::None,
                Token::Str("embeds"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("flags"),
                Token::Some,
                Token::U64(0),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Str("type"),
                Token::U8(0),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "PartialMember",
                    len: 8,
                },
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("flags"),
                Token::U64(flags.bits()),
                Token::Str("joined_at"),
                Token::Some,
                Token::Str("2021-08-10T12:18:37.000000+00:00"),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::Some,
                Token::Str("member nick"),
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("user"),
                Token::None,
                Token::StructEnd,
                Token::Str("mention_everyone"),
                Token::Bool(false),
                Token::Str("mention_roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("mentions"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("pinned"),
                Token::Bool(false),
                Token::Str("sticker_items"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "MessageSticker",
                    len: 3,
                },
                Token::Str("format_type"),
                Token::U8(1),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("sticker name"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("timestamp"),
                Token::Str("2020-02-02T02:02:02.020000+00:00"),
                Token::Str("tts"),
                Token::Bool(false),
                Token::StructEnd,
                Token::MapEnd,
                Token::Str("roles"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("400"),
                Token::Struct {
                    name: "Role",
                    len: 9,
                },
                Token::Str("color"),
                Token::U32(0),
                Token::Str("hoist"),
                Token::Bool(true),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("400"),
                Token::Str("managed"),
                Token::Bool(false),
                Token::Str("mentionable"),
                Token::Bool(true),
                Token::Str("name"),
                Token::Str("test"),
                Token::Str("permissions"),
                Token::Str("8"),
                Token::Str("position"),
                Token::I64(12),
                Token::Str("flags"),
                Token::U64(0),
                Token::StructEnd,
                Token::MapEnd,
                Token::Str("users"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("300"),
                Token::Struct {
                    name: "User",
                    len: 16,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("avatar_decoration"),
                Token::None,
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("email"),
                Token::Some,
                Token::Str("address@example.com"),
                Token::Str("flags"),
                Token::Some,
                Token::U64(131_584),
                Token::Str("global_name"),
                Token::Some,
                Token::Str("test"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("300"),
                Token::Str("locale"),
                Token::Some,
                Token::Str("en-us"),
                Token::Str("mfa_enabled"),
                Token::Some,
                Token::Bool(true),
                Token::Str("username"),
                Token::Str("test"),
                Token::Str("premium_type"),
                Token::Some,
                Token::U8(2),
                Token::Str("public_flags"),
                Token::Some,
                Token::U64(131_584),
                Token::Str("verified"),
                Token::Some,
                Token::Bool(true),
                Token::StructEnd,
                Token::MapEnd,
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
