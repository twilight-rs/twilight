use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, AuditLogReasonError, IntoRequest, Request},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{GuildId, RoleId, UserId};

/// Remove a role from a member in a guild, by id.
#[must_use = "requests must be configured and executed"]
pub struct RemoveRoleFromMember<'a> {
    guild_id: GuildId,
    http: &'a Client,
    role_id: RoleId,
    user_id: UserId,
    reason: Option<&'a str>,
}

impl<'a> RemoveRoleFromMember<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: GuildId,
        user_id: UserId,
        role_id: RoleId,
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
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for RemoveRoleFromMember<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl IntoRequest for RemoveRoleFromMember<'_> {
    fn into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::RemoveMemberRole {
            guild_id: self.guild_id.get(),
            role_id: self.role_id.get(),
            user_id: self.user_id.get(),
        });

        if let Some(reason) = self.reason {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}
