use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    channel::thread::ThreadMember,
    id::{
        Id,
        marker::{ChannelMarker, UserMarker},
    },
};

struct GetThreadMemberPath {
    channel_id: Id<ChannelMarker>,
    user_id: Id<UserMarker>,
}

/// Returns a [`ThreadMember`] in a thread.
///
/// [`ThreadMember`]: twilight_model::channel::thread::ThreadMember
#[must_use = "requests must be configured and executed"]
pub struct GetThreadMember<'a> {
    http: &'a Client,
    path: GetThreadMemberPath,
}

impl<'a> GetThreadMember<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        user_id: Id<UserMarker>,
    ) -> Self {
        Self {
            http,
            path: GetThreadMemberPath {
                channel_id,
                user_id,
            },
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
        let GetThreadMemberPath {
            channel_id,
            user_id,
        } = self.path;

        let path = Path::builder()
            .resource("channels")
            .id(channel_id)
            .resource("thread-members")
            .id(user_id)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
