use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Nullable, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
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
    avatar: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
}

/// Update a webhook by ID.
#[must_use = "requests must be configured and executed"]
pub struct UpdateWebhook<'a> {
    fields: Result<UpdateWebhookFields<'a>, ValidationError>,
    http: &'a Client,
    webhook_id: Id<WebhookMarker>,
    reason: Result<Option<&'a str>, ValidationError>,
}

/// Update a webhook by its ID.
impl<'a> UpdateWebhook<'a> {
    pub(crate) const fn new(http: &'a Client, webhook_id: Id<WebhookMarker>) -> Self {
        Self {
            fields: Ok(UpdateWebhookFields {
                avatar: None,
                channel_id: None,
                name: None,
            }),
            http,
            webhook_id,
            reason: Ok(None),
        }
    }

    /// Set the avatar of the webhook.
    ///
    /// This must be a Data URI, in the form of
    /// `data:image/{type};base64,{data}` where `{type}` is the image MIME type
    /// and `{data}` is the base64-encoded image. See [Discord Docs/Image Data].
    ///
    /// [Discord Docs/Image Data]: https://discord.com/developers/docs/reference#image-data
    pub fn avatar(mut self, avatar: Option<&'a str>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.avatar = Some(Nullable(avatar));
        }

        self
    }

    /// Move this webhook to a new channel.
    pub fn channel_id(mut self, channel_id: Id<ChannelMarker>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.channel_id = Some(channel_id);
        }

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
    pub fn name(mut self, name: &'a str) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_webhook_username(name)?;
            fields.name = Some(name);

            Ok(fields)
        });

        self
    }
}

impl<'a> AuditLogReason<'a> for UpdateWebhook<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for UpdateWebhook<'_> {
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

impl TryIntoRequest for UpdateWebhook<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;
        let mut request = Request::builder(&Route::UpdateWebhook {
            token: None,
            webhook_id: self.webhook_id.get(),
        })
        .json(&fields);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_update_webhook() -> Result<(), Box<dyn Error>> {
        const WEBHOOK_ID: Id<WebhookMarker> = Id::new(1);
        const CHANNEL_ID: Id<ChannelMarker> = Id::new(2);

        let client = Client::new("token".into());

        {
            let expected = r"{}";
            let actual = UpdateWebhook::new(&client, WEBHOOK_ID).try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }

        {
            let expected = r#"{"avatar":"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI"}"#;
            let actual = UpdateWebhook::new(&client, WEBHOOK_ID)
            .avatar(Some("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI"))
                .try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());

            let expected = r#"{"avatar":null}"#;
            let actual = UpdateWebhook::new(&client, WEBHOOK_ID)
                .avatar(None)
                .try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }

        {
            let expected = r#"{"channel_id":"2"}"#;
            let actual = UpdateWebhook::new(&client, WEBHOOK_ID)
                .channel_id(CHANNEL_ID)
                .try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }

        {
            let expected = r#"{"name":"Captain Hook"}"#;
            let actual = UpdateWebhook::new(&client, WEBHOOK_ID)
                .name("Captain Hook")
                .try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }

        {
            let expected = r#"{"avatar":null,"channel_id":"2","name":"Captain Hook"}"#;
            let actual = UpdateWebhook::new(&client, WEBHOOK_ID)
                .avatar(None)
                .channel_id(CHANNEL_ID)
                .name("Captain Hook")
                .try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }
        Ok(())
    }
}
