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
    guild::Emoji,
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
};
use twilight_validate::request::{audit_reason as validate_audit_reason, ValidationError};

#[derive(Serialize)]
struct CreateEmojiFields<'a> {
    image: &'a str,
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<&'a [Id<RoleMarker>]>,
}

/// Create an emoji in a guild.
///
/// The emoji must be a Data URI, in the form of
/// `data:image/{type};base64,{data}` where `{type}` is the image MIME type and
/// `{data}` is the base64-encoded image. See [Discord Docs/Image Data].
///
/// [Discord Docs/Image Data]: https://discord.com/developers/docs/reference#image-data
#[must_use = "requests must be configured and executed"]
pub struct CreateEmoji<'a> {
    fields: CreateEmojiFields<'a>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> CreateEmoji<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        name: &'a str,
        image: &'a str,
    ) -> Self {
        Self {
            fields: CreateEmojiFields {
                image,
                name,
                roles: None,
            },
            guild_id,
            http,
            reason: Ok(None),
        }
    }

    /// Whitelist roles for this emoji.
    ///
    /// See [Discord Docs/Emoji Object].
    ///
    /// [Discord Docs/Emoji Object]: https://discord.com/developers/docs/resources/emoji#emoji-object-emoji-structure
    pub const fn roles(mut self, roles: &'a [Id<RoleMarker>]) -> Self {
        self.fields.roles = Some(roles);

        self
    }
}

impl<'a> AuditLogReason<'a> for CreateEmoji<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for CreateEmoji<'_> {
    type Output = Result<Response<Emoji>, Error>;

    type IntoFuture = ResponseFuture<Emoji>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateEmoji<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::CreateEmoji {
            guild_id: self.guild_id.get(),
        });

        request = request.json(&self.fields);

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
    fn test_create_emoji() -> Result<(), Box<dyn Error>> {
        const GUILD_ID: Id<GuildMarker> = Id::new(1);
        const ROLE_ID: Id<RoleMarker> = Id::new(2);

        let client = Client::new("token".into());

        {
            let expected = r#"{"image":"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI","name":"square"}"#;
            let actual = CreateEmoji::new(
                &client,
                GUILD_ID,
                "square",
                "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI",
            )
            .try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }

        {
            let expected = r#"{"image":"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI","name":"square","roles":["2"]}"#;
            let actual = CreateEmoji::new(
                &client,
                GUILD_ID,
                "square",
                "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI",
            )
            .roles(&[ROLE_ID])
            .try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }
        Ok(())
    }
}
