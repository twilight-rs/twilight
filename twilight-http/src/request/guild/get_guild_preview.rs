use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::GuildPreview,
    id::{Id, marker::GuildMarker},
};

struct GetGuildPreviewPath {
    guild_id: Id<GuildMarker>,
}

/// For public guilds, get the guild preview.
///
/// This works even if the user is not in the guild.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildPreview<'a> {
    path: GetGuildPreviewPath,
    http: &'a Client,
}

impl<'a> GetGuildPreview<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            path: GetGuildPreviewPath { guild_id },
            http,
        }
    }
}

impl IntoFuture for GetGuildPreview<'_> {
    type Output = Result<Response<GuildPreview>, Error>;

    type IntoFuture = ResponseFuture<GuildPreview>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildPreview<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetGuildPreviewPath { guild_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("preview")
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
