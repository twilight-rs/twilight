use std::collections::HashMap;

use super::super::CommandBorrowed;
use crate::{
    client::Client,
    error::Error,
    request::{Request, RequestBuilder, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    application::command::{Command, CommandType},
    id::{marker::ApplicationMarker, Id},
};
use twilight_validate::command::{name as validate_name, CommandValidationError};

/// Create a new user global command.
///
/// Creating a command with the same name as an already-existing global command
/// will overwrite the old command. See
/// [Discord Docs/Create Global Application Command].
///
/// [Discord Docs/Create Global Application Command]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
#[must_use = "requests must be configured and executed"]
pub struct CreateGlobalUserCommand<'a> {
    application_id: Id<ApplicationMarker>,
    default_permission: Option<bool>,
    http: &'a Client,
    name: &'a str,
    name_localizations: Option<&'a HashMap<String, String>>,
}

impl<'a> CreateGlobalUserCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        name: &'a str,
    ) -> Result<Self, CommandValidationError> {
        validate_name(name)?;

        Ok(Self {
            application_id,
            default_permission: None,
            http,
            name,
            name_localizations: None,
        })
    }

    /// Whether the command is enabled by default when the app is added to a guild.
    pub const fn default_permission(mut self, default: bool) -> Self {
        self.default_permission = Some(default);

        self
    }

    /// Set the localization dictionary for the command name.
    ///
    /// Defaults to [`None`].
    pub fn name_localizations(
        mut self,
        localizations: &'a HashMap<String, String>,
    ) -> Result<Self, CommandValidationError> {
        for name in localizations.values() {
            validate_name(name)?;
        }

        self.name_localizations = Some(localizations);

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Command> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateGlobalUserCommand<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::CreateGlobalCommand {
            application_id: self.application_id.get(),
        })
        .json(&CommandBorrowed {
            application_id: Some(self.application_id),
            default_permission: self.default_permission,
            description: None,
            description_localizations: None,
            kind: CommandType::User,
            name: self.name,
            name_localizations: self.name_localizations,
            options: None,
        })
        .map(RequestBuilder::build)
    }
}
