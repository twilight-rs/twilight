use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{
    guild::scheduled_event::GuildScheduledEvent,
    id::{marker::GuildMarker, Id},
};

/// Get a list of scheduled events in a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildScheduledEvents<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    with_user_count: bool,
}

impl<'a> GetGuildScheduledEvents<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            guild_id,
            http,
            with_user_count: false,
        }
    }

    /// Set whether to include the number of subscribed users.
    pub const fn with_user_count(mut self, with_user_count: bool) -> Self {
        self.with_user_count = with_user_count;

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<GuildScheduledEvent>> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildScheduledEvents<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildScheduledEvents {
            guild_id: self.guild_id.get(),
            with_user_count: self.with_user_count,
        }))
    }
}
