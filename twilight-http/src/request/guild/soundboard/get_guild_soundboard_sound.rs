use crate::{
    client::Client,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
    Error,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::SoundboardSound,
    id::{
        marker::{GuildMarker, SoundboardSoundMarker},
        Id,
    },
};

#[must_use = "requests must be configured and executed"]
pub struct GetGuildSoundboardSound<'a> {
    http: &'a Client,
    guild_id: Id<GuildMarker>,
    sound_id: Id<SoundboardSoundMarker>,
}

impl<'a> GetGuildSoundboardSound<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        sound_id: Id<SoundboardSoundMarker>,
    ) -> Self {
        Self {
            http,
            guild_id,
            sound_id,
        }
    }
}

impl IntoFuture for GetGuildSoundboardSound<'_> {
    type Output = Result<Response<SoundboardSound>, Error>;

    type IntoFuture = ResponseFuture<SoundboardSound>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildSoundboardSound<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildSoundboardSound {
            guild_id: self.guild_id.get(),
            sound_id: self.sound_id.get(),
        }))
    }
}
