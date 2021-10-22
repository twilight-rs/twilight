use crate::{
    client::Client,
    request::Request,
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{ChannelId, UserId};

/// Remove another member from a thread.
///
/// Requires that the thread is not archived.
///
/// Requires the [`MANAGE_THREADS`] permission, unless both the thread is a
/// [`GuildPrivateThread`], and the current user is the creator of the thread.
///
/// [`GuildPrivateThread`]: twilight_model::channel::ChannelType::GuildPrivateThread
/// [`MANAGE_THREADS`]: twilight_model::guild::Permissions::MANAGE_THREADS
#[must_use = "requests must be configured and executed"]
pub struct RemoveThreadMember<'a> {
    channel_id: ChannelId,
    http: &'a Client,
    user_id: UserId,
}

impl<'a> RemoveThreadMember<'a> {
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
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let request = Request::from_route(&Route::RemoveThreadMember {
            channel_id: self.channel_id.get(),
            user_id: self.user_id.get(),
        });

        self.http.request(request)
    }
}
