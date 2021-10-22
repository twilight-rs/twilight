use crate::{
    client::Client,
    request::Request,
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{channel::thread::ThreadMember, id::ChannelId};

/// Returns the [`ThreadMember`]s of the thread.
///
/// [`ThreadMember`]: twilight_model::channel::thread::ThreadMember
#[must_use = "requests must be configured and executed"]
pub struct GetThreadMembers<'a> {
    channel_id: ChannelId,
    http: &'a Client,
}

impl<'a> GetThreadMembers<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self { channel_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<ThreadMember>> {
        let request = Request::from_route(&Route::GetThreadMembers {
            channel_id: self.channel_id.get(),
        });

        self.http.request(request)
    }
}
