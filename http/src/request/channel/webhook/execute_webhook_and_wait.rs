use super::execute_webhook::ExecuteWebhook;
use crate::{
    error::Error,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
};
use twilight_model::channel::Message;

/// Execute a webhook, sending a message to its channel, and then wait for the
/// message to be created.
///
/// # Examples
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
/// let id = Id::new(432);
///
/// let message = client
///     .execute_webhook(id, "webhook token")
///     .content("Pinkie...")?
///     .wait()
///     .exec()
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
#[must_use = "requests must be configured and executed"]
pub struct ExecuteWebhookAndWait<'a> {
    inner: ExecuteWebhook<'a>,
}

impl<'a> ExecuteWebhookAndWait<'a> {
    pub(crate) const fn new(inner: ExecuteWebhook<'a>) -> Self {
        Self { inner }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Message> {
        let http = self.inner.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for ExecuteWebhookAndWait<'_> {
    fn try_into_request(mut self) -> Result<Request, Error> {
        self.inner.request(true)
    }
}
