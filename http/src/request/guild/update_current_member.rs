use crate::{
    client::Client,
    request::{self, AuditLogReason, AuditLogReasonError, Request},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use twilight_model::id::GuildId;
use twilight_validate::misc::{nickname as validate_nickname, ValidationError};

#[derive(Serialize)]
struct UpdateCurrentMemberFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    nick: Option<&'a str>,
}

/// Update the user's member in a guild.
#[must_use = "requests must be configured and executed"]
pub struct UpdateCurrentMember<'a> {
    fields: UpdateCurrentMemberFields<'a>,
    guild_id: GuildId,
    http: &'a Client,
    reason: Option<&'a str>,
}

impl<'a> UpdateCurrentMember<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fields: UpdateCurrentMemberFields { nick: None },
            guild_id,
            http,
            reason: None,
        }
    }

    /// Set the current user's nickname.
    ///
    /// Set to [`None`] to clear the nickname.
    ///
    /// The minimum length is 1 UTF-16 character and the maximum is 32 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`Nickname`] if the nickname length is too
    /// short or too long.
    ///
    /// [`Nickname`]: twilight_validate::misc::ValidationErrorType::Nickname
    pub fn nick(mut self, nick: Option<&'a str>) -> Result<Self, ValidationError> {
        if let Some(nick) = nick {
            validate_nickname(nick)?;
        }

        self.fields.nick = nick;

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let mut request = Request::builder(&Route::UpdateCurrentMember {
            guild_id: self.guild_id.get(),
        });

        request = match request.json(&self.fields) {
            Ok(request) => request,
            Err(source) => return ResponseFuture::error(source),
        };

        if let Some(reason) = &self.reason {
            let header = match request::audit_header(reason) {
                Ok(header) => header,
                Err(source) => return ResponseFuture::error(source),
            };

            request = request.headers(header);
        }

        self.http.request(request.build())
    }
}

impl<'a> AuditLogReason<'a> for UpdateCurrentMember<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}
