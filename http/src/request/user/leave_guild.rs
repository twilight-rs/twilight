use crate::{
    client::Client,
    error::Error,
    request::{IntoRequest, Request},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::GuildId;

/// Leave a guild by id.
#[must_use = "requests must be configured and executed"]
pub struct LeaveGuild<'a> {
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> LeaveGuild<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self { guild_id, http }
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

impl IntoRequest for LeaveGuild<'_> {
    fn into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::LeaveGuild {
            guild_id: self.guild_id.get(),
        }))
    }
}
