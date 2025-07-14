use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::SoundboardSound,
    id::{marker::GuildMarker, Id},
};

#[must_use = "requests must be configured and executed"]
pub struct GetGuildSoundboardSounds<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildSoundboardSounds<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
    }
}

impl IntoFuture for GetGuildSoundboardSounds<'_> {
    type Output = Result<Response<ListBody<SoundboardSound>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<SoundboardSound>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildSoundboardSounds<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildSoundboardSounds {
            guild_id: self.guild_id.get(),
        }))
    }
}
