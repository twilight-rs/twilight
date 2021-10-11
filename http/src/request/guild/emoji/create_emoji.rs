use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, AuditLogReasonError, IntoRequest, Request},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    guild::Emoji,
    id::{GuildId, RoleId},
};

#[derive(Serialize)]
struct CreateEmojiFields<'a> {
    image: &'a str,
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<&'a [RoleId]>,
}

/// Create an emoji in a guild.
///
/// The emoji must be a Data URI, in the form of `data:image/{type};base64,{data}` where `{type}`
/// is the image MIME type and `{data}` is the base64-encoded image.  Refer to [the discord docs]
/// for more information about image data.
///
/// [the discord docs]: https://discord.com/developers/docs/reference#image-data
#[must_use = "requests must be configured and executed"]
pub struct CreateEmoji<'a> {
    fields: CreateEmojiFields<'a>,
    guild_id: GuildId,
    http: &'a Client,
    reason: Option<&'a str>,
}

impl<'a> CreateEmoji<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: GuildId,
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
            reason: None,
        }
    }

    /// Whitelist roles for this emoji.
    ///
    /// Refer to [the discord docs] for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/emoji
    pub const fn roles(mut self, roles: &'a [RoleId]) -> Self {
        self.fields.roles = Some(roles);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Emoji> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for CreateEmoji<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl IntoRequest for CreateEmoji<'_> {
    fn into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::CreateEmoji {
            guild_id: self.guild_id.get(),
        });

        request = request.json(&self.fields)?;

        if let Some(reason) = self.reason.as_ref() {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}
