use super::super::CommandBorrowed;
use crate::{
    client::Client,
    error::Error as HttpError,
    request::{Request, RequestBuilder, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    application::command::{Command, CommandOption, CommandType},
    id::{
        marker::{ApplicationMarker, GuildMarker},
        Id,
    },
};
use twilight_validate::command::{description as validate_description, CommandValidationError};

/// Create a chat input command in a guild.
///
/// The description must be between 1 and 100 characters in length. Creating a
/// guild command with the same name as an already-existing guild command in the
/// same guild will overwrite the old command. See [the discord docs] for more
/// information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
#[must_use = "requests must be configured and executed"]
pub struct CreateGuildChatInputCommand<'a> {
    application_id: Id<ApplicationMarker>,
    default_permission: Option<bool>,
    description: &'a str,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    name: &'a str,
    options: Option<&'a [CommandOption]>,
}

impl<'a> CreateGuildChatInputCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
        name: &'a str,
        description: &'a str,
    ) -> Result<Self, CommandValidationError> {
        validate_description(&description)?;

        Ok(Self {
            application_id,
            default_permission: None,
            description,
            guild_id,
            http,
            name,
            options: None,
        })
    }

    /// Whether the command is enabled by default when the app is added to
    /// a guild.
    pub fn default_permission(mut self, default: bool) -> Self {
        self.default_permission.replace(default);

        self
    }

    /// Add a list of command options.
    ///
    /// Required command options must be added before optional options.
    ///
    /// Errors
    ///
    /// Returns an error of type [`CommandOptionsRequiredFirst`] if a required
    /// option was added after an optional option. The problem option's index is
    /// provided.
    ///
    /// [`CommandOptionsRequiredFirst`]: twilight_validate::command::CommandValidationErrorType::CommandOptionsRequiredFirst
    pub const fn command_options(
        mut self,
        options: &'a [CommandOption],
    ) -> Result<Self, CommandValidationError> {
        let mut optional_option_added = false;
        let mut idx = 0;

        while idx < options.len() {
            let option = &options[idx];

            if !optional_option_added && !option.is_required() {
                optional_option_added = true;
            }

            if option.is_required() && optional_option_added {
                return Err(CommandValidationError::command_option_required_first(idx));
            }

            idx += 1;
        }

        self.options = Some(options);

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

impl TryIntoRequest for CreateGuildChatInputCommand<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        Request::builder(&Route::CreateGuildCommand {
            application_id: self.application_id.get(),
            guild_id: self.guild_id.get(),
        })
        .json(&CommandBorrowed {
            application_id: Some(self.application_id),
            default_permission: self.default_permission,
            description: Some(self.description),
            kind: CommandType::ChatInput,
            name: self.name,
            options: self.options,
        })
        .map(RequestBuilder::build)
    }
}
