#[cfg(not(target_os = "wasi"))]
use crate::response::{Response, ResponseFuture};
use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::Member,
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
};

/// Get a member of a guild, by id.
#[must_use = "requests must be configured and executed"]
pub struct GetMember<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    user_id: Id<UserMarker>,
}

impl<'a> GetMember<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Self {
        Self {
            guild_id,
            http,
            user_id,
        }
    }
}

#[cfg(not(target_os = "wasi"))]
impl IntoFuture for GetMember<'_> {
    type Output = Result<Response<Member>, Error>;

    type IntoFuture = ResponseFuture<Member>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetMember<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetMember {
            guild_id: self.guild_id.get(),
            user_id: self.user_id.get(),
        }))
    }
}
