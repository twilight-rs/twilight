use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    guild::MfaLevel,
    id::{Id, marker::GuildMarker},
};
use twilight_validate::request::{ValidationError, audit_reason as validate_audit_reason};

#[derive(Serialize)]
struct UpdateGuildMfaBody {
    level: MfaLevel,
}

struct UpdateGuildMfaPath {
    guild_id: Id<GuildMarker>,
}

/// Update a guild's MFA level.
#[must_use = "requests must be configured and executed"]
pub struct UpdateGuildMfa<'a> {
    body: UpdateGuildMfaBody,
    path: UpdateGuildMfaPath,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> UpdateGuildMfa<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>, level: MfaLevel) -> Self {
        Self {
            body: UpdateGuildMfaBody { level },
            path: UpdateGuildMfaPath { guild_id },
            http,
            reason: Ok(None),
        }
    }
}

impl IntoFuture for UpdateGuildMfa<'_> {
    type Output = Result<Response<MfaLevel>, Error>;

    type IntoFuture = ResponseFuture<MfaLevel>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for UpdateGuildMfa<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl TryIntoRequest for UpdateGuildMfa<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder_from_route(&Route::UpdateGuildMfa {
            guild_id: self.path.guild_id.get(),
        })
        .json(&self.body);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
