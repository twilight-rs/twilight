mod data;
mod option;

pub use self::{
    data::ApplicationCommandAutocompleteData,
    option::{
        ApplicationCommandAutocompleteDataOption, ApplicationCommandAutocompleteDataOptionType,
    },
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

/// Data present in an [`Interaction`] of type [`ApplicationCommandAutocomplete`].
///
/// [`Interaction`]: super::Interaction
/// [`ApplicationCommandAutocomplete`]: super::Interaction::ApplicationCommandAutocomplete
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationCommandAutocomplete {
    /// ID of the associated application.
    pub application_id: Id<ApplicationMarker>,
    /// The channel the interaction was triggered from.
    pub channel_id: Id<ChannelMarker>,
    /// Data from the invoked command.
    pub data: ApplicationCommandAutocompleteData,
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

impl Serialize for ApplicationCommandAutocomplete {
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
        state.serialize_field("type", &InteractionType::ApplicationCommandAutocomplete)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::{command::CommandType, interaction::Interaction},
        datetime::{Timestamp, TimestampParseError},
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_autocomplete() -> Result<(), TimestampParseError> {
        let joined_at = Timestamp::from_str("2015-04-26T06:26:56.936000+00:00")?;

        let value =
            Interaction::ApplicationCommandAutocomplete(Box::new(ApplicationCommandAutocomplete {
                application_id: Id::new(1),
                channel_id: Id::new(2),
                data: ApplicationCommandAutocompleteData {
                    id: Id::new(3),
                    name: "search".into(),
                    kind: CommandType::ChatInput,
                    options: Vec::from([ApplicationCommandAutocompleteDataOption {
                        focused: true,
                        kind: ApplicationCommandAutocompleteDataOptionType::Integer,
                        name: "issue".into(),
                        options: Vec::new(),
                        value: Some("1234".into()),
                    }]),
                    resolved: None,
                },
                guild_id: Some(Id::new(4)),
                guild_locale: None,
                id: Id::new(5),
                locale: "en-US".into(),
                member: Some(PartialMember {
                    avatar: None,
                    communication_disabled_until: None,
                    deaf: false,
                    joined_at,
                    mute: true,
                    nick: Some("a nickname".to_owned()),
                    permissions: None,
                    premium_since: None,
                    roles: Vec::from([Id::new(6)]),
                    user: None,
                }),
                token: "interaction_token".into(),
                user: None,
            }));

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Interaction",
                    len: 9,
                },
                Token::Str("application_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("data"),
                Token::Struct {
                    name: "ApplicationCommandAutocompleteData",
                    len: 4,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("name"),
                Token::Str("search"),
                Token::Str("type"),
                Token::U8(CommandType::ChatInput as u8),
                Token::Str("options"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "ApplicationCommandAutocompleteDataOption",
                    len: 4,
                },
                Token::Str("focused"),
                Token::Bool(true),
                Token::Str("type"),
                Token::U8(ApplicationCommandAutocompleteDataOptionType::Integer as u8),
                Token::Str("name"),
                Token::Str("issue"),
                Token::Str("value"),
                Token::Some,
                Token::Str("1234"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::Str("type"),
                Token::U8(InteractionType::ApplicationCommandAutocomplete as u8),
                Token::Str("locale"),
                Token::Str("en-US"),
                Token::Str("member"),
                Token::Struct {
                    name: "PartialMember",
                    len: 8,
                },
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("joined_at"),
                Token::Str("2015-04-26T06:26:56.936000+00:00"),
                Token::Str("mute"),
                Token::Bool(true),
                Token::Str("nick"),
                Token::Some,
                Token::Str("a nickname"),
                Token::Str("permissions"),
                Token::None,
                Token::Str("roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("6"),
                Token::SeqEnd,
                Token::Str("user"),
                Token::None,
                Token::StructEnd,
                Token::Str("token"),
                Token::Str("interaction_token"),
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
