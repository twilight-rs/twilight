use crate::request::prelude::*;
use dawn_model::{
    guild::Emoji,
    id::{EmojiId, GuildId, RoleId},
};

#[derive(Default, Serialize)]
struct UpdateEmojiFields {
    name: Option<String>,
    roles: Option<Vec<RoleId>>,
}

pub struct UpdateEmoji<'a> {
    emoji_id: EmojiId,
    fields: UpdateEmojiFields,
    fut: Option<Pending<'a, Emoji>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> UpdateEmoji<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, emoji_id: EmojiId) -> Self {
        Self {
            fields: UpdateEmojiFields::default(),
            emoji_id,
            fut: None,
            guild_id,
            http,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.fields.name.replace(name.into());

        self
    }

    pub fn roles(mut self, roles: Vec<RoleId>) -> Self {
        self.fields.roles.replace(roles);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(&self.fields)?,
            Route::UpdateEmoji {
                emoji_id: self.emoji_id.0,
                guild_id: self.guild_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateEmoji<'_>, Emoji);
