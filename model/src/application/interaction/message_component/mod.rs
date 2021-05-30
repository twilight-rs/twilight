use crate::channel::Message;
use crate::guild::PartialMember;
use crate::id::{ApplicationId, ChannelId, GuildId, InteractionId, MessageId};
use serde::{self, Deserialize, Serialize};

mod button_data;
use crate::application::interaction::InteractionType;
use crate::channel::message::MessageFlags;
use crate::user::User;
pub use button_data::ButtonInteractionData;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename(serialize = "Interaction"))]
pub struct MessageComponentInteraction {
    /// ID of the associated application.
    pub application_id: ApplicationId,
    /// The channel the interaction was triggered from.
    pub channel_id: ChannelId,
    /// Data from the invoked command.
    pub data: ButtonInteractionData,
    /// ID of the guild the interaction was triggered from.
    pub guild_id: Option<GuildId>,
    /// ID of the interaction.
    pub id: InteractionId,
    /// Kind of the interaction.
    #[serde(rename = "type")]
    pub kind: InteractionType,
    /// Member that triggered the interaction.
    ///
    /// Present when the command is used in a guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<PartialMember>,
    /// Message object for the message this button belongs to.
    /// This is currently NOT validated by the discord api and can be spoofed by malicious users
    pub message: ComponentMessage,
    /// Token of the interaction.
    pub token: String,
    /// User that triggered the interaction.
    ///
    /// Present when the command is used in a direct message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComponentMessage {
    Regular(Message),
    Ephemeral { id: MessageId, flags: MessageFlags },
}

impl ComponentMessage {
    pub fn get_id(&self) -> &MessageId {
        match self {
            ComponentMessage::Regular(message) => &message.id,
            ComponentMessage::Ephemeral { id, .. } => id,
        }
    }
}
