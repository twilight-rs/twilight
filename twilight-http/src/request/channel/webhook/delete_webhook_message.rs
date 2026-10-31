use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{ChannelMarker, MessageMarker, WebhookMarker},
};
use twilight_validate::request::{ValidationError, audit_reason as validate_audit_reason};

struct DeleteWebhookMessagePath<'a> {
    message_id: Id<MessageMarker>,
    thread_id: Option<Id<ChannelMarker>>,
    token: &'a str,
    webhook_id: Id<WebhookMarker>,
}

/// Delete a message created by a webhook.
///
/// # Examples
///
/// ```no_run
/// # use twilight_http::Client;
/// use twilight_http::request::AuditLogReason;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = Client::new("token".to_owned());
/// client
///     .delete_webhook_message(Id::new(1), "token here", Id::new(2))
///     .reason("reason here")
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct DeleteWebhookMessage<'a> {
    http: &'a Client,
    path: DeleteWebhookMessagePath<'a>,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> DeleteWebhookMessage<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        webhook_id: Id<WebhookMarker>,
        token: &'a str,
        message_id: Id<MessageMarker>,
    ) -> Self {
        Self {
            http,
            path: DeleteWebhookMessagePath {
                message_id,
                thread_id: None,
                token,
                webhook_id,
            },
            reason: Ok(None),
        }
    }

    /// Delete in a thread belonging to the channel instead of the channel
    /// itself.
    pub const fn thread_id(mut self, thread_id: Id<ChannelMarker>) -> Self {
        self.path.thread_id.replace(thread_id);

        self
    }
}

impl<'a> AuditLogReason<'a> for DeleteWebhookMessage<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for DeleteWebhookMessage<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for DeleteWebhookMessage<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let DeleteWebhookMessagePath {
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

        let mut request = Request::builder(Method::Delete, path).use_authorization_token(false);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
