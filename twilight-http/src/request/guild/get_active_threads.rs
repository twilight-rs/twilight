use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    channel::thread::ThreadsListing,
    id::{Id, marker::GuildMarker},
};

struct GetActiveThreadsPath {
    guild_id: Id<GuildMarker>,
}

/// Returns all active threads in the guild.
///
/// Includes public and private threads. Threads are ordered by their ID in
/// descending order.
#[must_use = "requests must be configured and executed"]
pub struct GetActiveThreads<'a> {
    path: GetActiveThreadsPath,
    http: &'a Client,
}

impl<'a> GetActiveThreads<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            path: GetActiveThreadsPath { guild_id },
            http,
        }
    }
}

impl IntoFuture for GetActiveThreads<'_> {
    type Output = Result<Response<ThreadsListing>, Error>;

    type IntoFuture = ResponseFuture<ThreadsListing>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetActiveThreads<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetActiveThreadsPath { guild_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("threads")
            .subresource("active")
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
