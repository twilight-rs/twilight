use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
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

pub struct DeleteGuildScheduledEventPath {
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
    http: &'a Client,
    path: DeleteGuildScheduledEventPath,
}

impl<'a> DeleteGuildScheduledEvent<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        scheduled_event_id: Id<ScheduledEventMarker>,
    ) -> Self {
        Self {
            http,
            path: DeleteGuildScheduledEventPath {
                guild_id,
                scheduled_event_id,
            },
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

impl TryIntoRequest for DeleteGuildScheduledEvent<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = Path::builder()
            .resource("guilds")
            .id(self.path.guild_id)
            .resource("scheduled-events")
            .id(self.path.scheduled_event_id)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Delete, path))
    }
}
