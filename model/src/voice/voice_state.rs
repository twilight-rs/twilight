use crate::{
    guild::member::{Member, OptionalMemberDeserializer},
    id::{ChannelId, GuildId, UserId},
};
use serde::{
    de::{Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::fmt::{Formatter, Result as FmtResult};

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceState {
    pub channel_id: Option<ChannelId>,
    pub deaf: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    /// Whether this user is streaming via "Go Live".
    #[serde(default)]
    pub self_stream: bool,
    pub session_id: String,
    pub suppress: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    pub user_id: UserId,
    pub request_to_speak_timestamp: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum Field {
    ChannelId,
    Deaf,
    GuildId,
    Member,
    Mute,
    SelfDeaf,
    SelfMute,
    SelfStream,
    SessionId,
    Suppress,
    Token,
    UserId,
    RequestToSpeakTimestamp,
}

struct VoiceStateVisitor;

impl<'de> Visitor<'de> for VoiceStateVisitor {
    type Value = VoiceState;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("struct VoiceState")
    }

    #[allow(clippy::too_many_lines)]
    fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
        let mut channel_id = None;
        let mut deaf = None;
        let mut guild_id = None;
        let mut member = None;
        let mut mute = None;
        let mut self_deaf = None;
        let mut self_mute = None;
        let mut self_stream = None;
        let mut session_id = None;
        let mut suppress = None;
        let mut token = None;
        let mut user_id = None;
        let mut request_to_speak_timestamp = None;

        let span = tracing::trace_span!("deserializing voice state");
        let _span_enter = span.enter();

        loop {
            let span_child = tracing::trace_span!("iterating over element");
            let _span_child_enter = span_child.enter();

            let key = match map.next_key() {
                Ok(Some(key)) => {
                    tracing::trace!(?key, "found key");

                    key
                }
                Ok(None) => break,
                Err(why) => {
                    // Encountered when we run into an unknown key.
                    map.next_value::<IgnoredAny>()?;

                    tracing::trace!("ran into an unknown key: {:?}", why);

                    continue;
                }
            };

            match key {
                Field::ChannelId => {
                    if channel_id.is_some() {
                        return Err(DeError::duplicate_field("channel_id"));
                    }

                    channel_id = map.next_value()?;
                }
                Field::Deaf => {
                    if deaf.is_some() {
                        return Err(DeError::duplicate_field("deaf"));
                    }

                    deaf = Some(map.next_value()?);
                }
                Field::GuildId => {
                    if guild_id.is_some() {
                        return Err(DeError::duplicate_field("guild_id"));
                    }

                    guild_id = map.next_value()?;
                }
                Field::Member => {
                    if member.is_some() {
                        return Err(DeError::duplicate_field("member"));
                    }

                    let deserializer =
                        OptionalMemberDeserializer::new(GuildId::new(1).expect("non zero"));

                    member = map.next_value_seed(deserializer)?;
                }
                Field::Mute => {
                    if mute.is_some() {
                        return Err(DeError::duplicate_field("mute"));
                    }

                    mute = Some(map.next_value()?);
                }
                Field::SelfDeaf => {
                    if self_deaf.is_some() {
                        return Err(DeError::duplicate_field("self_deaf"));
                    }

                    self_deaf = Some(map.next_value()?);
                }
                Field::SelfMute => {
                    if self_mute.is_some() {
                        return Err(DeError::duplicate_field("self_mute"));
                    }

                    self_mute = Some(map.next_value()?);
                }
                Field::SelfStream => {
                    if self_stream.is_some() {
                        return Err(DeError::duplicate_field("self_stream"));
                    }

                    self_stream = Some(map.next_value()?);
                }
                Field::SessionId => {
                    if session_id.is_some() {
                        return Err(DeError::duplicate_field("session_id"));
                    }

                    session_id = Some(map.next_value()?);
                }
                Field::Suppress => {
                    if suppress.is_some() {
                        return Err(DeError::duplicate_field("suppress"));
                    }

                    suppress = Some(map.next_value()?);
                }
                Field::Token => {
                    if token.is_some() {
                        return Err(DeError::duplicate_field("token"));
                    }

                    token = map.next_value()?;
                }
                Field::UserId => {
                    if user_id.is_some() {
                        return Err(DeError::duplicate_field("user_id"));
                    }

                    user_id = Some(map.next_value()?);
                }
                Field::RequestToSpeakTimestamp => {
                    if request_to_speak_timestamp.is_some() {
                        return Err(DeError::duplicate_field("request_to_speak_timestamp"));
                    }

                    request_to_speak_timestamp = map.next_value()?;
                }
            }
        }

        let deaf = deaf.ok_or_else(|| DeError::missing_field("deaf"))?;
        let mute = mute.ok_or_else(|| DeError::missing_field("mute"))?;
        let self_deaf = self_deaf.ok_or_else(|| DeError::missing_field("self_deaf"))?;
        let self_mute = self_mute.ok_or_else(|| DeError::missing_field("self_mute"))?;
        let session_id = session_id.ok_or_else(|| DeError::missing_field("session_id"))?;
        let suppress = suppress.ok_or_else(|| DeError::missing_field("suppress"))?;
        let user_id = user_id.ok_or_else(|| DeError::missing_field("user_id"))?;

        let self_stream = self_stream.unwrap_or_default();

        tracing::trace!(
            %deaf,
            %mute,
            %self_deaf,
            %self_mute,
            %self_stream,
            ?session_id,
            %suppress,
            %user_id,
        );

        if let (Some(guild_id), Some(member)) = (guild_id, member.as_mut()) {
            tracing::trace!(%guild_id, ?member, "setting member guild id");

            member.guild_id = guild_id;
        }

        Ok(VoiceState {
            channel_id,
            deaf,
            guild_id,
            member,
            mute,
            self_deaf,
            self_mute,
            self_stream,
            session_id,
            suppress,
            token,
            user_id,
            request_to_speak_timestamp,
        })
    }
}

impl<'de> Deserialize<'de> for VoiceState {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        const FIELDS: &[&str] = &[
            "channel_id",
            "deaf",
            "guild_id",
            "member",
            "mute",
            "self_deaf",
            "self_mute",
            "self_stream",
            "session_id",
            "suppress",
            "token",
            "user_id",
            "request_to_speak_timestamp",
        ];

        deserializer.deserialize_struct("VoiceState", FIELDS, VoiceStateVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, GuildId, Member, UserId, VoiceState};
    use crate::{id::RoleId, user::User};
    use serde_test::Token;

    #[test]
    fn test_voice_state() {
        let value = VoiceState {
            channel_id: Some(ChannelId::new(1).expect("non zero")),
            deaf: false,
            guild_id: Some(GuildId::new(2).expect("non zero")),
            member: None,
            mute: true,
            self_deaf: false,
            self_mute: true,
            self_stream: false,
            session_id: "a".to_owned(),
            suppress: true,
            token: None,
            user_id: UserId::new(3).expect("non zero"),
            request_to_speak_timestamp: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "VoiceState",
                    len: 11,
                },
                Token::Str("channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("2"),
                Token::Str("mute"),
                Token::Bool(true),
                Token::Str("self_deaf"),
                Token::Bool(false),
                Token::Str("self_mute"),
                Token::Bool(true),
                Token::Str("self_stream"),
                Token::Bool(false),
                Token::Str("session_id"),
                Token::Str("a"),
                Token::Str("suppress"),
                Token::Bool(true),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("3"),
                Token::Str("request_to_speak_timestamp"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_voice_state_complete() {
        let value = VoiceState {
            channel_id: Some(ChannelId::new(1).expect("non zero")),
            deaf: false,
            guild_id: Some(GuildId::new(2).expect("non zero")),
            member: Some(Member {
                deaf: false,
                guild_id: GuildId::new(2).expect("non zero"),
                hoisted_role: Some(RoleId::new(2).expect("non zero")),
                joined_at: Some("timestamp".to_owned()),
                mute: true,
                nick: Some("twilight".to_owned()),
                pending: false,
                premium_since: Some("timestamp".to_owned()),
                roles: Vec::new(),
                user: User {
                    accent_color: None,
                    avatar: None,
                    banner: None,
                    bot: false,
                    discriminator: 1,
                    email: None,
                    flags: None,
                    id: UserId::new(3).expect("non zero"),
                    locale: None,
                    mfa_enabled: None,
                    name: "twilight".to_owned(),
                    premium_type: None,
                    public_flags: None,
                    system: None,
                    verified: None,
                },
            }),
            mute: true,
            self_deaf: false,
            self_mute: true,
            self_stream: false,
            session_id: "a".to_owned(),
            suppress: true,
            token: Some("abc".to_owned()),
            user_id: UserId::new(3).expect("non zero"),
            request_to_speak_timestamp: Some("2021-04-21T22:16:50+0000".to_owned()),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "VoiceState",
                    len: 13,
                },
                Token::Str("channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("2"),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "Member",
                    len: 10,
                },
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("2"),
                Token::Str("hoisted_role"),
                Token::Some,
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("2"),
                Token::Str("joined_at"),
                Token::Some,
                Token::Str("timestamp"),
                Token::Str("mute"),
                Token::Bool(true),
                Token::Str("nick"),
                Token::Some,
                Token::Str("twilight"),
                Token::Str("pending"),
                Token::Bool(false),
                Token::Str("premium_since"),
                Token::Some,
                Token::Str("timestamp"),
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("user"),
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::None,
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("3"),
                Token::Str("username"),
                Token::Str("twilight"),
                Token::StructEnd,
                Token::StructEnd,
                Token::Str("mute"),
                Token::Bool(true),
                Token::Str("self_deaf"),
                Token::Bool(false),
                Token::Str("self_mute"),
                Token::Bool(true),
                Token::Str("self_stream"),
                Token::Bool(false),
                Token::Str("session_id"),
                Token::Str("a"),
                Token::Str("suppress"),
                Token::Bool(true),
                Token::Str("token"),
                Token::Some,
                Token::Str("abc"),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("3"),
                Token::Str("request_to_speak_timestamp"),
                Token::Some,
                Token::Str("2021-04-21T22:16:50+0000"),
                Token::StructEnd,
            ],
        );
    }
}
