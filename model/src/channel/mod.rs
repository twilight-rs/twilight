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

pub use self::{
    attachment::Attachment,
    category_channel::CategoryChannel,
    channel_mention::ChannelMention,
    channel_type::ChannelType,
    group::Group,
    message::Message,
    private_channel::PrivateChannel,
    reaction::Reaction,
    reaction_type::ReactionType,
    text_channel::TextChannel,
    voice_channel::VoiceChannel,
    webhook::Webhook,
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
    Category(CategoryChannel),
    Text(TextChannel),
    Voice(VoiceChannel),
}

#[cfg(feature = "serde-support")]
mod serde_mappable_seq_support {
    use super::GuildChannel;
    use crate::id::ChannelId;
    use serde_mappable_seq::Key;

    impl Key<'_, ChannelId> for GuildChannel {
        fn key(&self) -> ChannelId {
            match self {
                GuildChannel::Category(c) => c.id,
                GuildChannel::Text(c) => c.id,
                GuildChannel::Voice(c) => c.id,
            }
        }
    }
}
