use crate::request::prelude::*;
use twilight_model::id::{GuildId, IntegrationId};

pub struct SyncGuildIntegration<'a> {
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
    integration_id: IntegrationId,
}

impl<'a> SyncGuildIntegration<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, integration_id: IntegrationId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
            integration_id,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::SyncGuildIntegration {
                guild_id: self.guild_id.0,
                integration_id: self.integration_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(SyncGuildIntegration<'_>, ());
