use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::invite::WelcomeScreen,
    id::{marker::GuildMarker, Id},
};

/// Get the guild's welcome screen.
///
/// If the welcome screen is not enabled, this requires the [`MANAGE_GUILD`]
/// permission.
///
/// [`MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
#[must_use = "requests must be configured and executed"]
pub struct GetGuildWelcomeScreen<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildWelcomeScreen<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
    }
}

impl IntoFuture for GetGuildWelcomeScreen<'_> {
    type Output = Result<Response<WelcomeScreen>, Error>;

    type IntoFuture = ResponseFuture<WelcomeScreen>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildWelcomeScreen<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildWelcomeScreen {
            guild_id: self.guild_id.get(),
        }))
    }
}
