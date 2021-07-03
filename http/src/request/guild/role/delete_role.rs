use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, AuditLogReasonError, PendingResponse, Request},
    response::marker::EmptyBody,
    routing::Route,
};
use twilight_model::id::{GuildId, RoleId};

/// Delete a role in a guild, by id.
pub struct DeleteRole<'a> {
    fut: Option<PendingResponse<'a, EmptyBody>>,
    guild_id: GuildId,
    http: &'a Client,
    role_id: RoleId,
    reason: Option<String>,
}

impl<'a> DeleteRole<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, role_id: RoleId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
            role_id,
            reason: None,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let mut request = Request::builder(Route::DeleteRole {
            guild_id: self.guild_id.0,
            role_id: self.role_id.0,
        });

        if let Some(reason) = &self.reason {
            request = request.headers(request::audit_header(reason)?);
        }

        self.fut
            .replace(Box::pin(self.http.request(request.build())));

        Ok(())
    }
}

impl<'a> AuditLogReason for DeleteRole<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(DeleteRole<'_>, EmptyBody);
