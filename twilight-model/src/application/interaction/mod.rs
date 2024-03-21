//! Used when receiving interactions through gateway or webhooks.
//!
//! See [Discord Docs/Receiving and Responding].
//!
//! [Discord Docs/Receiving and Responding]: https://discord.com/developers/docs/interactions/receiving-and-responding

pub mod application_command;
pub mod message_component;
pub mod modal;

mod context_type;
mod interaction_type;
mod metadata;
mod resolved;

pub use self::{
    context_type::InteractionContextType,
    interaction_type::InteractionType,
    metadata::InteractionMetadata,
    resolved::{InteractionChannel, InteractionDataResolved, InteractionMember},
};

use self::{
    application_command::CommandData, message_component::MessageComponentInteractionData,
    modal::ModalInteractionData,
};
use crate::{
    channel::{Channel, Message},
    guild::{PartialMember, Permissions},
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, InteractionMarker, UserMarker},
        AnonymizableId, Id,
    },
    oauth::ApplicationIntegrationMap,
    user::User,
};
use serde::{
    de::{Error as DeError, IgnoredAny, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_value::{DeserializerError, Value};
use std::fmt::{Formatter, Result as FmtResult};

/// Payload received when a user executes an interaction.
///
/// See [Discord Docs/Interaction Object].
///
/// [Discord Docs/Interaction Object]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-structure
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Interaction {
    /// App's permissions in the channel the interaction was sent from.
    pub app_permissions: Permissions,
    /// ID of the associated application.
    pub application_id: Id<ApplicationMarker>,
    pub authorizing_integration_owners:
        ApplicationIntegrationMap<AnonymizableId<GuildMarker>, Id<UserMarker>>,
    /// The channel the interaction was invoked in.
    ///
    /// Present on all interactions types, except [`Ping`].
    ///
    /// [`Ping`]: InteractionType::Ping
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
    /// ID of the channel the interaction was invoked in.
    ///
    /// Present on all interactions types, except [`Ping`].
    ///
    /// [`Ping`]: InteractionType::Ping
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(
        note = "channel_id is deprecated in the discord API and will no be sent in the future, users should use the channel field instead."
    )]
    pub channel_id: Option<Id<ChannelMarker>>,
    /// Data from the interaction.
    ///
    /// This field present on [`ApplicationCommand`], [`MessageComponent`],
    /// [`ApplicationCommandAutocomplete`] and [`ModalSubmit`] interactions.
    /// The inner enum variant matches the interaction type.
    ///
    /// [`ApplicationCommand`]: InteractionType::ApplicationCommand
    /// [`MessageComponent`]: InteractionType::MessageComponent
    /// [`ApplicationCommandAutocomplete`]: InteractionType::ApplicationCommandAutocomplete
    /// [`ModalSubmit`]: InteractionType::ModalSubmit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionData>,
    /// ID of the guild the interaction was invoked in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// Guild’s preferred locale.
    ///
    /// Present when the interaction is invoked in a guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_locale: Option<String>,
    /// ID of the interaction.
    pub id: Id<InteractionMarker>,
    /// Type of interaction.
    #[serde(rename = "type")]
    pub kind: InteractionType,
    /// Selected language of the user who invoked the interaction.
    ///
    /// Present on all interactions types, except [`Ping`].
    ///
    /// [`Ping`]: InteractionType::Ping
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Member that invoked the interaction.
    ///
    /// Present when the interaction is invoked in a guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<PartialMember>,
    /// Message attached to the interaction.
    ///
    /// Present on [`MessageComponent`] interactions.
    ///
    /// [`MessageComponent`]: InteractionType::MessageComponent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    /// Token for responding to the interaction.
    pub token: String,
    /// User that invoked the interaction.
    ///
    /// Present when the interaction is invoked in a direct message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

impl Interaction {
    /// ID of the user that invoked the interaction.
    ///
    /// This will first check for the [`member`]'s
    /// [`user`][`PartialMember::user`]'s ID and then, if not present, check the
    /// [`user`]'s ID.
    ///
    /// [`member`]: Self::member
    /// [`user`]: Self::user
    pub const fn author_id(&self) -> Option<Id<UserMarker>> {
        if let Some(user) = self.author() {
            Some(user.id)
        } else {
            None
        }
    }

    /// The user that invoked the interaction.
    ///
    /// This will first check for the [`member`]'s
    /// [`user`][`PartialMember::user`] and then, if not present, check the
    /// [`user`].
    ///
    /// [`member`]: Self::member
    /// [`user`]: Self::user
    pub const fn author(&self) -> Option<&User> {
        match self.member.as_ref() {
            Some(member) if member.user.is_some() => member.user.as_ref(),
            _ => self.user.as_ref(),
        }
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

impl<'de> Deserialize<'de> for Interaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(InteractionVisitor)
    }
}

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum InteractionField {
    AppPermissions,
    ApplicationId,
    Channel,
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
    Version,
    AuthorizingIntegrationOwners,
}

struct InteractionVisitor;

impl<'de> Visitor<'de> for InteractionVisitor {
    type Value = Interaction;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("enum Interaction")
    }

    #[allow(clippy::too_many_lines, deprecated)]
    fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
        let mut app_permissions: Option<Permissions> = None;
        let mut application_id: Option<Id<ApplicationMarker>> = None;
        let mut channel: Option<Channel> = None;
        let mut channel_id: Option<Id<ChannelMarker>> = None;
        let mut data: Option<Value> = None;
        let mut guild_id: Option<Id<GuildMarker>> = None;
        let mut guild_locale: Option<String> = None;
        let mut id: Option<Id<InteractionMarker>> = None;
        let mut kind: Option<InteractionType> = None;
        let mut locale: Option<String> = None;
        let mut member: Option<PartialMember> = None;
        let mut message: Option<Message> = None;
        let mut token: Option<String> = None;
        let mut user: Option<User> = None;
        let mut authorizing_integration_owners: Option<
            ApplicationIntegrationMap<AnonymizableId<GuildMarker>, Id<UserMarker>>,
        > = None;

        loop {
            let key = match map.next_key() {
                Ok(Some(key)) => key,
                Ok(None) => break,
                Err(_) => {
                    map.next_value::<IgnoredAny>()?;

                    continue;
                }
            };

            match key {
                InteractionField::AppPermissions => {
                    if app_permissions.is_some() {
                        return Err(DeError::duplicate_field("app_permissions"));
                    }

                    app_permissions = map.next_value()?;
                }
                InteractionField::ApplicationId => {
                    if application_id.is_some() {
                        return Err(DeError::duplicate_field("application_id"));
                    }

                    application_id = Some(map.next_value()?);
                }
                InteractionField::Channel => {
                    if channel.is_some() {
                        return Err(DeError::duplicate_field("channel"));
                    }

                    channel = map.next_value()?;
                }
                InteractionField::ChannelId => {
                    if channel_id.is_some() {
                        return Err(DeError::duplicate_field("channel_id"));
                    }

                    channel_id = map.next_value()?;
                }
                InteractionField::Data => {
                    if data.is_some() {
                        return Err(DeError::duplicate_field("data"));
                    }

                    data = map.next_value()?;
                }
                InteractionField::GuildId => {
                    if guild_id.is_some() {
                        return Err(DeError::duplicate_field("guild_id"));
                    }

                    guild_id = map.next_value()?;
                }
                InteractionField::GuildLocale => {
                    if guild_locale.is_some() {
                        return Err(DeError::duplicate_field("guild_locale"));
                    }

                    guild_locale = map.next_value()?;
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

                    locale = map.next_value()?;
                }
                InteractionField::Member => {
                    if member.is_some() {
                        return Err(DeError::duplicate_field("member"));
                    }

                    member = map.next_value()?;
                }
                InteractionField::Message => {
                    if message.is_some() {
                        return Err(DeError::duplicate_field("message"));
                    }

                    message = map.next_value()?;
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

                    user = map.next_value()?;
                }
                InteractionField::Version => {
                    // Ignoring the version field.
                    map.next_value::<IgnoredAny>()?;
                }
                InteractionField::AuthorizingIntegrationOwners => {
                    if authorizing_integration_owners.is_some() {
                        return Err(DeError::duplicate_field("authorizing_integration_owners"));
                    }

                    authorizing_integration_owners = map.next_value()?;
                }
            }
        }

        let app_permissions =
            app_permissions.ok_or_else(|| DeError::missing_field("app_permissions"))?;
        let application_id =
            application_id.ok_or_else(|| DeError::missing_field("application_id"))?;
        let authorizing_integration_owners = authorizing_integration_owners
            .ok_or_else(|| DeError::missing_field("authorizing_integration_owners"))?;
        let id = id.ok_or_else(|| DeError::missing_field("id"))?;
        let token = token.ok_or_else(|| DeError::missing_field("token"))?;
        let kind = kind.ok_or_else(|| DeError::missing_field("kind"))?;

        let data = match kind {
            InteractionType::Ping => None,
            InteractionType::ApplicationCommand => {
                let data = data
                    .ok_or_else(|| DeError::missing_field("data"))?
                    .deserialize_into()
                    .map_err(DeserializerError::into_error)?;

                Some(InteractionData::ApplicationCommand(data))
            }
            InteractionType::MessageComponent => {
                let data = data
                    .ok_or_else(|| DeError::missing_field("data"))?
                    .deserialize_into()
                    .map_err(DeserializerError::into_error)?;

                Some(InteractionData::MessageComponent(data))
            }
            InteractionType::ApplicationCommandAutocomplete => {
                let data = data
                    .ok_or_else(|| DeError::missing_field("data"))?
                    .deserialize_into()
                    .map_err(DeserializerError::into_error)?;

                Some(InteractionData::ApplicationCommand(data))
            }
            InteractionType::ModalSubmit => {
                let data = data
                    .ok_or_else(|| DeError::missing_field("data"))?
                    .deserialize_into()
                    .map_err(DeserializerError::into_error)?;

                Some(InteractionData::ModalSubmit(data))
            }
        };

        Ok(Self::Value {
            app_permissions,
            application_id,
            channel,
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
            authorizing_integration_owners,
        })
    }
}

