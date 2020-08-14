use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::{
        permission_overwrite::PermissionOverwrite, CategoryChannel, ChannelType, GuildChannel,
        TextChannel, VoiceChannel,
    },
    guild::{
        DefaultMessageNotificationLevel, ExplicitContentFilter, PartialGuild, Permissions, Role,
        VerificationLevel,
    },
    id::{ChannelId, GuildId, RoleId},
};

/// The error returned when the guild can not be created as configured.
#[derive(Clone, Debug)]
pub enum CreateGuildError {
    /// The name of the guild is either fewer than 2 UTF-16 characters or more than 100 UTF-16
    /// characters.
    NameInvalid {
        /// Provided name.
        name: String,
    },
    /// The number of channels provided is too many.
    ///
    /// The maximum amount is 500.
    TooManyChannels {
        /// Provided channels.
        channels: Vec<GuildChannelFragment>,
    },
    /// The number of roles provided is too many.
    ///
    /// The maximum amount is 250.
    TooManyRoles {
        /// Provided roles.
        roles: Vec<RoleFragment>,
    },
}

impl Display for CreateGuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NameInvalid { .. } => f.write_str("the guild name is invalid"),
            Self::TooManyChannels { .. } => f.write_str("too many channels were provided"),
            Self::TooManyRoles { .. } => f.write_str("too many roles were provided"),
        }
    }
}

impl Error for CreateGuildError {}

#[derive(Serialize)]
struct CreateGuildFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    channels: Option<Vec<GuildChannel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_message_notifications: Option<DefaultMessageNotificationLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explicit_content_filter: Option<ExplicitContentFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<Vec<Role>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_level: Option<VerificationLevel>,
}

/// Fillable struct to send new roles to a guild create.
#[derive(Clone, Debug, Default)]
pub struct RoleFragment {
    pub color: Option<u32>,
    pub hoist: bool,
    pub id: Option<RoleId>,
    pub mentionable: bool,
    pub name: Option<String>,
    pub permissions: Option<Permissions>,
    pub position: Option<i64>,
}

#[derive(Clone, Debug)]
pub enum GuildChannelFragment {
    Category(CategoryChannelFragment),
    Text(TextChannelFragment),
    Voice(VoiceChannelFragment),
}

/// Fillable struct to send new category channels to a guild create.
#[derive(Clone, Debug)]
pub struct CategoryChannelFragment {
    pub id: ChannelId,
    pub name: String,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
}

impl Into<CategoryChannel> for CategoryChannelFragment {
    fn into(self) -> CategoryChannel {
        CategoryChannel {
            guild_id: Some(GuildId(1)),
            id: self.id,
            kind: ChannelType::GuildCategory,
            name: self.name,
            nsfw: false,
            parent_id: None,
            permission_overwrites: self.permission_overwrites.unwrap_or_else(Vec::new),
            position: 0,
        }
    }
}

/// Fillable struct to send new text channels to a guild create.
#[derive(Clone, Debug)]
pub struct TextChannelFragment {
    pub name: String,
    pub nsfw: bool,
    pub parent_id: Option<ChannelId>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub rate_limit_per_user: Option<u64>,
    pub topic: Option<String>,
}

impl Into<TextChannel> for TextChannelFragment {
    fn into(self) -> TextChannel {
        TextChannel {
            guild_id: Some(GuildId(1)),
            id: ChannelId(501),
            kind: ChannelType::GuildText,
            last_message_id: None,
            last_pin_timestamp: None,
            name: self.name,
            nsfw: self.nsfw,
            permission_overwrites: self.permission_overwrites.unwrap_or_else(Vec::new),
            parent_id: self.parent_id,
            position: 0,
            rate_limit_per_user: self.rate_limit_per_user,
            topic: self.topic,
        }
    }
}

/// Fillable struct to send new voice channels to a guild create.
#[derive(Clone, Debug)]
pub struct VoiceChannelFragment {
    pub bitrate: Option<u64>,
    pub name: String,
    pub parent_id: Option<ChannelId>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub user_limit: Option<u64>,
}

impl Into<VoiceChannel> for VoiceChannelFragment {
    fn into(self) -> VoiceChannel {
        VoiceChannel {
            bitrate: self.bitrate.unwrap_or(64000),
            guild_id: Some(GuildId(1)),
            id: ChannelId(502),
            kind: ChannelType::GuildVoice,
            name: self.name,
            permission_overwrites: self.permission_overwrites.unwrap_or_else(Vec::new),
            parent_id: self.parent_id,
            position: 0,
            user_limit: self.user_limit.or(Some(0)),
        }
    }
}

