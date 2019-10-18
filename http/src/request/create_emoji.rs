use super::prelude::*;
use dawn_model::{
    guild::Emoji,
    id::{GuildId, RoleId},
};

#[derive(Serialize)]
struct CreateEmojiFields {
    image: String,
    name: String,
    roles: Option<Vec<RoleId>>,
}

pub struct CreateEmoji<'a> {
    fut: Option<Pending<'a, Emoji>>,
    fields: CreateEmojiFields,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> CreateEmoji<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: impl Into<GuildId>,
        name: impl Into<String>,
        image: impl Into<String>,
    ) -> Self {
        Self {
            fields: CreateEmojiFields {
                image: image.into(),
                name: name.into(),
                roles: None,
            },
            fut: None,
            guild_id: guild_id.into(),
            http,
        }
    }

    pub fn roles(mut self, roles: Vec<RoleId>) -> Self {
        self.fields.roles.replace(roles);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(&self.fields)?,
            Route::CreateEmoji {
                guild_id: self.guild_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(CreateEmoji<'_>, Emoji);
