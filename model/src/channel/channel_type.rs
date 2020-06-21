use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr)]
#[repr(u8)]
pub enum ChannelType {
    GuildText = 0,
    Private = 1,
    GuildVoice = 2,
    Group = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore = 6,
}

impl ChannelType {
    pub fn name(self) -> &'static str {
        match self {
            Self::Group => "Group",
            Self::GuildCategory => "GuildCategory",
            Self::GuildNews => "GuildNews",
            Self::GuildStore => "GuildStore",
            Self::GuildText => "GuildText",
            Self::GuildVoice => "GuildVoice",
            Self::Private => "Private",
        }
    }
}
