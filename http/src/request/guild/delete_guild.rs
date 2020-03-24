use crate::request::prelude::*;
use twilight_model::id::GuildId;

pub struct DeleteGuild<'a> {
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> DeleteGuild<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::DeleteGuild {
                guild_id: self.guild_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(DeleteGuild<'_>, ());
