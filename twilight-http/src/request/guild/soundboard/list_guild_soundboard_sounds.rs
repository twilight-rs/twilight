use twilight_model::{
    guild::SoundboardSoundList,
    id::{Id, marker::GuildMarker},
};

use crate::{
    Client, Error, Response,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};

#[must_use]
pub struct ListGuildSoundboardSounds<'a> {
    http: &'a Client,
    guild_id: Id<GuildMarker>,
}

impl<'a> ListGuildSoundboardSounds<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { http, guild_id }
    }
}

impl IntoFuture for ListGuildSoundboardSounds<'_> {
    type Output = Result<Response<SoundboardSoundList>, Error>;

    type IntoFuture = ResponseFuture<SoundboardSoundList>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for ListGuildSoundboardSounds<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::ListGuildSoundboardSounds {
            guild_id: self.guild_id.get(),
        })
        .build()
    }
}
