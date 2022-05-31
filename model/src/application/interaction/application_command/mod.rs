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
    ///
    /// Should always be `InteractionType::ApplicationCommand`.
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

#[cfg(test)]
mod tests {
    use super::{ApplicationCommand, CommandData, CommandOptionValue};
    use crate::{
        application::{
            command::CommandType,
            interaction::{application_command::CommandDataOption, tests::user, InteractionType},
        },
        guild::PartialMember,
        id::{
            marker::{
                ApplicationMarker, ChannelMarker, GuildMarker, InteractionMarker, UserMarker,
            },
            Id,
        },
        util::datetime::{Timestamp, TimestampParseError},
    };
    use std::str::FromStr;

    const USER_ID: Id<UserMarker> = Id::new(7);

    #[test]
    fn test_author_id() -> Result<(), TimestampParseError> {
        let joined_at = Timestamp::from_str("2020-02-02T02:02:02.020000+00:00")?;

        let in_guild = ApplicationCommand {
            application_id: Id::<ApplicationMarker>::new(1),
            channel_id: Id::<ChannelMarker>::new(1),
            data: CommandData {
                id: Id::new(3),
                name: "search".to_owned(),
                kind: CommandType::ChatInput,
                options: Vec::from([CommandDataOption {
                    name: "issue".to_owned(),
                    value: CommandOptionValue::Integer(1234),
                }]),
                resolved: None,
                target_id: None,
            },
            guild_id: Some(Id::<GuildMarker>::new(1)),
            guild_locale: None,
            id: Id::<InteractionMarker>::new(1),
            kind: InteractionType::ApplicationCommand,
            locale: "en-US".to_owned(),
            member: Some(PartialMember {
                avatar: None,
                deaf: false,
                joined_at,
                mute: false,
                nick: None,
                permissions: None,
                premium_since: None,
                roles: Vec::new(),
                user: Some(user(USER_ID)),
                communication_disabled_until: None,
            }),
            token: "TOKEN".to_owned(),
            user: None,
        };

        assert_eq!(Some(USER_ID), in_guild.author_id());
        assert!(in_guild.is_guild());

        let in_dm = ApplicationCommand {
            member: None,
            user: Some(user(USER_ID)),
            ..in_guild
        };
        assert_eq!(Some(USER_ID), in_dm.author_id());
        assert!(in_dm.is_dm());

        Ok(())
    }
}
