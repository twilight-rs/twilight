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
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(&self.fields)?,
            Route::CreateGuildIntegration {
                guild_id: self.guild_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(CreateGuildIntegration<'_>, ());
