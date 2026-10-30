use crate::{
    client::Client,
    error::Error,
    request::{Method, Nullable, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    channel::Webhook,
    id::{Id, marker::WebhookMarker},
};
use twilight_validate::request::{ValidationError, webhook_username as validate_webhook_username};

#[derive(Serialize)]
struct UpdateWebhookWithTokenBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
}

struct UpdateWebhookWithTokenPath<'a> {
    token: &'a str,
    webhook_id: Id<WebhookMarker>,
}

/// Update a webhook, with a token, by ID.
#[must_use = "requests must be configured and executed"]
pub struct UpdateWebhookWithToken<'a> {
    body: Result<UpdateWebhookWithTokenBody<'a>, ValidationError>,
    http: &'a Client,
    path: UpdateWebhookWithTokenPath<'a>,
}

impl<'a> UpdateWebhookWithToken<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        webhook_id: Id<WebhookMarker>,
        token: &'a str,
    ) -> Self {
        Self {
            body: Ok(UpdateWebhookWithTokenBody {
                avatar: None,
                name: None,
            }),
            http,
            path: UpdateWebhookWithTokenPath { token, webhook_id },
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
        if let Ok(body) = self.body.as_mut() {
            body.avatar = Some(Nullable(avatar));
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
        self.body = self.body.and_then(|mut body| {
            validate_webhook_username(name)?;
            body.name = Some(name);

            Ok(body)
        });

        self
    }
}

impl IntoFuture for UpdateWebhookWithToken<'_> {
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

impl TryIntoRequest for UpdateWebhookWithToken<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let UpdateWebhookWithTokenPath { token, webhook_id } = self.path;

        let body = self.body.map_err(Error::validation)?;

        let path = Path::builder()
            .resource("webhooks")
            .id(webhook_id)
            .no_resource()
            .id(token)
            .build()
            .map_err(Error::path)?;

        Request::builder(Method::Patch, path)
            .use_authorization_token(false)
            .json(&body)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_update_webhook_with_token() -> Result<(), Box<dyn Error>> {
        const WEBHOOK_ID: Id<WebhookMarker> = Id::new(1);

        let client = Client::new("token".into());

        {
            let expected = r"{}";
            let actual =
                UpdateWebhookWithToken::new(&client, WEBHOOK_ID, "token").try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }

        {
            let expected = r#"{"avatar":"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI"}"#;
            let actual = UpdateWebhookWithToken::new(&client, WEBHOOK_ID, "token")
            .avatar(Some("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI"))
                .try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());

            let expected = r#"{"avatar":null}"#;
            let actual = UpdateWebhookWithToken::new(&client, WEBHOOK_ID, "token")
                .avatar(None)
                .try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }

        {
            let expected = r#"{"name":"Captain Hook"}"#;
            let actual = UpdateWebhookWithToken::new(&client, WEBHOOK_ID, "token")
                .name("Captain Hook")
                .try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }

        {
            let expected = r#"{"avatar":null,"name":"Captain Hook"}"#;
            let actual = UpdateWebhookWithToken::new(&client, WEBHOOK_ID, "token")
                .avatar(None)
                .name("Captain Hook")
                .try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }
        Ok(())
    }
}
