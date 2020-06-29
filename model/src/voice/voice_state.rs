use crate::{
    guild::member::{Member, OptionalMemberDeserializer},
    id::{ChannelId, GuildId, UserId},
};
use serde::{
    de::{Deserializer, Error as DeError, MapAccess, Visitor},
    Deserialize, Serialize,
};
use serde_mappable_seq::Key;
use std::fmt::{Formatter, Result as FmtResult};

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceState {
    pub channel_id: Option<ChannelId>,
    pub deaf: bool,
    pub guild_id: Option<GuildId>,
    pub member: Option<Member>,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    /// Whether this user is streaming via "Go Live".
    #[serde(default)]
    pub self_stream: bool,
    pub session_id: String,
    pub suppress: bool,
    pub token: Option<String>,
    pub user_id: UserId,
}

impl Key<'_, UserId> for VoiceState {
    fn key(&self) -> UserId {
        self.user_id
    }
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

        loop {
            let key = match map.next_key() {
                Ok(Some(key)) => key,
                Ok(None) => break,
                Err(_) => {
                    // Encountered when we run into an unknown key.
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

                    let deserializer = OptionalMemberDeserializer::new(GuildId(0));

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

        if let (Some(guild_id), Some(member)) = (guild_id, member.as_mut()) {
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
        ];

        deserializer.deserialize_struct("VoiceState", FIELDS, VoiceStateVisitor)
    }
}
