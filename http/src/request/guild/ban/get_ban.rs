use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    guild::Ban,
    id::{marker, Id},
};

/// Get information about a ban of a guild.
///
/// Includes the user banned and the reason.
#[must_use = "requests must be configured and executed"]
pub struct GetBan<'a> {
    guild_id: Id<marker::Guild>,
    http: &'a Client,
    user_id: Id<marker::User>,
}

impl<'a> GetBan<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<marker::Guild>,
        user_id: Id<marker::User>,
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
    pub fn exec(self) -> ResponseFuture<Ban> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetBan<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetBan {
            guild_id: self.guild_id.get(),
            user_id: self.user_id.get(),
        }))
    }
}
