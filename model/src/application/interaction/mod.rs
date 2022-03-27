//! Used when receiving interactions through gateway or webhooks.

pub mod application_command;
pub mod application_command_autocomplete;
pub mod message_component;
pub mod modal;

mod interaction_type;
mod ping;

use self::modal::ModalSubmitInteraction;
pub use self::{
    application_command::ApplicationCommand,
    application_command_autocomplete::ApplicationCommandAutocomplete,
    interaction_type::InteractionType, message_component::MessageComponentInteraction, ping::Ping,
};

use crate::{
    channel::Message,
    guild::PartialMember,
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, InteractionMarker, UserMarker},
        Id,
    },
    user::User,
};
use serde::{
    de::{Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor},
    Deserialize, Serialize,
};
use serde_value::{DeserializerError, Value};
use std::fmt::{Formatter, Result as FmtResult};

/// Payload received when a user executes an interaction.
///
/// Each variant corresponds to `InteractionType` in the Discord Docs. See
/// [Discord Docs/Interaction Object].
///
/// [Discord Docs/Interaction Object]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-structure
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Interaction {
    /// Ping variant.
    Ping(Box<Ping>),
    /// Application command variant.
    ApplicationCommand(Box<ApplicationCommand>),
    /// Application command autocomplete variant.
    ApplicationCommandAutocomplete(Box<ApplicationCommandAutocomplete>),
    /// Message component variant.
    MessageComponent(Box<MessageComponentInteraction>),
    /// Modal submit variant.
    ModalSubmit(Box<ModalSubmitInteraction>),
}

impl Interaction {
    /// Id of the associated application.
    pub const fn application_id(&self) -> Id<ApplicationMarker> {
        match self {
            Self::Ping(ping) => ping.application_id,
            Self::ApplicationCommand(command) => command.application_id,
            Self::ApplicationCommandAutocomplete(command) => command.application_id,
            Self::MessageComponent(component) => component.application_id,
            Self::ModalSubmit(modal) => modal.application_id,
        }
    }

    /// Return the ID of the inner interaction.
    pub const fn id(&self) -> Id<InteractionMarker> {
        match self {
            Self::Ping(ping) => ping.id,
            Self::ApplicationCommand(command) => command.id,
            Self::ApplicationCommandAutocomplete(command) => command.id,
            Self::MessageComponent(component) => component.id,
            Self::ModalSubmit(modal) => modal.id,
        }
    }

    /// Type of interaction.
    pub const fn kind(&self) -> InteractionType {
        match self {
            Interaction::Ping(_) => InteractionType::Ping,
            Interaction::ApplicationCommand(_) => InteractionType::ApplicationCommand,
            Interaction::ApplicationCommandAutocomplete(_) => {
                InteractionType::ApplicationCommandAutocomplete
            }
            Interaction::MessageComponent(_) => InteractionType::MessageComponent,
            Interaction::ModalSubmit(_) => InteractionType::ModalSubmit,
        }
    }
}

impl<'de> Deserialize<'de> for Interaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(InteractionVisitor)
    }
}

const fn author_id(user: Option<&User>, member: Option<&PartialMember>) -> Option<Id<UserMarker>> {
    if let Some(member) = member {
        if let Some(user) = &member.user {
            return Some(user.id);
        }
    }

    if let Some(user) = user {
        return Some(user.id);
    }

    None
}

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum InteractionField {
    ApplicationId,
    ChannelId,
    Data,
    GuildId,
    GuildLocale,
    Id,
    Locale,
    Member,
    Message,
    Token,
    Type,
    User,
}

struct InteractionVisitor;

