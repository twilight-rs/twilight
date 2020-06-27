use crate::{
    guild::member::{Member, MemberDeserializer},
    id::{ChannelId, GuildId, UserId},
};
use serde::{
    de::{DeserializeSeed, Deserializer, Error as DeError},
    Deserialize, Serialize,
};
use serde_mappable_seq::Key;
use serde_value::Value;

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

impl<'de> Deserialize<'de> for VoiceState {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let VoiceStateIntermediary {
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
        } = VoiceStateIntermediary::deserialize(deserializer)?;

        let member = if let Some(proto_member) = member {
            if let Some(guild_id) = guild_id {
                let member_deserializer = MemberDeserializer::new(guild_id);
                Some(member_deserializer.deserialize(proto_member).map_err(DeError::custom)?)
            } else {
                return Err(DeError::missing_field("guild_id"));
            }
        } else {
            None
        };

        Ok(Self {
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

#[allow(clippy::struct_excessive_bools)]
#[derive(Deserialize)]
struct VoiceStateIntermediary {
    pub channel_id: Option<ChannelId>,
    pub deaf: bool,
    pub guild_id: Option<GuildId>,
    pub member: Option<Value>,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    #[serde(default)]
    pub self_stream: bool,
    pub session_id: String,
    pub suppress: bool,
    pub token: Option<String>,
    pub user_id: UserId,
}
