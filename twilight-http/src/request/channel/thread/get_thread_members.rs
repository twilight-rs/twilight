use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    channel::thread::ThreadMember,
    id::{marker::ChannelMarker, Id},
};

/// Returns the [`ThreadMember`]s of the thread.
///
/// [`ThreadMember`]: twilight_model::channel::thread::ThreadMember
#[must_use = "requests must be configured and executed"]
pub struct GetThreadMembers<'a> {
    channel_id: Id<ChannelMarker>,
    http: &'a Client,
}

impl<'a> GetThreadMembers<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self { channel_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    #[deprecated(since = "0.14.0", note = "use `.await` or `into_future` instead")]
    pub fn exec(self) -> ResponseFuture<ListBody<ThreadMember>> {
        self.into_future()
    }
}

impl IntoFuture for GetThreadMembers<'_> {
    type Output = Result<Response<ListBody<ThreadMember>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<ThreadMember>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetThreadMembers<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetThreadMembers {
            channel_id: self.channel_id.get(),
        }))
    }
}
