use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture},
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    channel::Message,
    id::{
        Id,
        marker::{ChannelMarker, MessageMarker},
    },
};

#[derive(Serialize)]
pub(in crate::request) struct EndPollPath {
    channel_id: Id<ChannelMarker>,
    message_id: Id<MessageMarker>,
}

// Ends a poll in a channel.
#[must_use = "requests must be configured and executed"]
pub struct EndPoll<'a> {
    http: &'a Client,
    path: EndPollPath,
}

impl<'a> EndPoll<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    ) -> Self {
        Self {
            http,
            path: EndPollPath {
                channel_id,
                message_id,
            },
        }
    }
}

impl IntoFuture for EndPoll<'_> {
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

impl Route for EndPoll<'_> {
    type Fields = EndPollPath;

    const METHOD: Method = Method::Post;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("channels")
            .id(fields.channel_id)
            .resource("polls")
            .id(fields.message_id)
            .action("expire")
            .build()
    }
}

impl TryIntoRequest for EndPoll<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route_new::<Self>(self.path))
    }
}
