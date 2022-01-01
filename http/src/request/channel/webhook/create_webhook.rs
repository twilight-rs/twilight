use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, AuditLogReasonError, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    channel::Webhook,
    id::{marker::ChannelMarker, Id},
};

#[derive(Serialize)]
struct CreateWebhookFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<&'a str>,
    name: &'a str,
}

/// Create a webhook in a channel.
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
/// let channel_id = Id::new(123);
///
/// let webhook = client
///     .create_webhook(channel_id, "Twily Bot")
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct CreateWebhook<'a> {
    channel_id: Id<ChannelMarker>,
    fields: CreateWebhookFields<'a>,
    http: &'a Client,
    reason: Option<&'a str>,
}

impl<'a> CreateWebhook<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        name: &'a str,
    ) -> Self {
        Self {
            channel_id,
            fields: CreateWebhookFields { avatar: None, name },
            http,
            reason: None,
        }
    }

    /// Set the avatar of the webhook.
    ///
    /// This must be a Data URI, in the form of `data:image/{type};base64,{data}` where `{type}` is
    /// the image MIME type and `{data}` is the base64-encoded image. Refer to [the discord docs]
    /// for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/reference#image-data
    pub const fn avatar(mut self, avatar: &'a str) -> Self {
        self.fields.avatar = Some(avatar);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Webhook> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for CreateWebhook<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl TryIntoRequest for CreateWebhook<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::CreateWebhook {
            channel_id: self.channel_id.get(),
        });

        request = request.json(&self.fields)?;

        if let Some(reason) = self.reason.as_ref() {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}
