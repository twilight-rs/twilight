use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{
    marker::{GuildMarker, IntegrationMarker},
    Id,
};
use twilight_validate::request::{audit_reason as validate_audit_reason, ValidationError};

/// Delete an integration for a guild, by the integration's id.
#[must_use = "requests must be configured and executed"]
pub struct DeleteGuildIntegration<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    integration_id: Id<IntegrationMarker>,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> DeleteGuildIntegration<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        integration_id: Id<IntegrationMarker>,
    ) -> Self {
        Self {
            guild_id,
            http,
            integration_id,
            reason: Ok(None),
        }
    }
}

impl<'a> AuditLogReason<'a> for DeleteGuildIntegration<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for DeleteGuildIntegration<'_> {
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

impl TryIntoRequest for DeleteGuildIntegration<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::DeleteGuildIntegration {
            guild_id: self.guild_id.get(),
            integration_id: self.integration_id.get(),
        });

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
