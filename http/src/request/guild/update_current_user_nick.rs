use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use twilight_model::id::{marker::GuildMarker, Id};

#[derive(Serialize)]
struct UpdateCurrentUserNickFields<'a> {
    nick: &'a str,
}

/// Changes the user's nickname in a guild.
#[must_use = "requests must be configured and executed"]
pub struct UpdateCurrentUserNick<'a> {
    fields: UpdateCurrentUserNickFields<'a>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> UpdateCurrentUserNick<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>, nick: &'a str) -> Self {
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

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateCurrentUserNick<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdateNickname {
            guild_id: self.guild_id.get(),
        });

        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}
