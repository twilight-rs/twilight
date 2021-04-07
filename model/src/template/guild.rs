use super::TemplateRole;
use crate::{
    channel::GuildChannel,
    guild::{
        DefaultMessageNotificationLevel, ExplicitContentFilter, SystemChannelFlags,
        VerificationLevel,
    },
    id::ChannelId,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TemplateGuild {
    pub afk_channel_id: Option<ChannelId>,
    pub afk_timeout: u64,
    pub channels: Vec<GuildChannel>,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub description: Option<String>,
    pub explicit_content_filter: ExplicitContentFilter,
    pub icon_hash: Option<String>,
    pub name: String,
    pub preferred_locale: String,
    pub region: String,
    pub roles: Vec<TemplateRole>,
    pub system_channel_flags: SystemChannelFlags,
    pub system_channel_id: Option<ChannelId>,
    pub verification_level: VerificationLevel,
}
