use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    channel::Webhook,
    id::{Id, marker::WebhookMarker},
};

struct GetWebhookPath<'a> {
    id: Id<WebhookMarker>,
    token: Option<&'a str>,
}

/// Get a webhook by ID.
#[must_use = "requests must be configured and executed"]
pub struct GetWebhook<'a> {
    http: &'a Client,
    path: GetWebhookPath<'a>,
}

impl<'a> GetWebhook<'a> {
    pub(crate) const fn new(http: &'a Client, id: Id<WebhookMarker>) -> Self {
        Self {
            http,
            path: GetWebhookPath { id, token: None },
        }
    }

    /// Specify the token for auth, if not already authenticated with a Bot
    /// token.
    pub const fn token(mut self, token: &'a str) -> Self {
        self.path.token = Some(token);

        self
    }
}

impl IntoFuture for GetWebhook<'_> {
    type Output = Result<Response<Webhook>, Error>;

    type IntoFuture = ResponseFuture<Webhook>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetWebhook<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetWebhookPath { id, token } = self.path;

        let path_builder = Path::builder().resource("webhooks").id(id);

        let path = if let Some(token) = token {
            path_builder
                .no_resource()
                .id(token)
                .build()
                .map_err(Error::path)?
        } else {
            path_builder.build().map_err(Error::path)?
        };
        let mut request = Request::builder(Method::Get, path);

        let use_webhook_token = self.path.token.is_some();

        // If a webhook token has been configured, then we don't need to use
        // the client's authorization token.
        if use_webhook_token {
            request = request.use_authorization_token(false);
        }

        request.build()
    }
}
