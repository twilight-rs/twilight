use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    channel::Message,
    id::{
        Id,
        marker::{ChannelMarker, MessageMarker},
    },
};

struct GetMessagePath {
    channel_id: Id<ChannelMarker>,
    message_id: Id<MessageMarker>,
}

/// Get a message by [`Id<ChannelMarker>`] and [`Id<MessageMarker>`].
#[must_use = "requests must be configured and executed"]
pub struct GetMessage<'a> {
    http: &'a Client,
    path: GetMessagePath,
}

impl<'a> GetMessage<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    ) -> Self {
        Self {
            http,
            path: GetMessagePath {
                channel_id,
                message_id,
            },
        }
    }
}

impl IntoFuture for GetMessage<'_> {
    type Output = Result<Response<Message>, Error>;

    type IntoFuture = ResponseFuture<Message>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetMessage<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetMessagePath {
            channel_id,
            message_id,
        } = self.path;

        let path = Path::builder()
            .resource("channels")
            .id(channel_id)
            .resource("messages")
            .id(message_id)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
