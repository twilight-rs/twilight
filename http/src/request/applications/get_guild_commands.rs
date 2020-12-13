use crate::request::prelude::*;
use twilight_model::applications::Command;
use twilight_model::id::{ApplicationId, GuildId};

pub struct GetGuildCommands<'a> {
    application_id: ApplicationId,
    guild_id: GuildId,
    fut: Option<Pending<'a, Vec<Command>>>,
    http: &'a Client,
}

impl<'a> GetGuildCommands<'a> {
    pub(crate) fn new(http: &'a Client, application_id: ApplicationId, guild_id: GuildId) -> Self {
        Self {
            application_id,
            guild_id,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        let req = Request::from(Route::GetGuildCommands {
            application_id: self.application_id.0,
            guild_id: self.guild_id.0,
        });
        self.fut.replace(Box::pin(self.http.request(req)));

        Ok(())
    }
}

poll_req!(GetGuildCommands<'_>, Vec<Command>);
