use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::MemberBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
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

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<MemberBody> {
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

impl TryIntoRequest for GetMember<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetMember {
            guild_id: self.guild_id.get(),
            user_id: self.user_id.get(),
        }))
    }
}