/// Create a new request to create a guild.
///
/// The minimum length of the name is 2 UTF-16 characters and the maximum is 100 UTF-16 characters.
/// This endpoint can only be used by bots in less than 10 guilds.
///
/// # Errors
///
/// Returns [`CreateGuildError::NameInvalid`] if the name length is too short or too long.
///
/// [`CreateGuildError::NameInvalid`]: enum.CreateGuildError.html#variant.NameInvalid
pub struct CreateGuild<'a> {
    fields: CreateGuildFields,
    fut: Option<Pending<'a, PartialGuild>>,
    http: &'a Client,
}

impl<'a> CreateGuild<'a> {
    pub(crate) fn new(http: &'a Client, name: impl Into<String>) -> Result<Self, CreateGuildError> {
        Self::_new(http, name.into())
    }

    fn _new(http: &'a Client, name: String) -> Result<Self, CreateGuildError> {
        if !validate::guild_name(&name) {
            return Err(CreateGuildError::NameInvalid { name });
        }

        Ok(Self {
            fields: CreateGuildFields {
                channels: None,
                default_message_notifications: None,
                explicit_content_filter: None,
                icon: None,
                name,
                region: None,
                roles: None,
                verification_level: None,
            },
            fut: None,
            http,
        })
    }

    /// Set the channels to create with the guild.
    ///
    /// The maximum number of channels that can be provided is 500. A `Vec` of
    /// [`GuildChannelFragment`]s must be passed. The channels will be created in the order they
    /// are passed. If inserting [`CategoryChannelFragment`]s, all child channels must be after the
    /// category, and any non-children [`GuildChannelFragment`]s can not be between the children.
    /// To specify `PermissionOverwrite`s, set the `PermissionOverwriteType` to `Role`, and its
    /// `RoleId` to the id of a role you create with [`CreateGuild#roles`]. This `Vec` will be
    /// transformed into temporary instances of each channel type, and sent to Discord for
    /// validation.
    ///
    /// # Errors
    ///
    /// Returns [`CreateGuildError::TooManyChannels`] if the number of channels is over 500.
    ///
    /// [`GuildChannelFragment`]: enum.GuildChannelFragment.html
    /// [`CategoryChannelFragment`]: struct.CategoryChannelFragment.html
    /// [`CreateGuildError::TooManyChannels`]: enum.CreateGuildError.html#variant.TooManyChannels
    /// [`CreateGuild#roles`]: struct.CreateGuild.html#method.roles
    pub fn channels(
        mut self,
        fragments: Vec<GuildChannelFragment>,
    ) -> Result<Self, CreateGuildError> {
        // Error 30013
        // <https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#json>
        if fragments.len() > 500 {
            return Err(CreateGuildError::TooManyChannels {
                channels: fragments,
            });
        }

        let mut channels: Vec<GuildChannel> = Vec::new();

        for fragment in fragments {
            let channel: GuildChannel = match fragment {
                GuildChannelFragment::Category(c) => GuildChannel::Category(c.into()),
                GuildChannelFragment::Text(t) => GuildChannel::Text(t.into()),
                GuildChannelFragment::Voice(v) => GuildChannel::Voice(v.into()),
            };

            channels.push(channel);
        }

        self.fields.channels.replace(channels);

        Ok(self)
    }

    /// Set the default message notification level. Refer to [the discord docs] for more
    /// information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/guild#create-guild
    pub fn default_message_notifications(
        mut self,
        default_message_notifications: DefaultMessageNotificationLevel,
    ) -> Self {
        self.fields
            .default_message_notifications
            .replace(default_message_notifications);

        self
    }

    /// Set the explicit content filter level.
    pub fn explicit_content_filter(
        mut self,
        explicit_content_filter: ExplicitContentFilter,
    ) -> Self {
        self.fields
            .explicit_content_filter
            .replace(explicit_content_filter);

        self
    }

    /// Set the icon.
    ///
    /// This must be a Data URI, in the form of `data:image/{type};base64,{data}` where `{type}` is
    /// the image MIME type and `{data}` is the base64-encoded image. Refer to [the discord docs]
    /// for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/reference#image-data
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.fields.icon.replace(icon.into());

