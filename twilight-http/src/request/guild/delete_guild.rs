use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use std::future::IntoFuture;
use twilight_model::id::{Id, marker::GuildMarker};

struct DeleteGuildPath {
    guild_id: Id<GuildMarker>,
}

/// Delete a guild permanently. The user must be the owner.
#[must_use = "requests must be configured and executed"]
pub struct DeleteGuild<'a> {
    path: DeleteGuildPath,
    http: &'a Client,
}

impl<'a> DeleteGuild<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            path: DeleteGuildPath { guild_id },
            http,
        }
    }
}

impl IntoFuture for DeleteGuild<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for DeleteGuild<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let DeleteGuildPath { guild_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Delete, path))
    }
}
