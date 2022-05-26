use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, NullableField, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    channel::Webhook,
    id::{
        marker::{ChannelMarker, WebhookMarker},
        Id,
    },
};
use twilight_validate::request::{
    audit_reason as validate_audit_reason, webhook_username as validate_webhook_username,
    ValidationError,
};

#[derive(Serialize)]
struct UpdateWebhookFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<NullableField<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<NullableField<&'a str>>,
}

/// Update a webhook by ID.
#[must_use = "requests must be configured and executed"]
pub struct UpdateWebhook<'a> {
    fields: UpdateWebhookFields<'a>,
    http: &'a Client,
    webhook_id: Id<WebhookMarker>,
    reason: Option<&'a str>,
}

/// Update a webhook by its ID.
impl<'a> UpdateWebhook<'a> {
    pub(crate) const fn new(http: &'a Client, webhook_id: Id<WebhookMarker>) -> Self {
        Self {
            fields: UpdateWebhookFields {
                avatar: None,
                channel_id: None,
                name: None,
            },
            http,
            webhook_id,
            reason: None,
        }
    }

    /// Set the avatar of the webhook.
    ///
    /// This must be a Data URI, in the form of
    /// `data:image/{type};base64,{data}` where `{type}` is the image MIME type
    /// and `{data}` is the base64-encoded image. See [Discord Docs/Image Data].
    ///
    /// [Discord Docs/Image Data]: https://discord.com/developers/docs/reference#image-data
    pub const fn avatar(mut self, avatar: Option<&'a str>) -> Self {
        self.fields.avatar = Some(NullableField(avatar));

        self
    }

    /// Move this webhook to a new channel.
    pub const fn channel_id(mut self, channel_id: Id<ChannelMarker>) -> Self {
        self.fields.channel_id = Some(channel_id);

        self
    }

    /// Change the name of the webhook.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`WebhookUsername`] if the webhook's name is
    /// invalid.
    ///
    /// [`WebhookUsername`]: twilight_validate::request::ValidationErrorType::WebhookUsername
    pub fn name(mut self, name: Option<&'a str>) -> Result<Self, ValidationError> {
        if let Some(name) = name {
            validate_webhook_username(name)?;
        }

        self.fields.name = Some(NullableField(name));

        Ok(self)
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

impl<'a> AuditLogReason<'a> for UpdateWebhook<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, ValidationError> {
        validate_audit_reason(reason)?;

        self.reason.replace(reason);

        Ok(self)
    }
}

impl TryIntoRequest for UpdateWebhook<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdateWebhook {
            token: None,
            webhook_id: self.webhook_id.get(),
        });

        request = request.json(&self.fields)?;

        if let Some(reason) = self.reason.as_ref() {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}
