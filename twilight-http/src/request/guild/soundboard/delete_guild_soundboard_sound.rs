use twilight_model::id::{
    Id,
    marker::{GuildMarker, SoundboardMarker},
};
use twilight_validate::request::{ValidationError, audit_reason as validate_audit_reason};

use crate::{
    Client, Error, Response,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};

#[must_use]
pub struct DeleteGuildSoundboardSound<'a> {
    http: &'a Client,
    guild_id: Id<GuildMarker>,
    sound_id: Id<SoundboardMarker>,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> DeleteGuildSoundboardSound<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        sound_id: Id<SoundboardMarker>,
    ) -> Self {
        Self {
            http,
            guild_id,
            sound_id,
            reason: Ok(None),
        }
    }
}

impl<'a> AuditLogReason<'a> for DeleteGuildSoundboardSound<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for DeleteGuildSoundboardSound<'_> {
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

impl TryIntoRequest for DeleteGuildSoundboardSound<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::DeleteGuildSoundboardSound {
            guild_id: self.guild_id.get(),
            sound_id: self.sound_id.get(),
        });

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
