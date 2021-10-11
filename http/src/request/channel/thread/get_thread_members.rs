use crate::{
    client::Client,
    request::{IntoRequest, Request},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
    Error,
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
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for GetThreadMembers<'_> {
    fn into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetThreadMembers {
            channel_id: self.channel_id.get(),
        }))
    }
}
