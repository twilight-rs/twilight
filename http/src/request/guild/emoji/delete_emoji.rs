use crate::request::prelude::*;
use dawn_model::id::{EmojiId, GuildId};

pub struct DeleteEmoji<'a> {
    emoji_id: EmojiId,
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> DeleteEmoji<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, emoji_id: EmojiId) -> Self {
        Self {
            emoji_id,
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::DeleteEmoji {
                emoji_id: self.emoji_id.0,
                guild_id: self.guild_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(DeleteEmoji<'_>, ());
