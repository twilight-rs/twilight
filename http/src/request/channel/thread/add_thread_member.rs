use crate::{
    client::Client,
    error::Error,
    request::{IntoRequest, Request},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{ChannelId, UserId};

/// Add another member to a thread.
///
/// Requires the ability to send messages in the thread, and that the thread is
/// not archived.
#[must_use = "requests must be configured and executed"]
pub struct AddThreadMember<'a> {
    channel_id: ChannelId,
    http: &'a Client,
    user_id: UserId,
}

impl<'a> AddThreadMember<'a> {
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
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for AddThreadMember<'_> {
    fn into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::AddThreadMember {
            channel_id: self.channel_id.get(),
            user_id: self.user_id.get(),
        }))
    }
}
