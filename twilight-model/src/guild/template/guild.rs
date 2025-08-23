use super::TemplateRole;
use crate::{
    channel::Channel,
    guild::{
        AfkTimeout, DefaultMessageNotificationLevel, ExplicitContentFilter, SystemChannelFlags,
        VerificationLevel,
    },
    id::{Id, marker::ChannelMarker},
    util::image_hash::ImageHash,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TemplateGuild {
    pub afk_channel_id: Option<Id<ChannelMarker>>,
    pub afk_timeout: AfkTimeout,
    pub channels: Vec<Channel>,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub description: Option<String>,
    pub explicit_content_filter: ExplicitContentFilter,
    pub icon_hash: Option<ImageHash>,
    pub name: String,
    pub preferred_locale: String,
    pub roles: Vec<TemplateRole>,
    pub system_channel_flags: SystemChannelFlags,
    pub system_channel_id: Option<Id<ChannelMarker>>,
    pub verification_level: VerificationLevel,
}
