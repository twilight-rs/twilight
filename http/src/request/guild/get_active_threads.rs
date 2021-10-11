use crate::{
    client::Client,
    error::Error,
    request::{IntoRequest, Request},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{channel::thread::ThreadsListing, id::GuildId};

/// Returns all active threads in the guild.
///
/// Includes public and private threads. Threads are ordered by their ID in
/// descending order.
#[must_use = "requests must be configured and executed"]
pub struct GetActiveThreads<'a> {
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetActiveThreads<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self { guild_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ThreadsListing> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for GetActiveThreads<'_> {
    fn into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetActiveThreads {
            guild_id: self.guild_id.get(),
        }))
    }
}
