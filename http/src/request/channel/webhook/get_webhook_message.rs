use crate::{
    client::Client,
    error::Error,
    request::{IntoRequest, Request},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    channel::Message,
    id::{MessageId, WebhookId},
};

/// Get a webhook message by [`WebhookId`], token, and [`MessageId`].
///
/// [`WebhookId`]: twilight_model::id::WebhookId
/// [`MessageId`]: twilight_model::id::MessageId
#[must_use = "requests must be configured and executed"]
pub struct GetWebhookMessage<'a> {
    http: &'a Client,
    message_id: MessageId,
    token: &'a str,
    webhook_id: WebhookId,
}

impl<'a> GetWebhookMessage<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        webhook_id: WebhookId,
        token: &'a str,
        message_id: MessageId,
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
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for GetWebhookMessage<'_> {
    fn into_request(self) -> Result<Request, Error> {
        Ok(Request::builder(&Route::GetWebhookMessage {
            message_id: self.message_id.get(),
            token: self.token,
            webhook_id: self.webhook_id.get(),
        })
        .use_authorization_token(false)
        .build())
    }
}
