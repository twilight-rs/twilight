use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{ChannelMarker, UserMarker},
};

/// Remove another member from a thread.
///
/// Requires that the thread is not archived.
///
/// Requires the [`MANAGE_THREADS`] permission, unless both the thread is a
/// [`PrivateThread`], and the current user is the creator of the thread.
///
/// [`PrivateThread`]: twilight_model::channel::ChannelType::PrivateThread
struct RemoveThreadMemberPath {
    channel_id: Id<ChannelMarker>,
    user_id: Id<UserMarker>,
}

/// [`MANAGE_THREADS`]: twilight_model::guild::Permissions::MANAGE_THREADS
#[must_use = "requests must be configured and executed"]
pub struct RemoveThreadMember<'a> {
    http: &'a Client,
    path: RemoveThreadMemberPath,
}

impl<'a> RemoveThreadMember<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        user_id: Id<UserMarker>,
    ) -> Self {
        Self {
            http,
            path: RemoveThreadMemberPath {
                channel_id,
                user_id,
            },
        }
    }
}

impl IntoFuture for RemoveThreadMember<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for RemoveThreadMember<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let RemoveThreadMemberPath {
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

        Ok(Request::new(Method::Delete, path))
    }
}
