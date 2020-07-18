use crate::json_to_vec;
use crate::request::prelude::*;
use std::{
    borrow::Cow,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::GuildChannel,
    guild::{
        DefaultMessageNotificationLevel, ExplicitContentFilter, PartialGuild, Role,
        VerificationLevel,
    },
};

/// The error returned when the guild can not be created as configured.
#[derive(Clone, Debug)]
pub enum CreateGuildError {
    /// The name of the guild is either fewer than 2 UTF-16 characters or more than 100 UTF-16
    /// characters.
    NameInvalid,
    /// The number of channels provided is too many.
    ///
    /// The maximum amount is 500.
    TooManyChannels,
    /// The number of roles provided is too many.
    ///
    /// The maximum amount is 250.
    TooManyRoles,
}

impl Display for CreateGuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NameInvalid => f.write_str("the guild name is invalid"),
            Self::TooManyChannels => f.write_str("too many channels were provided"),
            Self::TooManyRoles => f.write_str("too many roles were provided"),
        }
    }
}

impl Error for CreateGuildError {}

#[derive(Serialize)]
struct CreateGuildFields<'a> {
    channels: Option<Cow<'a, [Cow<'a, GuildChannel>]>>,
    default_message_notifications: Option<DefaultMessageNotificationLevel>,
    explicit_content_filter: Option<ExplicitContentFilter>,
    icon: Option<Cow<'a, str>>,
    name: Cow<'a, str>,
    region: Option<Cow<'a, str>>,
    roles: Option<Cow<'a, [Role]>>,
    verification_level: Option<VerificationLevel>,
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
/// [`CreateGuildError::NameInvalid`]: ../request/guild/enum.CreateGuildError.html#variant.NameInvalid
pub struct CreateGuild<'a> {
    fields: CreateGuildFields<'a>,
    fut: Option<Pending<'a, PartialGuild>>,
    http: &'a Client,
}

impl<'a> CreateGuild<'a> {
    pub(crate) fn new(
        http: &'a Client,
        name: impl Into<Cow<'a, str>>,
    ) -> Result<Self, CreateGuildError> {
        let name = name.into();

        if !validate::guild_name(&name) {
            return Err(CreateGuildError::NameInvalid);
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
    /// The maximum number of channels that can be provided is 500.
    ///
    /// # Errors
    ///
    /// Returns [`CreateGuildError::TooManyChannels`] if the number of channels is over 500.
    ///
    /// [`CreateGuildError::TooManyChannels`]: enum.CreateGuildError.html#variant.TooManyChannels
    pub fn channels(
        mut self,
        channels: impl Into<Cow<'a, [Cow<'a, GuildChannel>]>>,
    ) -> Result<Self, CreateGuildError> {
        let channels = channels.into();

        // Error 30013
        // <https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#json>
        if channels.len() > 500 {
            return Err(CreateGuildError::TooManyChannels);
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
    pub fn icon(mut self, icon: impl Into<Cow<'a, str>>) -> Self {
        self.fields.icon.replace(icon.into());

        self
    }

    /// Specify the voice server region for the guild. Refer to [the discord docs] for more
    /// information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/voice#voice-region-object
    pub fn region(mut self, region: impl Into<Cow<'a, str>>) -> Self {
        self.fields.region.replace(region.into());

        self
    }

    /// Set the roles to create with the guild.
    ///
    /// The maximum number of roles that can be provided is 250.
    ///
    /// # Errors
    ///
    /// Returns [`CreateGuildError::TooManyRoles`] if the number of roles is
    /// over 250.
    ///
    /// [`CreateGuildError::TooManyRoles`]: enum.CreateGuildError.html#variant.TooManyRoles
    pub fn roles(mut self, roles: impl Into<Cow<'a, [Role]>>) -> Result<Self, CreateGuildError> {
        let roles = roles.into();

        if roles.len() > 250 {
            return Err(CreateGuildError::TooManyRoles);
        }

        self.fields.roles.replace(roles);

        Ok(self)
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            json_to_vec(&self.fields)?,
            Route::CreateGuild,
        )))));

        Ok(())
    }
}

poll_req!(CreateGuild<'_>, PartialGuild);
