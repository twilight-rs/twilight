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
        marker::{EmojiMarker, GuildMarker, RoleMarker},
        Id,
    },
};
use twilight_validate::request::{audit_reason as validate_audit_reason, ValidationError};

#[derive(Serialize)]
struct UpdateEmojiFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<&'a [Id<RoleMarker>]>,
}

/// Update an emoji in a guild, by id.
#[must_use = "requests must be configured and executed"]
pub struct UpdateEmoji<'a> {
    emoji_id: Id<EmojiMarker>,
    fields: UpdateEmojiFields<'a>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> UpdateEmoji<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        emoji_id: Id<EmojiMarker>,
    ) -> Self {
        Self {
            fields: UpdateEmojiFields {
                name: None,
                roles: None,
            },
            emoji_id,
            guild_id,
            http,
            reason: Ok(None),
        }
    }

    /// Change the name of the emoji.
    pub const fn name(mut self, name: &'a str) -> Self {
        self.fields.name = Some(name);

        self
    }

    /// Change the roles that the emoji is whitelisted to.
    pub const fn roles(mut self, roles: &'a [Id<RoleMarker>]) -> Self {
        self.fields.roles = Some(roles);

        self
    }
}

impl<'a> AuditLogReason<'a> for UpdateEmoji<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for UpdateEmoji<'_> {
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

impl TryIntoRequest for UpdateEmoji<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdateEmoji {
            emoji_id: self.emoji_id.get(),
            guild_id: self.guild_id.get(),
        });

        request = request.json(&self.fields);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
