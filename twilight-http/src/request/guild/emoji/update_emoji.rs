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
        Id,
        marker::{EmojiMarker, GuildMarker, RoleMarker},
    },
};
use twilight_validate::request::{ValidationError, audit_reason as validate_audit_reason};

#[derive(Serialize)]
struct UpdateEmojiBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<&'a [Id<RoleMarker>]>,
}

struct UpdateEmojiPath {
    emoji_id: Id<EmojiMarker>,
    guild_id: Id<GuildMarker>,
}

/// Update an emoji in a guild, by id.
#[must_use = "requests must be configured and executed"]
pub struct UpdateEmoji<'a> {
    body: UpdateEmojiBody<'a>,
    http: &'a Client,
    path: UpdateEmojiPath,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> UpdateEmoji<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        emoji_id: Id<EmojiMarker>,
    ) -> Self {
        Self {
            body: UpdateEmojiBody {
                name: None,
                roles: None,
            },
            http,
            path: UpdateEmojiPath { emoji_id, guild_id },
            reason: Ok(None),
        }
    }

    /// Change the name of the emoji.
    pub const fn name(mut self, name: &'a str) -> Self {
        self.body.name = Some(name);

        self
    }

    /// Change the roles that the emoji is whitelisted to.
    pub const fn roles(mut self, roles: &'a [Id<RoleMarker>]) -> Self {
        self.body.roles = Some(roles);

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
            emoji_id: self.path.emoji_id.get(),
            guild_id: self.path.guild_id.get(),
        });

        request = request.json(&self.body);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
