pub mod embed;
pub mod message;
pub mod permission_overwrite;

mod attachment;
mod category_channel;
mod channel_mention;
mod channel_type;
mod group;
mod private_channel;
mod reaction;
mod reaction_type;
mod text_channel;
mod voice_channel;
mod webhook;
mod webhook_type;

pub use self::{
    attachment::Attachment, category_channel::CategoryChannel, channel_mention::ChannelMention,
    channel_type::ChannelType, group::Group, message::Message, private_channel::PrivateChannel,
    reaction::Reaction, reaction_type::ReactionType, text_channel::TextChannel,
    voice_channel::VoiceChannel, webhook::Webhook, webhook_type::WebhookType,
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Channel {
    Group(Group),
    Guild(GuildChannel),
    Private(PrivateChannel),
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GuildChannel {
    // Order here matters: the text and voice channel has all the fields that a
    // category channel has, so we need to make category channels deserialize
    // last.
    Voice(VoiceChannel),
    Text(TextChannel),
    Category(CategoryChannel),
}

#[cfg(feature = "serde-support")]
mod if_serde_support {
    use super::GuildChannel;
    use crate::id::ChannelId;
    use serde_mappable_seq::Key;

    impl Key<'_, ChannelId> for GuildChannel {
        fn key(&self) -> ChannelId {
            match self {
                Self::Category(c) => c.id,
                Self::Text(c) => c.id,
                Self::Voice(c) => c.id,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::super::{CategoryChannel, ChannelType, GuildChannel, TextChannel, VoiceChannel};
        use crate::id::{ChannelId, GuildId};
        use serde_test::Token;

        #[test]
        fn test_guild_category_channel_serialization() {
            let expected = CategoryChannel {
                id: ChannelId(1),
                guild_id: Some(GuildId(2)),
                kind: ChannelType::GuildCategory,
                name: "foo".to_owned(),
                nsfw: false,
                parent_id: None,
                permission_overwrites: Vec::new(),
                position: 3,
            };

            serde_test::assert_tokens(
                &expected,
                &[
                    Token::Struct {
                        name: "CategoryChannel",
                        len: 8,
                    },
                    Token::String("id"),
                    Token::NewtypeStruct { name: "ChannelId" },
                    Token::String("1"),
                    Token::String("guild_id"),
                    Token::Some,
                    Token::NewtypeStruct { name: "GuildId" },
                    Token::String("2"),
                    Token::String("type"),
                    Token::U8(4),
                    Token::String("name"),
                    Token::String("foo"),
                    Token::String("nsfw"),
                    Token::Bool(false),
                    Token::String("parent_id"),
                    Token::None,
                    Token::String("permission_overwrites"),
                    Token::Seq { len: Some(0) },
                    Token::SeqEnd,
                    Token::String("position"),
                    Token::I64(3),
                    Token::StructEnd,
                ],
            );
        }

        #[test]
        fn test_guild_text_channel_serialization() {
            let expected = GuildChannel::Text(TextChannel {
                id: ChannelId(1),
                guild_id: Some(GuildId(2)),
                kind: ChannelType::GuildText,
                last_message_id: None,
                last_pin_timestamp: None,
                name: "foo".to_owned(),
                nsfw: true,
                permission_overwrites: Vec::new(),
                parent_id: None,
                position: 3,
                rate_limit_per_user: Some(10),
                topic: Some("a topic".to_owned()),
            });

            serde_test::assert_tokens(
                &expected,
                &[
                    Token::Struct {
                        name: "TextChannel",
                        len: 12,
                    },
                    Token::String("id"),
                    Token::NewtypeStruct { name: "ChannelId" },
                    Token::String("1"),
                    Token::String("guild_id"),
                    Token::Some,
                    Token::NewtypeStruct { name: "GuildId" },
                    Token::String("2"),
                    Token::String("type"),
                    Token::U8(0),
                    Token::String("last_message_id"),
                    Token::None,
                    Token::String("last_pin_timestamp"),
                    Token::None,
                    Token::String("name"),
                    Token::String("foo"),
                    Token::String("nsfw"),
                    Token::Bool(true),
                    Token::String("permission_overwrites"),
                    Token::Seq { len: Some(0) },
                    Token::SeqEnd,
                    Token::String("parent_id"),
                    Token::None,
                    Token::String("position"),
                    Token::I64(3),
                    Token::String("rate_limit_per_user"),
                    Token::Some,
                    Token::U64(10),
                    Token::String("topic"),
                    Token::Some,
                    Token::String("a topic"),
                    Token::StructEnd,
                ],
            );
        }

        #[test]
        fn test_guild_voice_channel_serialization() {
            let expected = GuildChannel::Voice(VoiceChannel {
                id: ChannelId(1),
                bitrate: 124_000,
                guild_id: Some(GuildId(2)),
                kind: ChannelType::GuildVoice,
                name: "foo".to_owned(),
                permission_overwrites: Vec::new(),
                parent_id: None,
                position: 3,
                user_limit: Some(7),
            });

            serde_test::assert_tokens(
                &expected,
                &[
                    Token::Struct {
                        name: "VoiceChannel",
                        len: 9,
                    },
                    Token::String("id"),
                    Token::NewtypeStruct { name: "ChannelId" },
                    Token::String("1"),
                    Token::String("bitrate"),
                    Token::U64(124_000),
                    Token::String("guild_id"),
                    Token::Some,
                    Token::NewtypeStruct { name: "GuildId" },
                    Token::String("2"),
                    Token::String("type"),
                    Token::U8(2),
                    Token::String("name"),
                    Token::String("foo"),
                    Token::String("permission_overwrites"),
                    Token::Seq { len: Some(0) },
                    Token::SeqEnd,
                    Token::String("parent_id"),
                    Token::None,
                    Token::String("position"),
                    Token::I64(3),
                    Token::String("user_limit"),
                    Token::Some,
                    Token::U64(7),
                    Token::StructEnd,
                ],
            );
        }
    }
}
