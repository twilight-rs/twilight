use crate::{
    client::Client,
    request::{Request, TryIntoRequest},
    response::{marker::MemberBody, Response, ResponseFuture},
    routing::Route,
    Error,
};
use std::future::IntoFuture;
use twilight_model::id::{marker::GuildMarker, Id};

/// Get information about the current user in a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetCurrentUserGuildMember<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetCurrentUserGuildMember<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
    }

    #[deprecated(since = "0.14.0", note = "use `.await` or `into_future` instead")]
    pub fn exec(self) -> ResponseFuture<MemberBody> {
        self.into_future()
    }
}

impl IntoFuture for GetCurrentUserGuildMember<'_> {
    type Output = Result<Response<MemberBody>, Error>;

    type IntoFuture = ResponseFuture<MemberBody>;

    fn into_future(self) -> Self::IntoFuture {
        let guild_id = self.guild_id;
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => {
                let mut future = http.request(request);
                future.set_guild_id(guild_id);

                future
            }
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetCurrentUserGuildMember<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetCurrentUserGuildMember {
            guild_id: self.guild_id.get(),
        }))
    }
}
