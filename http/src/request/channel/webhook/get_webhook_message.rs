use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
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
pub struct GetWebhookMessage<'a> {
    fut: Option<PendingResponse<'a, Message>>,
    http: &'a Client,
    message_id: MessageId,
    token: String,
    webhook_id: WebhookId,
}

impl<'a> GetWebhookMessage<'a> {
    pub(crate) fn new(
        http: &'a Client,
        webhook_id: WebhookId,
        token: impl Into<String>,
        message_id: MessageId,
    ) -> Self {
        Self {
            fut: None,
            http,
            message_id,
            token: token.into(),
            webhook_id,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::builder(Route::GetWebhookMessage {
            message_id: self.message_id.0,
            token: self.token.clone(),
            webhook_id: self.webhook_id.0,
        })
        .use_authorization_token(false)
        .build();

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetWebhookMessage<'_>, Message);
