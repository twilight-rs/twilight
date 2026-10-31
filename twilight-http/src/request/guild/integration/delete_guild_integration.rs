use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{GuildMarker, IntegrationMarker},
};
use twilight_validate::request::{ValidationError, audit_reason as validate_audit_reason};

struct DeleteGuildIntegrationPath {
    guild_id: Id<GuildMarker>,
    integration_id: Id<IntegrationMarker>,
}

/// Delete an integration for a guild, by the integration's id.
#[must_use = "requests must be configured and executed"]
pub struct DeleteGuildIntegration<'a> {
    http: &'a Client,
    path: DeleteGuildIntegrationPath,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> DeleteGuildIntegration<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        integration_id: Id<IntegrationMarker>,
    ) -> Self {
        Self {
            http,
            path: DeleteGuildIntegrationPath {
                guild_id,
                integration_id,
            },
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
        let DeleteGuildIntegrationPath {
            guild_id,
            integration_id,
        } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("integrations")
            .id(integration_id)
            .build()
            .map_err(Error::path)?;

        let mut request = Request::builder(Method::Delete, path);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
