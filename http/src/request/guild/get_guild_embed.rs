use crate::request::prelude::*;
use twilight_model::{guild::GuildEmbed, id::GuildId};

pub struct GetGuildEmbed<'a> {
    fut: Option<Pending<'a, Option<GuildEmbed>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildEmbed<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetGuildEmbed {
                guild_id: self.guild_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetGuildEmbed<'_>, Option<GuildEmbed>);
