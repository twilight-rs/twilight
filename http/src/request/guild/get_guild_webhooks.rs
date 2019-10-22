use crate::request::prelude::*;
use dawn_model::{channel::Webhook, id::GuildId};

pub struct GetGuildWebhooks<'a> {
    fut: Option<Pending<'a, Vec<Webhook>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildWebhooks<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetGuildWebhooks {
                guild_id: self.guild_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetGuildWebhooks<'_>, Vec<Webhook>);
