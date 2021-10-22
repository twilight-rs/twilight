use crate::{
    client::Client,
    request::Request,
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{ApplicationId, CommandId, GuildId};

/// Delete a command in a guild, by ID.
#[must_use = "requests must be configured and executed"]
pub struct DeleteGuildCommand<'a> {
    application_id: ApplicationId,
    command_id: CommandId,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> DeleteGuildCommand<'a> {
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

    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let request = Request::from_route(&Route::DeleteGuildCommand {
            application_id: self.application_id.get(),
            command_id: self.command_id.get(),
            guild_id: self.guild_id.get(),
        });

        self.http.request(request)
    }
}
