use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::widget::GuildWidgetSettings,
    id::{Id, marker::GuildMarker},
};

struct GetGuildWidgetSettingsPath {
    guild_id: Id<GuildMarker>,
}

/// Get a guild's widget settings.
///
/// See [Discord Docs/Get Guild Widget Settings].
///
/// [Discord Docs/Get Guild Widget Settings]: https://discord.com/developers/docs/resources/guild#get-guild-widget-settings
#[must_use = "requests must be configured and executed"]
pub struct GetGuildWidgetSettings<'a> {
    path: GetGuildWidgetSettingsPath,
    http: &'a Client,
}

impl<'a> GetGuildWidgetSettings<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            path: GetGuildWidgetSettingsPath { guild_id },
            http,
        }
    }
}

impl IntoFuture for GetGuildWidgetSettings<'_> {
    type Output = Result<Response<GuildWidgetSettings>, Error>;

    type IntoFuture = ResponseFuture<GuildWidgetSettings>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildWidgetSettings<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetGuildWidgetSettingsPath { guild_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("widget")
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
