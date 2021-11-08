use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::{
    guild::Ban,
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};

/// Get information about a ban of a guild.
///
/// Includes the user banned and the reason.
#[must_use = "requests must be configured and executed"]
pub struct GetBan<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    user_id: Id<UserMarker>,
}

impl<'a> GetBan<'a> {
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
    pub fn exec(self) -> ResponseFuture<Ban> {
        let request = Request::from_route(&Route::GetBan {
            guild_id: self.guild_id.get(),
            user_id: self.user_id.get(),
        });

        self.http.request(request)
    }
}
