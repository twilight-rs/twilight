#[cfg(not(target_os = "wasi"))]
use crate::response::{Response, ResponseFuture, marker::EmptyBody};
use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{GuildMarker, RoleMarker},
};
use twilight_validate::request::{ValidationError, audit_reason as validate_audit_reason};

/// Delete a role in a guild, by id.
#[must_use = "requests must be configured and executed"]
pub struct DeleteRole<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    role_id: Id<RoleMarker>,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> DeleteRole<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        role_id: Id<RoleMarker>,
    ) -> Self {
        Self {
            guild_id,
            http,
            role_id,
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

#[cfg(not(target_os = "wasi"))]
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
        let mut request = Request::builder(&Route::DeleteRole {
            guild_id: self.guild_id.get(),
            role_id: self.role_id.get(),
        });

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
