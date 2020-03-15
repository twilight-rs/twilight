use crate::request::prelude::*;
use dawn_model::id::{GuildId, IntegrationId};

#[derive(Serialize)]
struct CreateGuildIntegrationFields {
    id: IntegrationId,
    #[serde(rename = "type")]
    kind: String,
}

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

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                serde_json::to_vec(&self.fields)?,
                headers,
                Route::CreateGuildIntegration {
                    guild_id: self.guild_id.0,
                },
            ))
        } else {
            Request::from((
                serde_json::to_vec(&self.fields)?,
                Route::CreateGuildIntegration {
                    guild_id: self.guild_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(CreateGuildIntegration<'_>, ());
