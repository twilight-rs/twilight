use crate::request::prelude::*;
use twilight_model::id::{GuildId, RoleId};

/// Delete a role in a guild, by id.
pub struct DeleteRole<'a> {
    fut: Option<Pending<'a, ()>>,
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
                Route::DeleteRole {
                    guild_id: self.guild_id.0,
                    role_id: self.role_id.0,
                },
            ))
        } else {
            Request::from(Route::DeleteRole {
                guild_id: self.guild_id.0,
                role_id: self.role_id.0,
            })
        };

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

impl<'a> AuditLogReason for DeleteRole<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        let reason = AuditLogReasonError::validate(reason.into())?;
        self.reason.replace(reason);

        Ok(self)
    }
}

poll_req!(DeleteRole<'_>, ());
