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

pub struct DeleteGuildScheduledEventFields {
    guild_id: Id<GuildMarker>,
    scheduled_event_id: Id<ScheduledEventMarker>,
}

/// Delete a scheduled event in a guild.
///
/// # Examples
///
/// ```no_run
/// # use twilight_http::Client;
/// # use twilight_model::id::Id;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = Client::new("token".to_owned());
/// let guild_id = Id::new(1);
/// let scheduled_event_id = Id::new(2);
///
/// client
///     .delete_guild_scheduled_event(guild_id, scheduled_event_id)
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct DeleteGuildScheduledEvent<'a> {
    fields: DeleteGuildScheduledEventFields,
    http: &'a Client,
}

impl<'a> DeleteGuildScheduledEvent<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        scheduled_event_id: Id<ScheduledEventMarker>,
    ) -> Self {
        Self {
            fields: DeleteGuildScheduledEventFields {
                guild_id,
                scheduled_event_id,
            },
            http,
        }
    }
}

impl IntoFuture for DeleteGuildScheduledEvent<'_> {
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

impl Route for DeleteGuildScheduledEvent<'_> {
    type Fields = DeleteGuildScheduledEventFields;

    const METHOD: Method = Method::Delete;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("guilds")
            .id(fields.guild_id)
            .resource("scheduled-events")
            .id(fields.scheduled_event_id)
            .build()
    }
}

impl TryIntoRequest for DeleteGuildScheduledEvent<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route_new::<Self>(self.fields))
    }
}
