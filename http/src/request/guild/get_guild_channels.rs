use crate::request::prelude::*;
use twilight_model::{channel::GuildChannel, id::GuildId};

/// Get the channels in a guild.
pub struct GetGuildChannels<'a> {
    fut: Option<Pending<'a, Vec<GuildChannel>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildChannels<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetChannels {
                guild_id: self.guild_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetGuildChannels<'_>, Vec<GuildChannel>);
