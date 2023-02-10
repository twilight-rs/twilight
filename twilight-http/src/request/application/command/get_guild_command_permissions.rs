use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    application::command::permissions::GuildCommandPermissions,
    id::{
        marker::{ApplicationMarker, GuildMarker},
        Id,
    },
};

/// Get command permissions for all commands from the current application in a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildCommandPermissions<'a> {
    application_id: Id<ApplicationMarker>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildCommandPermissions<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
    ) -> Self {
        Self {
            application_id,
            guild_id,
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

impl TryIntoRequest for GetGuildCommandPermissions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildCommandPermissions {
            application_id: self.application_id.get(),
            guild_id: self.guild_id.get(),
        }))
    }
}
