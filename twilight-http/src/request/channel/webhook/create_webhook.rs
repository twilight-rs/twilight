use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    channel::Webhook,
    id::{marker::ChannelMarker, Id},
};
use twilight_validate::request::{
    audit_reason as validate_audit_reason, webhook_username as validate_webhook_username,
    ValidationError,
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
/// let webhook = client.create_webhook(channel_id, "Twily Bot").await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct CreateWebhook<'a> {
    channel_id: Id<ChannelMarker>,
    fields: Result<CreateWebhookFields<'a>, ValidationError>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> CreateWebhook<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: Id<ChannelMarker>, name: &'a str) -> Self {
        let fields = Ok(CreateWebhookFields { avatar: None, name }).and_then(|fields| {
            validate_webhook_username(name)?;

            Ok(fields)
        });

        Self {
            channel_id,
            fields,
            http,
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
    pub fn avatar(mut self, avatar: &'a str) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.avatar = Some(avatar);

            fields
        });

        self
    }
}

impl<'a> AuditLogReason<'a> for CreateWebhook<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for CreateWebhook<'_> {
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

impl TryIntoRequest for CreateWebhook<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;
        let mut request = Request::builder(&Route::CreateWebhook {
            channel_id: self.channel_id.get(),
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
    fn test_create_webhook() -> Result<(), Box<dyn Error>> {
        const CHANNEL_ID: Id<ChannelMarker> = Id::new(1);

        let client = Client::new("token".into());

        {
            let expected = r#"{"name":"Spidey Bot"}"#;
            let actual =
                CreateWebhook::new(&client, CHANNEL_ID, "Spidey Bot").try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }

        {
            let expected = r#"{"avatar":"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI","name":"Spidey Bot"}"#;
            let actual = CreateWebhook::new(&client, CHANNEL_ID, "Spidey Bot")
            .avatar(
                "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI",
            )
                .try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }

        Ok(())
    }
}