/// Additional [`Interaction`] data, such as the invoking user.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum InteractionData {
    /// Data received for the [`ApplicationCommand`] and [`ApplicationCommandAutocomplete`]
    /// interaction types.
    ///
    /// [`ApplicationCommand`]: InteractionType::ApplicationCommand
    /// [`ApplicationCommandAutocomplete`]: InteractionType::ApplicationCommandAutocomplete
    ApplicationCommand(Box<CommandData>),
    /// Data received for the [`MessageComponent`] interaction type.
    ///
    /// [`MessageComponent`]: InteractionType::MessageComponent
    MessageComponent(Box<MessageComponentInteractionData>),
    /// Data received for the [`ModalSubmit`] interaction type.
    ///
    /// [`ModalSubmit`]: InteractionType::ModalSubmit
    ModalSubmit(ModalInteractionData),
}

#[cfg(test)]
mod tests {
    use super::{
        application_command::{CommandData, CommandDataOption, CommandOptionValue},
        Interaction, InteractionData, InteractionDataResolved, InteractionMember, InteractionType,
    };
    use crate::{
        application::command::{CommandOptionType, CommandType},
        channel::Channel,
        guild::{MemberFlags, PartialMember, Permissions},
        id::Id,
        oauth::ApplicationIntegrationMap,
        test::image_hash,
        user::User,
        util::datetime::{Timestamp, TimestampParseError},
    };
    use serde_test::Token;
    use std::{collections::HashMap, str::FromStr};

