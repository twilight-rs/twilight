use crate::request::prelude::*;
use dawn_model::{guild::Guild, id::GuildId};

pub struct GetGuild<'a> {
    fut: Option<Pending<'a, Option<Guild>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuild<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetGuild {
                guild_id: self.guild_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetGuild<'_>, Option<Guild>);
