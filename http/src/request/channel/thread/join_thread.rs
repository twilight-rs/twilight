use crate::{
    client::Client,
    request::Request,
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::ChannelId;

/// Add the current user to a thread.
#[must_use = "requests must be configured and executed"]
pub struct JoinThread<'a> {
    channel_id: ChannelId,
    http: &'a Client,
}

impl<'a> JoinThread<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self { channel_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let request = Request::from_route(&Route::JoinThread {
            channel_id: self.channel_id.get(),
        });

        self.http.request(request)
    }
}
