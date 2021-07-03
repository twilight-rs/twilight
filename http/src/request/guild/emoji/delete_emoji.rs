use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, AuditLogReasonError, PendingResponse, Request},
    response::marker::EmptyBody,
    routing::Route,
};
use twilight_model::id::{EmojiId, GuildId};

/// Delete an emoji in a guild, by id.
pub struct DeleteEmoji<'a> {
    emoji_id: EmojiId,
    fut: Option<PendingResponse<'a, EmptyBody>>,
    guild_id: GuildId,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> DeleteEmoji<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, emoji_id: EmojiId) -> Self {
        Self {
            emoji_id,
            fut: None,
            guild_id,
            http,
            reason: None,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let mut request = Request::builder(Route::DeleteEmoji {
            emoji_id: self.emoji_id.0,
            guild_id: self.guild_id.0,
        });

        if let Some(reason) = self.reason.as_ref() {
            request = request.headers(request::audit_header(reason)?);
        }

        self.fut
            .replace(Box::pin(self.http.request(request.build())));

        Ok(())
    }
}

impl<'a> AuditLogReason for DeleteEmoji<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(DeleteEmoji<'_>, EmptyBody);
