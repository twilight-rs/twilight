use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::widget::GuildWidget,
    id::{marker::GuildMarker, Id},
};

/// Get a guild's widget
///
/// See [Discord Docs/Get Guild Widget].
///
/// [Discord Docs/Get Guild Widget]: https://discord.com/developers/docs/resources/guild#get-guild-widget
#[must_use = "requests must be configured and executed"]
pub struct GetGuildWidget<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildWidget<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
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
        Ok(Request::from_route(&Route::GetGuildWidget {
            guild_id: self.guild_id.get(),
        }))
    }
}
