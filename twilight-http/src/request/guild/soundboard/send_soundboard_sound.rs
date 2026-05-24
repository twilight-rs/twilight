use serde::Serialize;
use twilight_model::id::{
    Id,
    marker::{ChannelMarker, GuildMarker, SoundboardMarker},
};

use crate::{
    Client, Error, Response,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};

#[derive(Serialize)]
struct SendSoundboardSoundFields {
    sound_id: Id<SoundboardMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_guild_id: Option<Id<GuildMarker>>,
}

#[must_use]
pub struct SendSoundboardSound<'a> {
    channel_id: Id<ChannelMarker>,
    fields: SendSoundboardSoundFields,
    http: &'a Client,
}

impl<'a> SendSoundboardSound<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        sound_id: Id<SoundboardMarker>,
    ) -> Self {
        let fields = SendSoundboardSoundFields {
            sound_id,
            source_guild_id: None,
        };

        Self {
            channel_id,
            fields,
            http,
        }
    }

    /// Set the source guild, this is nessecary if you use the sound from an external guild.
    pub const fn source_guild_id(mut self, source_guild_id: Id<GuildMarker>) -> Self {
        self.fields.source_guild_id = Some(source_guild_id);

        self
    }
}

impl IntoFuture for SendSoundboardSound<'_> {
    type Output = Result<Response<()>, Error>;

    type IntoFuture = ResponseFuture<()>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for SendSoundboardSound<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::SendSoundboardSound {
            channel_id: self.channel_id.get(),
        })
        .json(&self.fields)
        .build()
    }
}
