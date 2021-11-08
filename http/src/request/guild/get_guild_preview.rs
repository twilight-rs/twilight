use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::{
    guild::GuildPreview,
    id::{marker::GuildMarker, Id},
};

/// For public guilds, get the guild preview.
///
/// This works even if the user is not in the guild.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildPreview<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildPreview<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<GuildPreview> {
        let request = Request::from_route(&Route::GetGuildPreview {
            guild_id: self.guild_id.get(),
        });

        self.http.request(request)
    }
}
