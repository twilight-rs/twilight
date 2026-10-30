use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use std::future::IntoFuture;
use twilight_model::id::{Id, marker::WebhookMarker};
use twilight_validate::request::{ValidationError, audit_reason as validate_audit_reason};

struct DeleteWebhookPath<'a> {
    token: Option<&'a str>,
    webhook_id: Id<WebhookMarker>,
}

/// Delete a webhook by its ID.
#[must_use = "requests must be configured and executed"]
pub struct DeleteWebhook<'a> {
    http: &'a Client,
    path: DeleteWebhookPath<'a>,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> DeleteWebhook<'a> {
    pub(crate) const fn new(http: &'a Client, id: Id<WebhookMarker>) -> Self {
        Self {
            http,
            path: DeleteWebhookPath {
                token: None,
                webhook_id: id,
            },
            reason: Ok(None),
        }
    }

    /// Specify the token for auth, if not already authenticated with a Bot token.
    pub const fn token(mut self, token: &'a str) -> Self {
        self.path.token = Some(token);

        self
    }
}

impl<'a> AuditLogReason<'a> for DeleteWebhook<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
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
        let DeleteWebhookPath { webhook_id, token } = self.path;

        let path_builder = Path::builder().resource("webhooks").id(webhook_id);

        let path = if let Some(token) = token {
            path_builder
                .no_resource()
                .id(token)
                .build()
                .map_err(Error::path)?
        } else {
            path_builder.build().map_err(Error::path)?
        };
        let mut request = Request::builder(Method::Delete, path);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
