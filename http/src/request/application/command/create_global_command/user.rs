use super::super::CommandBorrowed;
use crate::{
    client::Client,
    error::Error,
    request::{Request, RequestBuilder},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    application::command::{Command, CommandType},
    id::ApplicationId,
};

/// Create a new user global command.
///
/// Creating a command with the same name as an already-existing global command
/// will overwrite the old command. See [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
#[must_use = "requests must be configured and executed"]
pub struct CreateGlobalUserCommand<'a> {
    application_id: ApplicationId,
    default_permission: Option<bool>,
    http: &'a Client,
    name: &'a str,
}

impl<'a> CreateGlobalUserCommand<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: ApplicationId,
        name: &'a str,
    ) -> Self {
        Self {
            application_id,
            default_permission: None,
            http,
            name,
        }
    }

    /// Whether the command is enabled by default when the app is added to a guild.
    pub const fn default_permission(mut self, default: bool) -> Self {
        self.default_permission = Some(default);

        self
    }

    fn request(&self) -> Result<Request, Error> {
        Request::builder(&Route::CreateGlobalCommand {
            application_id: self.application_id.get(),
        })
        .json(&CommandBorrowed {
            application_id: Some(self.application_id),
            default_permission: self.default_permission,
            description: None,
            kind: CommandType::User,
            name: self.name,
            options: None,
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
