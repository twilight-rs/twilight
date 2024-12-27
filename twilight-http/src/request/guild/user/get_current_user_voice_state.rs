use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    id::{marker::GuildMarker, Id},
    voice::VoiceState,
};

/// Get voice state of the current user by guild id.
#[must_use = "requests must be configured and executed"]
pub struct GetCurrentUserVoiceState<'a> {
    http: &'a Client,
    guild_id: Id<GuildMarker>,
}

impl<'a> GetCurrentUserVoiceState<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { http, guild_id }
    }
}

impl IntoFuture for GetCurrentUserVoiceState<'_> {
    type Output = Result<Response<VoiceState>, Error>;

    type IntoFuture = ResponseFuture<VoiceState>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetCurrentUserVoiceState<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetCurrentUserVoiceState {
            guild_id: self.guild_id.get(),
        }))
    }
}
