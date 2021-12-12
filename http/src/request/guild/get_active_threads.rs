use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    channel::thread::ThreadsListing,
    id::{marker::GuildMarker, Id},
};

/// Returns all active threads in the guild.
///
/// Includes public and private threads. Threads are ordered by their ID in
/// descending order.
#[must_use = "requests must be configured and executed"]
pub struct GetActiveThreads<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetActiveThreads<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ThreadsListing> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetActiveThreads<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetActiveThreads {
            guild_id: self.guild_id.get(),
        }))
    }
}
