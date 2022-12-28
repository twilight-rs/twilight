use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{marker::WebhookMarker, Id};
use twilight_validate::request::{audit_reason as validate_audit_reason, ValidationError};

struct DeleteWebhookParams<'a> {
    token: Option<&'a str>,
}

/// Delete a webhook by its ID.
#[must_use = "requests must be configured and executed"]
pub struct DeleteWebhook<'a> {
    fields: DeleteWebhookParams<'a>,
    http: &'a Client,
    id: Id<WebhookMarker>,
    reason: Option<&'a str>,
}

impl<'a> DeleteWebhook<'a> {
    pub(crate) const fn new(http: &'a Client, id: Id<WebhookMarker>) -> Self {
        Self {
            fields: DeleteWebhookParams { token: None },
            http,
            id,
            reason: None,
        }
    }

    /// Specify the token for auth, if not already authenticated with a Bot token.
    pub const fn token(mut self, token: &'a str) -> Self {
        self.fields.token = Some(token);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    #[deprecated(since = "0.14.0", note = "use `.await` or `into_future` instead")]
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        self.into_future()
    }
}

impl<'a> AuditLogReason<'a> for DeleteWebhook<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, ValidationError> {
        validate_audit_reason(reason)?;

        self.reason.replace(reason);

        Ok(self)
    }
}

impl IntoFuture for DeleteWebhook<'_> {
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

impl TryIntoRequest for DeleteWebhook<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::DeleteWebhook {
            webhook_id: self.id.get(),
            token: self.fields.token,
        });

        if let Some(reason) = self.reason.as_ref() {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}
