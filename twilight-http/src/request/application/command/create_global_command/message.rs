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
    id::{marker::ApplicationMarker, Id},
};
use twilight_validate::command::{name as validate_name, CommandValidationError};

struct CreateGlobalMessageCommandFields<'a> {
    default_member_permissions: Option<Permissions>,
    dm_permission: Option<bool>,
    name: &'a str,
    name_localizations: Option<&'a HashMap<String, String>>,
    nsfw: Option<bool>,
}

/// Create a new message global command.
///
/// Creating a command with the same name as an already-existing global command
/// will overwrite the old command. See
/// [Discord Docs/Create Global Application Command].
///
/// [Discord Docs/Create Global Application Command]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
#[must_use = "requests must be configured and executed"]
pub struct CreateGlobalMessageCommand<'a> {
    application_id: Id<ApplicationMarker>,
    fields: Result<CreateGlobalMessageCommandFields<'a>, CommandValidationError>,
    http: &'a Client,
}

impl<'a> CreateGlobalMessageCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        name: &'a str,
    ) -> Self {
        let fields = Ok(CreateGlobalMessageCommandFields {
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
            application_id,
            fields,
            http,
        }
    }

    /// Default permissions required for a member to run the command.
    ///
    /// Defaults to [`None`].
    pub fn default_member_permissions(mut self, default: Permissions) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.default_member_permissions = Some(default);
        }

        self
    }

    /// Set whether the command is available in DMs.
    ///
    /// Defaults to [`None`].
    pub fn dm_permission(mut self, dm_permission: bool) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
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
        self.fields = self.fields.and_then(|mut fields| {
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
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.nsfw = Some(nsfw);
        }

        self
    }
}

impl IntoFuture for CreateGlobalMessageCommand<'_> {
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

impl TryIntoRequest for CreateGlobalMessageCommand<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;

        Request::builder(&Route::CreateGlobalCommand {
            application_id: self.application_id.get(),
        })
        .json(&CommandBorrowed {
            application_id: Some(self.application_id),
            default_member_permissions: fields.default_member_permissions,
            dm_permission: fields.dm_permission,
            description: None,
            description_localizations: None,
            kind: CommandType::Message,
            name: fields.name,
            name_localizations: fields.name_localizations,
            nsfw: fields.nsfw,
            options: None,
        })
        .build()
    }
}
