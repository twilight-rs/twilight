use crate::{
    client::Client,
    error::Error,
    request::{IntoRequest, Request},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use twilight_model::id::GuildId;

#[derive(Serialize)]
struct UpdateCurrentUserNickFields<'a> {
    nick: &'a str,
}

/// Changes the user's nickname in a guild.
#[must_use = "requests must be configured and executed"]
pub struct UpdateCurrentUserNick<'a> {
    fields: UpdateCurrentUserNickFields<'a>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> UpdateCurrentUserNick<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId, nick: &'a str) -> Self {
        Self {
            fields: UpdateCurrentUserNickFields { nick },
            guild_id,
            http,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for UpdateCurrentUserNick<'_> {
    fn into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdateNickname {
            guild_id: self.guild_id.get(),
        });

        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}
