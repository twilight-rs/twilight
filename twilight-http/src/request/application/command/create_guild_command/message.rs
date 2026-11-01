use super::super::CommandBorrowed;
use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::{collections::HashMap, future::IntoFuture};
use twilight_model::{
    application::command::{Command, CommandType},
    guild::Permissions,
    id::{
        Id,
        marker::{ApplicationMarker, GuildMarker},
    },
};
use twilight_validate::command::{CommandValidationError, name as validate_name};

struct CreateGuildMessageCommandBody<'a> {
    default_member_permissions: Option<Permissions>,
    name: &'a str,
    name_localizations: Option<&'a HashMap<String, String>>,
    nsfw: Option<bool>,
}

struct CreateGuildMessageCommandPath {
    application_id: Id<ApplicationMarker>,
    guild_id: Id<GuildMarker>,
}

/// Create a message command in a guild.
///
/// Creating a guild command with the same name as an already-existing guild
/// command in the same guild will overwrite the old command. See
/// [Discord Docs/Create Guild Application Command].
///
/// [Discord Docs/Create Guild Application Command]: https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
#[must_use = "requests must be configured and executed"]
pub struct CreateGuildMessageCommand<'a> {
    body: Result<CreateGuildMessageCommandBody<'a>, CommandValidationError>,
    http: &'a Client,
    path: CreateGuildMessageCommandPath,
}

impl<'a> CreateGuildMessageCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
        name: &'a str,
    ) -> Self {
        let body = Ok(CreateGuildMessageCommandBody {
            default_member_permissions: None,
            name,
            name_localizations: None,
            nsfw: None,
        })
        .and_then(|body| {
            validate_name(name)?;

            Ok(body)
        });

        Self {
            body,
            http,
            path: CreateGuildMessageCommandPath {
                application_id,
                guild_id,
            },
        }
    }

    /// Default permissions required for a member to run the command.
    ///
    /// Defaults to [`None`].
    pub const fn default_member_permissions(mut self, default: Permissions) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.default_member_permissions = Some(default);
        }

        self
    }

    /// Set the localization dictionary for the command name.
    ///
    /// Defaults to [`None`].
    ///
    /// # Errors
    ///
    /// Returns an error of type [`NameLengthInvalid`] if the name is invalid.
    ///
    /// [`NameLengthInvalid`]: twilight_validate::command::CommandValidationErrorType::NameLengthInvalid
    pub fn name_localizations(mut self, localizations: &'a HashMap<String, String>) -> Self {
        self.body = self.body.and_then(|mut body| {
            for name in localizations.values() {
                validate_name(name)?;
            }

            body.name_localizations = Some(localizations);

            Ok(body)
        });

        self
    }

    /// Set whether the command is age-restricted.
    ///
    /// Defaults to not being specified, which uses Discord's default.
    pub const fn nsfw(mut self, nsfw: bool) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.nsfw = Some(nsfw);
        }

        self
    }
}

impl IntoFuture for CreateGuildMessageCommand<'_> {
    type Output = Result<Response<Command>, Error>;

    type IntoFuture = ResponseFuture<Command>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateGuildMessageCommand<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let body = self.body.map_err(Error::validation)?;

        Request::builder(&Route::CreateGuildCommand {
            application_id: self.path.application_id.get(),
            guild_id: self.path.guild_id.get(),
        })
        .json(&CommandBorrowed {
            application_id: Some(self.path.application_id),
            default_member_permissions: body.default_member_permissions,
            dm_permission: None,
            description: None,
            description_localizations: None,
            kind: CommandType::Message,
            name: body.name,
            name_localizations: body.name_localizations,
            nsfw: body.nsfw,
            options: None,
        })
        .build()
    }
}
