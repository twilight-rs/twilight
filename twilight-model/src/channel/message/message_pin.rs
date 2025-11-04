use crate::channel::Message;
use crate::util::Timestamp;
use serde::{Deserialize, Serialize};

/// Message Pin object. [docs]
///
/// [docs]: https://discord.com/developers/docs/resources/message#message-pin-object
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessagePin {
    /// When the message was pinned.
    pub pinned_at: Timestamp,
    /// The message which was pinned.
    pub message: Message,
}

#[cfg(test)]
mod tests {
    use super::MessagePin;
    use crate::channel::Message;
    use crate::channel::message::sticker::StickerFormatType;
    use crate::channel::message::{MessageCall, MessageFlags, MessageSticker, MessageType};
    use crate::guild::{MemberFlags, PartialMember};
    use crate::id::Id;
    use crate::test::image_hash;
    use crate::user::User;
    use crate::util::Timestamp;
    use serde_test::Token;
    use std::str::FromStr;
    #[test]
    #[allow(clippy::too_many_lines, deprecated)]
    fn message_pin() {
        let joined_at = Some(Timestamp::from_str("2020-01-01T00:00:00.000000+00:00").unwrap());
        let timestamp = Timestamp::from_micros(1_580_608_922_020_000).expect("non zero");
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

        let msg_value = Message {
            activity: None,
            application: None,
            application_id: None,
            attachments: Vec::new(),
            author: User {
                accent_color: None,
                avatar: Some(image_hash::AVATAR),
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
                primary_guild: None,
                public_flags: None,
                system: None,
                verified: None,
            },
            call: Some(MessageCall {
                ended_timestamp: None,
                participants: Vec::new(),
            }),
            channel_id: Id::new(2),
            components: Vec::new(),
            content: "ping".to_owned(),
            edited_timestamp: None,
            embeds: Vec::new(),
            flags: Some(MessageFlags::empty()),
            guild_id: Some(Id::new(1)),
            id: Id::new(4),
            interaction: None,
            kind: MessageType::Regular,
            member: Some(PartialMember {
                avatar: None,
                avatar_decoration_data: None,
                banner: None,
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
            role_subscription_data: None,
            sticker_items: vec![MessageSticker {
                format_type: StickerFormatType::Png,
                id: Id::new(1),
                name: "sticker name".to_owned(),
            }],
            referenced_message: None,
            timestamp,
            thread: None,
            tts: false,
            webhook_id: None,
            interaction_metadata: None,
        };

        let value = MessagePin {
            pinned_at: Timestamp::from_secs(1).unwrap(),
            message: msg_value,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MessagePin",
                    len: 2,
                },
                Token::Str("pinned_at"),
                Token::Str("1970-01-01T00:00:01.000000+00:00"),
                Token::Str("message"),
                Token::Struct {
                    name: "Message",
                    len: 19,
                },
                Token::Str("attachments"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("author"),
                Token::Struct {
                    name: "User",
                    len: 10,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("avatar_decoration"),
                Token::None,
                Token::Str("avatar_decoration_data"),
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
                Token::Str("call"),
                Token::Some,
                Token::Struct {
                    name: "MessageCall",
                    len: 2,
                },
                Token::Str("ended_timestamp"),
                Token::None,
                Token::Str("participants"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
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
                Token::Str("2020-01-01T00:00:00.000000+00:00"),
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
                Token::StructEnd,
            ],
        );
    }
}
