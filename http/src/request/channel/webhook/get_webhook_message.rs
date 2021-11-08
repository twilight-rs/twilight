use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::{
    channel::Message,
    id::{
        marker::{MessageMarker, WebhookMarker},
        Id,
    },
};

/// Get a webhook message by webhook ID, token, and message ID.
#[must_use = "requests must be configured and executed"]
pub struct GetWebhookMessage<'a> {
    http: &'a Client,
    message_id: Id<MessageMarker>,
    token: &'a str,
    webhook_id: Id<WebhookMarker>,
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
            message_id,
            token,
            webhook_id,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Message> {
        let request = Request::builder(&Route::GetWebhookMessage {
            message_id: self.message_id.get(),
            token: self.token,
            webhook_id: self.webhook_id.get(),
        })
        .use_authorization_token(false)
        .build();

        self.http.request(request)
    }
}
