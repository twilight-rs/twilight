use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::EmptyBody},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{GuildMarker, RoleMarker},
};
use twilight_validate::request::{ValidationError, audit_reason as validate_audit_reason};

struct DeleteRolePath {
    guild_id: Id<GuildMarker>,
    role_id: Id<RoleMarker>,
}

/// Delete a role in a guild, by id.
#[must_use = "requests must be configured and executed"]
pub struct DeleteRole<'a> {
    http: &'a Client,
    path: DeleteRolePath,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> DeleteRole<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        role_id: Id<RoleMarker>,
    ) -> Self {
        Self {
            http,
            path: DeleteRolePath { guild_id, role_id },
            reason: Ok(None),
        }
    }
}

impl<'a> AuditLogReason<'a> for DeleteRole<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for DeleteRole<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for DeleteRole<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder_from_route(&Route::DeleteRole {
            guild_id: self.path.guild_id.get(),
            role_id: self.path.role_id.get(),
        });

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