impl<'de> Visitor<'de> for InteractionVisitor {
    type Value = Interaction;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("enum Interaction")
    }

    #[allow(clippy::too_many_lines)]
    fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
        let mut application_id: Option<Id<ApplicationMarker>> = None;
        let mut channel_id: Option<Id<ChannelMarker>> = None;
        let mut data: Option<Value> = None;
        let mut guild_id: Option<Option<Id<GuildMarker>>> = None;
        let mut guild_locale: Option<Option<String>> = None;
        let mut id: Option<Id<InteractionMarker>> = None;
        let mut member: Option<Option<PartialMember>> = None;
        let mut message: Option<Message> = None;
        let mut token: Option<String> = None;
        let mut kind: Option<InteractionType> = None;
        let mut locale: Option<String> = None;
        let mut user: Option<Option<User>> = None;

        #[cfg(feature = "tracing")]
        let span = tracing::trace_span!("deserializing interaction");
        #[cfg(feature = "tracing")]
        let _span_enter = span.enter();

        loop {
            #[cfg(feature = "tracing")]
            let span_child = tracing::trace_span!("iterating over interaction");
            #[cfg(feature = "tracing")]
            let _span_child_enter = span_child.enter();

            let key = match map.next_key() {
                Ok(Some(key)) => {
                    #[cfg(feature = "tracing")]
                    tracing::trace!(?key, "found key");

                    key
                }
                Ok(None) => break,
                #[cfg(feature = "tracing")]
                Err(why) => {
                    // Encountered when we run into an unknown key.
                    map.next_value::<IgnoredAny>()?;

                    tracing::trace!("ran into an unknown key: {:?}", why);

                    continue;
                }
                #[cfg(not(feature = "tracing"))]
                Err(_) => {
                    // Encountered when we run into an unknown key.
                    map.next_value::<IgnoredAny>()?;

                    continue;
                }
            };

            match key {
                InteractionField::ApplicationId => {
                    if application_id.is_some() {
                        return Err(DeError::duplicate_field("application_id"));
                    }

                    application_id = Some(map.next_value()?);
                }
                InteractionField::ChannelId => {
                    if channel_id.is_some() {
                        return Err(DeError::duplicate_field("channel_id"));
                    }

                    channel_id = Some(map.next_value()?);
                }
                InteractionField::Data => {
                    if data.is_some() {
                        return Err(DeError::duplicate_field("data"));
                    }

                    data = Some(map.next_value()?);
                }
                InteractionField::GuildId => {
                    if guild_id.is_some() {
                        return Err(DeError::duplicate_field("guild_id"));
                    }

                    guild_id = Some(map.next_value()?);
                }
                InteractionField::GuildLocale => {
                    if guild_locale.is_some() {
                        return Err(DeError::duplicate_field("guild_locale"));
                    }

                    guild_locale = Some(map.next_value()?);
                }
                InteractionField::Id => {
                    if id.is_some() {
                        return Err(DeError::duplicate_field("id"));
                    }

                    id = Some(map.next_value()?);
                }
                InteractionField::Locale => {
                    if locale.is_some() {
                        return Err(DeError::duplicate_field("locale"));
                    }

                    locale = Some(map.next_value()?);
                }
                InteractionField::Member => {
                    if member.is_some() {
                        return Err(DeError::duplicate_field("member"));
                    }

                    member = Some(map.next_value()?);
                }
                InteractionField::Message => {
                    if message.is_some() {
                        return Err(DeError::duplicate_field("message"));
                    }

                    message = Some(map.next_value()?);
                }
                InteractionField::Token => {
                    if token.is_some() {
                        return Err(DeError::duplicate_field("token"));
                    }

                    token = Some(map.next_value()?);
                }
                InteractionField::Type => {
                    if kind.is_some() {
                        return Err(DeError::duplicate_field("kind"));
                    }

                    kind = Some(map.next_value()?);
                }
                InteractionField::User => {
                    if user.is_some() {
                        return Err(DeError::duplicate_field("user"));
                    }

                    user = Some(map.next_value()?);
                }
            }
        }

        let application_id =
            application_id.ok_or_else(|| DeError::missing_field("application_id"))?;
        let id = id.ok_or_else(|| DeError::missing_field("id"))?;
        let token = token.ok_or_else(|| DeError::missing_field("token"))?;
        let kind = kind.ok_or_else(|| DeError::missing_field("kind"))?;

        #[cfg(feature = "tracing")]
        tracing::trace!(
            %application_id,
            %id,
            %token,
            ?kind,
            "common fields of all variants exist"
        );

        Ok(match kind {
            InteractionType::Ping => {
                #[cfg(feature = "tracing")]
                tracing::trace!("handling ping");

                Self::Value::Ping(Box::new(Ping {
                    application_id,
                    id,
                    kind,
                    token,
                }))
            }
            InteractionType::ApplicationCommand => {
                let channel_id = channel_id.ok_or_else(|| DeError::missing_field("channel_id"))?;
                let data = data
                    .ok_or_else(|| DeError::missing_field("data"))?
                    .deserialize_into()
                    .map_err(DeserializerError::into_error)?;

                let guild_id = guild_id.unwrap_or_default();
                let guild_locale = guild_locale.unwrap_or_default();
                let locale = locale.ok_or_else(|| DeError::missing_field("locale"))?;
                let member = member.unwrap_or_default();
                let user = user.unwrap_or_default();

                #[cfg(feature = "tracing")]
                tracing::trace!(%channel_id, "handling application command");

                let command = Box::new(ApplicationCommand {
                    application_id,
                    channel_id,
                    data,
                    guild_id,
                    guild_locale,
                    id,
                    kind,
                    locale,
                    member,
                    token,
                    user,
                });

                Self::Value::ApplicationCommand(command)
            }
            InteractionType::ApplicationCommandAutocomplete => {
                let channel_id = channel_id.ok_or_else(|| DeError::missing_field("channel_id"))?;
                let data = data
                    .ok_or_else(|| DeError::missing_field("data"))?
                    .deserialize_into()
                    .map_err(DeserializerError::into_error)?;

                let guild_id = guild_id.unwrap_or_default();
                let guild_locale = guild_locale.unwrap_or_default();
                let locale = locale.ok_or_else(|| DeError::missing_field("locale"))?;
                let member = member.unwrap_or_default();
                let user = user.unwrap_or_default();

                #[cfg(feature = "tracing")]
                tracing::trace!(%channel_id, "handling application command autocomplete");

                let command = Box::new(ApplicationCommandAutocomplete {
                    application_id,
                    channel_id,
                    data,
                    guild_id,
                    guild_locale,
                    id,
                    kind,
                    locale,
                    member,
                    token,
                    user,
                });

                Self::Value::ApplicationCommandAutocomplete(command)
            }
            InteractionType::MessageComponent => {
                let channel_id = channel_id.ok_or_else(|| DeError::missing_field("channel_id"))?;
                let data = data
                    .ok_or_else(|| DeError::missing_field("data"))?
                    .deserialize_into()
                    .map_err(|_| {
                        DeError::custom("expected MessageComponentInteractionData struct")
                    })?;
                let message = message.ok_or_else(|| DeError::missing_field("message"))?;

                let guild_id = guild_id.unwrap_or_default();
                let guild_locale = guild_locale.unwrap_or_default();
                let locale = locale.ok_or_else(|| DeError::missing_field("locale"))?;
                let member = member.unwrap_or_default();
                let user = user.unwrap_or_default();

                Self::Value::MessageComponent(Box::new(MessageComponentInteraction {
                    application_id,
                    channel_id,
                    data,
                    guild_id,
                    guild_locale,
                    id,
                    kind,
                    locale,
                    member,
                    message,
                    token,
                    user,
                }))
            }
            InteractionType::ModalSubmit => {
                let channel_id = channel_id.ok_or_else(|| DeError::missing_field("channel_id"))?;
                let data = data
                    .ok_or_else(|| DeError::missing_field("data"))?
                    .deserialize_into()
                    .map_err(|_| DeError::custom("expected ModalInteractionData struct"))?;

                let guild_id = guild_id.unwrap_or_default();
                let member = member.unwrap_or_default();
                let user = user.unwrap_or_default();

                Self::Value::ModalSubmit(Box::new(ModalSubmitInteraction {
                    application_id,
                    channel_id,
                    data,
                    guild_id,
                    id,
                    kind,
                    member,
                    token,
                    user,
                }))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::{
            command::{CommandOptionType, CommandType},
            interaction::{
                application_command::{
                    ApplicationCommand, CommandData, CommandDataOption,
                    CommandInteractionDataResolved, CommandOptionValue, InteractionMember,
                },
                Interaction, InteractionType,
            },
        },
        datetime::{Timestamp, TimestampParseError},
        guild::{PartialMember, Permissions},
        id::{marker::UserMarker, Id},
        test::image_hash,
        user::User,
    };
    use serde_test::Token;
    use std::{collections::HashMap, str::FromStr};

    pub(super) fn user(id: Id<UserMarker>) -> User {
        User {
            accent_color: None,
            avatar: None,
            banner: None,
            bot: false,
            discriminator: 4444,
            email: None,
            flags: None,
            id,
            locale: None,
            mfa_enabled: None,
            name: "twilight".to_owned(),
            premium_type: None,
            public_flags: None,
            system: None,
            verified: None,
        }
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_interaction_full() -> Result<(), TimestampParseError> {
        let joined_at = Timestamp::from_str("2020-01-01T00:00:00.000000+00:00")?;

        let value = Interaction::ApplicationCommand(Box::new(ApplicationCommand {
            application_id: Id::new(100),
            channel_id: Id::new(200),
            data: CommandData {
                id: Id::new(300),
                name: "command name".into(),
                kind: CommandType::ChatInput,
                options: Vec::from([CommandDataOption {
                    focused: false,
                    name: "member".into(),
                    value: CommandOptionValue::User(Id::new(600)),
                }]),
                resolved: Some(CommandInteractionDataResolved {
                    attachments: HashMap::new(),
                    channels: HashMap::new(),
                    members: IntoIterator::into_iter([(
                        Id::new(600),
                        InteractionMember {
                            avatar: None,
                            communication_disabled_until: None,
                            joined_at,
                            nick: Some("nickname".into()),
                            pending: false,
                            permissions: Permissions::empty(),
                            premium_since: None,
                            roles: Vec::new(),
                        },
                    )])
                    .collect(),
                    messages: HashMap::new(),
                    roles: HashMap::new(),
                    users: IntoIterator::into_iter([(
                        Id::new(600),
                        User {
                            accent_color: None,
                            avatar: Some(image_hash::AVATAR),
                            banner: None,
                            bot: false,
                            discriminator: 1111,
                            email: None,
                            flags: None,
                            id: Id::new(600),
                            locale: None,
                            mfa_enabled: None,
                            name: "username".into(),
                            premium_type: None,
                            public_flags: None,
                            system: None,
                            verified: None,
                        },
                    )])
                    .collect(),
                }),
                target_id: None,
            },
            guild_id: Some(Id::new(400)),
            guild_locale: Some("de".to_owned()),
            id: Id::new(500),
            kind: InteractionType::ApplicationCommand,
            locale: "en-GB".to_owned(),
            member: Some(PartialMember {
                avatar: None,
                communication_disabled_until: None,
                deaf: false,
                joined_at,
                mute: false,
                nick: Some("nickname".into()),
                permissions: Some(Permissions::empty()),
                premium_since: None,
                roles: Vec::new(),
                user: Some(User {
                    accent_color: None,
                    avatar: Some(image_hash::AVATAR),
                    banner: None,
                    bot: false,
                    discriminator: 1111,
                    email: None,
                    flags: None,
                    id: Id::new(600),
                    locale: None,
                    mfa_enabled: None,
                    name: "username".into(),
                    premium_type: None,
                    public_flags: None,
                    system: None,
                    verified: None,
                }),
            }),
            token: "interaction token".into(),
            user: None,
        }));

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Interaction",
                    len: 10,
                },
                Token::Str("application_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("100"),
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("200"),
                Token::Str("data"),
                Token::Struct {
                    name: "CommandData",
                    len: 5,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("300"),
                Token::Str("name"),
                Token::Str("command name"),
                Token::Str("type"),
                Token::U8(1),
                Token::Str("options"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "CommandDataOption",
                    len: 3,
                },
                Token::Str("name"),
                Token::Str("member"),
                Token::Str("type"),
                Token::U8(CommandOptionType::User as u8),
                Token::Str("value"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("600"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("resolved"),
                Token::Some,
                Token::Struct {
                    name: "CommandInteractionDataResolved",
                    len: 2,
                },
                Token::Str("members"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("600"),
                Token::Struct {
                    name: "InteractionMember",
                    len: 6,
                },
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("joined_at"),
                Token::Str("2020-01-01T00:00:00.000000+00:00"),
                Token::Str("nick"),
                Token::Some,
                Token::Str("nickname"),
                Token::Str("pending"),
                Token::Bool(false),
                Token::Str("permissions"),
                Token::Str("0"),
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
                Token::MapEnd,
                Token::Str("users"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("600"),
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("1111"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("600"),
                Token::Str("username"),
                Token::Str("username"),
                Token::StructEnd,
                Token::MapEnd,
                Token::StructEnd,
                Token::StructEnd,
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("400"),
                Token::Str("guild_locale"),
                Token::Some,
                Token::String("de"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("500"),
                Token::Str("type"),
                Token::U8(2),
                Token::Str("locale"),
                Token::Str("en-GB"),
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
                Token::Str("2020-01-01T00:00:00.000000+00:00"),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::Some,
                Token::Str("nickname"),
                Token::Str("permissions"),
                Token::Some,
                Token::Str("0"),
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("user"),
                Token::Some,
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("1111"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("600"),
                Token::Str("username"),
                Token::Str("username"),
                Token::StructEnd,
                Token::StructEnd,
                Token::Str("token"),
                Token::Str("interaction token"),
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
