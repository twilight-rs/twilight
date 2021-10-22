use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::{
    application::command::Command,
    id::{ApplicationId, CommandId, GuildId},
};

/// Retrieve a global command for an application.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildCommand<'a> {
    application_id: ApplicationId,
    command_id: CommandId,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildCommand<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: ApplicationId,
        guild_id: GuildId,
        command_id: CommandId,
    ) -> Self {
        Self {
            application_id,
            command_id,
            guild_id,
            http,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Command> {
        let request = Request::from_route(&Route::GetGuildCommand {
            application_id: self.application_id.get(),
            command_id: self.command_id.get(),
            guild_id: self.guild_id.get(),
        });

        self.http.request(request)
    }
}
