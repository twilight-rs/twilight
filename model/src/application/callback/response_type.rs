use serde_repr::{Deserialize_repr, Serialize_repr};

/// Contains the possible response type integers for an interaction.
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
pub enum ResponseType {
    Pong = 1,
    ChannelMessageWithSource = 4,
    DeferredChannelMessageWithSource = 5,
    DeferredUpdateMessage = 6,
    UpdateMessage = 7,
}

impl ResponseType {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Pong => "Pong",
            Self::ChannelMessageWithSource => "ChannelMessageWithSource",
            Self::DeferredChannelMessageWithSource => "DeferredChannelMessageWithSource",
            Self::DeferredUpdateMessage => "DeferredUpdateMessage",
            Self::UpdateMessage => "UpdateMessage",
        }
    }
}
