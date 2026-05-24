use serde::Serialize;
use twilight_model::{
    guild::SoundboardSound,
    id::{
        Id,
        marker::{EmojiMarker, GuildMarker},
    },
};
use twilight_validate::request::{ValidationError, audit_reason as validate_audit_reason};

use crate::{
    Client, Error, Response,
    request::{self, AuditLogReason, Nullable, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};

#[derive(Serialize)]
struct CreateGuildSoundboardSoundFields<'a> {
    /// Name of the soundboard sound (2-32 characters)
    name: &'a str,
    /// The data URI of the mp3 or ogg sound data, base64 encoded, similar to image data
    sound: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume: Option<Nullable<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji_id: Option<Nullable<Id<EmojiMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji_name: Option<Nullable<&'a str>>,
}

#[must_use]
pub struct CreateGuildSoundboardSound<'a> {
    http: &'a Client,
    guild_id: Id<GuildMarker>,
    fields: CreateGuildSoundboardSoundFields<'a>,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> CreateGuildSoundboardSound<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        name: &'a str,
        sound: &'a str,
    ) -> Self {
        let fields = CreateGuildSoundboardSoundFields {
            name,
            sound,
            volume: None,
            emoji_id: None,
            emoji_name: None,
        };

        Self {
            fields,
            http,
            guild_id,
            reason: Ok(None),
        }
    }

    /// Set the volume of the soundboard sound, from 0 to 1, defaults to 1.
    pub const fn volume(mut self, volume: Option<f64>) -> Self {
        self.fields.volume = Some(Nullable(volume));

        self
    }

    /// Set the id of the custom emoji for the soundboard sound.
    pub const fn emoji_id(mut self, emoji_id: Option<Id<EmojiMarker>>) -> Self {
        self.fields.emoji_id = Some(Nullable(emoji_id));

        self
    }

    /// Set the unicode character of a standard emoji for the soundboard sound.
    pub const fn emoji_name(mut self, emoji_name: Option<&'a str>) -> Self {
        self.fields.emoji_name = Some(Nullable(emoji_name));

        self
    }
}

impl<'a> AuditLogReason<'a> for CreateGuildSoundboardSound<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for CreateGuildSoundboardSound<'_> {
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

impl TryIntoRequest for CreateGuildSoundboardSound<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::CreateGuildSoundboardSound {
            guild_id: self.guild_id.get(),
        })
        .json(&self.fields);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
