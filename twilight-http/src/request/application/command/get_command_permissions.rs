use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    application::command::permissions::GuildCommandPermissions,
    id::{
        Id,
        marker::{ApplicationMarker, CommandMarker, GuildMarker},
    },
};

pub struct GetCommandPermissionsFields {
    application_id: Id<ApplicationMarker>,
    command_id: Id<CommandMarker>,
    guild_id: Id<GuildMarker>,
}

/// Fetch command permissions for a command from the current application in a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetCommandPermissions<'a> {
    fields: GetCommandPermissionsFields,
    http: &'a Client,
}

impl<'a> GetCommandPermissions<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
        command_id: Id<CommandMarker>,
    ) -> Self {
        Self {
            fields: GetCommandPermissionsFields {
                application_id,
                command_id,
                guild_id,
            },
            http,
        }
    }
}

impl IntoFuture for GetCommandPermissions<'_> {
    type Output = Result<Response<GuildCommandPermissions>, Error>;

    type IntoFuture = ResponseFuture<GuildCommandPermissions>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl Route for GetCommandPermissions<'_> {
    type Fields = GetCommandPermissionsFields;

    const METHOD: Method = Method::Get;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("applications")
            .id(fields.application_id)
            .resource("guilds")
            .id(fields.guild_id)
            .resource("commands")
            .id(fields.command_id)
            .resource("permissions")
            .build()
    }
}

impl TryIntoRequest for GetCommandPermissions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route_new::<Self>(self.fields))
    }
}
