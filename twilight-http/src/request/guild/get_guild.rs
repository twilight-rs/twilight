use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::Guild,
    id::{Id, marker::GuildMarker},
};

struct GetGuildPath {
    guild_id: Id<GuildMarker>,
    with_counts: bool,
}

/// Get information about a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetGuild<'a> {
    path: GetGuildPath,
    http: &'a Client,
}

impl<'a> GetGuild<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            path: GetGuildPath {
                guild_id,
                with_counts: false,
            },
            http,
        }
    }

    /// Sets if you want to receive `approximate_member_count` and `approximate_presence_count` in
    /// the guild structure.
    pub const fn with_counts(mut self, with: bool) -> Self {
        self.path.with_counts = with;

        self
    }
}

impl IntoFuture for GetGuild<'_> {
    type Output = Result<Response<Guild>, Error>;

    type IntoFuture = ResponseFuture<Guild>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuild<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetGuildPath {
            guild_id,
            with_counts,
        } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .parameter(("with_counts", with_counts))
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
