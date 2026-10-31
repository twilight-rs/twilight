use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    application::command::Command,
    id::{
        Id,
        marker::{ApplicationMarker, CommandMarker, GuildMarker},
    },
};

#[expect(clippy::struct_field_names)]
pub struct GetGuildCommandPath {
    application_id: Id<ApplicationMarker>,
    command_id: Id<CommandMarker>,
    guild_id: Id<GuildMarker>,
}

/// Retrieve a global command for an application.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildCommand<'a> {
    http: &'a Client,
    path: GetGuildCommandPath,
}

impl<'a> GetGuildCommand<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
        command_id: Id<CommandMarker>,
    ) -> Self {
        Self {
            http,
            path: GetGuildCommandPath {
                application_id,
                command_id,
                guild_id,
            },
        }
    }
}

impl IntoFuture for GetGuildCommand<'_> {
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

impl TryIntoRequest for GetGuildCommand<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = Path::builder()
            .resource("applications")
            .id(self.path.application_id)
            .resource("guilds")
            .id(self.path.guild_id)
            .resource("commands")
            .id(self.path.command_id)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
