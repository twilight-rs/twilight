use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
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

pub(crate) struct GetGuildCommandPermissionsFields {
    application_id: Id<ApplicationMarker>,
    guild_id: Id<GuildMarker>,
}

/// Get command permissions for all commands from the current application in a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildCommandPermissions<'a> {
    fields: GetGuildCommandPermissionsFields,
    http: &'a Client,
}

impl<'a> GetGuildCommandPermissions<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
    ) -> Self {
        Self {
            fields: GetGuildCommandPermissionsFields {
                application_id,
                guild_id,
            },
            http,
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

impl Route for GetGuildCommandPermissions<'_> {
    type Fields = GetGuildCommandPermissionsFields;

    const METHOD: Method = Method::Get;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("applications")
            .id(fields.application_id)
            .resource("guilds")
            .id(fields.guild_id)
            .resource("commands")
            .subresource("permissions")
            .build()
    }
}

impl TryIntoRequest for GetGuildCommandPermissions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route_new::<Self>(self.fields))
    }
}
