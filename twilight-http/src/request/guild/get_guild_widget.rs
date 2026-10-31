use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::widget::GuildWidget,
    id::{Id, marker::GuildMarker},
};

struct GetGuildWidgetPath {
    guild_id: Id<GuildMarker>,
}

/// Get a guild's widget
///
/// See [Discord Docs/Get Guild Widget].
///
/// [Discord Docs/Get Guild Widget]: https://discord.com/developers/docs/resources/guild#get-guild-widget
#[must_use = "requests must be configured and executed"]
pub struct GetGuildWidget<'a> {
    path: GetGuildWidgetPath,
    http: &'a Client,
}

impl<'a> GetGuildWidget<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            path: GetGuildWidgetPath { guild_id },
            http,
        }
    }
}

impl IntoFuture for GetGuildWidget<'_> {
    type Output = Result<Response<GuildWidget>, Error>;

    type IntoFuture = ResponseFuture<GuildWidget>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildWidget<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetGuildWidgetPath { guild_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("widget.json")
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
