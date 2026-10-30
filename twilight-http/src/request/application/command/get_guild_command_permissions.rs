use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::ListBody},
};
use std::future::IntoFuture;
use twilight_model::{
    application::command::permissions::GuildCommandPermissions,
    id::{
        Id,
        marker::{ApplicationMarker, GuildMarker},
    },
};

pub struct GetGuildCommandPermissionsPath {
    application_id: Id<ApplicationMarker>,
    guild_id: Id<GuildMarker>,
}

/// Get command permissions for all commands from the current application in a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildCommandPermissions<'a> {
    http: &'a Client,
    path: GetGuildCommandPermissionsPath,
}

impl<'a> GetGuildCommandPermissions<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
    ) -> Self {
        Self {
            http,
            path: GetGuildCommandPermissionsPath {
                application_id,
                guild_id,
            },
        }
    }
}

impl IntoFuture for GetGuildCommandPermissions<'_> {
    type Output = Result<Response<ListBody<GuildCommandPermissions>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<GuildCommandPermissions>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildCommandPermissions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = Path::builder()
            .resource("applications")
            .id(self.path.application_id)
            .resource("guilds")
            .id(self.path.guild_id)
            .resource("commands")
            .subresource("permissions")
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
