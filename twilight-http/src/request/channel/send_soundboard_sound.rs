use crate::{
    client::Client,
    error::Error,
    request::Request,
    request::TryIntoRequest,
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker, SoundboardSoundMarker},
    Id,
};

#[derive(Serialize)]
pub(crate) struct SendSoundboardSoundFields {
    sound_id: Id<SoundboardSoundMarker>,
    guild_id: Option<Id<GuildMarker>>,
}

/// Send a soundboard sound in a channel.
#[must_use = "requests must be configured and executed"]
pub struct SendSoundboardSound<'a> {
    channel_id: Id<ChannelMarker>,
    fields: SendSoundboardSoundFields,
    http: &'a Client,
}

impl<'a> SendSoundboardSound<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        sound_id: Id<SoundboardSoundMarker>,
    ) -> Self {
        Self {
            channel_id,
            fields: SendSoundboardSoundFields {
                sound_id,
                guild_id: None,
            },
            http,
        }
    }

    /// Set the guild ID the soundboard sound specified is associated with.
    ///
    /// This is required to use soundboard sounds from other servers.
    pub fn guild_id(mut self, guild_id: Id<GuildMarker>) -> Self {
        self.fields.guild_id.replace(guild_id);
        self
    }
}

impl IntoFuture for SendSoundboardSound<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

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
