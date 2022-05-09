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
    guild::Permissions,
    id::{marker::ApplicationMarker, Id},
};
use twilight_validate::command::{
    chat_input_name as validate_chat_input_name, description as validate_description,
    options as validate_options, CommandValidationError,
};

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
    application_id: Id<ApplicationMarker>,
    default_member_permissions: Option<Permissions>,
    dm_permission: Option<bool>,
    description: &'a str,
    http: &'a Client,
    name: &'a str,
    options: Option<&'a [CommandOption]>,
}

impl<'a> CreateGlobalChatInputCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        name: &'a str,
        description: &'a str,
    ) -> Result<Self, CommandValidationError> {
        validate_description(&description)?;

        validate_chat_input_name(name)?;

        Ok(Self {
            application_id,
            default_member_permissions: None,
            dm_permission: None,
            description,
            http,
            name,
            options: None,
        })
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
    pub fn command_options(
        mut self,
        options: &'a [CommandOption],
    ) -> Result<Self, CommandValidationError> {
        validate_options(options)?;

        self.options = Some(options);

        Ok(self)
    }

    /// Default permissions required for a member to run the command.
    ///
    /// Defaults to [`None`].
    pub const fn default_member_permissions(mut self, default: Permissions) -> Self {
        self.default_member_permissions = Some(default);

        self
    }

    /// Set whether the command is available in DMs.
    ///
    /// Defaults to [`None`].
    pub const fn dm_permission(mut self, dm_permission: bool) -> Self {
        self.dm_permission = Some(dm_permission);

        self
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

impl TryIntoRequest for CreateGlobalChatInputCommand<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        Request::builder(&Route::CreateGlobalCommand {
            application_id: self.application_id.get(),
        })
        .json(&CommandBorrowed {
            application_id: Some(self.application_id),
            default_member_permissions: self.default_member_permissions,
            dm_permission: self.dm_permission,
            description: Some(self.description),
            kind: CommandType::ChatInput,
            name: self.name,
            options: self.options,
        })
        .map(RequestBuilder::build)
    }
}
