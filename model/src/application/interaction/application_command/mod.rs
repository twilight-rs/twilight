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
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, InteractionMarker},
        Id,
    },
    user::User,
};
use serde::{ser::SerializeStruct, Serialize};

/// Data present in an [`Interaction`] of type [`ApplicationCommand`].
///
/// [`Interaction`]: super::Interaction
/// [`ApplicationCommand`]: super::Interaction::ApplicationCommand
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationCommand {
    /// ID of the associated application.
    pub application_id: Id<ApplicationMarker>,
    /// The channel the interaction was triggered from.
    pub channel_id: Id<ChannelMarker>,
    /// Data from the invoked command.
    pub data: CommandData,
    /// ID of the guild the interaction was triggered from.
    pub guild_id: Option<Id<GuildMarker>>,
    /// Guild's preferred locale.
    ///
    /// Present when the command is used in a guild.
    ///
    /// Defaults to `en-US`.
    pub guild_locale: Option<String>,
    /// ID of the interaction.
    pub id: Id<InteractionMarker>,
    /// Selected language of the user who triggered the interaction.
    pub locale: String,
    /// Member that triggered the interaction.
    ///
    /// Present when the command is used in a guild.
    pub member: Option<PartialMember>,
    /// Token of the interaction.
    pub token: String,
    /// User that triggered the interaction.
    ///
    /// Present when the command is used in a direct message.
    pub user: Option<User>,
}

impl Serialize for ApplicationCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let len = 7
            + usize::from(self.guild_id.is_some())
            + usize::from(self.guild_locale.is_some())
            + usize::from(self.member.is_some())
            + usize::from(self.user.is_some());

        let mut state = serializer.serialize_struct("Interaction", len)?;
        state.serialize_field("application_id", &self.application_id)?;
        state.serialize_field("channel_id", &self.channel_id)?;
        state.serialize_field("data", &self.data)?;
        if let Some(guild_id) = self.guild_id {
            state.serialize_field("guild_id", &guild_id)?;
        }
        if let Some(guild_locale) = &self.guild_locale {
            state.serialize_field("guild_locale", &guild_locale)?;
        }
        state.serialize_field("id", &self.id)?;
        state.serialize_field("type", &InteractionType::ApplicationCommand)?;
        state.serialize_field("locale", &self.locale)?;
        if let Some(member) = &self.member {
            state.serialize_field("member", &member)?;
        }
        state.serialize_field("token", &self.token)?;
        if let Some(user) = &self.user {
            state.serialize_field("user", &user)?;
        }
        state.end()
    }
}
