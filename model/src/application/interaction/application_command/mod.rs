mod data;
mod option;
mod resolved;

pub use self::{
    data::CommandData,
    option::{CommandDataOption, CommandOptionValue},
    resolved::{CommandInteractionDataResolved, InteractionChannel, InteractionMember},
};

use crate::{
    application::interaction::InteractionType,
    guild::PartialMember,
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, InteractionMarker, UserMarker},
        Id,
    },
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
    pub application_id: Id<ApplicationMarker>,
    /// ID of the channel the interaction was invoked in.
    pub channel_id: Id<ChannelMarker>,
    /// Data from the invoked command.
    pub data: CommandData,
    /// ID of the guild the interaction was invoked in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// Guild's preferred locale.
    ///
    /// Present when the command is used in a guild.
    ///
    /// Defaults to `en-US`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_locale: Option<String>,
    /// ID of the interaction.
    pub id: Id<InteractionMarker>,
    /// Kind of the interaction.
    #[serde(rename = "type")]
    pub kind: InteractionType,
    /// Selected language of the user who invoked the interaction.
    pub locale: String,
    /// Member that invoked the interaction.
    ///
    /// Present when the command is used in a guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<PartialMember>,
    /// Token of the interaction.
    pub token: String,
    /// User that invoked the interaction.
    ///
    /// Present when the command is used in a direct message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

impl ApplicationCommand {
    /// ID of the user that invoked the interaction.
    ///
    /// This will first check for the [`member`]'s
    /// [`user`][`PartialMember::user`]'s ID and, if not present, then check the
    /// [`user`]'s ID.
    ///
    /// [`member`]: Self::member
    /// [`user`]: Self::user
    pub const fn author_id(&self) -> Option<Id<UserMarker>> {
        super::author_id(self.user.as_ref(), self.member.as_ref())
    }

    /// Whether the interaction was invoked in a DM.
    pub const fn is_dm(&self) -> bool {
        self.user.is_some()
    }

    /// Whether the interaction was invoked in a guild.
    pub const fn is_guild(&self) -> bool {
        self.member.is_some()
    }
}
