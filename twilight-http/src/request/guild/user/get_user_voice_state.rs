use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
    voice::VoiceState,
};

struct GetUserVoiceStatePath {
    guild_id: Id<GuildMarker>,
    user_id: Id<UserMarker>,
}

/// Get voice state of a user by guild and user ids.
#[must_use = "requests must be configured and executed"]
pub struct GetUserVoiceState<'a> {
    http: &'a Client,
    path: GetUserVoiceStatePath,
}

impl<'a> GetUserVoiceState<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Self {
        Self {
            http,
            path: GetUserVoiceStatePath { guild_id, user_id },
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
        let GetUserVoiceStatePath { guild_id, user_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("voice-states")
            .id(user_id)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
