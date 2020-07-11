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

use crate::id::{ChannelId, MessageId};
use serde::{
    de::{
        DeserializeSeed, Deserializer, Error as DeError, IgnoredAny, MapAccess, SeqAccess, Visitor,
    },
    Deserialize, Serialize,
};
use serde_mappable_seq::Key;
use std::{
    collections::HashMap,
    fmt::{Formatter, Result as FmtResult},
};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Channel {
    Group(Group),
    Guild(GuildChannel),
    Private(PrivateChannel),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GuildChannel {
    Category(CategoryChannel),
    Text(TextChannel),
    Voice(VoiceChannel),
}

impl Key<'_, ChannelId> for GuildChannel {
    fn key(&self) -> ChannelId {
        match self {
            Self::Category(c) => c.id,
            Self::Text(c) => c.id,
            Self::Voice(c) => c.id,
        }
    }
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum GuildChannelField {
    Bitrate,
    GuildId,
    Id,
    LastMessageId,
    LastPinTimestamp,
    Name,
    Nsfw,
    ParentId,
    PermissionOverwrites,
    Position,
    RateLimitPerUser,
    Topic,
    Type,
    UserLimit,
}

struct GuildChannelVisitor;

impl<'de> Visitor<'de> for GuildChannelVisitor {
    type Value = GuildChannel;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("enum GuildChannel")
    }

    #[allow(clippy::too_many_lines)]
    fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
        const VARIANTS: &[&str] = &[
            "GuildCategory",
            "GuildNews",
            "GuildStore",
            "GuildText",
            "GuildVoice",
        ];

        let mut bitrate = None;
        let mut guild_id = None;
        let mut id = None;
        let mut kind = None;
        let mut last_message_id: Option<Option<MessageId>> = None;
        let mut last_pin_timestamp: Option<Option<String>> = None;
        let mut name = None;
        let mut nsfw = None;
        let mut parent_id: Option<Option<ChannelId>> = None;
        let mut permission_overwrites = None;
        let mut position = None;
        let mut rate_limit_per_user = None;
        let mut topic: Option<Option<String>> = None;
        let mut user_limit = None;

        loop {
            let key = match map.next_key() {
                Ok(Some(key)) => key,
                Ok(None) => break,
                Err(_) => {
                    // Encountered when we run into an unknown key.
                    map.next_value::<IgnoredAny>()?;

                    continue;
                }
            };

            match key {
                GuildChannelField::Bitrate => {
                    if bitrate.is_some() {
                        return Err(DeError::duplicate_field("bitrate"));
                    }

                    bitrate = Some(map.next_value()?);
                }
                GuildChannelField::GuildId => {
                    if guild_id.is_some() {
                        return Err(DeError::duplicate_field("guild_id"));
                    }

                    guild_id = Some(map.next_value()?);
                }
                GuildChannelField::Id => {
                    if id.is_some() {
                        return Err(DeError::duplicate_field("id"));
                    }

                    id = Some(map.next_value()?);
                }
                GuildChannelField::Type => {
                    if kind.is_some() {
                        return Err(DeError::duplicate_field("type"));
                    }

                    kind = Some(map.next_value()?);
                }
                GuildChannelField::LastMessageId => {
                    if last_message_id.is_some() {
                        return Err(DeError::duplicate_field("last_message_id"));
                    }

                    last_message_id = Some(map.next_value()?);
                }
                GuildChannelField::LastPinTimestamp => {
                    if last_pin_timestamp.is_some() {
                        return Err(DeError::duplicate_field("last_pin_timestamp"));
                    }

                    last_pin_timestamp = Some(map.next_value()?);
                }
                GuildChannelField::Name => {
                    if name.is_some() {
                        return Err(DeError::duplicate_field("name"));
                    }

                    name = Some(map.next_value()?);
                }
                GuildChannelField::Nsfw => {
                    if nsfw.is_some() {
                        return Err(DeError::duplicate_field("nsfw"));
                    }

                    nsfw = Some(map.next_value()?);
                }
                GuildChannelField::ParentId => {
                    if parent_id.is_some() {
                        return Err(DeError::duplicate_field("parent_id"));
                    }

                    parent_id = Some(map.next_value()?);
                }
                GuildChannelField::PermissionOverwrites => {
                    if permission_overwrites.is_some() {
                        return Err(DeError::duplicate_field("permission_overwrites"));
                    }

                    permission_overwrites = Some(map.next_value()?);
                }
                GuildChannelField::Position => {
                    if position.is_some() {
                        return Err(DeError::duplicate_field("position"));
                    }

                    position = Some(map.next_value()?);
                }
                GuildChannelField::RateLimitPerUser => {
                    if rate_limit_per_user.is_some() {
                        return Err(DeError::duplicate_field("rate_limit_per_user"));
                    }

                    rate_limit_per_user = Some(map.next_value()?);
                }
                GuildChannelField::Topic => {
                    if topic.is_some() {
                        return Err(DeError::duplicate_field("topic"));
                    }

                    topic = Some(map.next_value()?);
                }
                GuildChannelField::UserLimit => {
                    if user_limit.is_some() {
                        return Err(DeError::duplicate_field("user_limit"));
                    }

                    user_limit = Some(map.next_value()?);
                }
            }
        }

        // Now let's get all of the fields required by every guild channel
        // type.
        let id = id.ok_or_else(|| DeError::missing_field("id"))?;
        let kind = kind.ok_or_else(|| DeError::missing_field("type"))?;
        let name = name.ok_or_else(|| DeError::missing_field("name"))?;
        let permission_overwrites =
            permission_overwrites.ok_or_else(|| DeError::missing_field("permission_overwrites"))?;
        let position = position.ok_or_else(|| DeError::missing_field("position"))?;

        let nsfw = nsfw.unwrap_or_default();
        let parent_id = parent_id.unwrap_or_default();

        Ok(match kind {
            ChannelType::GuildCategory => GuildChannel::Category(CategoryChannel {
                id,
                guild_id,
                kind,
                name,
                nsfw,
                permission_overwrites,
                parent_id,
                position,
            }),
            ChannelType::GuildVoice => {
                let bitrate = bitrate.ok_or_else(|| DeError::missing_field("bitrate"))?;
                let user_limit = user_limit.ok_or_else(|| DeError::missing_field("user_limit"))?;

                GuildChannel::Voice(VoiceChannel {
                    id,
                    bitrate,
                    guild_id,
                    kind,
                    name,
                    permission_overwrites,
                    parent_id,
                    position,
                    user_limit,
                })
            }
            ChannelType::GuildNews | ChannelType::GuildStore | ChannelType::GuildText => {
                let last_message_id = last_message_id.unwrap_or_default();
                let last_pin_timestamp = last_pin_timestamp.unwrap_or_default();
                let topic = topic.unwrap_or_default();

                GuildChannel::Text(TextChannel {
                    id,
                    guild_id,
                    kind,
                    last_message_id,
                    last_pin_timestamp,
                    name,
                    nsfw,
                    permission_overwrites,
                    parent_id,
                    position,
                    rate_limit_per_user,
                    topic,
                })
            }
            other => return Err(DeError::unknown_variant(other.name(), VARIANTS)),
        })
    }
}

