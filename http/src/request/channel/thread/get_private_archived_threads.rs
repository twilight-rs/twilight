use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::{channel::thread::ThreadsListing, id::ChannelId};

/// Returns archived private threads in the channel.
///
/// Requires both [`READ_MESSAGE_HISTORY`] and [`MANAGE_THREADS`].
///
/// [`MANAGE_THREADS`]: twilight_model::guild::Permissions::MANAGE_THREADS
/// [`READ_MESSAGE_HISTORY`]: twilight_model::guild::Permissions::READ_MESSAGE_HISTORY
#[must_use = "requests must be configured and executed"]
pub struct GetPrivateArchivedThreads<'a> {
    before: Option<&'a str>,
    channel_id: ChannelId,
    http: &'a Client,
    limit: Option<u64>,
}

impl<'a> GetPrivateArchivedThreads<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            before: None,
            channel_id,
            http,
            limit: None,
        }
    }

    /// Return threads before this ISO 8601 timestamp.
    pub const fn before(mut self, before: &'a str) -> Self {
        self.before = Some(before);

        self
    }

    /// Maximum number of threads to return.
    pub const fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ThreadsListing> {
        let request = Request::from_route(&Route::GetPrivateArchivedThreads {
            before: self.before,
            channel_id: self.channel_id.get(),
            limit: self.limit,
        });

        self.http.request(request)
    }
}
