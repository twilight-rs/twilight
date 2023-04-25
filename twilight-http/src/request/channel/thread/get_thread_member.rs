use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    channel::thread::ThreadMember,
    id::{
        marker::{ChannelMarker, UserMarker},
        Id,
    },
};

/// Returns a [`ThreadMember`] in a thread.
///
/// [`ThreadMember`]: twilight_model::channel::thread::ThreadMember
#[must_use = "requests must be configured and executed"]
pub struct GetThreadMember<'a> {
    channel_id: Id<ChannelMarker>,
    http: &'a Client,
    user_id: Id<UserMarker>,
}

impl<'a> GetThreadMember<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        user_id: Id<UserMarker>,
    ) -> Self {
        Self {
            channel_id,
            http,
            user_id,
        }
    }
}

impl IntoFuture for GetThreadMember<'_> {
    type Output = Result<Response<ThreadMember>, Error>;

    type IntoFuture = ResponseFuture<ThreadMember>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetThreadMember<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetThreadMember {
            channel_id: self.channel_id.get(),
            user_id: self.user_id.get(),
        }))
    }
}
