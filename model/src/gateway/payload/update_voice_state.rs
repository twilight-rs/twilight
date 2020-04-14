use crate::{
    gateway::opcode::OpCode,
    id::{ChannelId, GuildId},
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UpdateVoiceState {
    pub d: UpdateVoiceStateInfo,
    pub op: OpCode,
}

impl UpdateVoiceState {
    pub fn new(
        guild_id: impl Into<GuildId>,
        channel_id: impl Into<Option<ChannelId>>,
        self_deaf: bool,
        self_mute: bool,
    ) -> Self {
        Self {
            d: UpdateVoiceStateInfo::new(guild_id, channel_id, self_deaf, self_mute),
            op: OpCode::VoiceStateUpdate,
        }
    }
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UpdateVoiceStateInfo {
    pub channel_id: Option<ChannelId>,
    pub guild_id: GuildId,
    pub self_deaf: bool,
    pub self_mute: bool,
}

impl UpdateVoiceStateInfo {
    pub fn new(
        guild_id: impl Into<GuildId>,
        channel_id: impl Into<Option<ChannelId>>,
        self_deaf: bool,
        self_mute: bool,
    ) -> Self {
        Self::_new(guild_id.into(), channel_id.into(), self_deaf, self_mute)
    }

    fn _new(
        guild_id: GuildId,
        channel_id: Option<ChannelId>,
        self_deaf: bool,
        self_mute: bool,
    ) -> Self {
        Self {
            channel_id,
            guild_id,
            self_deaf,
            self_mute,
        }
    }
}
