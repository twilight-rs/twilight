use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::{
    channel::thread::ThreadMember,
    id::{ChannelId, UserId},
};

/// Returns a [`ThreadMember`] in a thread.
///
/// [`ThreadMember`]: twilight_model::channel::thread::ThreadMember
#[must_use = "requests must be configured and executed"]
pub struct GetThreadMember<'a> {
    channel_id: ChannelId,
    http: &'a Client,
    user_id: UserId,
}

impl<'a> GetThreadMember<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: ChannelId, user_id: UserId) -> Self {
        Self {
            channel_id,
            http,
            user_id,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ThreadMember> {
        let request = Request::from_route(&Route::GetThreadMember {
            channel_id: self.channel_id.get(),
            user_id: self.user_id.get(),
        });

        self.http.request(request)
    }
}
