use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::GuildChannel,
    guild::{
        DefaultMessageNotificationLevel,
        ExplicitContentFilter,
        PartialGuild,
        Role,
        VerificationLevel,
    },
};

#[derive(Clone, Debug)]
pub enum CreateGuildError {
    /// The name of the guild is either fewer than 2 UTF-8 characters or more
    /// than 100 UTF-8 characters.
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
struct CreateGuildFields {
    channels: Option<Vec<GuildChannel>>,
    default_message_notifications: Option<DefaultMessageNotificationLevel>,
    explicit_content_filter: Option<ExplicitContentFilter>,
    icon: Option<String>,
    name: String,
    region: Option<String>,
    roles: Option<Vec<Role>>,
    verification_level: Option<VerificationLevel>,
}

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
            return Err(CreateGuildError::NameInvalid);
        }

        Ok(Self {
            fields: CreateGuildFields {
                channels: None,
                default_message_notifications: None,
                explicit_content_filter: None,
                icon: None,
                name: name.into(),
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
    /// Returns [`CreateGuildError::TooManyChannels`] if the number of channels
    /// is over 500.
    ///
    /// [`CreateGuildError::TooManyChannels`]: enum.CreateGuildError.html#variant.TooManyChannels
    pub fn channels(mut self, channels: Vec<GuildChannel>) -> Result<Self, CreateGuildError> {
        // Error 30013
        // <https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#json>
        if channels.len() > 500 {
            return Err(CreateGuildError::TooManyChannels);
        }

        self.fields.channels.replace(channels);

        Ok(self)
    }

    pub fn default_message_notifications(
        mut self,
        default_message_notifications: DefaultMessageNotificationLevel,
    ) -> Self {
        self.fields
            .default_message_notifications
            .replace(default_message_notifications);

        self
    }

    pub fn explicit_content_filter(
        mut self,
        explicit_content_filter: ExplicitContentFilter,
    ) -> Self {
        self.fields
            .explicit_content_filter
            .replace(explicit_content_filter);

        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.fields.icon.replace(icon.into());

        self
    }

    pub fn region(mut self, region: impl Into<String>) -> Self {
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
    pub fn roles(mut self, roles: Vec<Role>) -> Result<Self, CreateGuildError> {
        if roles.len() > 250 {
            return Err(CreateGuildError::TooManyRoles);
        }

        self.fields.roles.replace(roles);

        Ok(self)
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(&self.fields)?,
            Route::CreateGuild,
        )))));

        Ok(())
    }
}

poll_req!(CreateGuild<'_>, PartialGuild);
