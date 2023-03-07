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
    id::{
        marker::{ApplicationMarker, GuildMarker},
        Id,
    },
};
use twilight_validate::command::{
    chat_input_name as validate_chat_input_name, description as validate_description,
    options as validate_options, CommandValidationError,
};

struct CreateGuildChatInputCommandFields<'a> {
    default_member_permissions: Option<Permissions>,
    description: &'a str,
    description_localizations: Option<&'a HashMap<String, String>>,
    name: &'a str,
    name_localizations: Option<&'a HashMap<String, String>>,
    nsfw: Option<bool>,
    options: Option<&'a [CommandOption]>,
}

/// Create a chat input command in a guild.
///
/// The description must be between 1 and 100 characters in length. Creating a
/// guild command with the same name as an already-existing guild command in the
/// same guild will overwrite the old command. See
/// [Discord Docs/Create Global Application Command].
///
/// [Discord Docs/Create Global Application Command]: https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
#[must_use = "requests must be configured and executed"]
pub struct CreateGuildChatInputCommand<'a> {
    application_id: Id<ApplicationMarker>,
    fields: Result<CreateGuildChatInputCommandFields<'a>, CommandValidationError>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> CreateGuildChatInputCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
        name: &'a str,
        description: &'a str,
    ) -> Self {
        let fields = Ok(CreateGuildChatInputCommandFields {
            default_member_permissions: None,
            description,
            description_localizations: None,
            name,
            name_localizations: None,
            nsfw: None,
            options: None,
        })
        .and_then(|fields| {
            validate_description(description)?;

            validate_chat_input_name(name)?;

            Ok(fields)
        });

        Self {
            application_id,
            fields,
            guild_id,
            http,
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
        self.fields = self.fields.and_then(|mut fields| {
            validate_options(options)?;

            fields.options = Some(options);

            Ok(fields)
        });

        self
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
        self.fields = self.fields.and_then(|mut fields| {
            for description in localizations.values() {
                validate_description(description)?;
            }

            fields.description_localizations = Some(localizations);

            Ok(fields)
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
        self.fields = self.fields.and_then(|mut fields| {
            for name in localizations.values() {
                validate_chat_input_name(name)?;
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

impl IntoFuture for CreateGuildChatInputCommand<'_> {
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

impl TryIntoRequest for CreateGuildChatInputCommand<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;

        Request::builder(&Route::CreateGuildCommand {
            application_id: self.application_id.get(),
            guild_id: self.guild_id.get(),
        })
        .json(&CommandBorrowed {
            application_id: Some(self.application_id),
            default_member_permissions: fields.default_member_permissions,
            dm_permission: None,
            description: Some(fields.description),
            description_localizations: fields.description_localizations,
            kind: CommandType::ChatInput,
            name: fields.name,
            name_localizations: fields.name_localizations,
            nsfw: fields.nsfw,
            options: fields.options,
        })
        .build()
    }
}
