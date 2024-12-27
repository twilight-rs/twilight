use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
    voice::VoiceState,
};

/// Get voice state of a user by guild and user ids.
#[must_use = "requests must be configured and executed"]
pub struct GetUserVoiceState<'a> {
    http: &'a Client,
    guild_id: Id<GuildMarker>,
    user_id: Id<UserMarker>,
}

impl<'a> GetUserVoiceState<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Self {
        Self {
            http,
            guild_id,
            user_id,
        }
    }
}

impl IntoFuture for GetUserVoiceState<'_> {
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

impl TryIntoRequest for GetUserVoiceState<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetUserVoiceState {
            guild_id: self.guild_id.get(),
            user_id: self.user_id.get(),
        }))
    }
}
