use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::ListBody},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    guild::{Role, RolePosition},
    id::{Id, marker::GuildMarker},
};
use twilight_validate::request::{ValidationError, audit_reason as validate_audit_reason};

#[derive(Serialize)]
#[serde(transparent)]
struct UpdateRolePositionsBody<'a>(&'a [RolePosition]);

struct UpdateRolePositionsPath {
    guild_id: Id<GuildMarker>,
}

/// Modify the position of the roles.
///
/// The minimum amount of roles to modify, is a swap between two roles.
#[must_use = "requests must be configured and executed"]
pub struct UpdateRolePositions<'a> {
    body: UpdateRolePositionsBody<'a>,
    http: &'a Client,
    path: UpdateRolePositionsPath,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> UpdateRolePositions<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        roles: &'a [RolePosition],
    ) -> Self {
        Self {
            body: UpdateRolePositionsBody(roles),
            http,
            path: UpdateRolePositionsPath { guild_id },
            reason: Ok(None),
        }
    }
}

impl<'a> AuditLogReason<'a> for UpdateRolePositions<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for UpdateRolePositions<'_> {
    type Output = Result<Response<ListBody<Role>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Role>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateRolePositions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdateRolePositions {
            guild_id: self.path.guild_id.get(),
        })
        .json(&self.body);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
