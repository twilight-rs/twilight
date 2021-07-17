use super::{CommandBorrowed, InteractionError, InteractionErrorType};
use crate::{
    client::Client,
    error::Error as HttpError,
    request::{validate, Request, RequestBuilder},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    application::command::{Command, CommandOption},
    id::{ApplicationId, GuildId},
};

/// Create a new command in a guild.
///
/// The name must be between 3 and 32 characters in length, and the description
/// must be between 1 and 100 characters in length. Creating a guild command
/// with the same name as an already-existing guild command in the same guild
/// will overwrite the old command. See [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#create-guild-application-command
pub struct CreateGuildCommand<'a> {
    application_id: ApplicationId,
    default_permission: Option<bool>,
    description: &'a str,
    guild_id: GuildId,
    http: &'a Client,
    name: &'a str,
    options: Option<&'a [CommandOption]>,
}

impl<'a> CreateGuildCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: ApplicationId,
        guild_id: GuildId,
        name: &'a str,
        description: &'a str,
    ) -> Result<Self, InteractionError> {
        if !validate::command_name(name) {
            return Err(InteractionError {
                kind: InteractionErrorType::CommandNameValidationFailed,
            });
        }

        if !validate::command_description(description) {
            return Err(InteractionError {
                kind: InteractionErrorType::CommandDescriptionValidationFailed,
            });
        }

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
    /// Returns an [`InteractionErrorType::CommandOptionsRequiredFirst`]
    /// if a required option was added after an optional option. The problem
    /// option's index is provided.
    pub const fn command_options(
        mut self,
        options: &'a [CommandOption],
    ) -> Result<Self, InteractionError> {
        let mut optional_option_added = false;
        let mut idx = 0;

        while idx < options.len() {
            let option = &options[idx];

            if !optional_option_added && !option.is_required() {
                optional_option_added = true;
            }

            if option.is_required() && optional_option_added {
                return Err(InteractionError {
                    kind: InteractionErrorType::CommandOptionsRequiredFirst { index: idx },
                });
            }

            idx += 1;
        }

        self.options = Some(options);

        Ok(self)
    }

    fn request(&self) -> Result<Request<'a>, HttpError> {
        Request::builder(Route::CreateGuildCommand {
            application_id: self.application_id.0,
            guild_id: self.guild_id.0,
        })
        .json(&CommandBorrowed {
            application_id: Some(self.application_id),
            default_permission: self.default_permission,
            description: self.description,
            name: self.name,
            options: self.options,
        })
        .map(RequestBuilder::build)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Command> {
        match self.request() {
            Ok(request) => self.http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}