impl<'de> Deserialize<'de> for GuildChannel {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(GuildChannelVisitor)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GuildChannelMapDeserializer;

struct GuildChannelMapVisitor;

impl<'de> Visitor<'de> for GuildChannelMapVisitor {
    type Value = HashMap<ChannelId, GuildChannel>;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a sequence of guild channels")
    }

    fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
        let mut map = seq
            .size_hint()
            .map_or_else(HashMap::new, HashMap::with_capacity);

        while let Some(channel) = seq.next_element()? {
            let id = match channel {
                GuildChannel::Category(ref c) => c.id,
                GuildChannel::Text(ref t) => t.id,
                GuildChannel::Voice(ref v) => v.id,
            };

            map.insert(id, channel);
        }

        Ok(map)
    }
}

impl<'de> DeserializeSeed<'de> for GuildChannelMapDeserializer {
    type Value = HashMap<ChannelId, GuildChannel>;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_seq(GuildChannelMapVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::{CategoryChannel, ChannelType, GuildChannel, TextChannel, VoiceChannel};
    use crate::{
        channel::permission_overwrite::PermissionOverwrite,
        id::{ChannelId, GuildId, MessageId},
    };

    // The deserializer for GuildChannel should skip over fields names that
    // it couldn't deserialize.
    #[test]
    fn test_guild_channel_unknown_field_deserialization() {
        let input = serde_json::json!({
            "type": 0,
            "topic": "a",
            "rate_limit_per_user": 0,
            "position": 0,
            "permission_overwrites": [],
            "parent_id": null,
            "nsfw": false,
            "name": "hey",
            "last_message_id": "3",
            "id": "2",
            "guild_id": "1",
            "guild_hashes": {
                "version": 1,
                "roles": {
                    "hash": "aaaaaaaaaaa"
                },
                "metadata": {
                    "hash": "bbbbbbbbbbb"
                },
                "channels": {
                    "hash": "ccccccccccc"
                }
            },
            "unknown_field": "the deserializer should skip unknown field names",
        });

        let expected = GuildChannel::Text(TextChannel {
            guild_id: Some(GuildId(1)),
            id: ChannelId(2),
            kind: ChannelType::GuildText,
            last_message_id: Some(MessageId(3)),
            last_pin_timestamp: None,
            name: "hey".to_owned(),
            nsfw: false,
            parent_id: None,
            permission_overwrites: Vec::new(),
            position: 0,
            rate_limit_per_user: Some(0),
            topic: Some("a".to_owned()),
        });

        assert_eq!(expected, serde_json::from_value(input).unwrap());
    }

    #[test]
    fn test_guild_category_channel_deserialization() {
        let expected = GuildChannel::Category(CategoryChannel {
            id: ChannelId(1),
            guild_id: Some(GuildId(2)),
            kind: ChannelType::GuildCategory,
            name: "foo".to_owned(),
            nsfw: false,
            parent_id: None,
            permission_overwrites: Vec::new(),
            position: 3,
        });
        let permission_overwrites: Vec<PermissionOverwrite> = Vec::new();

        assert_eq!(
            expected,
            serde_json::from_value(serde_json::json!({
                "id": "1",
                "guild_id": Some("2"),
                "name": "foo",
                "nsfw": false,
                "parent_id": None::<ChannelId>,
                "permission_overwrites": permission_overwrites,
                "position": 3,
                "type": 4,
            }))
            .unwrap()
        );
    }

    #[test]
    fn test_guild_news_channel_deserialization() {
        let expected = GuildChannel::Text(TextChannel {
            id: ChannelId(1),
            guild_id: Some(GuildId(2)),
            kind: ChannelType::GuildNews,
            last_message_id: Some(MessageId(4)),
            last_pin_timestamp: None,
            name: "news".to_owned(),
            nsfw: true,
            permission_overwrites: Vec::new(),
            parent_id: Some(ChannelId(5)),
            position: 3,
            rate_limit_per_user: None,
            topic: Some("a news channel".to_owned()),
        });
        let permission_overwrites: Vec<PermissionOverwrite> = Vec::new();

        assert_eq!(
            expected,
            serde_json::from_value(serde_json::json!({
                "id": "1",
                "guild_id": "2",
                "name": "news",
                "nsfw": true,
                "last_message_id": "4",
                "parent_id": "5",
                "permission_overwrites": permission_overwrites,
                "position": 3,
                "topic": "a news channel",
                "type": ChannelType::GuildNews,
            }))
            .unwrap()
        );
    }

    #[test]
    fn test_guild_store_channel_deserialization() {
        let expected = GuildChannel::Text(TextChannel {
            id: ChannelId(1),
            guild_id: Some(GuildId(2)),
            kind: ChannelType::GuildStore,
            last_message_id: None,
            last_pin_timestamp: None,
            name: "store".to_owned(),
            nsfw: false,
            permission_overwrites: Vec::new(),
            parent_id: None,
            position: 2,
            rate_limit_per_user: None,
            topic: Some("a store channel".to_owned()),
        });
        let permission_overwrites: Vec<PermissionOverwrite> = Vec::new();

        assert_eq!(
            expected,
            serde_json::from_value(serde_json::json!({
                "id": "1",
                "guild_id": "2",
                "name": "store",
                "nsfw": false,
                "permission_overwrites": permission_overwrites,
                "position": 2,
                "topic": "a store channel",
                "type": ChannelType::GuildStore,
            }))
            .unwrap()
        );
    }

    #[test]
    fn test_guild_text_channel_deserialization() {
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
        let permission_overwrites: Vec<PermissionOverwrite> = Vec::new();

        assert_eq!(
            expected,
            serde_json::from_value(serde_json::json!({
                "id": "1",
                "guild_id": "2",
                "last_message_id": None::<MessageId>,
                "last_pin_timestamp": None::<MessageId>,
                "name": "foo",
                "nsfw": true,
                "permission_overwrites": permission_overwrites,
                "parent_id": None::<ChannelId>,
                "position": 3,
                "rate_limit_per_user": 10,
                "topic": Some("a topic"),
                "type": 0,
            }))
            .unwrap()
        );
    }

    #[test]
    fn test_guild_voice_channel_deserialization() {
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
        let permission_overwrites: Vec<PermissionOverwrite> = Vec::new();

        assert_eq!(
            expected,
            serde_json::from_value(serde_json::json!({
                "id": "1",
                "bitrate": 124_000,
                "guild_id": "2",
                "name": "foo",
                "permission_overwrites": permission_overwrites,
                "parent_id": None::<ChannelId>,
                "position": 3,
                "type": 2,
                "user_limit": 7,
            }))
            .unwrap()
        );
    }
}
