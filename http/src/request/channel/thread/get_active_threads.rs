use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::{channel::thread::ThreadsListing, id::ChannelId};

/// Returns all active threads in the channel.
///
/// Includes public and private threads. Threads are ordered by their ID in
/// descending order.
pub struct GetActiveThreads<'a> {
    channel_id: ChannelId,
    http: &'a Client,
}

impl<'a> GetActiveThreads<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self { channel_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ThreadsListing> {
        let request = Request::from_route(&Route::GetActiveThreads {
            channel_id: self.channel_id.0,
        });

        self.http.request(request)
    }
}
