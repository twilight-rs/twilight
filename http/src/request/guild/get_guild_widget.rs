use crate::request::prelude::*;
use twilight_model::{guild::GuildWidget, id::GuildId};

pub struct GetGuildWidget<'a> {
    fut: Option<PendingOption<'a>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildWidget<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut
            .replace(Box::pin(self.http.request_bytes(Request::from(
                Route::GetGuildWidget {
                    guild_id: self.guild_id.0,
                },
            ))));

        Ok(())
    }
}

poll_req!(opt, GetGuildWidget<'_>, GuildWidget);
