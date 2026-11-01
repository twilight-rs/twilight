use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::EmptyBody},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{ChannelMarker, UserMarker},
};

struct AddThreadMemberPath {
    channel_id: Id<ChannelMarker>,
    user_id: Id<UserMarker>,
}

/// Add another member to a thread.
///
/// Requires the ability to send messages in the thread, and that the thread is
/// not archived.
#[must_use = "requests must be configured and executed"]
pub struct AddThreadMember<'a> {
    http: &'a Client,
    path: AddThreadMemberPath,
}

impl<'a> AddThreadMember<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        user_id: Id<UserMarker>,
    ) -> Self {
        Self {
            http,
            path: AddThreadMemberPath {
                channel_id,
                user_id,
            },
        }
    }
}

impl IntoFuture for AddThreadMember<'_> {
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

impl TryIntoRequest for AddThreadMember<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::AddThreadMember {
            channel_id: self.path.channel_id.get(),
            user_id: self.path.user_id.get(),
        }))
    }
}
