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
    guild::{PartialMember, Permissions},
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, InteractionMarker, UserMarker},
        Id,
    },
    user::User,
};
use serde::Serialize;

/// Data present in an [`Interaction`] of type [`ApplicationCommandAutocomplete`].
///
/// [`Interaction`]: super::Interaction
/// [`ApplicationCommandAutocomplete`]: super::Interaction::ApplicationCommandAutocomplete
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename(serialize = "Interaction"))]
pub struct ApplicationCommandAutocomplete {
    /// App's permissions in the channel the interaction was sent from.
    ///
    /// None if the interaction happens in a direct message channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_permissions: Option<Permissions>,
    /// ID of the associated application.
    pub application_id: Id<ApplicationMarker>,
    /// ID of the channel the interaction was invoked in.
    pub channel_id: Id<ChannelMarker>,
    /// Data from the invoked command.
    pub data: ApplicationCommandAutocompleteData,
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
    /// Should always be `InteractionType::ApplicationCommandAutocomplete`.
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

impl ApplicationCommandAutocomplete {
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
    use super::*;
    use crate::{
        application::{
            command::CommandType,
            interaction::{tests::user, Interaction},
        },
        guild::Permissions,
        util::datetime::{Timestamp, TimestampParseError},
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn autocomplete() -> Result<(), TimestampParseError> {
        let joined_at = Timestamp::from_str("2015-04-26T06:26:56.936000+00:00")?;

        let value =
            Interaction::ApplicationCommandAutocomplete(Box::new(ApplicationCommandAutocomplete {
                app_permissions: Some(Permissions::SEND_MESSAGES),
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
                kind: InteractionType::ApplicationCommandAutocomplete,
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
                    len: 10,
                },
                Token::Str("app_permissions"),
                Token::Some,
                Token::Str("2048"),
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
                Token::Some,
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
                Token::Some,
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

    const USER_ID: Id<UserMarker> = Id::new(7);

    #[test]
    fn author_id() -> Result<(), TimestampParseError> {
        let joined_at = Timestamp::from_str("2020-02-02T02:02:02.020000+00:00")?;

        let in_guild = ApplicationCommandAutocomplete {
            app_permissions: Some(Permissions::MENTION_EVERYONE),
            application_id: Id::<ApplicationMarker>::new(1),
            channel_id: Id::<ChannelMarker>::new(1),
            data: ApplicationCommandAutocompleteData {
                id: Id::new(3),
                name: "search".to_owned(),
                kind: CommandType::ChatInput,
                options: Vec::from([ApplicationCommandAutocompleteDataOption {
                    focused: true,
                    kind: ApplicationCommandAutocompleteDataOptionType::Integer,
                    name: "issue".to_owned(),
                    options: Vec::new(),
                    value: Some("1234".to_owned()),
                }]),
                resolved: None,
            },
            guild_id: Some(Id::<GuildMarker>::new(1)),
            guild_locale: None,
            id: Id::<InteractionMarker>::new(1),
            kind: InteractionType::ApplicationCommandAutocomplete,
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

        let in_dm = ApplicationCommandAutocomplete {
            member: None,
            user: Some(user(USER_ID)),
            ..in_guild
        };
        assert_eq!(Some(USER_ID), in_dm.author_id());
        assert!(in_dm.is_dm());

        Ok(())
    }
}
