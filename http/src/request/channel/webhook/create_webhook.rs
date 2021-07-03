use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, AuditLogReasonError, PendingResponse, Request},
    routing::Route,
};
use serde::Serialize;
use twilight_model::{channel::Webhook, id::ChannelId};

#[derive(Serialize)]
struct CreateWebhookFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<String>,
    name: String,
}

/// Create a webhook in a channel.
///
/// # Examples
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::ChannelId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token");
/// let channel_id = ChannelId(123);
///
/// let webhook = client
///     .create_webhook(channel_id, "Twily Bot")
///     .await?;
/// # Ok(()) }
/// ```
pub struct CreateWebhook<'a> {
    channel_id: ChannelId,
    fields: CreateWebhookFields,
    fut: Option<PendingResponse<'a, Webhook>>,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> CreateWebhook<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId, name: impl Into<String>) -> Self {
        Self {
            channel_id,
            fields: CreateWebhookFields {
                avatar: None,
                name: name.into(),
            },
            fut: None,
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
    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.fields.avatar.replace(avatar.into());

        self
    }

    fn start(&mut self) -> Result<(), Error> {
        let mut request = Request::builder(Route::CreateWebhook {
            channel_id: self.channel_id.0,
        })
        .json(&self.fields)?;

        if let Some(reason) = self.reason.as_ref() {
            request = request.headers(request::audit_header(reason)?);
        }

        self.fut
            .replace(Box::pin(self.http.request(request.build())));

        Ok(())
    }
}

impl<'a> AuditLogReason for CreateWebhook<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(CreateWebhook<'_>, Webhook);
