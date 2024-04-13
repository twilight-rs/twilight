use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::widget::GuildWidgetSettings,
    id::{marker::GuildMarker, Id},
};

/// Get a guild's widget settings.
///
/// See [Discord Docs/Get Guild Widget Settings].
///
/// [Discord Docs/Get Guild Widget Settings]: https://discord.com/developers/docs/resources/guild#get-guild-widget-settings
#[must_use = "requests must be configured and executed"]
pub struct GetGuildWidgetSettings<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildWidgetSettings<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
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
        Ok(Request::from_route(&Route::GetGuildWidgetSettings {
            guild_id: self.guild_id.get(),
        }))
    }
}
