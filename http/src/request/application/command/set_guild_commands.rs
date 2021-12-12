use crate::{
    client::Client,
    error::Error,
    request::{Request, RequestBuilder, TryIntoRequest},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{
    application::command::Command,
    id::{
        marker::{ApplicationMarker, GuildMarker},
        Id,
    },
};

/// Set a guild's commands.
///
/// This method is idempotent: it can be used on every start, without being
/// ratelimited if there aren't changes to the commands.
///
/// The [`Command`] struct has an [associated builder] in the
/// [`twilight-util`] crate.
///
/// [`twilight-util`]: https://docs.rs/twilight-util/latest/index.html
/// [associated builder]: https://docs.rs/twilight-util/latest/builder/command/struct.CommandBuilder.html
#[must_use = "requests must be configured and executed"]
pub struct SetGuildCommands<'a> {
    commands: &'a [Command],
    application_id: Id<ApplicationMarker>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> SetGuildCommands<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
        commands: &'a [Command],
    ) -> Self {
        Self {
            commands,
            application_id,
            guild_id,
            http,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<Command>> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for SetGuildCommands<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::SetGuildCommands {
            application_id: self.application_id.get(),
            guild_id: self.guild_id.get(),
        })
        .json(&self.commands)
        .map(RequestBuilder::build)
    }
}
