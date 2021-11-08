use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::{
    id::{marker::GuildMarker, Id},
    invite::WelcomeScreen,
};

/// Get the guild's welcome screen.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildWelcomeScreen<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildWelcomeScreen<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<WelcomeScreen> {
        let request = Request::from_route(&Route::GetGuildWelcomeScreen {
            guild_id: self.guild_id.get(),
        });

        self.http.request(request)
    }
}
