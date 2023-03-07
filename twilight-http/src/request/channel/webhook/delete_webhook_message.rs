use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker, WebhookMarker},
    Id,
};
use twilight_validate::request::{audit_reason as validate_audit_reason, ValidationError};

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
    message_id: Id<MessageMarker>,
    reason: Result<Option<&'a str>, ValidationError>,
    thread_id: Option<Id<ChannelMarker>>,
    token: &'a str,
    webhook_id: Id<WebhookMarker>,
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
            message_id,
            reason: Ok(None),
            thread_id: None,
            token,
            webhook_id,
        }
    }

    /// Delete in a thread belonging to the channel instead of the channel
    /// itself.
    pub fn thread_id(mut self, thread_id: Id<ChannelMarker>) -> Self {
        self.thread_id.replace(thread_id);

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
        let mut request = Request::builder(&Route::DeleteWebhookMessage {
            message_id: self.message_id.get(),
            thread_id: self.thread_id.map(Id::get),
            token: self.token,
            webhook_id: self.webhook_id.get(),
        })
        .use_authorization_token(false);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}

#[cfg(test)]
mod tests {
    use super::DeleteWebhookMessage;
    use crate::{
        client::Client,
        request::{Request, TryIntoRequest},
        routing::Route,
    };
    use twilight_model::id::Id;

    #[test]
    fn request() {
        let client = Client::new("token".to_owned());
        let builder = DeleteWebhookMessage::new(&client, Id::new(1), "token", Id::new(2));
        let actual = builder
            .try_into_request()
            .expect("failed to create request");

        let expected = Request::from_route(&Route::DeleteWebhookMessage {
            message_id: 2,
            thread_id: None,
            token: "token",
            webhook_id: 1,
        });

        assert_eq!(expected.body, actual.body);
        assert_eq!(expected.path, actual.path);
    }
}
