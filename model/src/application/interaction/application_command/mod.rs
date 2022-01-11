mod data;

pub use self::data::{
    CommandData, CommandDataOption, CommandInteractionDataResolved, CommandOptionValue,
    InteractionChannel, InteractionMember,
};

use super::InteractionType;
use crate::{
    guild::PartialMember,
    id::{ApplicationId, ChannelId, GuildId, InteractionId},
    user::User,
};
use serde::Serialize;

/// Data present in an [`Interaction`] of type [`ApplicationCommand`].
///
/// [`Interaction`]: super::Interaction
/// [`ApplicationCommand`]: super::Interaction::ApplicationCommand
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename(serialize = "Interaction"))]
pub struct ApplicationCommand {
    /// ID of the associated application.
    pub application_id: ApplicationId,
    /// The channel the interaction was triggered from.
    pub channel_id: ChannelId,
    /// Data from the invoked command.
    pub data: CommandData,
    /// ID of the guild the interaction was triggered from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// The guild's preferred locale.
    ///
    /// Present when the command is used in a guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_locale: Option<String>,
    /// ID of the interaction.
    pub id: InteractionId,
    /// Kind of the interaction.
    #[serde(rename = "type")]
    pub kind: InteractionType,
    /// The selected language of the user who triggered the interaction.
    pub locale: String,
    /// Member that triggered the interaction.
    ///
    /// Present when the command is used in a guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<PartialMember>,
    /// Token of the interaction.
    pub token: String,
    /// User that triggered the interaction.
    ///
    /// Present when the command is used in a direct message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}
