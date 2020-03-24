use crate::request::prelude::*;
use twilight_model::id::GuildId;

pub struct LeaveGuild<'a> {
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> LeaveGuild<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from(
            Route::LeaveGuild {
                guild_id: self.guild_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(LeaveGuild<'_>, ());
