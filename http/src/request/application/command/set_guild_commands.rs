use crate::{
    client::Client,
    error::Error,
    request::{Request, RequestBuilder},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{
    application::command::Command,
    id::{ApplicationId, GuildId},
};

/// Set a guild's commands.
///
/// This method is idempotent: it can be used on every start, without being
/// ratelimited if there aren't changes to the commands.
#[must_use = "requests must be configured and executed"]
pub struct SetGuildCommands<'a> {
    commands: &'a [Command],
    application_id: ApplicationId,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> SetGuildCommands<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: ApplicationId,
        guild_id: GuildId,
        commands: &'a [Command],
    ) -> Self {
        Self {
            commands,
            application_id,
            guild_id,
            http,
        }
    }

    fn request(&self) -> Result<Request, Error> {
        Request::builder(&Route::SetGuildCommands {
            application_id: self.application_id.get(),
            guild_id: self.guild_id.get(),
        })
        .json(&self.commands)
        .map(RequestBuilder::build)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<Command>> {
        match self.request() {
            Ok(request) => self.http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}
