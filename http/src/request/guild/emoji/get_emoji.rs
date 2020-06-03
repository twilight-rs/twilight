use crate::request::prelude::*;
use twilight_model::{
    guild::Emoji,
    id::{EmojiId, GuildId},
};

pub struct GetEmoji<'a> {
    emoji_id: EmojiId,
    fut: Option<PendingOption<'a>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetEmoji<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, emoji_id: EmojiId) -> Self {
        Self {
            emoji_id,
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut
            .replace(Box::pin(self.http.request_bytes(Request::from(
                Route::GetEmoji {
                    emoji_id: self.emoji_id.0,
                    guild_id: self.guild_id.0,
                },
            ))));

        Ok(())
    }
}

poll_req!(opt, GetEmoji<'_>, Emoji);
