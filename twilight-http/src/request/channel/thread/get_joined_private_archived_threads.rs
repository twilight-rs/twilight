use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    channel::thread::ThreadsListing,
    id::{Id, marker::ChannelMarker},
};

struct GetJoinedPrivateArchivedThreadsPath {
    before: Option<Id<ChannelMarker>>,
    channel_id: Id<ChannelMarker>,
    limit: Option<u64>,
}

/// Returns archived private threads in the channel that the current user has
/// joined.
///
/// Threads are ordered by their ID in descending order.
#[must_use = "requests must be configured and executed"]
pub struct GetJoinedPrivateArchivedThreads<'a> {
    http: &'a Client,
    path: GetJoinedPrivateArchivedThreadsPath,
}

impl<'a> GetJoinedPrivateArchivedThreads<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            http,
            path: GetJoinedPrivateArchivedThreadsPath {
                before: None,
                channel_id,
                limit: None,
            },
        }
    }

    /// Return threads before this ID.
    pub const fn before(mut self, before: Id<ChannelMarker>) -> Self {
        self.path.before = Some(before);

        self
    }

    /// Maximum number of threads to return.
    pub const fn limit(mut self, limit: u64) -> Self {
        self.path.limit = Some(limit);

        self
    }
}

impl IntoFuture for GetJoinedPrivateArchivedThreads<'_> {
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

impl TryIntoRequest for GetJoinedPrivateArchivedThreads<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetJoinedPrivateArchivedThreadsPath {
            before,
            channel_id,
            limit,
        } = self.path;

        let path = Path::builder()
            .resource("channels")
            .id(channel_id)
            .resource("users")
            .me()
            .resource("threads")
            .subresource("archived")
            .action("private")
            .parameter(("before", before))
            .parameter(("limit", limit))
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