        self
    }

    /// Set overrides to the everyone role.
    pub fn override_everyone(mut self, fragment: RoleFragment) -> Self {
        let everyone = Role {
            color: fragment.color.unwrap_or(0),
            hoist: fragment.hoist,
            id: RoleId(1),
            managed: false,
            mentionable: fragment.mentionable,
            name: fragment.name.unwrap_or_else(|| String::from("@everyone")),
            permissions: fragment.permissions.unwrap_or_else(Permissions::empty),
            position: fragment.position.unwrap_or(0),
        };

        let mut roles: Vec<Role> = vec![everyone];

        // if some roles have already been set, retain them
        if self.fields.roles.is_some() {
            let fields_roles = self.fields.roles.take().unwrap();
            roles.extend_from_slice(&fields_roles[1..]);
        }

        self.fields.roles.replace(roles);

        self
    }

    /// Specify the voice server region for the guild. Refer to [the discord docs] for more
    /// information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/voice#voice-region-object
    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.fields.region.replace(region.into());

        self
    }

    /// Set the roles to create with the guild.
    ///
    /// The maximum number of roles that can be provided is 250. A `Vec` of [`RoleFragment`]
    /// structs must be provided. These will be turned into instances of `Role`s that are sent to
    /// Discord. If specifying `Some` for [`RoleFragment.id`], do not use the id of 1.
    ///
    /// # Errors
    ///
    /// Returns [`CreateGuildError::TooManyRoles`] if the number of roles is
    /// over 250.
    ///
    /// [`CreateGuildError::TooManyRoles`]: enum.CreateGuildError.html#variant.TooManyRoles
    /// [`RoleFragment`]: struct.RoleFragment.html
    /// [`RoleFragment.id`]: struct.RoleFragment.html#structfield.id
    pub fn roles(mut self, fragments: Vec<RoleFragment>) -> Result<Self, CreateGuildError> {
        if fragments.len() > 250 {
            return Err(CreateGuildError::TooManyRoles { roles: fragments });
        }

        let mut roles: Vec<Role> = fragments
            .iter()
            .map(|f| Role {
                color: f.color.unwrap_or(0),
                hoist: f.hoist,
                id: f.id.unwrap_or_else(|| RoleId(2)),
                managed: false,
                mentionable: f.mentionable,
                name: f.name.clone().unwrap_or_else(String::new),
                permissions: f.permissions.unwrap_or_else(Permissions::empty),
                position: f.position.unwrap_or(0),
            })
            .collect();

        // if there are already overrides, retain the first role in the vec.
        // discord understands the first role to be overrides for @everyone.
        if self.fields.roles.is_some() {
            let mut fields_roles = self.fields.roles.take().unwrap();
            roles.insert(0, fields_roles.remove(0));
        }

        self.fields.roles.replace(roles);

        Ok(self)
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            crate::json_to_vec(&self.fields)?,
            Route::CreateGuild,
        )))));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{CategoryChannelFragment, TextChannelFragment, VoiceChannelFragment};
    use twilight_model::{
        channel::{CategoryChannel, ChannelType, TextChannel, VoiceChannel},
        id::{ChannelId, GuildId},
    };

    #[test]
    fn category_channel() {
        let fragment = CategoryChannelFragment {
            id: ChannelId(1),
            name: "cat channel".to_string(),
            permission_overwrites: None,
        };

        let category: CategoryChannel = fragment.into();

        assert_eq!(
            category,
            CategoryChannel {
                guild_id: Some(GuildId(1)),
                id: ChannelId(1),
                kind: ChannelType::GuildCategory,
                name: "cat channel".to_string(),
                nsfw: false,
                parent_id: None,
                permission_overwrites: Vec::new(),
                position: 0,
            }
        );
    }

    #[test]
    fn text_channel() {
        let fragment = TextChannelFragment {
            name: "text channel".to_string(),
            nsfw: false,
            parent_id: None,
            permission_overwrites: None,
            rate_limit_per_user: None,
            topic: Some("topic".to_string()),
        };

        let text: TextChannel = fragment.into();

        assert_eq!(
            text,
            TextChannel {
                guild_id: Some(GuildId(1)),
                id: ChannelId(501),
                kind: ChannelType::GuildText,
                last_message_id: None,
                last_pin_timestamp: None,
                name: "text channel".to_string(),
                nsfw: false,
                permission_overwrites: Vec::new(),
                parent_id: None,
                position: 0,
                rate_limit_per_user: None,
                topic: Some("topic".to_string()),
            }
        );
    }

    #[test]
    fn voice_channel() {
        let fragment = VoiceChannelFragment {
            bitrate: Some(64000),
            name: "voice channel".to_string(),
            parent_id: None,
            permission_overwrites: None,
            user_limit: None,
        };

        let voice: VoiceChannel = fragment.into();

        assert_eq!(
            voice,
            VoiceChannel {
                bitrate: 64000,
                guild_id: Some(GuildId(1)),
                id: ChannelId(502),
                kind: ChannelType::GuildVoice,
                name: "voice channel".to_string(),
                permission_overwrites: Vec::new(),
                parent_id: None,
                position: 0,
                user_limit: Some(0),
            }
        );
    }
}

poll_req!(CreateGuild<'_>, PartialGuild);
