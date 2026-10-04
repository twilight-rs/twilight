use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::scheduled_event::GuildScheduledEvent,
    id::{
        Id,
        marker::{GuildMarker, ScheduledEventMarker},
    },
};

pub(crate) struct GetGuildScheduledEventFields {
    guild_id: Id<GuildMarker>,
    scheduled_event_id: Id<ScheduledEventMarker>,
    with_user_count: bool,
}

/// Get a scheduled event in a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildScheduledEvent<'a> {
    fields: GetGuildScheduledEventFields,
    http: &'a Client,
}

impl<'a> GetGuildScheduledEvent<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        scheduled_event_id: Id<ScheduledEventMarker>,
    ) -> Self {
        Self {
            fields: GetGuildScheduledEventFields {
                guild_id,
                scheduled_event_id,
                with_user_count: false,
            },
            http,
        }
    }

    /// Set whether to include the number of subscribed users.
    pub const fn with_user_count(mut self, with_user_count: bool) -> Self {
        self.fields.with_user_count = with_user_count;

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

impl Route for GetGuildScheduledEvent<'_> {
    type Fields = GetGuildScheduledEventFields;

    const METHOD: Method = Method::Get;

    fn path(fields: Self::Fields) -> Path {
        let builder = Path::builder()
            .resource("guilds")
            .id(fields.guild_id)
            .resource("scheduled-events")
            .id(fields.scheduled_event_id);

        if fields.with_user_count == true {
            builder.parameter("with_user_count", true).build()
        } else {
            builder.build()
        }
    }
}

impl TryIntoRequest for GetGuildScheduledEvent<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route_new::<Self>(self.fields))
    }
}
