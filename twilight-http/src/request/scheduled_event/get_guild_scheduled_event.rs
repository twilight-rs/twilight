use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::scheduled_event::GuildScheduledEvent,
    id::{
        marker::{GuildMarker, ScheduledEventMarker},
        Id,
    },
};

/// Get a scheduled event in a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildScheduledEvent<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    scheduled_event_id: Id<ScheduledEventMarker>,
    with_user_count: bool,
}

impl<'a> GetGuildScheduledEvent<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        scheduled_event_id: Id<ScheduledEventMarker>,
    ) -> Self {
        Self {
            guild_id,
            http,
            scheduled_event_id,
            with_user_count: false,
        }
    }

    /// Set whether to include the number of subscribed users.
    pub const fn with_user_count(mut self, with_user_count: bool) -> Self {
        self.with_user_count = with_user_count;

        self
    }
}

impl IntoFuture for GetGuildScheduledEvent<'_> {
    type Output = Result<Response<GuildScheduledEvent>, Error>;

    type IntoFuture = ResponseFuture<GuildScheduledEvent>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildScheduledEvent<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildScheduledEvent {
            guild_id: self.guild_id.get(),
            scheduled_event_id: self.scheduled_event_id.get(),
            with_user_count: self.with_user_count,
        }))
    }
}
