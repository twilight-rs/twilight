use crate::request::prelude::*;
use dawn_model::id::{GuildId, UserId};

pub struct DeleteBan<'a> {
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
    user_id: UserId,
}

impl<'a> DeleteBan<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, user_id: UserId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
            user_id,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from(
            Route::DeleteBan {
                guild_id: self.guild_id.0,
                user_id: self.user_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(DeleteBan<'_>, ());
