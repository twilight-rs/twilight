use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{marker, Id};

/// Delete a command in a guild, by ID.
#[must_use = "requests must be configured and executed"]
pub struct DeleteGuildCommand<'a> {
    application_id: Id<marker::Application>,
    command_id: Id<marker::Command>,
    guild_id: Id<marker::Guild>,
    http: &'a Client,
}

impl<'a> DeleteGuildCommand<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<marker::Application>,
        guild_id: Id<marker::Guild>,
        command_id: Id<marker::Command>,
    ) -> Self {
        Self {
            application_id,
            command_id,
            guild_id,
            http,
        }
    }

    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for DeleteGuildCommand<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::DeleteGuildCommand {
            application_id: self.application_id.get(),
            command_id: self.command_id.get(),
            guild_id: self.guild_id.get(),
        }))
    }
}
