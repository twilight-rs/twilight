use crate::request::prelude::*;
use dawn_model::{guild::GuildPreview, id::GuildId};

pub struct GetGuildPreview<'a> {
    fut: Option<Pending<'a, GuildPreview>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildPreview<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetGuildPreview {
                guild_id: self.guild_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetGuildPreview<'_>, GuildPreview);
