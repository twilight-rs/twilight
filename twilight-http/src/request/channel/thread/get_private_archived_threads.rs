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

struct GetPrivateArchivedThreadsPath<'a> {
    before: Option<&'a str>,
    channel_id: Id<ChannelMarker>,
    limit: Option<u64>,
}

/// Returns archived private threads in the channel.
///
/// Requires both [`READ_MESSAGE_HISTORY`] and [`MANAGE_THREADS`].
///
/// [`MANAGE_THREADS`]: twilight_model::guild::Permissions::MANAGE_THREADS
/// [`READ_MESSAGE_HISTORY`]: twilight_model::guild::Permissions::READ_MESSAGE_HISTORY
#[must_use = "requests must be configured and executed"]
pub struct GetPrivateArchivedThreads<'a> {
    http: &'a Client,
    path: GetPrivateArchivedThreadsPath<'a>,
}

impl<'a> GetPrivateArchivedThreads<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            http,
            path: GetPrivateArchivedThreadsPath {
                before: None,
                channel_id,
                limit: None,
            },
        }
    }

    /// Return threads before this ISO 8601 timestamp.
    pub const fn before(mut self, before: &'a str) -> Self {
        self.path.before = Some(before);

        self
    }

    /// Maximum number of threads to return.
    pub const fn limit(mut self, limit: u64) -> Self {
        self.path.limit = Some(limit);

        self
    }
}

impl IntoFuture for GetPrivateArchivedThreads<'_> {
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

impl TryIntoRequest for GetPrivateArchivedThreads<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetPrivateArchivedThreadsPath {
            before,
            channel_id,
            limit,
        } = self.path;

        let path = Path::builder()
            .resource("channels")
            .id(channel_id)
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
