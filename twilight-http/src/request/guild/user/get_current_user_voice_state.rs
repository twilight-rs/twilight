use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    id::{Id, marker::GuildMarker},
    voice::VoiceState,
};

struct GetCurrentUserVoiceStatePath {
    guild_id: Id<GuildMarker>,
}

/// Get voice state of the current user by guild id.
#[must_use = "requests must be configured and executed"]
pub struct GetCurrentUserVoiceState<'a> {
    http: &'a Client,
    path: GetCurrentUserVoiceStatePath,
}

impl<'a> GetCurrentUserVoiceState<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            http,
            path: GetCurrentUserVoiceStatePath { guild_id },
        }
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
        let GetCurrentUserVoiceStatePath { guild_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("voice-states")
            .me()
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
