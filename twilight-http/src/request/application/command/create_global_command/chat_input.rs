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
    application::command::{Command, CommandOption, CommandType},
    guild::Permissions,
    id::{Id, marker::ApplicationMarker},
};
use twilight_validate::command::{
    CommandValidationError, chat_input_name as validate_chat_input_name,
    description as validate_description, options as validate_options,
};

struct CreateGlobalChatInputCommandBody<'a> {
    default_member_permissions: Option<Permissions>,
    description: &'a str,
    description_localizations: Option<&'a HashMap<String, String>>,
    dm_permission: Option<bool>,
    name: &'a str,
    name_localizations: Option<&'a HashMap<String, String>>,
    nsfw: Option<bool>,
    options: Option<&'a [CommandOption]>,
}

struct CreateGlobalChatInputCommandPath {
    application_id: Id<ApplicationMarker>,
}

/// Create a new chat input global command.
///
/// The description must be between 1 and 100 characters in length. Creating a
/// command with the same name as an already-existing global command will
/// overwrite the old command. See
/// [Discord Docs/Create Global Application Command].
///
/// [Discord Docs/Create Global Application Command]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
#[must_use = "requests must be configured and executed"]
pub struct CreateGlobalChatInputCommand<'a> {
    body: Result<CreateGlobalChatInputCommandBody<'a>, CommandValidationError>,
    http: &'a Client,
    path: CreateGlobalChatInputCommandPath,
}

impl<'a> CreateGlobalChatInputCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        name: &'a str,
        description: &'a str,
    ) -> Self {
        let body = Ok(CreateGlobalChatInputCommandBody {
            default_member_permissions: None,
            description,
            description_localizations: None,
            dm_permission: None,
            name,
            name_localizations: None,
            nsfw: None,
            options: None,
        })
        .and_then(|body| {
            validate_description(description)?;

            validate_chat_input_name(name)?;

            Ok(body)
        });

        Self {
            body,
            http,
            path: CreateGlobalChatInputCommandPath { application_id },
        }
    }

    /// Add a list of command options.
    ///
    /// Required command options must be added before optional options.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`OptionsRequiredFirst`] if a required option
    /// was added after an optional option. The problem option's index is
    /// provided.
    ///
    /// [`OptionsRequiredFirst`]: twilight_validate::command::CommandValidationErrorType::OptionsRequiredFirst
    pub fn command_options(mut self, options: &'a [CommandOption]) -> Self {
        self.body = self.body.and_then(|mut body| {
            validate_options(options)?;

            body.options = Some(options);

            Ok(body)
        });

        self
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

    /// Set whether the command is available in DMs.
    ///
    /// Defaults to [`None`].
    pub const fn dm_permission(mut self, dm_permission: bool) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.dm_permission = Some(dm_permission);
        }

        self
    }

    /// Set the localization dictionary for the command description.
    ///
    /// Defaults to [`None`].
    ///
    /// # Errors
    ///
    /// Returns an error of type [`DescriptionInvalid`] if the description is
    /// invalid.
    ///
    /// [`DescriptionInvalid`]: twilight_validate::command::CommandValidationErrorType::DescriptionInvalid
    pub fn description_localizations(mut self, localizations: &'a HashMap<String, String>) -> Self {
        self.body = self.body.and_then(|mut body| {
            for description in localizations.values() {
                validate_description(description)?;
            }

            body.description_localizations = Some(localizations);

            Ok(body)
        });

        self
    }

    /// Set the localization dictionary for the command name.
    ///
    /// Defaults to [`None`].
    ///
    /// # Errors
    ///
    /// Returns an error of type [`NameLengthInvalid`] if the length is invalid.
    ///
    /// Returns an error of type [`NameCharacterInvalid`] if the name contains a
    /// non-alphanumeric character or an uppercase character for which a
    /// lowercase variant exists.
    ///
    /// [`NameLengthInvalid`]: twilight_validate::command::CommandValidationErrorType::NameLengthInvalid
    /// [`NameCharacterInvalid`]: twilight_validate::command::CommandValidationErrorType::NameCharacterInvalid
    pub fn name_localizations(mut self, localizations: &'a HashMap<String, String>) -> Self {
        self.body = self.body.and_then(|mut body| {
            for name in localizations.values() {
                validate_chat_input_name(name)?;
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

impl IntoFuture for CreateGlobalChatInputCommand<'_> {
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

impl TryIntoRequest for CreateGlobalChatInputCommand<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let body = self.body.map_err(Error::validation)?;

        Request::builder(&Route::CreateGlobalCommand {
            application_id: self.path.application_id.get(),
        })
        .json(&CommandBorrowed {
            application_id: Some(self.path.application_id),
            default_member_permissions: body.default_member_permissions,
            dm_permission: body.dm_permission,
            description: Some(body.description),
            description_localizations: body.description_localizations,
            kind: CommandType::ChatInput,
            name: body.name,
            name_localizations: body.name_localizations,
            nsfw: body.nsfw,
            options: body.options,
        })
        .build()
    }
}
