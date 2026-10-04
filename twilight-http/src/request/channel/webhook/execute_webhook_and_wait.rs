use super::ExecuteWebhook;
use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
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
///     .content("Pinkie...")
///     .wait()
///     .await?
///     .model()
///     .await?;
///
/// println!("message id: {}", message.id);
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct ExecuteWebhookAndWait<'a> {
    http: &'a Client,
    inner: ExecuteWebhook<'a>,
}

impl<'a> ExecuteWebhookAndWait<'a> {
    pub(crate) const fn new(http: &'a Client, inner: ExecuteWebhook<'a>) -> Self {
        Self { http, inner }
    }
}

impl IntoFuture for ExecuteWebhookAndWait<'_> {
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

impl TryIntoRequest for ExecuteWebhookAndWait<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        self.inner.try_into_request()
    }
}
