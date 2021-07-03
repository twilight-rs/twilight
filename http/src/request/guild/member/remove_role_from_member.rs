use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, AuditLogReasonError, PendingResponse, Request},
    response::marker::EmptyBody,
    routing::Route,
};
use twilight_model::id::{GuildId, RoleId, UserId};

/// Remove a role from a member in a guild, by id.
pub struct RemoveRoleFromMember<'a> {
    fut: Option<PendingResponse<'a, EmptyBody>>,
    guild_id: GuildId,
    http: &'a Client,
    role_id: RoleId,
    user_id: UserId,
    reason: Option<String>,
}

impl<'a> RemoveRoleFromMember<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: impl Into<GuildId>,
        user_id: impl Into<UserId>,
        role_id: impl Into<RoleId>,
    ) -> Self {
        Self {
            fut: None,
            guild_id: guild_id.into(),
            http,
            role_id: role_id.into(),
            user_id: user_id.into(),
            reason: None,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let mut request = Request::builder(Route::RemoveMemberRole {
            guild_id: self.guild_id.0,
            role_id: self.role_id.0,
            user_id: self.user_id.0,
        });

        if let Some(reason) = self.reason.as_ref() {
            request = request.headers(request::audit_header(reason)?);
        }

        self.fut
            .replace(Box::pin(self.http.request(request.build())));

        Ok(())
    }
}

impl<'a> AuditLogReason for RemoveRoleFromMember<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(RemoveRoleFromMember<'_>, EmptyBody);
