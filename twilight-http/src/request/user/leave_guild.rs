#[cfg(not(target_os = "wasi"))]
use crate::response::{Response, ResponseFuture, marker::EmptyBody};
use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{Id, marker::GuildMarker};

/// Leave a guild by id.
#[must_use = "requests must be configured and executed"]
pub struct LeaveGuild<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> LeaveGuild<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
    }
}

#[cfg(not(target_os = "wasi"))]
impl IntoFuture for LeaveGuild<'_> {
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

impl TryIntoRequest for LeaveGuild<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::LeaveGuild {
            guild_id: self.guild_id.get(),
        }))
    }
}
