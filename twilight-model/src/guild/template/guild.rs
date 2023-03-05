use super::TemplateRole;
use crate::{
    channel::Channel,
    guild::{
        AfkTimeout, DefaultMessageNotificationLevel, ExplicitContentFilter, SystemChannelFlags,
        VerificationLevel,
    },
    id::{marker::ChannelMarker, Id},
    util::image_hash::ImageHash,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct TemplateGuild {
    #[cfg_attr(feature = "rkyv", with(crate::id::IdNiche))]
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
    #[cfg_attr(feature = "rkyv", with(crate::id::IdNiche))]
    pub system_channel_id: Option<Id<ChannelMarker>>,
    pub verification_level: VerificationLevel,
}
