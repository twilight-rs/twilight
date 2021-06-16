use super::execute_webhook::ExecuteWebhook;
use crate::{error::Error, request::PendingResponse};
use twilight_model::channel::Message;

/// Execute a webhook, sending a message to its channel, and then wait for the
/// message to be created.
///
/// # Examples
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::WebhookId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token");
/// let id = WebhookId(432);
///
/// let message = client
///     .execute_webhook(id, "webhook token")
///     .content("Pinkie...")
///     .wait()
///     .await?
///     .model()
///     .await?;
///
/// println!("message id: {}", message.id);
/// # Ok(()) }
/// ```
///
/// [`content`]: Self::content
/// [`embeds`]: Self::embeds
/// [`file`]: Self::file
pub struct ExecuteWebhookAndWait<'a> {
    fut: Option<PendingResponse<'a, Message>>,
    inner: ExecuteWebhook<'a>,
}

impl<'a> ExecuteWebhookAndWait<'a> {
    pub(crate) fn new(inner: ExecuteWebhook<'a>) -> Self {
        Self { fut: None, inner }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = self.inner.request(true)?;
        self.fut.replace(Box::pin(self.inner.http.request(request)));

        Ok(())
    }
}

poll_req!(ExecuteWebhookAndWait<'_>, Message);
