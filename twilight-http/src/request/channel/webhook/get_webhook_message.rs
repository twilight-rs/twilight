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
        marker::{ChannelMarker, MessageMarker, WebhookMarker},
    },
};

struct GetWebhookMessagePath<'a> {
    message_id: Id<MessageMarker>,
    thread_id: Option<Id<ChannelMarker>>,
    token: &'a str,
    webhook_id: Id<WebhookMarker>,
}

/// Get a webhook message by webhook ID, token, and message ID.
#[must_use = "requests must be configured and executed"]
pub struct GetWebhookMessage<'a> {
    http: &'a Client,
    path: GetWebhookMessagePath<'a>,
}

impl<'a> GetWebhookMessage<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        webhook_id: Id<WebhookMarker>,
        token: &'a str,
        message_id: Id<MessageMarker>,
    ) -> Self {
        Self {
            http,
            path: GetWebhookMessagePath {
                message_id,
                thread_id: None,
                token,
                webhook_id,
            },
        }
    }

    /// Get a message in a thread belonging to the channel instead of the
    /// channel itself.
    pub const fn thread_id(mut self, thread_id: Id<ChannelMarker>) -> Self {
        self.path.thread_id.replace(thread_id);

        self
    }
}

impl IntoFuture for GetWebhookMessage<'_> {
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

impl TryIntoRequest for GetWebhookMessage<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetWebhookMessagePath {
            message_id,
            thread_id,
            token,
            webhook_id,
        } = self.path;

        let path = Path::builder()
            .resource("webhooks")
            .id(webhook_id)
            .no_resource()
            .id(token)
            .resource("messages")
            .id(message_id)
            .parameter(("thread_id", thread_id))
            .build()
            .map_err(Error::path)?;

        Request::builder(Method::Get, path)
            .use_authorization_token(false)
            .build()
    }
}
