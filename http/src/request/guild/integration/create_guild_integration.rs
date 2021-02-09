use crate::request::prelude::*;
use twilight_model::id::{GuildId, IntegrationId};

#[derive(Serialize)]
struct CreateGuildIntegrationFields {
    id: IntegrationId,
    #[serde(rename = "type")]
    kind: String,
}

/// Create a guild integration from the current user to the guild.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/resources/guild#create-guild-integration
pub struct CreateGuildIntegration<'a> {
    fields: CreateGuildIntegrationFields,
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> CreateGuildIntegration<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: GuildId,
        integration_id: IntegrationId,
        kind: impl Into<String>,
    ) -> Self {
        Self {
            fields: CreateGuildIntegrationFields {
                id: integration_id,
                kind: kind.into(),
            },
            fut: None,
            guild_id,
            http,
            reason: None,
        }
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
                headers,
                Route::CreateGuildIntegration {
                    guild_id: self.guild_id.0,
                },
            ))
        } else {
            Request::from((
                crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
                Route::CreateGuildIntegration {
                    guild_id: self.guild_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

impl<'a> AuditLogReason for CreateGuildIntegration<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(CreateGuildIntegration<'_>, ());