    #[test]
    #[allow(clippy::too_many_lines, deprecated)]
    fn test_interaction_full() -> Result<(), TimestampParseError> {
        let joined_at = Some(Timestamp::from_str("2020-01-01T00:00:00.000000+00:00")?);
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

        let value = Interaction {
            app_permissions: Permissions::SEND_MESSAGES,
            application_id: Id::new(100),
            channel: Some(Channel {
                bitrate: None,
                guild_id: None,
                id: Id::new(400),
                kind: crate::channel::ChannelType::GuildText,
                last_message_id: None,
                last_pin_timestamp: None,
                name: None,
                nsfw: None,
                owner_id: None,
                parent_id: None,
                permission_overwrites: None,
                position: None,
                rate_limit_per_user: None,
                recipients: None,
                rtc_region: None,
                topic: None,
                user_limit: None,
                application_id: None,
                applied_tags: None,
                available_tags: None,
                default_auto_archive_duration: None,
                default_forum_layout: None,
                default_reaction_emoji: None,
                default_sort_order: None,
                default_thread_rate_limit_per_user: None,
                flags: None,
                icon: None,
                invitable: None,
                managed: None,
                member: None,
                member_count: None,
                message_count: None,
                newly_created: None,
                thread_metadata: None,
                video_quality_mode: None,
            }),
            channel_id: Some(Id::new(200)),
            data: Some(InteractionData::ApplicationCommand(Box::new(CommandData {
                guild_id: None,
                id: Id::new(300),
                name: "command name".into(),
                kind: CommandType::ChatInput,
                options: Vec::from([CommandDataOption {
                    name: "member".into(),
                    value: CommandOptionValue::User(Id::new(600)),
                }]),
                resolved: Some(InteractionDataResolved {
                    attachments: HashMap::new(),
                    channels: HashMap::new(),
                    members: IntoIterator::into_iter([(
                        Id::new(600),
                        InteractionMember {
                            avatar: None,
                            communication_disabled_until: None,
                            flags,
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
                            avatar_decoration: None,
                            banner: None,
                            bot: false,
                            discriminator: 1111,
                            email: None,
                            flags: None,
                            global_name: Some("test".into()),
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
            }))),
            guild_id: Some(Id::new(400)),
            guild_locale: Some("de".to_owned()),
            id: Id::new(500),
            kind: InteractionType::ApplicationCommand,
            locale: Some("en-GB".to_owned()),
            member: Some(PartialMember {
                avatar: None,
                communication_disabled_until: None,
                deaf: false,
                flags,
                joined_at,
                mute: false,
                nick: Some("nickname".into()),
                permissions: Some(Permissions::empty()),
                premium_since: None,
                roles: Vec::new(),
                user: Some(User {
                    accent_color: None,
                    avatar: Some(image_hash::AVATAR),
                    avatar_decoration: None,
                    banner: None,
                    bot: false,
                    discriminator: 1111,
                    email: None,
                    flags: None,
                    global_name: Some("test".into()),
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
            message: None,
            token: "interaction token".into(),
            user: None,
            authorizing_integration_owners: ApplicationIntegrationMap {
                guild: None,
                user: None,
            },
        };

        // TODO: switch the `assert_tokens` see #2190
        serde_test::assert_ser_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Interaction",
                    len: 13,
                },
                Token::Str("app_permissions"),
                Token::Str("2048"),
                Token::Str("application_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("100"),
                Token::Str("channel"),
                Token::Some,
                Token::Struct {
                    name: "Channel",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("400"),
                Token::Str("type"),
                Token::U8(0),
                Token::StructEnd,
                Token::Str("channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("200"),
                Token::Str("data"),
                Token::Some,
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
                    name: "InteractionDataResolved",
                    len: 2,
                },
                Token::Str("members"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("600"),
                Token::Struct {
                    name: "InteractionMember",
                    len: 7,
                },
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("flags"),
                Token::U64(flags.bits()),
                Token::Str("joined_at"),
                Token::Some,
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
                    len: 9,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("avatar_decoration"),
                Token::None,
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("1111"),
                Token::Str("global_name"),
                Token::Some,
                Token::Str("test"),
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
                Token::U8(InteractionType::ApplicationCommand as u8),
                Token::Str("locale"),
                Token::Some,
                Token::Str("en-GB"),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "PartialMember",
                    len: 9,
                },
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("flags"),
                Token::U64(flags.bits()),
                Token::Str("joined_at"),
                Token::Some,
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
                    len: 9,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("avatar_decoration"),
                Token::None,
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("1111"),
                Token::Str("global_name"),
                Token::Some,
                Token::Str("test"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("600"),
                Token::Str("username"),
                Token::Str("username"),
                Token::StructEnd,
                Token::StructEnd,
                Token::Str("token"),
                Token::Str("interaction token"),
                Token::Str("authorizing_integration_owners"),
                Token::Struct {
                    name: "ApplicationIntegrationMap",
                    len: 2,
                },
                Token::Str("guild"),
                Token::None,
                Token::Str("user"),
                Token::None,
                Token::StructEnd,
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
