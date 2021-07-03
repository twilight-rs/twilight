use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, AuditLogReasonError, PendingResponse, Request},
    response::marker::EmptyBody,
    routing::Route,
};
use twilight_model::id::{GuildId, IntegrationId};

/// Delete an integration for a guild, by the integration's id.
pub struct DeleteGuildIntegration<'a> {
    fut: Option<PendingResponse<'a, EmptyBody>>,
    guild_id: GuildId,
    http: &'a Client,
    integration_id: IntegrationId,
    reason: Option<String>,
}

impl<'a> DeleteGuildIntegration<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, integration_id: IntegrationId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
            integration_id,
            reason: None,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let mut request = Request::builder(Route::DeleteGuildIntegration {
            guild_id: self.guild_id.0,
            integration_id: self.integration_id.0,
        });

        if let Some(reason) = self.reason.as_ref() {
            request = request.headers(request::audit_header(reason)?);
        }

        self.fut
            .replace(Box::pin(self.http.request(request.build())));

        Ok(())
    }
}

impl<'a> AuditLogReason for DeleteGuildIntegration<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(DeleteGuildIntegration<'_>, EmptyBody);
