use crate::{
    gateway::opcode::OpCode,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker},
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UpdateVoiceState {
    pub d: UpdateVoiceStateInfo,
    pub op: OpCode,
}

impl UpdateVoiceState {
    pub fn new(
        guild_id: impl Into<Id<GuildMarker>>,
        channel_id: impl Into<Option<Id<ChannelMarker>>>,
        self_deaf: bool,
        self_mute: bool,
    ) -> Self {
        Self {
            d: UpdateVoiceStateInfo::new(guild_id, channel_id, self_deaf, self_mute),
            op: OpCode::VoiceStateUpdate,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UpdateVoiceStateInfo {
    pub channel_id: Option<Id<ChannelMarker>>,
    pub guild_id: Id<GuildMarker>,
    pub self_deaf: bool,
    pub self_mute: bool,
}

impl UpdateVoiceStateInfo {
    pub fn new(
        guild_id: impl Into<Id<GuildMarker>>,
        channel_id: impl Into<Option<Id<ChannelMarker>>>,
        self_deaf: bool,
        self_mute: bool,
    ) -> Self {
        Self::_new(guild_id.into(), channel_id.into(), self_deaf, self_mute)
    }

    const fn _new(
        guild_id: Id<GuildMarker>,
        channel_id: Option<Id<ChannelMarker>>,
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
