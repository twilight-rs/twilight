use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    channel::Message,
    id::{marker, Id},
};

/// Get a webhook message by webhook ID, token, and message ID.
#[must_use = "requests must be configured and executed"]
pub struct GetWebhookMessage<'a> {
    http: &'a Client,
    message_id: Id<marker::Message>,
    thread_id: Option<Id<marker::Channel>>,
    token: &'a str,
    webhook_id: Id<marker::Webhook>,
}

impl<'a> GetWebhookMessage<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        webhook_id: Id<marker::Webhook>,
        token: &'a str,
        message_id: Id<marker::Message>,
    ) -> Self {
        Self {
            http,
            message_id,
            thread_id: None,
            token,
            webhook_id,
        }
    }

    /// Get a message in a thread belonging to the channel instead of the
    /// channel itself.
    pub fn thread_id(mut self, thread_id: Id<marker::Channel>) -> Self {
        self.thread_id.replace(thread_id);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Message> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetWebhookMessage<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::builder(&Route::GetWebhookMessage {
            message_id: self.message_id.get(),
            thread_id: self.thread_id.map(Id::get),
            token: self.token,
            webhook_id: self.webhook_id.get(),
        })
        .use_authorization_token(false)
        .build())
    }
}
