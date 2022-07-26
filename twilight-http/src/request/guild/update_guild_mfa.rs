use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    guild::MfaLevel,
    id::{marker::GuildMarker, Id},
};

#[derive(Serialize)]
struct UpdateGuildMfaFields {
    level: MfaLevel,
}

/// Update a guild's MFA level.
#[must_use = "requests must be configured and executed"]
pub struct UpdateGuildMfa<'a> {
    fields: UpdateGuildMfaFields,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> UpdateGuildMfa<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>, level: MfaLevel) -> Self {
        Self {
            fields: UpdateGuildMfaFields { level },
            guild_id,
            http,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<MfaLevel> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateGuildMfa<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdateGuildMfa {
            guild_id: self.guild_id.get(),
        });

        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}
