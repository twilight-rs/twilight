use crate::request::prelude::*;
use twilight_model::{
    channel::Webhook,
    id::{ChannelId, WebhookId},
};

#[derive(Default, Serialize)]
struct UpdateWebhookFields {
    #[allow(clippy::option_option)]
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<Option<String>>,
    #[allow(clippy::option_option)]
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<ChannelId>,
    #[allow(clippy::option_option)]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Option<String>>,
}

/// Update a webhook by ID.
pub struct UpdateWebhook<'a> {
    fields: UpdateWebhookFields,
    fut: Option<Pending<'a, Webhook>>,
    http: &'a Client,
    webhook_id: WebhookId,
    reason: Option<String>,
}

/// Update a webhook by its ID.
impl<'a> UpdateWebhook<'a> {
    pub(crate) fn new(http: &'a Client, webhook_id: WebhookId) -> Self {
        Self {
            fields: UpdateWebhookFields::default(),
            fut: None,
            http,
            webhook_id,
            reason: None,
        }
    }

    /// Set the avatar of the webhook.
    ///
    /// See [Discord Docs/Image Data] for more information. This must be a Data URI, in the form of
    /// `data:image/{type};base64,{data}` where `{type}` is the image MIME type and `{data}` is the
    /// base64-encoded image.
    ///
    /// [Discord Docs/Image Data]: https://discord.com/developers/docs/reference#image-data
    pub fn avatar(mut self, avatar: impl Into<Option<String>>) -> Self {
        self.fields.avatar.replace(avatar.into());

        self
    }

    /// Move this webhook to a new channel.
    pub fn channel_id(mut self, channel_id: impl Into<ChannelId>) -> Self {
        self.fields.channel_id.replace(channel_id.into());

        self
    }

    /// Change the name of the webhook.
    pub fn name(mut self, name: impl Into<Option<String>>) -> Self {
        self.fields.name.replace(name.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
                headers,
                Route::UpdateWebhook {
                    token: None,
                    webhook_id: self.webhook_id.0,
                },
            ))
        } else {
            Request::from((
                crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
                Route::UpdateWebhook {
                    token: None,
                    webhook_id: self.webhook_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

impl<'a> AuditLogReason for UpdateWebhook<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(UpdateWebhook<'_>, Webhook);
