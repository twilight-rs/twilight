use dawn_model::{
    guild::Emoji,
    id::{EmojiId, GuildId, RoleId},
};
use super::prelude::*;

#[derive(Serialize)]
pub struct UpdateEmoji<'a> {
    name: Option<String>,
    roles: Option<Vec<RoleId>>,
    #[serde(skip)]
    emoji_id: EmojiId,
    #[serde(skip)]
    fut: Option<Pin<Box<dyn Future<Output = Result<Emoji>> + Send + 'a>>>,
    #[serde(skip)]
    guild_id: GuildId,
    #[serde(skip)]
    http: &'a Client,
}

impl<'a> UpdateEmoji<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: impl Into<GuildId>,
        emoji_id: impl Into<EmojiId>,
    ) -> Self {
        Self {
            name: None,
            roles: None,
            emoji_id: emoji_id.into(),
            fut: None,
            guild_id: guild_id.into(),
            http,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name.replace(name.into());

        self
    }

    pub fn roles(mut self, roles: Vec<RoleId>) -> Self {
        self.roles.replace(roles);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(self)?,
            Route::UpdateEmoji {
                emoji_id: self.emoji_id.0,
                guild_id: self.guild_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateEmoji<'_>, Emoji);
