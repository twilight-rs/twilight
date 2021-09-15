use crate::{
    client::Client,
    request::{self, AuditLogReason, AuditLogReasonError, IntoRequest, Request},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{GuildId, IntegrationId};

/// Delete an integration for a guild, by the integration's id.
#[must_use = "requests must be configured and executed"]
pub struct DeleteGuildIntegration<'a> {
    guild_id: GuildId,
    http: &'a Client,
    integration_id: IntegrationId,
    reason: Option<&'a str>,
}

impl<'a> DeleteGuildIntegration<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: GuildId,
        integration_id: IntegrationId,
    ) -> Self {
        Self {
            guild_id,
            http,
            integration_id,
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

impl<'a> AuditLogReason<'a> for DeleteGuildIntegration<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl IntoRequest for DeleteGuildIntegration<'_> {
    fn into_request(self) -> Result<Request, crate::Error> {
        let mut request = Request::builder(&Route::DeleteGuildIntegration {
            guild_id: self.guild_id.get(),
            integration_id: self.integration_id.get(),
        });

        if let Some(reason) = self.reason.as_ref() {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}
