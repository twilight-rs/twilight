use serde::Serialize;
use twilight_model::{
    guild::SoundboardSound,
    id::{
        Id,
        marker::{EmojiMarker, GuildMarker, SoundboardMarker},
    },
};
use twilight_validate::{
    request::{ValidationError, audit_reason as validate_audit_reason},
    soundboard::{self, SoundboardValidationError},
};

use crate::{
    Client, Error, Response,
    request::{self, AuditLogReason, Nullable, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};

#[derive(Serialize)]
struct UpdateGuildSoundboardSoundFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume: Option<Nullable<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji_id: Option<Nullable<Id<EmojiMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji_name: Option<Nullable<&'a str>>,
}

#[must_use]
pub struct UpdateGuildSoundboardSound<'a> {
    http: &'a Client,
    guild_id: Id<GuildMarker>,
    sound_id: Id<SoundboardMarker>,
    fields: Result<UpdateGuildSoundboardSoundFields<'a>, SoundboardValidationError>,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> UpdateGuildSoundboardSound<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        sound_id: Id<SoundboardMarker>,
    ) -> Self {
        let fields = Ok(UpdateGuildSoundboardSoundFields {
            name: None,
            volume: None,
            emoji_id: None,
            emoji_name: None,
        });

        Self {
            fields,
            http,
            guild_id,
            sound_id,
            reason: Ok(None),
        }
    }

    /// Set the name of the soundboard sound (2-32 characters)
    pub fn name(mut self, name: &'a str) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            soundboard::name(name)?;
            fields.name = Some(name);
            Ok(fields)
        });

        self
    }

    /// Set the volume of the soundboard sound, from 0 to 1, defaults to 1.
    pub fn volume(mut self, volume: Option<f64>) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            if let Some(volume) = volume {
                soundboard::volume(volume)?;
            }
            fields.volume = Some(Nullable(volume));
            Ok(fields)
        });

        self
    }

    /// Set the id of the custom emoji for the soundboard sound.
    pub const fn emoji_id(mut self, emoji_id: Option<Id<EmojiMarker>>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.emoji_id = Some(Nullable(emoji_id));
        }

        self
    }

    /// Set the unicode character of a standard emoji for the soundboard sound.
    pub const fn emoji_name(mut self, emoji_name: Option<&'a str>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.emoji_name = Some(Nullable(emoji_name));
        }

        self
    }
}

impl<'a> AuditLogReason<'a> for UpdateGuildSoundboardSound<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for UpdateGuildSoundboardSound<'_> {
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

impl TryIntoRequest for UpdateGuildSoundboardSound<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;
        let mut request = Request::builder(&Route::UpdateGuildSoundboardSound {
            guild_id: self.guild_id.get(),
            sound_id: self.sound_id.get(),
        })
        .json(&fields);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
