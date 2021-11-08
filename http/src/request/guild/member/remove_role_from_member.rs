use crate::{
    client::Client,
    request::{self, AuditLogReason, AuditLogReasonError, Request},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{
    marker::{GuildMarker, RoleMarker, UserMarker},
    Id,
};

/// Remove a role from a member in a guild, by id.
#[must_use = "requests must be configured and executed"]
pub struct RemoveRoleFromMember<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    role_id: Id<RoleMarker>,
    user_id: Id<UserMarker>,
    reason: Option<&'a str>,
}

impl<'a> RemoveRoleFromMember<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
        role_id: Id<RoleMarker>,
    ) -> Self {
        Self {
            guild_id,
            http,
            role_id,
            user_id,
            reason: None,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let mut request = Request::builder(&Route::RemoveMemberRole {
            guild_id: self.guild_id.get(),
            role_id: self.role_id.get(),
            user_id: self.user_id.get(),
        });

        if let Some(reason) = self.reason {
            let header = match request::audit_header(reason) {
                Ok(header) => header,
                Err(source) => return ResponseFuture::error(source),
            };

            request = request.headers(header);
        }

        self.http.request(request.build())
    }
}

impl<'a> AuditLogReason<'a> for RemoveRoleFromMember<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}
