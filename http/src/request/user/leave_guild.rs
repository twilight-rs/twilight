use crate::{
    client::Client,
    request::Request,
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
        let request = Request::from_route(&Route::LeaveGuild {
            guild_id: self.guild_id.get(),
        });

        self.http.request(request)
    }
}
