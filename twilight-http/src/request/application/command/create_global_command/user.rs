use super::super::CommandBorrowed;
use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture},
};
use std::{collections::HashMap, future::IntoFuture};
use twilight_model::{
    application::command::{Command, CommandType},
    guild::Permissions,
    id::{Id, marker::ApplicationMarker},
};
use twilight_validate::command::{CommandValidationError, name as validate_name};

struct CreateGlobalUserCommandBody<'a> {
    default_member_permissions: Option<Permissions>,
    dm_permission: Option<bool>,
    name: &'a str,
    name_localizations: Option<&'a HashMap<String, String>>,
    nsfw: Option<bool>,
}

pub(crate) struct CreateGlobalUserCommandFields {
    application_id: Id<ApplicationMarker>,
}

/// Create a new user global command.
///
/// Creating a command with the same name as an already-existing global command
/// will overwrite the old command. See
/// [Discord Docs/Create Global Application Command].
///
/// [Discord Docs/Create Global Application Command]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
#[must_use = "requests must be configured and executed"]
pub struct CreateGlobalUserCommand<'a> {
    body: Result<CreateGlobalUserCommandBody<'a>, CommandValidationError>,
    fields: CreateGlobalUserCommandFields,
    http: &'a Client,
}

impl<'a> CreateGlobalUserCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        name: &'a str,
    ) -> Self {
        let body = Ok(CreateGlobalUserCommandBody {
            default_member_permissions: None,
            dm_permission: None,
            name,
            name_localizations: None,
            nsfw: None,
        })
        .and_then(|fields| {
            validate_name(name)?;

            Ok(fields)
        });

        Self {
            body,
            fields: CreateGlobalUserCommandFields { application_id },
            http,
        }
    }

    /// Default permissions required for a member to run the command.
    ///
    /// Defaults to [`None`].
    pub const fn default_member_permissions(mut self, default: Permissions) -> Self {
        if let Ok(fields) = self.body.as_mut() {
            fields.default_member_permissions = Some(default);
        }

        self
    }

    /// Set whether the command is available in DMs.
    ///
    /// Defaults to [`None`].
    pub const fn dm_permission(mut self, dm_permission: bool) -> Self {
        if let Ok(fields) = self.body.as_mut() {
            fields.dm_permission = Some(dm_permission);
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
        self.body = self.body.and_then(|mut fields| {
            for name in localizations.values() {
                validate_name(name)?;
            }

            fields.name_localizations = Some(localizations);

            Ok(fields)
        });

        self
    }

    /// Set whether the command is age-restricted.
    ///
    /// Defaults to not being specified, which uses Discord's default.
    pub const fn nsfw(mut self, nsfw: bool) -> Self {
        if let Ok(fields) = self.body.as_mut() {
            fields.nsfw = Some(nsfw);
        }

        self
    }
}

impl IntoFuture for CreateGlobalUserCommand<'_> {
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

impl Route for CreateGlobalUserCommand<'_> {
    type Fields = CreateGlobalUserCommandFields;

    const METHOD: Method = Method::Post;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("applications")
            .id(fields.application_id)
            .resource("commands")
            .build()
    }
}

impl TryIntoRequest for CreateGlobalUserCommand<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let body = self.body.map_err(Error::validation)?;
        let application_id = self.fields.application_id;

        Request::builder_new::<Self>(self.fields)
            .json(&CommandBorrowed {
                application_id: Some(application_id),
                default_member_permissions: body.default_member_permissions,
                dm_permission: body.dm_permission,
                description: None,
                description_localizations: None,
                kind: CommandType::User,
                name: body.name,
                name_localizations: body.name_localizations,
                nsfw: body.nsfw,
                options: None,
            })
            .build()
    }
}
