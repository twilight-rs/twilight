use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture, marker::ListBody},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::scheduled_event::GuildScheduledEvent,
    id::{Id, marker::GuildMarker},
};

pub(crate) struct GetGuildScheduledEventsFields {
    guild_id: Id<GuildMarker>,
    with_user_count: bool,
}

/// Get a list of scheduled events in a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildScheduledEvents<'a> {
    fields: GetGuildScheduledEventsFields,
    http: &'a Client,
}

impl<'a> GetGuildScheduledEvents<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            fields: GetGuildScheduledEventsFields {
                guild_id,
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

impl IntoFuture for GetGuildScheduledEvents<'_> {
    type Output = Result<Response<ListBody<GuildScheduledEvent>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<GuildScheduledEvent>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl Route for GetGuildScheduledEvents<'_> {
    type Fields = GetGuildScheduledEventsFields;

    const METHOD: Method = Method::Get;

    fn path(fields: Self::Fields) -> Path {
        let builder = Path::builder()
            .resource("guilds")
            .id(fields.guild_id)
            .resource("scheduled-events");

        if fields.with_user_count {
            builder.parameter("with_user_count", true).build()
        } else {
            builder.build()
        }
    }
}

impl TryIntoRequest for GetGuildScheduledEvents<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route_new::<Self>(self.fields))
    }
}
