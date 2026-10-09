use twilight_model::{
    guild::SoundboardSound,
    id::{
        Id,
        marker::{GuildMarker, SoundboardMarker},
    },
};

use crate::{
    Client, Error, Response,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};

#[must_use]
pub struct GetGuildSoundboardSound<'a> {
    http: &'a Client,
    guild_id: Id<GuildMarker>,
    sound_id: Id<SoundboardMarker>,
}

impl<'a> GetGuildSoundboardSound<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        sound_id: Id<SoundboardMarker>,
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
        Request::builder(&Route::GetGuildSoundboardSound {
            guild_id: self.guild_id.get(),
            sound_id: self.sound_id.get(),
        })
        .build()
    }
}
