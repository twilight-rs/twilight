use crate::request::prelude::*;
use twilight_model::id::{GuildId, RoleId, UserId};

/// Remove a role from a member in a guild, by id.
pub struct RemoveRoleFromMember<'a> {
    fut: Option<Pending<'a, ()>>,
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

    #[deprecated(note = "you've used the request's reason method which is deprecated; \
                please import the request::AuditLogReason trait")]
    /// Attach an audit log reason to this request.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                headers,
                Route::RemoveMemberRole {
                    guild_id: self.guild_id.0,
                    role_id: self.role_id.0,
                    user_id: self.user_id.0,
                },
            ))
        } else {
            Request::from(Route::RemoveMemberRole {
                guild_id: self.guild_id.0,
                role_id: self.role_id.0,
                user_id: self.user_id.0,
            })
        };

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

impl<'a> AuditLogReason for RemoveRoleFromMember<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        let reason = AuditLogReasonError::validate(reason.into())?;
        self.reason.replace(reason);

        Ok(self)
    }
}

poll_req!(RemoveRoleFromMember<'_>, ());
